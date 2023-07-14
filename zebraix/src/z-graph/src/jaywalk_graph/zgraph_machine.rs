// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::jaywalk_graph::zgraph_base::PortDataVec;
use crate::jaywalk_graph::zgraph_base::PortPieceTyped;
use crate::jaywalk_graph::zgraph_base::ZGraphError;
use crate::jaywalk_graph::zgraph_base::ZMachineTypestate;
use crate::jaywalk_graph::zgraph_base::ZNodeStateData;
use crate::jaywalk_graph::zgraph_base::ZNodeTypeFinder;
use crate::jaywalk_graph::zgraph_base::ZPiece;
use crate::jaywalk_graph::zgraph_base::ZPieceType;
use crate::jaywalk_graph::zgraph_base::ZRendererData;
use crate::jaywalk_graph::zgraph_graphdef::ZGraphDef;
use crate::jaywalk_graph::zgraph_graphdef::ZNodeDef;
use crate::jaywalk_graph::zgraph_graphdef::ZPortDef;
use crate::jaywalk_graph::zgraph_registry::ZNodeCategory;
use crate::jaywalk_graph::zgraph_registry::ZNodeRegistration;
use crate::jaywalk_graph::zgraph_registry::ZRegistry;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub enum DebugLine {
   SimpleString(String),
}

// Floatable version of edge copier
pub struct PortDataCopier {
   pub src_node: Rc<RefCell<ZNode>>,
   pub src_port_data: PortDataVec,
   pub src_index: usize,
   pub dest_port_data: PortDataVec,
   pub dest_index: usize,
   pub port_def: Option<ZPortDef>,
   // #[derive(Clone, Serialize, Deserialize)]
   // pub struct ZPortDef(pub String, pub String, pub String);
}

impl PortDataCopier {
   const VOID_SENTINEL: usize = usize::MAX;
}

#[allow(dead_code)]
pub struct ZNode {
   pub name: String,
   pub node_type: Rc<ZNodeRegistration>,
   pub node_type_finder: Option<ZNodeTypeFinder>,
   pub node_state_data: ZNodeStateData,

   pub data_copiers_src_copy: Vec<Rc<RefCell<PortDataCopier>>>,
   pub data_copiers_dest_copy: Vec<Rc<RefCell<PortDataCopier>>>,
   pub data_ports_src_copy: PortDataVec,
   pub data_ports_dest_copy: PortDataVec,

   pub subgraph_nodes: Vec<Rc<RefCell<ZNode>>>, // Prolly init with 0 reserve.
   pub subgraph_node_map: Option<HashMap<String, usize>>,

   pub is_active: bool,
}

pub struct ZMachine {
   pub typestate: ZMachineTypestate,
   pub has_graph_def: bool,

   pub graph_def: ZGraphDef,
   // // Mirroring ZGraphDef.
   // pub category: ZGraphDefCategory,
   // pub name: String,
   // pub description: Option<String>,
   // pub nodes: JVec<ZNodeDef>,
   // pub edges: JVec<ZEdgeDef>,
   pub renderer_data: ZRendererData,

   pub realized_node: Rc<RefCell<ZNode>>,

   // Node registry
   pub registry: ZRegistry,

   pub null_node: Rc<RefCell<ZNode>>,

   pub floating_port_data: PortDataVec,
}

impl ZMachine {
   pub fn new() -> Self {
      let registry = ZRegistry::default();
      let null_node_type = registry.get_null_noderegistration().clone();
      let subgraph_node_type = registry.get_subgraph_noderegistration().clone();

      let null_node = Rc::new(RefCell::new(ZNode {
         name: "Null node".to_string(),
         node_state_data: None,
         node_type: null_node_type,
         node_type_finder: None,
         data_copiers_src_copy: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
         data_copiers_dest_copy: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
         data_ports_src_copy: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
         data_ports_dest_copy: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
         subgraph_nodes: Vec::<Rc<RefCell<ZNode>>>::default(),
         subgraph_node_map: None,
         is_active: false,
      }));

      let realized_node = Rc::new(RefCell::new(ZNode {
         name: "User graph".to_string(),
         node_state_data: None,
         node_type: subgraph_node_type,
         node_type_finder: None,
         data_copiers_src_copy: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
         data_copiers_dest_copy: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
         data_ports_src_copy: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
         data_ports_dest_copy: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
         subgraph_nodes: Vec::<Rc<RefCell<ZNode>>>::default(),
         subgraph_node_map: None,
         is_active: false,
      }));

      // floating_port_data is a sentinel, used to indicate that a
      // connection is floating.
      let floating_port_data = Rc::new(RefCell::new(Vec::<ZPiece>::default()));

      Self {
         typestate: ZMachineTypestate::Init,
         has_graph_def: false,

         graph_def: ZGraphDef::default(),

         // Renderer.
         renderer_data: None,

         // Node registry
         registry,
         null_node,
         floating_port_data,
         realized_node,
      }
   }

