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
use std::any::Any;

// The idea is to keep the base minimal, but that will be difficult.

#[derive(Serialize, Deserialize)]
pub enum Types {
   Void = 0,
   Integer,
   Real,
   Bool,
   Weighted,
   WeightedView,
   //
   Group,
   GraphOutput,
   GraphInput,
   BoxedText,
   Circle,
   TextLine,
}

#[derive(Serialize, Deserialize)]
pub enum ZDataByType {
   Void = 0,
   Dirty,
   Derived,
   Fit,
}

// ZData should be a set of data, a set of ports, and is passed to
// invocation functions.
#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ZData {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

#[derive(PartialEq, Eq)]
pub enum ZMachineTypestate {
   Init = 0,
   Deffed,
   Constructed,
   Calculated,
   Inked,
   Finished,
}

pub type ZNodeStateData = Option<Box<dyn Any>>;
pub type ZRendererData = Option<Box<dyn Any>>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZNodeTypeFinder {
   ByString(String),
}

#[derive(Debug)]
pub enum ZGraphError {
   IncorrectTypestateTransition,
   DuplicateGraphDef,
   MissingGraphDef,
   DuplicateRendererSetup,
   RendererForConstruction,
   FinishNoRenderer,
   UnfinishedRenderer,
}
