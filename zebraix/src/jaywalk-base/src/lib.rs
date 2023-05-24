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

pub mod jaywalk_graph;

// Struct
//
// jaywalk
// rendering
// zebraix-layout: jaywalk+rendering.
// co-combined: dominance and co-dominance zebraix layouts. Narrow usage: merge to single jaywalk
//      with different default edges.
//
// graphs have index vectors and maps for which there are renumbering functions.
//
// Prime and obverse ordinals are not necessarily contiguous.
//
// Final (for rendering) graphs only have extrapolated source and sink if included in rendering.
// During manipulation begin and end markers can give begin and end ranges in index vectors that
// drop the extrapolated nodes as required.
//
// LOGICAL JAYWALK
//
// nodes_by_primary: Master node indices ordered by non-contiguous prime indices.
// nodes_by_obverse
// edges_by_nodes map<from-node, to-node> -> edge index.
//
// nodes vec<Node> The indices into these are the master node indices.
// edges vec<Edge> The indices into these are the master edge indices.
//
// Phantom indices are different from unrendered nodes. They are for spacing, not for graph
// connections.
//
// phantom_prime vec<prime index> After initial processing, ordered prime indices to be skipped.
// phantom_obverse vec<obverse index>
//
// EMBELLISHMENTS
//
// node_rendering vec<NodeRender>  The indices into these are the master node indices.
// edges_rendering vec<EdgeRender> The indices into these are the master edge indices.
//
// Rendering elements have derived_members that by name base rendering off like rendering
//      elements.  Included among these are "base" elements.
//
// base_node_rendering map<String> -> NodeRender
// base_edge_rendering map<String> -> EdgeRender
//
//
// Lib
//
// jaywalk-base: Including construct and basic manipulation utils, including codominance switch.
// render-layout: Everything except actual SVG and PNG specifics. Includes options.
// render-svg
// render-png
// fractional: Coords back into indices. Do we even have a serious use case for this?
// combined: Extra utils for creating graphs with combination of dominance and co-dominance.

#[cfg(test)]
mod tests {
   #[test]
   fn it_works() {
      assert_eq!(2 + 2, 4);
   }
}