   // User-level graphdef for overall drawing.
   pub fn provide_graph_def(&mut self, graph_def: ZGraphDef) -> Result<(), ZGraphError> {
      if self.has_graph_def {
         return Err(ZGraphError::DuplicateGraphDef);
      }

      self.graph_def = graph_def;
      self.has_graph_def = true;
      Ok(())
   }

   pub fn transition_to_deffed(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Init {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }
      if !self.has_graph_def {
         return Err(ZGraphError::MissingGraphDef);
      }

      //

      self.build_from_graphdef().unwrap();

      //

      self.typestate = ZMachineTypestate::Deffed;
      Ok(())
   }

   pub fn transition_to_constructed(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Deffed {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }
      if self.renderer_data.is_none() {
         return Err(ZGraphError::RendererForConstruction);
      }

      let realized_node: &mut ZNode = &mut self.realized_node.borrow_mut();
      assert!(realized_node.is_active);
      realized_node.run_constructors(&mut self.renderer_data)?;

      self.typestate = ZMachineTypestate::Constructed;
      Ok(())
   }

   pub fn transition_to_calculated(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Constructed {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }

      let realized_node: &mut ZNode = &mut self.realized_node.borrow_mut();
      assert!(realized_node.is_active);
      // In the outer user graph, the copiers copy inputs.
      realized_node.run_src_copiers().unwrap();
      realized_node.run_calculators(&mut self.renderer_data)?;

      self.typestate = ZMachineTypestate::Calculated;
      Ok(())
   }

   pub fn transition_to_inked(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Calculated {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }

      let realized_node: &mut ZNode = &mut self.realized_node.borrow_mut();
      assert!(realized_node.is_active);
      realized_node.run_inkings(&mut self.renderer_data)?;

      self.typestate = ZMachineTypestate::Inked;
      Ok(())
   }

   pub fn transition_to_finished(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Inked {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }
      if self.renderer_data.is_some() {
         return Err(ZGraphError::UnfinishedRenderer);
      }
      self.typestate = ZMachineTypestate::Finished;
      Ok(())
   }

