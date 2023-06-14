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
use crate::jaywalk_graph::zgraph_base::ZRendererData;
use crate::jaywalk_graph::zgraph_graphdef::ZGraphDef;
use crate::jaywalk_graph::zgraph_registry::ZRegistry;
use crate::jaywalk_graph::zgraph_svg::test_text_construction;
use crate::jaywalk_graph::zgraph_svg::test_text_inking;

#[derive(Debug)]
pub enum DebugLine {
   SimpleString(String),
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
   pub test_text_node_data: ZNodeStateData,

   // Node registry
   registry: ZRegistry,
}

impl ZMachine {
   pub fn new() -> Self {
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
         test_text_node_data: None,

         // Node registry
         registry: ZRegistry::default(),
      }
   }

   // User-level graphdef for overall drawing.
   pub fn provide_graph_def(&mut self, graph_def: &ZGraphDef) -> Result<(), ZGraphError> {
      if self.has_graph_def {
         return Err(ZGraphError::DuplicateGraphDef);
      }

      self.graph_def = graph_def.clone();
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

      //

      test_text_construction(
         &mut self.renderer_data,
         &mut self.test_text_node_data,
         &ZData::default(),
         &mut ZData::default(),
      );

      //

      self.typestate = ZMachineTypestate::Constructed;
      Ok(())
   }

   pub fn transition_to_calculated(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Constructed {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }
      self.typestate = ZMachineTypestate::Calculated;
      Ok(())
   }

   pub fn transition_to_inked(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Calculated {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }

      //

      test_text_inking(
         &mut self.renderer_data,
         &mut self.test_text_node_data,
         &ZData::default(),
         &mut ZData::default(),
      );

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
      return Ok(());
   }
}

impl Default for ZMachine {
   fn default() -> Self {
      Self::new()
   }
}
