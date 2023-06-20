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
   pub inbound_data_copier: Vec<PortDataCopier>,
}

#[derive(Default)]
pub struct RealizedGraph {
   // Subgraph expansion is contextual.  That is, it is based on the
   // union of all data_copiers consuming its results.
   pub subgraph_nodes: Vec<Rc<RefCell<ZNode>>>,
   pub subgraph_node_map: HashMap<String, usize>,
   // node_map= HashMap<String, String>::new();
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

   pub realized_graph: RealizedGraph,

   // Node registry
   pub registry: ZRegistry,

   pub null_node: Rc<RefCell<ZNode>>,
}

impl ZMachine {
   pub fn new() -> Self {
      let registry = ZRegistry::default();
      let null_node_type = registry.get_null_noderegistration().clone();

      Self {
         typestate: ZMachineTypestate::Init,
         has_graph_def: false,

         graph_def: ZGraphDef::default(),
         // // ZGraphDef fields.
         // category: ZGraphDefCategory::default(),
         // name: String::default(),
         // description: Option::<String>::default(),
         // nodes: JVec::<ZNodeDef>::default(),
         // edges: JVec::<ZEdgeDef>::default(),

         // Renderer.
         renderer_data: None,
         realized_graph: RealizedGraph::default(),

         // Node registry
         registry: registry,
         null_node: Rc::new(RefCell::new(ZNode {
            name: "Null node".to_string(),
            node_state_data: None,
            node_type: null_node_type,
            node_type_finder: None,
            inbound_data_copier: Vec::<PortDataCopier>::default(),
         })),
      }
   }

   // User-level graphdef for overall drawing.
   pub fn provide_graph_def(&mut self, graph_def: ZGraphDef) -> Result<(), ZGraphError> {
      if self.has_graph_def {
         return Err(ZGraphError::DuplicateGraphDef);
      }

      self.graph_def = graph_def;
      // self.graph_def = graph_def.clone();
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

      for n in &mut self.realized_graph.subgraph_nodes {
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

      for n in &mut self.realized_graph.subgraph_nodes {
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

      for n in &mut self.realized_graph.subgraph_nodes {
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
      self.realized_graph.build_from_graphdef(&self.graph_def, &self.registry, &self.null_node)
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
   pub src_index: i32,
   pub dst_port_data: PortDataVec,
   pub dst_index: i32,
}

// Need mutable vector of edge copiers, one for input, one for
// output. User graph provided with already-finalized output
// destination. Creates and returns vector of input edge copiers.
impl RealizedGraph {
   fn build_from_graphdef(
      &mut self,
      graph_def: &ZGraphDef,
      registry: &ZRegistry,
      null_node: &Rc<RefCell<ZNode>>,
   ) -> Result<(), ZGraphError> {
      let node_defs: &Vec<ZNodeDef> = &graph_def.nodes;
      let null_noderegistration: &Rc<ZNodeRegistration> = registry.get_null_noderegistration();

      // floating_port_data is a sentinel, used to indicate that a
      // connection is floating.
      let floating_port_data = Rc::new(RefCell::new(Vec::<ZPiece>::default()));
      let external_sink = Rc::new(RefCell::new(Vec::<ZPiece>::default()));
      external_sink.borrow_mut().push(ZPiece::Void);
      let mut external_copier = PortDataCopier {
         src_node: null_node.clone(),
         src_port_data: floating_port_data,
         src_index: 0,
         dst_port_data: external_sink,
         dst_index: 0,
      };

      {
         // 1st pass: set up minimal vector of realized nodes.
         let mut node_map_size: usize = self.subgraph_nodes.len();
         for n_def in node_defs {
            self.subgraph_nodes.push(Rc::new(RefCell::new(ZNode {
               name: n_def.name.clone(),
               node_state_data: None,
               node_type: null_noderegistration.clone(),
               node_type_finder: None,
               inbound_data_copier: Vec::<PortDataCopier>::default(),
            })));

            assert!(!self.subgraph_node_map.contains_key(&n_def.name));
            self.subgraph_node_map.insert(n_def.name.clone(), node_map_size);
            node_map_size += 1;
         }
      }

      // Need to reverse.  Also hereafter only use node information
      // internally: after finding node registrations, no longer work
      // with node_defs.

      // 2nd pass vector of created nodes and node_defs matches before and after.
      {
         for n_def in node_defs {
            let node_type: &Rc<ZNodeRegistration> = registry.find(&n_def.element).unwrap();

            let node: &mut ZNode = &mut self.subgraph_nodes
               [*self.subgraph_node_map.get(&n_def.name).unwrap()]
            .borrow_mut();
            node.node_type = node_type.clone();
            node.node_type_finder = Some(n_def.element.clone());
         }
      }

      Ok(())
   }
}