   fn build_from_graphdef(&mut self) -> Result<(), ZGraphError> {
      {
         let realized_node: &mut ZNode = &mut self.realized_node.borrow_mut();

         // In ordinary mode of operation, for a user graph, we create one copier for each output.
         assert!(realized_node.data_copiers_dest_copy.is_empty());
         for port_def in &self.graph_def.output_ports {
            let expanded_graph_data_copier = Rc::new(RefCell::new(PortDataCopier {
               src_node: self.null_node.clone(),
               src_port_data: self.floating_port_data.clone(),
               src_index: PortDataCopier::VOID_SENTINEL,
               dest_port_data: realized_node.data_ports_dest_copy.clone(),
               dest_index: PortDataCopier::VOID_SENTINEL,
               port_def: Some(port_def.clone()),
            }));
            realized_node.data_copiers_dest_copy.push(expanded_graph_data_copier.clone());
            // The "src copiers" are a temporary parking spot.
            realized_node.data_copiers_src_copy.push(expanded_graph_data_copier);
            realized_node.is_active = true;
            eprintln!("Port attachment: {}", port_def.0);
         }
      }

      {
         let realized_node: &mut ZNode = &mut self.realized_node.borrow_mut();

         // realized_node.data_copiers_dest_copy populated before.
         //
         // realized_node.data_copiers_src_copy populated by builder.
         realized_node.build_subgraph_from_graphdef(
            &self.graph_def,
            &self.registry,
            &self.null_node,
            &self.floating_port_data,
         )?;
      }

      {
         let realized_node: &mut ZNode = &mut self.realized_node.borrow_mut();

         // Create custom type "registration" with source port typing.
         //
         let subnode_type: &ZNodeRegistration = realized_node.node_type.as_ref();
         let mut replacement_registration: ZNodeRegistration = subnode_type.clone();
         for input in &self.graph_def.inputs {
            replacement_registration.ports_src_copy.push(input.clone());
         }
         realized_node.node_type = Rc::new(replacement_registration);
      }

      {
         // XXX Is this needed?

         let realized_node: &ZNode = &self.realized_node.borrow();
         for copier in &realized_node.data_copiers_src_copy {
            let new_src_node = &mut copier.borrow_mut();
            assert!(Rc::ptr_eq(&new_src_node.src_node, &self.null_node));
            new_src_node.src_node = self.realized_node.clone();
         }
      }

      let mut unsourced_copiers = Vec::<Rc<RefCell<PortDataCopier>>>::default();
      ZNode::create_src_data_for_node(
         &self.realized_node,
         &mut unsourced_copiers,
         &self.null_node,
      )?;

      {
         // Recreate custom type "registration" with destination port typing.

         // This is not intended to be fast.
         let realized_node: &mut ZNode = &mut self.realized_node.borrow_mut();

         let subnode_type: &ZNodeRegistration = realized_node.node_type.as_ref();
         let mut replacement_registration: ZNodeRegistration = subnode_type.clone();

         replacement_registration.ports_dest_copy.reserve(self.graph_def.output_ports.len());

         assert_eq!(self.graph_def.output_ports.len(), realized_node.data_copiers_dest_copy.len());

         for (i, wrapped_copier) in realized_node.data_copiers_dest_copy.iter().enumerate() {
            let port_def = &self.graph_def.output_ports[i];
            let copier: &mut PortDataCopier = &mut wrapped_copier.borrow_mut();
            assert_eq!(port_def.0, copier.port_def.as_ref().unwrap().0);

            let piece_type = if copier.src_index == PortDataCopier::VOID_SENTINEL {
               ZPieceType::Void
            } else if Rc::ptr_eq(&copier.src_node, &self.realized_node) {
               let src_node: &ZNode = realized_node;
               ZPieceType::get_piece_type_from_data(
                  &src_node.data_ports_src_copy.borrow()[copier.src_index],
               )
            } else {
               let wrapped_src_node = copier.src_node.borrow();
               let src_node: &ZNode = &wrapped_src_node;
               let x = ZPieceType::get_piece_type_from_data(
                  &src_node.data_ports_src_copy.borrow()[copier.src_index],
               );
               x
            };

            replacement_registration
               .ports_dest_copy
               .push(PortPieceTyped(port_def.0.clone(), piece_type));
         }
         realized_node.node_type = Rc::new(replacement_registration);
      }

      ZNode::create_dest_data_for_node(
         &self.realized_node,
         &mut unsourced_copiers,
         &self.null_node,
      )?;

      Ok(())
   }
}

impl Default for ZMachine {
   fn default() -> Self {
      Self::new()
   }
}

#[inline(always)]
fn is_leaf_node(subnode_type: &Rc<ZNodeRegistration>) -> bool {
   subnode_type.category != ZNodeCategory::Subgraph
}

