// Copyright 2026 Google LLC
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

pub mod conquer_iterator;
#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use std::cmp;
use std::cmp::Ordering;
use std::collections::VecDeque;
use zvx_base::is_default;

// Memory and intermediate storage is not treated as a key concern.  Domainance graphs are not
// intended (in this implementation) to have large numbers of nodes.  This is in contrast to
// many sort implementations, in which intermediate storage is a concern.

pub type IndexType = usize;
pub type RankType = i32;

// These are indices that are impossible in protobuf input repeated messages.
pub const SINK_IMPORT_PSEUDO_INDEX: IndexType = DominanceNode::default_index() - 1;
pub const SOURCE_IMPORT_PSEUDO_INDEX: IndexType = DominanceNode::default_index() - 2;

// DominanceNode indices index the vector of nodes in a DominanceGraph.
#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Eq, Clone)]
pub struct DominanceNode {
   #[serde(
      skip_serializing_if = "DominanceNode::is_default_rank",
      default = "DominanceNode::default_rank"
   )]
   pub prime_rank: RankType,
   #[serde(
      skip_serializing_if = "DominanceNode::is_default_rank",
      default = "DominanceNode::default_rank"
   )]
   pub obverse_rank: RankType,
   // The import_index is used contextually. If the data comes from a richer source one can pull,
   // say, rendering information.
   #[serde(
      skip_serializing_if = "DominanceNode::is_default_index",
      default = "DominanceNode::default_index"
   )]
   import_index: IndexType,

   #[serde(skip_serializing_if = "is_default")]
   pub parents: Vec<IndexType>,
   #[serde(skip_serializing_if = "is_default")]
   pub children: Vec<IndexType>,
}

impl DominanceNode {
   #[must_use]
   const fn default_index() -> IndexType {
      IndexType::MAX
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   const fn is_default_index(v: &IndexType) -> bool {
      *v == Self::default_index()
   }
   #[must_use]
   const fn default_rank() -> RankType {
      RankType::MIN
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   const fn is_default_rank(v: &RankType) -> bool {
      *v == Self::default_rank()
   }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DominanceGraph {
   // // Ranges of ranks that include any imputed source or sink.
   // prime_min: IndexType,
   // prime_max: IndexType,
   // obverse_min: IndexType,
   // obverse_max: IndexType,

   // Sources and sinks are only imputed if there is not already one.
   source_index: IndexType,
   pub imputed_source: bool,
   sink_index: IndexType,
   pub imputed_sink: bool,

   pub nodes: Vec<DominanceNode>,
}

impl Default for DominanceGraph {
   fn default() -> Self {
      Self {
         source_index: 0,
         imputed_source: true,
         sink_index: 0,
         imputed_sink: true,

         nodes: Vec::<DominanceNode>::default(),
      }
   }
}

#[inline]
fn rank_cmp(i: &DominanceNode, j: &DominanceNode) -> Ordering {
   match i.prime_rank.cmp(&j.prime_rank) {
      Ordering::Less => Ordering::Less,
      Ordering::Greater => Ordering::Greater,
      Ordering::Equal => i.obverse_rank.cmp(&j.obverse_rank),
   }
}

impl DominanceGraph {
   #[must_use]
   pub fn new_from_pairs(
      pairs: &[(RankType, RankType)],
      imputed_source: bool,
      imputed_sink: bool,
   ) -> Self {
      let mut node_vec = Vec::<DominanceNode>::with_capacity(pairs.len());
      for pair in pairs {
         node_vec.push(DominanceNode {
            prime_rank: pair.0,
            obverse_rank: pair.1,
            ..Default::default()
         });
      }

      Self { nodes: node_vec, imputed_source, imputed_sink, ..Default::default() }
   }

   // Create source and sink nodes as required. Sort nodes in increasing order of
   // prime rank.
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::missing_errors_doc)]
   pub fn flesh_out_graph_nodes(&mut self) -> Result<(), &'static str> {
      let mut min_obverse = RankType::MAX;
      let mut max_obverse = RankType::MIN;
      // Should ensure that at least 1 node.
      for (i, node) in self.nodes.iter_mut().enumerate() {
         node.import_index = i;
         min_obverse = cmp::min(min_obverse, node.obverse_rank);
         max_obverse = cmp::max(max_obverse, node.obverse_rank);
      }
      assert_eq!(self.nodes[1].import_index, 1);

      self.nodes.sort_unstable_by(rank_cmp);
      let min_prime = self.nodes.first().unwrap().prime_rank;
      let max_prime = self.nodes.last().unwrap().prime_rank;

      // If first and last nodes are not naturally root source and sink, force their creation.
      self.imputed_source |= self.nodes.first().unwrap().obverse_rank != min_obverse;
      self.imputed_sink |= self.nodes.last().unwrap().prime_rank != max_obverse;

      if self.imputed_source {
         self.nodes.insert(
            0,
            DominanceNode {
               prime_rank: min_prime - 1,
               obverse_rank: min_obverse - 1,
               import_index: SOURCE_IMPORT_PSEUDO_INDEX,
               ..Default::default()
            },
         );
      }
      if self.imputed_sink {
         self.nodes.push(DominanceNode {
            prime_rank: max_prime + 1,
            obverse_rank: max_obverse + 1,
            import_index: SINK_IMPORT_PSEUDO_INDEX,
            ..Default::default()
         });
      }

      self.source_index = 0;
      self.sink_index = self.nodes.len() - 1;

      // We could error on coincident nodes.
      Ok(())
   }

   // Apply order-dimension 2 properties to construct graph connections (edges).
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::missing_errors_doc)]
   pub fn connect_graph(&mut self) -> Result<(), &'static str> {
      let node_count: usize = self.nodes.len();
      let mut scratch_parents = vec![VecDeque::<IndexType>::new(); node_count];
      let mut scratch_children = vec![VecDeque::<IndexType>::new(); node_count];

      for c in (0..node_count).rev() {
         let child_obverse: RankType = self.nodes[c].obverse_rank;
         let mut max_parent_obverse = RankType::MIN; // For each node, keep max parent rank.
         for p in (0..c).rev() {
            // If there are two nodes with same prime, the highest obverse should be encountered
            // first.
            let parent_obverse = self.nodes[p].obverse_rank;
            if (parent_obverse <= child_obverse) && (parent_obverse > max_parent_obverse) {
               max_parent_obverse = parent_obverse;
               scratch_parents[c].push_back(p);
               scratch_children[p].push_back(c);
            }
         }
      }

      // Create efficient shrink-wrapped vector edge structures.  In the long run we may find
      // that building directly in place will work.
      //
      // The preferred final ordering is for children to be "left-to-right" and parents
      // "right-to-left", so that the relationships are independent of a 180-degree rotation of
      // the graph.
      for i in (0..node_count).rev() {
         let mut children = Vec::<IndexType>::from(scratch_children.pop().unwrap());
         children.reverse();
         self.nodes[i].children = children;
         self.nodes[i].children.shrink_to_fit();
         self.nodes[i].parents = Vec::<IndexType>::from(scratch_parents.pop().unwrap());
         self.nodes[i].parents.shrink_to_fit();
      }

      Ok(())
   }
}
