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

use crate::jaywalk_graph::zgraph_base::ZData;
use crate::jaywalk_graph::zgraph_base::ZGraphError;
use crate::jaywalk_graph::zgraph_base::ZMachineTypestate;
use crate::jaywalk_graph::zgraph_base::ZNodeStateData;
use crate::jaywalk_graph::zgraph_base::ZNodeTypeFinder;
use crate::jaywalk_graph::zgraph_base::ZPiece;
use crate::jaywalk_graph::zgraph_base::ZRendererData;
use crate::jaywalk_graph::zgraph_graphdef::ZGraphDef;
use crate::jaywalk_graph::zgraph_graphdef::ZNodeDef;
use crate::jaywalk_graph::zgraph_graphdef::ZPortDef;
use crate::jaywalk_graph::zgraph_registry::ZNodeRegistration;
use crate::jaywalk_graph::zgraph_registry::ZRegistry;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub enum DebugLine {
   SimpleString(String),
}

#[allow(dead_code)]
pub struct ZNode {
   pub name: String,
   pub node_type: Rc<ZNodeRegistration>,
   pub node_type_finder: Option<ZNodeTypeFinder>,
   pub node_state_data: ZNodeStateData,

   pub inbound_data_copiers: Vec<Rc<RefCell<PortDataCopier>>>,
   pub outbound_data_copiers: Vec<Rc<RefCell<PortDataCopier>>>,
   pub input_data_ports: PortDataVec,
   pub output_data_ports: PortDataVec,

   pub subgraph_nodes: Vec<Rc<RefCell<ZNode>>>, // Prolly init with 0 reserve.
   pub subgraph_node_map: Option<HashMap<String, usize>>,
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

   // Probably delete, and just use node_state_data in outermost.
   pub global_sink_data_ports: PortDataVec,
   // pub global_sink_copiers: Vec<Rc<RefCell<PortDataCopier>>>,
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
         inbound_data_copiers: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
         outbound_data_copiers: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
         input_data_ports: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
         output_data_ports: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
         subgraph_nodes: Vec::<Rc<RefCell<ZNode>>>::default(),
         subgraph_node_map: None,
      }));

      let realized_node = Rc::new(RefCell::new(ZNode {
         name: "User graph".to_string(),
         node_state_data: None,
         node_type: subgraph_node_type,
         node_type_finder: None,
         inbound_data_copiers: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
         outbound_data_copiers: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
         input_data_ports: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
         output_data_ports: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
         subgraph_nodes: Vec::<Rc<RefCell<ZNode>>>::default(),
         subgraph_node_map: None,
      }));

      // floating_port_data is a sentinel, used to indicate that a
      // connection is floating.
      let floating_port_data = Rc::new(RefCell::new(Vec::<ZPiece>::default()));
      let global_sink_data_ports = Rc::new(RefCell::new(Vec::<ZPiece>::default()));
      global_sink_data_ports.borrow_mut().push(ZPiece::Void);
      // let global_sink_copiers = Vec::<Rc<RefCell<PortDataCopier>>>::default();

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
         global_sink_data_ports,
         // global_sink_copiers,
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

      //

      for n in &mut self.realized_node.borrow_mut().subgraph_nodes {
         let node: &mut ZNode = &mut n.borrow_mut();
         let node_element = &node.node_type;
         if node_element.construction_fn.is_some() {
            node_element.construction_fn.unwrap()(
               &mut self.renderer_data,
               &mut node.node_state_data,
               &ZData::default(),
               &mut ZData::default(),
            );
         }
      }

      //

      self.typestate = ZMachineTypestate::Constructed;
      Ok(())
   }

   pub fn transition_to_calculated(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Constructed {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }

      //

      for n in &mut self.realized_node.borrow_mut().subgraph_nodes {
         let node: &mut ZNode = &mut n.borrow_mut();
         let node_element = &node.node_type;
         if node_element.calculation_fn.is_some() {
            node_element.calculation_fn.unwrap()(
               &mut self.renderer_data,
               &mut node.node_state_data,
               &ZData::default(),
               &mut ZData::default(),
            );
         }
      }

      //

      self.typestate = ZMachineTypestate::Calculated;
      Ok(())
   }

   pub fn transition_to_inked(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Calculated {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }

      //

      for n in &mut self.realized_node.borrow_mut().subgraph_nodes {
         let node: &mut ZNode = &mut n.borrow_mut();
         let node_element = &node.node_type;
         if node_element.inking_fn.is_some() {
            node_element.inking_fn.unwrap()(
               &mut self.renderer_data,
               &mut node.node_state_data,
               &ZData::default(),
               &mut ZData::default(),
            );
         }
      }

      //

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
      let realized_node: &mut ZNode = &mut self.realized_node.borrow_mut();

      // In ordinary mode of operation, for a user graph, we create one copier for each output.
      // self.global_sink_copiers.clear();
      assert!(realized_node.outbound_data_copiers.is_empty());
      for port_def in &self.graph_def.output_ports {
         realized_node.outbound_data_copiers.push(Rc::new(RefCell::new(PortDataCopier {
            src_node: self.null_node.clone(),
            src_port_data: self.floating_port_data.clone(),
            src_index: PortDataCopier::VOID_SENTINEL,
            dst_port_data: self.global_sink_data_ports.clone(),
            dst_index: PortDataCopier::VOID_SENTINEL,
            port_def: Some(port_def.clone()),
         })));
         eprintln!("Port attachment: {}", port_def.0);
         // pub global_sink_data_ports: PortDataVec,
         // pub global_sink_copiers: Vec<PortDataCopier>,
      }

      // realized_node.outbound_data_copiers populated before.
      //
      // realized_node.inbound_data_copiers populated by builder.
      realized_node.build_subgraph_from_graphdef(
         // &mut self.global_sink_copiers,
         &self.graph_def,
         &self.registry,
         &self.null_node,
         &self.floating_port_data,
      )

      // XXX
      //
      // After building graph, will need (for user graph) to created
      // output data vec and plumb into global_sink_copiers.
   }
}