impl ZNode {
   fn finish_leaf_node(
      &mut self,
      node_num: usize,
      n_def: &ZNodeDef,
      _registry: &ZRegistry,
      null_node: &Rc<RefCell<ZNode>>,
      floating_port_data: &PortDataVec,
   ) -> Result<(), ZGraphError> {
      let subnode: &mut ZNode = &mut self.subgraph_nodes[node_num].borrow_mut();

      assert!(subnode.data_copiers_dest_copy.is_empty());
      subnode.data_copiers_dest_copy.clear();

      for edge in &n_def.edges {
         eprintln!("   Adding subnode dependency on: {}", edge.src_node);
         let is_internal_src_node: bool = edge.src_node != "input";
         let src_node_znode: &Rc<RefCell<ZNode>> = if is_internal_src_node {
            let node_num: usize =
               *self.subgraph_node_map.as_ref().unwrap().get(&edge.src_node).unwrap();
            &self.subgraph_nodes[node_num]
         } else {
            eprintln!("      !! Edge source is not internal");
            null_node
         };

         for connection in &edge.connections {
            // Name among source nodes's outputs, name among dest node's inputs.
            eprintln!(
               "      Adding data copier: {}:{} to {}:{}",
               edge.src_node, connection.0, n_def.name, connection.1
            );

            // Only leaf nodes actually create copiers (plus
            // the very outer user-graph realization).
            let edges_copier = Rc::new(RefCell::new(PortDataCopier {
               src_node: src_node_znode.clone(),
               src_port_data: floating_port_data.clone(),
               src_index: PortDataCopier::VOID_SENTINEL,
               dest_port_data: floating_port_data.clone(),
               dest_index: PortDataCopier::VOID_SENTINEL,
               port_def: Some(ZPortDef(
                  connection.1.clone(),
                  edge.src_node.clone(),
                  connection.0.clone(),
               )),
            }));
            subnode.data_copiers_dest_copy.push(edges_copier.clone());

            if is_internal_src_node {
               let mut src_node_znode_bmut = src_node_znode.borrow_mut();
               src_node_znode_bmut.data_copiers_src_copy.push(edges_copier);
               src_node_znode_bmut.is_active = true;
            } else {
               self.data_copiers_src_copy.push(edges_copier);
               self.is_active = true;
            }
         }
      }
      Ok(())
   }

   fn create_src_data_for_node(
      wrapped_subnode: &Rc<RefCell<ZNode>>,
      unsourced_copiers: &mut Vec<Rc<RefCell<PortDataCopier>>>,
      null_node: &Rc<RefCell<ZNode>>,
   ) -> Result<(), ZGraphError> {
      let subnode: &mut ZNode = &mut wrapped_subnode.borrow_mut();
      let subnode_type: &ZNodeRegistration = subnode.node_type.as_ref();

      assert_eq!(subnode.is_active, !subnode.data_copiers_src_copy.is_empty());
      if subnode.data_copiers_src_copy.is_empty() {
         return Ok(());
      }
      {
         let src_ports_types: &Vec<PortPieceTyped> = &subnode_type.ports_src_copy;
         let src_data_copiers: &mut Vec<Rc<RefCell<PortDataCopier>>> =
            &mut subnode.data_copiers_src_copy;

         {
            let src_ports_data: &mut Vec<ZPiece> = &mut subnode.data_ports_src_copy.borrow_mut();
            assert!(src_ports_data.is_empty());
            src_ports_data.clear();
            src_ports_data.reserve(src_ports_types.len());
            for port_type in src_ports_types {
               src_ports_data.push(ZPiece::piece_data_default_for_piece_type(&port_type.1));
            }
         }

         let mut port_names = Vec::<&String>::default();
         let mut port_name_map = HashMap::<String, usize>::default();
         port_names.reserve(src_ports_types.len());

         for (i, entry_type) in src_ports_types.iter().enumerate() {
            port_names.push(&entry_type.0);
            port_name_map.insert(entry_type.0.clone(), i);
         }
         for wrapped_copier in src_data_copiers {
            let mut copier: &mut PortDataCopier = &mut wrapped_copier.borrow_mut();

            if Rc::ptr_eq(&copier.src_node, null_node) {
               unsourced_copiers.push(wrapped_copier.clone());
               continue;
            }

            assert!(Rc::ptr_eq(wrapped_subnode, &copier.src_node));
            copier.src_port_data = subnode.data_ports_src_copy.clone();
            let port_name: &String = &copier.port_def.as_ref().unwrap().2;

            if port_name == "void" {
               copier.src_index = PortDataCopier::VOID_SENTINEL;
            } else {
               let gotten = port_name_map.get(port_name);
               assert!(
                  gotten.is_some(),
                  "Unable to find port \"{}\" in src node named \"{}\" for destination \"{}\"",
                  &port_name,
                  subnode.name,
                  "Unknown"
               );
               copier.src_index = *gotten.unwrap();
            }
         }
      }
      Ok(())
   }

