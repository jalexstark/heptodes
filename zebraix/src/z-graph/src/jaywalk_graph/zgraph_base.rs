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

// pub mod jaywalk_foundation;
// pub mod jaywalk_traiting;

use crate::jaywalk_graph::jaywalk_foundation::is_default;
// use crate::jaywalk_graph::jaywalk_foundation::EmptyVec;
use crate::jaywalk_graph::jaywalk_traiting::is_mult_ident_f64;
use crate::jaywalk_graph::jaywalk_traiting::mult_ident_f64;
use crate::jaywalk_graph::JVec;
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;

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

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ZData {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct ZNodeTypeFinder {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ZNodeDef {
   pub name: String,
   #[serde(skip_serializing_if = "is_default")]
   pub description: Option<String>,

   // #[serde(skip_serializing_if = "is_default")]
   #[serde(default)]
   pub node_type: ZNodeTypeFinder,

   #[serde(skip_serializing_if = "is_default")]
   pub preset_data: Option<ZData>,
}

// impl EmptyVec for ZNodeDef {
//    type Item = ZNodeDef;

//    fn empty_vec() -> &'static Vec<ZNodeDef> {
//       static EMPTY_VEC: Vec<ZNodeDef> = Vec::<ZNodeDef>::new();
//       &EMPTY_VEC
//    }
// }

#[derive(Serialize, Deserialize)]
pub struct ZEdgeDef {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

// impl EmptyVec for ZEdgeDef {
//    type Item = ZEdgeDef;

//    fn empty_vec() -> &'static Vec<ZEdgeDef> {
//       static EMPTY_VEC: Vec<ZEdgeDef> = Vec::<ZEdgeDef>::new();
//       &EMPTY_VEC
//    }
// }

#[derive(Serialize, Deserialize)]
pub struct ZEdgeDataDef {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

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

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum ZGraphDefCategory {
   // GraphDef is complete, connecting input and output, and expanded
   // on construction.
   UserGraph = 0,
   // Library: Similar to UserGraph.
   LibraryGraph,
   // Library-like element not expanded for sizing, only if not
   // supported by renderer.
   LibraryDrawable,
   // Element not expanded for sizing, in set of renderer-required.
   BuiltinDrawable,
   // Source that introduces new free variable, and outputs weighted
   // with unit weight.
   FreeWeighted,
   // LP-understood operator, single Weighted output.
   OperatorWeighted,
   // Not intended to have anything renderable. Cannot output
   // Weighted. Can output multiple non-Weighted.
   OperatorGeneral,
   // A general expression, which must expand to a sub-graph, even if
   // to single ExpressionGraph.
   ExpressionGraph,
   // An expression that Zebraix evaluates in one go. Cannot output
   // Weighted. Can write multiple outputs.
   ExpressionEvaluator,
   // Converts weighted to real, demanding finalization.
   Finalizer,
   // Sink that consumes one or more weighted and adds to objective
   // function.
   Objective,
   // Source that introduces preset value.
   Preset,
}

impl Default for ZGraphDefCategory {
   fn default() -> Self {
      ZGraphDefCategory::UserGraph
   }
}

#[derive(Serialize, Deserialize)]
pub struct ZGraphDef {
   #[serde(default, skip_serializing_if = "is_default")]
   pub category: ZGraphDefCategory,

   pub name: String,
   #[serde(skip_serializing_if = "is_default")]
   pub description: Option<String>,

   #[serde(default)]
   pub nodes: JVec<ZNodeDef>,
   #[serde(default)]
   pub edges: JVec<ZEdgeDef>,
}
