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
use crate::jaywalk_graph::zgraph_base::ZGraphError;
use crate::jaywalk_graph::zgraph_base::ZNodeTypeFinder;
use crate::jaywalk_graph::zgraph_base::ZPiece;
use crate::jaywalk_graph::zgraph_base::ZRendererData;
use crate::jaywalk_graph::zgraph_graphdef::PresetPiece;
use crate::jaywalk_graph::zgraph_graphdef::ZGraphDef;
use crate::jaywalk_graph::zgraph_node::PortDataCopier;
use crate::jaywalk_graph::zgraph_node::VoidFilter;
use crate::jaywalk_graph::zgraph_node::ZNode;
use crate::jaywalk_graph::zgraph_registry::ZRegistry;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq)]
pub enum ZMachineTypestate {
   Init = 0,
   Deffed,
   Constructed,
   Calculated,
   Inked,
   Finished,
}

#[derive(Debug)]
pub enum DebugLine {
   SimpleString(String),
}

pub struct ZMachine {
   pub typestate: ZMachineTypestate,
   pub has_graph_def: bool,

   pub graph_def: ZGraphDef,
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
      let null_node = ZNode::new_null_node("Null node", &registry);
      let realized_node = ZNode::new_basic_node(
         "User graph",
         &Vec::<PresetPiece>::new(),
         &ZNodeTypeFinder::SubGraphNodeType,
         &registry,
      );

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

      assert!(graph_def.is_precompiled);

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

      self.build_from_graphdef().unwrap();

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
      assert!(self.graph_def.is_precompiled);
      ZNode::reregister_user_graph_input_porting(&self.realized_node, &self.graph_def.inputs)?;
      ZNode::seed_user_graph_outputs(
         VoidFilter::NonVoid,
         &self.realized_node,
         &self.graph_def.output_ports_as_links,
         &self.null_node,
         &self.floating_port_data,
      )?;

      // realized_node.data_copiers_dest_copy populated before.
      //
      // realized_node.data_copiers_src_copy populated by builder.
      ZNode::realize_from_usergraph_graphdef(
         &self.realized_node,
         &self.graph_def,
         &self.registry,
         &self.null_node,
         &self.floating_port_data,
      )?;

      Ok(())
   }
}

impl Default for ZMachine {
   fn default() -> Self {
      Self::new()
   }
}

impl ZNode {
   fn run_src_copiers(&mut self) -> Result<(), ZGraphError> {
      for wrapped_copier in &self.data_copiers_src_copy {
         let copier: &PortDataCopier = &wrapped_copier.borrow_mut();
         assert_ne!(copier.src_index, PortDataCopier::FLOATING_SENTINEL);
         if copier.src_index == PortDataCopier::VOID_SENTINEL {
            // eprintln!("--- Skipping port for node \"{}\"", &self.name);
            continue;
         }

         let src_port_data: &Vec<ZPiece> = &copier.src_port_data.borrow();
         let dest_port_data: &mut Vec<ZPiece> = &mut copier.dest_port_data.borrow_mut();

         eprintln!(
            "Copying for {} src data = \"{:?}\", dest data = \"{:?}\"",
            copier.port_def.as_ref().unwrap().name,
            src_port_data[copier.src_index],
            dest_port_data[copier.dest_index]
         );

         dest_port_data[copier.dest_index] = src_port_data[copier.src_index].clone();

         eprintln!(
            "Copying for {} src data = \"{:?}\", dest data = \"{:?}\"",
            copier.port_def.as_ref().unwrap().name,
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