   fn create_dest_data_for_node(
      wrapped_subnode: &Rc<RefCell<ZNode>>,
      unsourced_copiers: &mut Vec<Rc<RefCell<PortDataCopier>>>,
      null_node: &Rc<RefCell<ZNode>>,
   ) -> Result<(), ZGraphError> {
      let subnode: &mut ZNode = &mut wrapped_subnode.borrow_mut();
      let subnode_type: &ZNodeRegistration = subnode.node_type.as_ref();

      if subnode.data_copiers_src_copy.is_empty() {
         return Ok(());
      }
      {
         let dest_ports_types: &Vec<PortPieceTyped> = &subnode_type.ports_dest_copy;
         let dest_data_copiers: &mut Vec<Rc<RefCell<PortDataCopier>>> =
            &mut subnode.data_copiers_dest_copy;
         {
            let dest_ports_data: &mut Vec<ZPiece> = &mut subnode.data_ports_dest_copy.borrow_mut();
            assert!(dest_ports_data.is_empty());
            dest_ports_data.clear();
            dest_ports_data.reserve(dest_ports_types.len());
            for port_type in dest_ports_types {
               dest_ports_data.push(ZPiece::piece_data_default_for_piece_type(&port_type.1));
            }
         }

         let mut port_names = Vec::<&String>::default();
         let mut port_name_map = HashMap::<String, usize>::default();
         port_names.reserve(dest_ports_types.len());

         for (i, entry_type) in dest_ports_types.iter().enumerate() {
            port_names.push(&entry_type.0);
            port_name_map.insert(entry_type.0.clone(), i);
         }
         for wrapped_copier in dest_data_copiers {
            let mut copier: &mut PortDataCopier = &mut wrapped_copier.borrow_mut();

            if Rc::ptr_eq(&copier.src_node, null_node) {
               unsourced_copiers.push(wrapped_copier.clone());
            }

            // assert!(Rc::ptr_eq(&self.subgraph_nodes[node_num], &copier.dest_node));
            copier.dest_port_data = subnode.data_ports_dest_copy.clone();
            let port_name: &String = &copier.port_def.as_ref().unwrap().0;

            if port_name == "void" {
               copier.dest_index = PortDataCopier::VOID_SENTINEL;
            } else {
               let gotten = port_name_map.get(port_name);
               if gotten.is_none() {
                  let src_name_try = copier.src_node.try_borrow();
                  let src_name: String = if let Ok(src_name_node) = src_name_try {
                     src_name_node.name.clone()
                  } else {
                     "Not sure (".to_string() + &subnode.name + "???)"
                  };
                  assert!(
                        gotten.is_some(),
                        "Unable to find port \"{}\" in dest for src node named \"{}\" for destination \"{}\"",
                        &port_name,
                        &src_name,
                        subnode.name,
                   );
               }
               copier.dest_index = *gotten.unwrap();
            }

            if copier.src_index != PortDataCopier::VOID_SENTINEL {
               // Check src and dest port element types.
               let src_piece_type = ZPieceType::get_piece_type_from_data(
                  &copier.src_port_data.borrow()[copier.src_index],
               );
               let dest_piece_type = ZPieceType::get_piece_type_from_data(
                  &copier.dest_port_data.borrow()[copier.dest_index],
               );
               eprintln!(
                  "Copier has src type {:?} and dest type {:?}",
                  src_piece_type, dest_piece_type
               );
               assert!(src_piece_type == dest_piece_type);
            }
         }
      }
      Ok(())
   }

   fn populate_preset_data_for_node(
      wrapped_subnode: &Rc<RefCell<ZNode>>,
      n_def: &ZNodeDef,
   ) -> Result<(), ZGraphError> {
      let subnode: &mut ZNode = &mut wrapped_subnode.borrow_mut();
      let subnode_type: &ZNodeRegistration = subnode.node_type.as_ref();

      if subnode.data_copiers_src_copy.is_empty() {
         return Ok(());
      }
      if subnode_type.category != ZNodeCategory::PresetData {
         return Ok(());
      }
      for (i, preset_item) in n_def.preset_data.iter().enumerate() {
         let src_ports_data: &mut Vec<ZPiece> = &mut subnode.data_ports_src_copy.borrow_mut();
         src_ports_data[i] = preset_item.1.clone();
      }
      Ok(())
   }

