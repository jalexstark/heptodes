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

use crate::jaywalk_graph::jaywalk_foundation::is_default;
use crate::jaywalk_graph::zgraph_base::PortPieceTyped;
use crate::jaywalk_graph::zgraph_base::ZCanvas;
use crate::jaywalk_graph::zgraph_base::ZGraphError;
use crate::jaywalk_graph::zgraph_base::ZNodeTypeFinder;
use crate::jaywalk_graph::zgraph_base::ZPiece;
use serde::{Deserialize, Serialize};
use std::collections::LinkedList;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
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

// Name among source nodes's outputs, name among dest node's inputs.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Connection(pub String, pub String);

#[derive(Clone, Serialize, Deserialize)]
pub struct ZEdgeDef {
   #[serde(skip_serializing_if = "is_default")]
   pub name: Option<String>,

   pub src_node: String,
   // pub dest_node: String,
   pub connections: Vec<Connection>, // Input-output.
}

// #[derive(Serialize, Deserialize)]
// pub struct ZEdgeDataDef {
//    #[serde(skip_serializing_if = "is_mult_ident_f64")]
//    #[serde(default = "mult_ident_f64")]
//    pub field_c: f64,
// }

#[derive(Clone, Serialize, Deserialize)]
pub struct PresetPiece(pub String, pub ZPiece);

// Says which internal node output ports are exposed as graph output
// ports, and sets external name.
//
// Port name, src node name, src port name.
#[derive(Clone, Serialize, Deserialize)]
pub struct ZPortDef(pub String, pub String, pub String);

#[derive(Clone, Deserialize, Default)]
pub struct ZLinkPort {
   pub name: String,
   pub is_void: bool,
   pub src_node_name: String,
   pub src_port: String,
   pub src_port_full: Option<String>,
   pub src_subfields: LinkedList<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ZNodeDef {
   #[serde(skip_serializing, default)]
   pub is_precompiled: bool,
   pub name: String,
   #[serde(skip_serializing_if = "is_default")]
   pub description: Option<String>,

   // #[serde(skip_serializing_if = "is_default")]
   // #[serde(default)]
   pub element: ZNodeTypeFinder,

   #[serde(skip_serializing_if = "Vec::is_empty", default)]
   pub edges: Vec<ZEdgeDef>,

   // Input, output and preset nodes only. Others come from registered elements.
   #[serde(skip_serializing_if = "Vec::is_empty", default)]
   pub ports: Vec<PortPieceTyped>,

   // Preset data nodes can input, in which case fields are overridden by merging.
   #[serde(skip_serializing_if = "Vec::is_empty", default)]
   pub preset_data: Vec<PresetPiece>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ZGraphDef {
   #[serde(skip_serializing, default)]
   pub is_precompiled: bool,
   #[serde(default, skip_serializing_if = "is_default")]
   pub category: ZGraphDefCategory,

   pub name: String,
   #[serde(skip_serializing_if = "Option::is_none")]
   pub description: Option<String>,

   #[serde(skip_serializing_if = "Vec::is_empty", default)]
   pub inputs: Vec<PortPieceTyped>,

   // Subgraphs should not have outputs, but instead output_ports, for
   // which the types are inferred.
   #[serde(skip_serializing_if = "Vec::is_empty", default)]
   pub outputs: Vec<PortPieceTyped>,

   #[serde(default)]
   pub nodes: Vec<ZNodeDef>,

   #[serde(default)]
   pub output_ports: Vec<ZPortDef>,

   #[serde(skip_serializing, default)]
   pub output_ports_as_links: Vec<ZLinkPort>,

   #[serde(skip_serializing_if = "Option::is_none")]
   pub canvas: Option<ZCanvas>,
}

impl ZNodeDef {
   pub fn precompile(&mut self) {
      assert!(!self.is_precompiled);
      //
      self.is_precompiled = true;
   }
}

impl ZGraphDef {
   pub fn precompile(&mut self) -> Result<&mut Self, ZGraphError> {
      assert!(!self.is_precompiled);
      assert!(self.output_ports_as_links.is_empty());
      self.output_ports_as_links.clear();
      for port_def in &self.output_ports {
         self.output_ports_as_links.push(ZLinkPort {
            name: port_def.0.clone(),
            src_node_name: port_def.1.clone(),
            src_port: port_def.2.clone(),
            is_void: port_def.2 == "void",
            ..Default::default()
         });
      }
      self.is_precompiled = true;

      for node in &mut self.nodes {
         node.precompile();
      }
      Ok(self)
   }
}
