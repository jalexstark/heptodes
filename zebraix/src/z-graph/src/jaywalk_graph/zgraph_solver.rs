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

use crate::jaywalk_graph::jaywalk_traiting::is_mult_ident_f64;
use crate::jaywalk_graph::jaywalk_traiting::mult_ident_f64;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ZWeighted {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ZWeightedView {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ZFree {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}