impl Default for ZMachine {
   fn default() -> Self {
      Self::new()
   }
}

type PortDataVec = Rc<RefCell<Vec<ZPiece>>>;

// Floatable version of edge copier
pub struct PortDataCopier {
   pub src_node: Rc<RefCell<ZNode>>,
   pub src_port_data: PortDataVec,
   pub src_index: usize,
   pub dst_port_data: PortDataVec,
   pub dst_index: usize,
   pub port_def: Option<ZPortDef>,
   // #[derive(Clone, Serialize, Deserialize)]
   // pub struct ZPortDef(pub String, pub String, pub String);
}

impl PortDataCopier {
   const VOID_SENTINEL: usize = usize::MAX;
}

#[inline(always)]
fn is_leaf_node(subnode_type: &Rc<ZNodeRegistration>) -> bool {
   !subnode_type.is_subgraph_type
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

      assert!(subnode.inbound_data_copiers.is_empty());
      subnode.inbound_data_copiers.clear();

      for edge in &n_def.edges {
         eprintln!("   Adding subnode dependency on: {}", edge.src_node);
         let is_internal_src_node: bool = edge.src_node != "input";
         let src_node_znode: &Rc<RefCell<ZNode>>;
         if is_internal_src_node {
            let node_num: usize =
               *self.subgraph_node_map.as_ref().unwrap().get(&edge.src_node).unwrap();
            src_node_znode = &self.subgraph_nodes[node_num];
         } else {
            src_node_znode = &null_node;
         }

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
               dst_port_data: floating_port_data.clone(),
               dst_index: PortDataCopier::VOID_SENTINEL,
               port_def: Some(ZPortDef(
                  connection.1.clone(),
                  edge.src_node.clone(),
                  connection.0.clone(),
               )),
            }));
            subnode.inbound_data_copiers.push(edges_copier.clone());

            // XXX At this point we should be able to create input data vector and clean up dst connection.

            if is_internal_src_node {
               src_node_znode.borrow_mut().outbound_data_copiers.push(edges_copier);
            } else {
               self.inbound_data_copiers.push(edges_copier);
            }
         }
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
               inbound_data_copiers: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
               outbound_data_copiers: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
               input_data_ports: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
               output_data_ports: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
               subgraph_nodes: Vec::<Rc<RefCell<ZNode>>>::default(),
               subgraph_node_map: None,
            })));

            assert!(!subgraph_node_map.contains_key(&n_def.name));
            subgraph_node_map.insert(n_def.name.clone(), node_map_size);
            node_map_size += 1;
         }
         assert_eq!(node_map_size, subgraph_size);
         self.subgraph_node_map = Some(subgraph_node_map);
      }

      // XXX

      // --- Sub tasks when required for above

      // Create function to build copiers for node input signature.

      // Create function to build port data vectors from port type vectors.

      // When visiting a node, if not a subgraph itself, then create output data vector.

      // Populate preset data in output port data.

      // ---

      // Inter-pass replumbing of copiers out of subgraph. Replumb to
      // internal nodes for which each is outbound.
      {
         let &mut subgraph_node_map = &mut self.subgraph_node_map.as_ref().unwrap();
         for external_copier in &self.outbound_data_copiers {
            let borrow_hold: &PortDataCopier = &external_copier.borrow();
            let port_def: &ZPortDef = borrow_hold.port_def.as_ref().unwrap();
            // Port name, src node name, src port name.
            let node_num: usize = *subgraph_node_map.get(&port_def.1).unwrap();
            let src_node_znode: &Rc<RefCell<ZNode>> = &self.subgraph_nodes[node_num];
            src_node_znode.borrow_mut().outbound_data_copiers.push(external_copier.clone());
         }
         self.outbound_data_copiers.clear(); // Finish "move" of copiers to their internal nodes.
      }

      // 2nd pass vector of created nodes and node_defs matches before and after.
      {
         let node_defs: &Vec<ZNodeDef> = &graph_def.nodes;
         let subgraph_size = node_defs.len();

         for i in (0..subgraph_size).rev() {
            let n_def = &node_defs[i];

            eprintln!("Processing subnode: {}", n_def.name);

            let subnode_type: &Rc<ZNodeRegistration> = registry.find(&n_def.element).unwrap();
            let node_num: usize =
               *self.subgraph_node_map.as_ref().unwrap().get(&n_def.name).unwrap();

            {
               let subnode: &mut ZNode = &mut self.subgraph_nodes[node_num].borrow_mut();

               if subnode.outbound_data_copiers.is_empty() {
                  continue;
               }
               subnode.node_type = subnode_type.clone();
               subnode.node_type_finder = Some(n_def.element.clone());
            }

            if is_leaf_node(subnode_type) {
               self.finish_leaf_node(node_num, n_def, &registry, null_node, floating_port_data)?;
            } else {
               eprintln!(
                  "Apply input remapping from all self.inbound_data_copiers for {}",
                  n_def.name
               );
            }
         }
      }

      // Hereafter avoid use of graphdef.

      Ok(())
   }
}