   // Need mutable vector of edge copiers, one for input, one for
   // output. User graph provided with already-finalized output
   // destination. Creates and returns vector of input edge copiers.
   fn build_subgraph_from_graphdef(
      &mut self,
      graph_def: &ZGraphDef,
      registry: &ZRegistry,
      null_node: &Rc<RefCell<ZNode>>,
      floating_port_data: &PortDataVec,
   ) -> Result<(), ZGraphError> {
      {
         // Create vector of nodes for subgraph.

         let node_defs: &Vec<ZNodeDef> = &graph_def.nodes;
         let subgraph_size = node_defs.len();
         let null_noderegistration: &Rc<ZNodeRegistration> = registry.get_null_noderegistration();

         // 1st pass: set up minimal vector of realized nodes.
         let mut node_map_size: usize = self.subgraph_nodes.len();
         assert_eq!(node_map_size, 0);
         self.subgraph_nodes.reserve_exact(subgraph_size);
         let mut subgraph_node_map = HashMap::<String, usize>::default();

         for n_def in node_defs {
            self.subgraph_nodes.push(Rc::new(RefCell::new(ZNode {
               name: n_def.name.clone(),
               node_state_data: None,
               node_type: null_noderegistration.clone(),
               node_type_finder: None,
               data_copiers_src_copy: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
               data_copiers_dest_copy: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
               data_ports_src_copy: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
               data_ports_dest_copy: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
               subgraph_nodes: Vec::<Rc<RefCell<ZNode>>>::default(),
               subgraph_node_map: None,
               is_active: false,
            })));

            assert!(!subgraph_node_map.contains_key(&n_def.name));
            subgraph_node_map.insert(n_def.name.clone(), node_map_size);
            node_map_size += 1;
         }
         assert_eq!(node_map_size, subgraph_size);
         self.subgraph_node_map = Some(subgraph_node_map);
      }

      // Inter-pass replumbing of copiers out of subgraph. Replumb to
      // internal nodes for which each is outbound.
      {
         let &mut subgraph_node_map = &mut self.subgraph_node_map.as_ref().unwrap();
         let mut direct_in_out_copiers = Vec::<Rc<RefCell<PortDataCopier>>>::default();
         for external_copier in &self.data_copiers_src_copy {
            let borrow_hold: &mut PortDataCopier = &mut external_copier.borrow_mut();
            let port_def: &ZPortDef = borrow_hold.port_def.as_ref().unwrap();

            let connects_to_internal: bool = port_def.1 != "input";
            if connects_to_internal {
               // Port name, src node name, src port name.
               let node_num: usize = *subgraph_node_map.get(&port_def.1).unwrap();
               let src_node_znode: &Rc<RefCell<ZNode>> = &self.subgraph_nodes[node_num];
               {
                  let mut src_node_znode_bmut = src_node_znode.borrow_mut();
                  src_node_znode_bmut.data_copiers_src_copy.push(external_copier.clone());
                  src_node_znode_bmut.is_active = true;
               }
               borrow_hold.src_node = src_node_znode.clone();
            } else {
               direct_in_out_copiers.push(external_copier.clone());
            }
         }
         self.data_copiers_src_copy = direct_in_out_copiers; // Finish "move" of copiers to their internal nodes.
      }

      // 2nd pass vector of created nodes and node_defs matches before and after.
      {
         let node_defs: &Vec<ZNodeDef> = &graph_def.nodes;
         let subgraph_size = node_defs.len();

         for node_num in (0..subgraph_size).rev() {
            let n_def = &node_defs[node_num];

            eprintln!("Processing subnode: {}", n_def.name);

            let subnode_type: &Rc<ZNodeRegistration> = registry.find(&n_def.element).unwrap();
            assert_eq!(
               node_num,
               *self.subgraph_node_map.as_ref().unwrap().get(&n_def.name).unwrap()
            );

            {
               let subnode: &mut ZNode = &mut self.subgraph_nodes[node_num].borrow_mut();

               if subnode.data_copiers_src_copy.is_empty() {
                  continue;
               }
               subnode.node_type = subnode_type.clone();
               subnode.node_type_finder = Some(n_def.element.clone());
            }

            if is_leaf_node(subnode_type) {
               self.finish_leaf_node(node_num, n_def, registry, null_node, floating_port_data)?;
            } else {
               eprintln!(
                  "Apply input remapping from all self.data_copiers_src_copy for {}",
                  n_def.name
               );
            }
         }
      }

      // Populate type information with custom "node registration"
      // for preset data nodes. This could probably be folded in to
      // later loops for efficiency, but is performed separately for
      // now for semantic clarity.
      {
         let subgraph_size = self.subgraph_nodes.len();
         let node_defs: &Vec<ZNodeDef> = &graph_def.nodes;

         for node_num in (0..subgraph_size).rev() {
            let subnode: &mut ZNode = &mut self.subgraph_nodes[node_num].borrow_mut();
            let subnode_type: &ZNodeRegistration = subnode.node_type.as_ref();
            let n_def = &node_defs[node_num];

            if subnode.data_copiers_src_copy.is_empty() {
               continue;
            }
            if subnode_type.category == ZNodeCategory::PresetData {
               let mut replacement_registration: ZNodeRegistration = subnode_type.clone();
               // Replace output port typing.
               for preset_item in &n_def.preset_data {
                  replacement_registration.ports_src_copy.push(PortPieceTyped(
                     preset_item.0.clone(),
                     ZPieceType::get_piece_type_from_data(&preset_item.1),
                  ));
               }
               subnode.node_type = Rc::new(replacement_registration);
            }
         }
      }

      let mut unsourced_copiers = Vec::<Rc<RefCell<PortDataCopier>>>::default();
      // Hereafter avoid use of graphdef.

      // Late pass, after pruning optimizations, is to create port data vectors.
      {
         // Copiers src data ports.

         for node_num in (0..self.subgraph_nodes.len()).rev() {
            let wrapped_subnode: &Rc<RefCell<ZNode>> = &self.subgraph_nodes[node_num];
            ZNode::create_src_data_for_node(wrapped_subnode, &mut unsourced_copiers, null_node)?;
         }
      }
      assert!(unsourced_copiers.is_empty());

      // It is likely possible to have only src_data_copiers as
      // Vec<Box<PortDataCopier>> and at this point move the
      // copiers to similar dest_data_copiers. However, such a
      // simplification should only be done when we have
      // established the desirability of invoking copiers as
      // sub-iteration.
      {
         // Copiers dest data ports.

         let node_defs: &Vec<ZNodeDef> = &graph_def.nodes;
         for node_num in (0..self.subgraph_nodes.len()).rev() {
            let wrapped_subnode: &Rc<RefCell<ZNode>> = &self.subgraph_nodes[node_num];
            ZNode::create_dest_data_for_node(wrapped_subnode, &mut unsourced_copiers, null_node)?;

            let n_def = &node_defs[node_num];
            ZNode::populate_preset_data_for_node(wrapped_subnode, n_def)?;
         }

         // Fill preset data.
      }

      // Begin: Debugging of unsourced copiers.
      for wrapped_copier in &self.data_copiers_dest_copy {
         let copier: &PortDataCopier = &wrapped_copier.borrow();

         if Rc::ptr_eq(&copier.src_node, null_node) {
            unsourced_copiers.push(wrapped_copier.clone());
            continue;
         }
      }
      for wrapped_copier in &self.data_copiers_src_copy {
         let copier: &PortDataCopier = &wrapped_copier.borrow_mut();
         let &port_def = &copier.port_def.as_ref().unwrap();
         eprintln!("direct copier: {}, {}, {}", port_def.0, port_def.1, port_def.2);
      }
      for wrapped_copier in &unsourced_copiers {
         let copier: &PortDataCopier = &wrapped_copier.borrow_mut();
         let &port_def = &copier.port_def.as_ref().unwrap();
         eprintln!("internally unsourced copier: {}, {}, {}", port_def.0, port_def.1, port_def.2);
      }
      assert_eq!(self.data_copiers_src_copy.len(), unsourced_copiers.len());
      // End: Debugging of unsourced copiers.

      // A late-late pass could merge adjacent copiers.

      Ok(())
   }

