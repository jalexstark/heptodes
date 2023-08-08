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

use crate::jaywalk_graph::zgraph_base::PortPieceTyped;
use crate::jaywalk_graph::zgraph_base::ZNodeStateData;
use crate::jaywalk_graph::zgraph_base::ZNodeTypeFinder;
use crate::jaywalk_graph::zgraph_base::ZPiece;
use crate::jaywalk_graph::zgraph_base::ZRendererData;
use crate::jaywalk_graph::zgraph_graphdef::ZGraphDef;
use derive_builder::Builder;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZNodeCategory {
   OtherCategory,
   Subgraph,
   PresetData,
   Aggregator,
   Disaggregator,
}

impl Default for ZNodeCategory {
   fn default() -> Self {
      ZNodeCategory::OtherCategory
   }
}

#[derive(Default, Builder, Clone)]
pub struct ZNodeRegistration {
   pub name: String,
   #[builder(setter(strip_option), default)]
   pub construction_fn: Option<ZNodeInvocationFn>,
   #[builder(setter(strip_option), default)]
   pub calculation_fn: Option<ZNodeInvocationFn>,
   #[builder(setter(strip_option), default)]
   pub inking_fn: Option<ZNodeInvocationFn>,
   #[builder(default)]
   pub ports_src_copy: Vec<PortPieceTyped>,
   #[builder(default)]
   pub ports_dest_copy: Vec<PortPieceTyped>,
   #[builder(default)]
   pub graph_def: ZGraphDef,
   #[builder(default)]
   pub category: ZNodeCategory,
}

pub struct ZRegistry {
   pub node_registrations: HashMap<String, Rc<ZNodeRegistration>>,
}

type ZNodeInvocationFn = fn(&mut ZRendererData, &mut ZNodeStateData, &[ZPiece], &mut [ZPiece]);

#[derive(Debug)]
pub enum ZRegistryError {
   ZRegistryNotFound,
}

impl ZRegistry {
   pub fn new() -> Self {
      let mut new_registry: Self =
         Self { node_registrations: HashMap::<String, Rc<ZNodeRegistration>>::default() };
      new_registry.register_new(
         ZNodeRegistrationBuilder::default().name("Null".to_string()).build().unwrap(),
      );
      new_registry.register_new(
         ZNodeRegistrationBuilder::default()
            .name("Subgraph".to_string())
            .category(ZNodeCategory::Subgraph)
            .build()
            .unwrap(),
      );
      new_registry
   }

   pub fn register_new(&mut self, new_element: ZNodeRegistration) {
      self
         .node_registrations
         .insert(new_element.name.clone(), Rc::<ZNodeRegistration>::new(new_element));
   }

   pub fn find(&self, finder: &ZNodeTypeFinder) -> Result<&Rc<ZNodeRegistration>, ZRegistryError> {
      let search = match finder {
         ZNodeTypeFinder::ByString(s) => self.node_registrations.get(s),
         ZNodeTypeFinder::NullNodeType => self.node_registrations.get("Null"),
         ZNodeTypeFinder::SubGraphNodeType => self.node_registrations.get("Subgraph"),
      };
      if let Some(node_registration) = search {
         Ok(node_registration)
      } else {
         Err(ZRegistryError::ZRegistryNotFound)
      }
   }

   // Consider retiring, after introduction of specific ZNodeTypeFinder enums.
   pub fn get_null_noderegistration(&self) -> &Rc<ZNodeRegistration> {
      let null_finder = ZNodeTypeFinder::NullNodeType;
      let null_node_reg: &Rc<ZNodeRegistration> = self.find(&null_finder).unwrap();
      null_node_reg
   }

   // Consider retiring, after introduction of specific ZNodeTypeFinder enums.
   pub fn get_subgraph_noderegistration(&self) -> &Rc<ZNodeRegistration> {
      let subgraph_finder = ZNodeTypeFinder::SubGraphNodeType;
      let subgraph_node_reg: &Rc<ZNodeRegistration> = self.find(&subgraph_finder).unwrap();
      subgraph_node_reg
   }
}

impl Default for ZRegistry {
   fn default() -> Self {
      Self::new()
   }
}
