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
use crate::jaywalk_graph::zgraph_base::ZData;
use crate::jaywalk_graph::zgraph_base::ZNodeStateData;
use crate::jaywalk_graph::zgraph_base::ZNodeTypeFinder;
use crate::jaywalk_graph::zgraph_base::ZRendererData;
use derive_builder::Builder;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default, Builder)]
pub struct ZNodeRegistration {
   pub name: String,
   #[builder(setter(strip_option), default)]
   pub construction_fn: Option<ZNodeInvocationFn>,
   #[builder(setter(strip_option), default)]
   pub calculation_fn: Option<ZNodeInvocationFn>,
   #[builder(setter(strip_option), default)]
   pub inking_fn: Option<ZNodeInvocationFn>,
   #[builder(default)]
   pub input_ports: Vec<PortPieceTyped>,
   #[builder(default)]
   pub output_ports: Vec<PortPieceTyped>,
   #[builder(default)]
   pub is_subgraph_type: bool,
}

pub struct ZRegistry {
   pub node_registrations: HashMap<String, Rc<ZNodeRegistration>>,
}

type ZNodeInvocationFn = fn(&mut ZRendererData, &mut ZNodeStateData, &ZData, &mut ZData);

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
            .is_subgraph_type(true)
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
      match finder {
         ZNodeTypeFinder::ByString(s) => {
            let search = self.node_registrations.get(s);
            if let Some(node_registration) = search {
               Ok(node_registration)
            } else {
               Err(ZRegistryError::ZRegistryNotFound)
            }
         }
      }
   }

   pub fn get_null_noderegistration(&self) -> &Rc<ZNodeRegistration> {
      let null_finder = ZNodeTypeFinder::ByString("Null".to_string());
      let null_node_reg: &Rc<ZNodeRegistration> = self.find(&null_finder).unwrap();
      null_node_reg
   }

   pub fn get_subgraph_noderegistration(&self) -> &Rc<ZNodeRegistration> {
      let subgraph_finder = ZNodeTypeFinder::ByString("Subgraph".to_string());
      let subgraph_node_reg: &Rc<ZNodeRegistration> = self.find(&subgraph_finder).unwrap();
      subgraph_node_reg
   }
}

impl Default for ZRegistry {
   fn default() -> Self {
      Self::new()
   }
}