   fn run_src_copiers(&mut self) -> Result<(), ZGraphError> {
      for wrapped_copier in &self.data_copiers_src_copy {
         let copier: &PortDataCopier = &wrapped_copier.borrow_mut();
         if copier.src_index == PortDataCopier::VOID_SENTINEL {
            // eprintln!("--- Skipping port for node \"{}\"", &self.name);
            continue;
         }

         let src_port_data: &Vec<ZPiece> = &copier.src_port_data.borrow();
         let dest_port_data: &mut Vec<ZPiece> = &mut copier.dest_port_data.borrow_mut();

         eprintln!(
            "Copying for {} src data = \"{:?}\", dest data = \"{:?}\"",
            copier.port_def.as_ref().unwrap().0,
            src_port_data[copier.src_index],
            dest_port_data[copier.dest_index]
         );

         dest_port_data[copier.dest_index] = src_port_data[copier.src_index].clone();

         eprintln!(
            "Copying for {} src data = \"{:?}\", dest data = \"{:?}\"",
            copier.port_def.as_ref().unwrap().0,
            src_port_data[copier.src_index],
            dest_port_data[copier.dest_index]
         );
      }
      Ok(())
   }

   fn run_constructors(&mut self, renderer_data: &mut ZRendererData) -> Result<(), ZGraphError> {
      // Assumes self node is active, not run on leaf node, no calculators run on this node.

      for n in &mut self.subgraph_nodes {
         let node: &mut ZNode = &mut n.borrow_mut();
         if node.is_active {
            if node.subgraph_nodes.is_empty() {
               let node_element = &node.node_type;
               if node_element.construction_fn.is_some() {
                  node_element.construction_fn.unwrap()(
                     renderer_data,
                     &mut node.node_state_data,
                     &node.data_ports_dest_copy.borrow(),
                     &mut node.data_ports_src_copy.borrow_mut(),
                  );
               }
            } else {
               node.run_constructors(renderer_data)?;
            }
         }
      }

      Ok(())
   }

