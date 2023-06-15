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

use crate::jaywalk_graph::zgraph_registry::ZNodeRegistrationBuilder;
use crate::jaywalk_graph::zgraph_registry::ZRegistry;

pub fn register_builtin_library(registry: &mut ZRegistry) {
   registry
      .register_new(ZNodeRegistrationBuilder::default().name("Group".to_string()).build().unwrap());
   registry
      .register_new(ZNodeRegistrationBuilder::default().name("Input".to_string()).build().unwrap());
   registry.register_new(
      ZNodeRegistrationBuilder::default().name("Output".to_string()).build().unwrap(),
   );
}
