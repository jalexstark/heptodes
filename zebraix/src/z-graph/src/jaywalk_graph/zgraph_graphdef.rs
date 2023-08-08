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

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ZLinkPort {
   pub name: String,
   #[serde(skip_serializing_if = "is_default", default)]
   pub is_void: bool,
   #[serde(skip_serializing_if = "is_default", default)]
   pub is_graph_input: bool,
   pub src_node_name: String,
   pub src_port: String,
   #[serde(skip_serializing_if = "is_default")]
   pub src_port_full: Option<String>,
   #[serde(skip_serializing_if = "LinkedList::is_empty", default)]
   pub src_subfields: LinkedList<String>,
}

impl ZLinkPort {
   // It is not necessary to re-process a link if only the name is changed.
   #[inline(always)]
   fn finish(&mut self) {
      assert!(self.src_port_full.is_none());
      assert!(self.src_subfields.is_empty());
      let ports_seq: Vec<&str> = self.src_port.split('.').collect();
      if ports_seq.len() > 1 {
         self.src_port_full = Some(self.src_port.clone());
         self.src_subfields.clear();
         for s in &ports_seq[1..] {
            self.src_subfields.push_back(s.to_string());
         }
         self.src_port = ports_seq[0].to_string();
      }

      self.is_void = self.src_port == "void";
      self.is_graph_input = self.src_node_name == "inputs";
   }

   pub fn has_subfield(&self) -> bool {
      !self.src_subfields.is_empty()
   }

   pub fn src_port_full_name(&self) -> String {
      if self.src_port_full.is_some() {
         self.src_port_full.as_ref().unwrap().clone()
      } else {
         self.src_port.clone()
      }
   }

   #[must_use]
   pub fn pop(&mut self) -> String {
      assert!(!self.src_subfields.is_empty());
      let popped_name: String = self.src_subfields.pop_front().unwrap();
      self.src_port = popped_name.clone();
      assert_ne!(popped_name, "void", "It is illegal for a sub-field name to be \"void\".");

      popped_name
   }

   // The front of the downstream should have already been popped as needed. The merge further
   // removes the downstream `src_port`, replacing it with the upstream `src_port`. Indeed, all
   // of the source is taken from the upstream link.
   //
   // It is also permissible for the upstream to be a sub-field selection, in which case the
   // upstream chain becomes the front of a merged stream.
   //
   // This could be greatly simplified whenever the downstream linkport does not have subfield
   // spec.
   pub fn merge_upstream_into_down(upstream: &ZLinkPort, downstream: &mut ZLinkPort) {
      assert!(!upstream.is_void);
      assert!(!downstream.is_void);
      // downstream.name = unchanged;
      downstream.src_node_name = upstream.src_node_name.clone();
      downstream.src_port = upstream.src_port.clone();
      let mut new_port_full = upstream.src_port_full_name();
      new_port_full.push_str(&downstream.src_port_full_name()[..]);
      downstream.src_port_full = Some(new_port_full);
      for upper_field in upstream.src_subfields.iter().rev() {
         downstream.src_subfields.push_front(upper_field.clone());
      }
      downstream.is_void = upstream.is_void;
      downstream.is_graph_input = upstream.is_graph_input;
      assert!(!downstream.is_void, "It is illegal for source of sub-field to link to void.");
   }
}

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

#[derive(Clone, Serialize, Deserialize)]
pub struct PresetPiece(pub String, pub ZPiece);

// Says which internal node output ports are exposed as graph output
// ports, and sets external name.
//
// Port name, src node name, src port name.
#[derive(Clone, Serialize, Deserialize)]
pub struct ZPortDef(pub String, pub String, pub String);

#[derive(Clone, Serialize, Deserialize)]
pub struct ZNodeDef {
   #[serde(skip_serializing, default)]
   pub is_precompiled: bool,
   pub name: String,
   #[serde(skip_serializing_if = "is_default")]
   pub description: Option<String>,

   pub element: ZNodeTypeFinder,

   #[serde(skip_serializing_if = "Vec::is_empty", default)]
   pub edges: Vec<ZEdgeDef>,

   #[serde(skip_serializing_if = "Vec::is_empty", default)]
   pub links: Vec<ZPortDef>,

   // all_links basicaly concatenates the link definitions in links and edges.
   #[serde(skip_serializing, default)]
   pub all_links: Vec<ZLinkPort>,

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
      assert!(self.all_links.is_empty());
      for port_def in &self.links {
         self.all_links.push(ZLinkPort {
            name: port_def.0.clone(),
            src_node_name: port_def.1.clone(),
            src_port: port_def.2.clone(),
            ..Default::default()
         });
      }
      for edge in &self.edges {
         for connection in &edge.connections {
            self.all_links.push(ZLinkPort {
               name: connection.1.clone(),
               src_node_name: edge.src_node.clone(),
               src_port: connection.0.clone(),
               ..Default::default()
            });
         }
      }
      for link in &mut self.all_links {
         link.finish();
      }
      self.is_precompiled = true;
   }
}

impl ZGraphDef {
   pub fn precompile(&mut self) -> Result<&mut Self, ZGraphError> {
      assert!(!self.is_precompiled);
      assert!(self.output_ports_as_links.is_empty());
      self.output_ports_as_links.clear();
      for port_def in &self.output_ports {
         let mut link = ZLinkPort {
            name: port_def.0.clone(),
            src_node_name: port_def.1.clone(),
            src_port: port_def.2.clone(),
            ..Default::default()
         };
         link.finish();
         self.output_ports_as_links.push(link);
      }
      self.is_precompiled = true;

      for node in &mut self.nodes {
         node.precompile();
      }
      Ok(self)
   }
}