   fn run_calculators(&mut self, renderer_data: &mut ZRendererData) -> Result<(), ZGraphError> {
      // Assumes self node is active, not run on leaf node, no calculators run on this node.

      for n in &mut self.subgraph_nodes {
         let node: &mut ZNode = &mut n.borrow_mut();
         if node.is_active {
            if node.subgraph_nodes.is_empty() {
               // eprintln!("+++ Leaf copying for src node \"{}\"", &node.name);
               let node_element = &node.node_type;
               if node_element.calculation_fn.is_some() {
                  node_element.calculation_fn.unwrap()(
                     renderer_data,
                     &mut node.node_state_data,
                     &node.data_ports_dest_copy.borrow(),
                     &mut node.data_ports_src_copy.borrow_mut(),
                  );
               }
               node.run_src_copiers()?;
            } else {
               // eprintln!("=== Recursing for src node \"{}\"", &node.name);
               node.run_calculators(renderer_data)?;
               // src copiers should be empty for non-outer subgraph nodes.
            }
         }
      }

      Ok(())
   }

   fn run_inkings(&mut self, renderer_data: &mut ZRendererData) -> Result<(), ZGraphError> {
      // Assumes self node is active, not run on leaf node, no calculators run on this node.

      for n in &mut self.subgraph_nodes {
         let node: &mut ZNode = &mut n.borrow_mut();
         if node.is_active {
            if node.subgraph_nodes.is_empty() {
               let node_element = &node.node_type;
               if node_element.inking_fn.is_some() {
                  node_element.inking_fn.unwrap()(
                     renderer_data,
                     &mut node.node_state_data,
                     &node.data_ports_dest_copy.borrow(),
                     &mut node.data_ports_src_copy.borrow_mut(),
                  );
               }
            } else {
               node.run_inkings(renderer_data)?;
            }
         }
      }

      Ok(())
   }
}
