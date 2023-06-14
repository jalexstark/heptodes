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
use crate::jaywalk_graph::zgraph_base::ZRendererData;
use derive_builder::Builder;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default, Builder)]
pub struct ZNodeRegistration {
   name: String,
   construction_fn: Option<ZNodeInvocationFn>,
   calculation_fn: Option<ZNodeInvocationFn>,
   inking_fn: Option<ZNodeInvocationFn>,
}

#[derive(Default)]
pub struct ZRegistry {
   pub node_registrations: HashMap<String, Rc<ZNodeRegistration>>,
}

type ZNodeInvocationFn = fn(&mut ZRendererData, &mut ZNodeStateData, &ZData, &mut ZData);

impl ZRegistry {
   pub fn register_new(&mut self, _node_registration: ZNodeRegistration) {}
}
