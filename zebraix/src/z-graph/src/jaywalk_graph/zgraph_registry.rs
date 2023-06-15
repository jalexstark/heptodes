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
}

#[derive(Default)]
pub struct ZRegistry {
   pub node_registrations: HashMap<String, Rc<ZNodeRegistration>>,
}

type ZNodeInvocationFn = fn(&mut ZRendererData, &mut ZNodeStateData, &ZData, &mut ZData);

#[derive(Debug)]
pub enum ZRegistryError {
   ZRegistryNotFound,
}

impl ZRegistry {
   pub fn register_new(&mut self, new_element: ZNodeRegistration) {
      self
         .node_registrations
         .insert(new_element.name.clone(), Rc::<ZNodeRegistration>::new(new_element));
   }

   pub fn find(&self, finder: &ZNodeTypeFinder) -> Result<&Rc<ZNodeRegistration>, ZRegistryError> {
      match finder {
         ZNodeTypeFinder::ByString(s) => {
            let search = self.node_registrations.get(s);
            if search.is_some() {
               return Ok(search.unwrap());
            } else {
               return Err(ZRegistryError::ZRegistryNotFound);
            }
         }
      }
   }
}
