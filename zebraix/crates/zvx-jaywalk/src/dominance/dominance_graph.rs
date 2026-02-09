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

#[cfg(test)]
mod tests;

use crate::dominance::conquer_iterator::MergeStep;
use crate::dominance::conquer_iterator::MinusPlusShift;
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

struct AuxData {
   left_root: IndexType,
   right_root: IndexType,
   left_terminal: IndexType,
   right_terminal: IndexType,
   final_root: IndexType,
   final_terminal: IndexType,
   // left_merge_prev: IndexType,
   // right_merge_prev: IndexType,
   // partial_rev: Vec<IndexType>,
   left_block_low: IndexType,
   left_block_mid: IndexType,
   left_block_high: IndexType,
   right_block_low: IndexType,
   right_block_mid: IndexType,
   right_block_high: IndexType,

   // left_tracer_rank: IndexType,
   // right_tracer_rank: IndexType,
   // reverse_links: Vec<IndexType>, // Used for (partial) backward sorted links.
   scratch_parents: Vec<VecDeque<IndexType>>,
   scratch_children: Vec<VecDeque<IndexType>>,
   accum_parents: Vec<VecDeque<IndexType>>,
   accum_children: Vec<VecDeque<IndexType>>,
   cross_wn_links: Vec<IndexType>,
   cross_es_links: Vec<IndexType>,
   // Journalling across merge steps.
   sorted_next: Vec<IndexType>,
   sorted_prev: Vec<IndexType>,
   // A block's sort root is in the lowest index for the block.  Initialized as N separate
   // size-1 sorts.  May be used as scratch space during a merge step.
   sorted_roots: Vec<IndexType>,
}

// impl AuxData {
//    fn new(size: usize) -> Self {

//    }
// }

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
      self.imputed_sink |= self.nodes.last().unwrap().obverse_rank != max_obverse;

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
   //
   // This version does not work for graphs with a single node.
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::missing_errors_doc)]
   #[cfg(test)]
   pub fn reference_backwards_connect_graph(&mut self) -> Result<(), &'static str> {
      let node_count: usize = self.nodes.len();
      let mut scratch_parents = vec![VecDeque::<IndexType>::new(); node_count];
      let mut scratch_children = vec![VecDeque::<IndexType>::new(); node_count];

      let iter = MinusPlusShift::new(node_count);
      for merge_step in iter {
         for c in (merge_step.middle..merge_step.upper).rev() {
            let child_obverse: RankType = self.nodes[c].obverse_rank;
            let mut max_parent_obverse = if scratch_parents[c].is_empty() {
               RankType::MIN
            } else {
               self.nodes[*scratch_parents[c].back().unwrap()].obverse_rank
            }; // For each node, keep max parent rank.
            for p in (merge_step.lower..merge_step.middle).rev() {
               // If there are two nodes with same prime, the highest obverse should be encountered
               // first.
               let parent_obverse = self.nodes[p].obverse_rank;
               if (parent_obverse <= child_obverse) && (parent_obverse > max_parent_obverse) {
                  max_parent_obverse = parent_obverse;
                  scratch_parents[c].push_back(p);
               }
            }
         }
      }

      for c in (0..node_count).rev() {
         for p in &scratch_parents[c] {
            scratch_children[*p].push_back(c);
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

   // Apply order-dimension 2 properties to construct graph connections (edges).
   //
   // This version does not work for graphs with a single node.
   #[cfg(test)]
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::missing_errors_doc)]
   pub fn reference_forewards_connect_graph(&mut self) -> Result<(), &'static str> {
      let node_count: usize = self.nodes.len();
      let mut scratch_children = vec![VecDeque::<IndexType>::new(); node_count];
      let mut scratch_parents = vec![VecDeque::<IndexType>::new(); node_count];

      let iter = MinusPlusShift::new(node_count);
      for merge_step in iter {
         #[allow(clippy::needless_range_loop)]
         for p in merge_step.lower..merge_step.middle {
            let parent_obverse: RankType = self.nodes[p].obverse_rank;
            let mut min_child_obverse = if scratch_children[p].is_empty() {
               RankType::MAX
            } else {
               self.nodes[*scratch_children[p].back().unwrap()].obverse_rank
            }; // For each node, keep max child rank.
            for c in merge_step.middle..merge_step.upper {
               // If there are two nodes with same prime, the highest obverse should be encountered
               // first.
               let child_obverse = self.nodes[c].obverse_rank;
               if (child_obverse >= parent_obverse) && (child_obverse < min_child_obverse) {
                  min_child_obverse = child_obverse;
                  scratch_children[p].push_back(c);
               }
            }
         }
      }

      for (p, children) in scratch_children.iter().enumerate() {
         for c in children {
            scratch_parents[*c].push_front(p);
         }
      }

      // Create efficient shrink-wrapped vector edge structures.  In the long run we may find
      // that building directly in place will work.
      //
      // The preferred final ordering is for parents to be "left-to-right" and children
      // "right-to-left", so that the relationships are independent of a 180-degree rotation of
      // the graph.
      for i in (0..node_count).rev() {
         self.nodes[i].parents = Vec::<IndexType>::from(scratch_parents.pop().unwrap());
         self.nodes[i].parents.shrink_to_fit();
         self.nodes[i].children = Vec::<IndexType>::from(scratch_children.pop().unwrap());
         self.nodes[i].children.shrink_to_fit();
      }

      Ok(())
   }

   // // Apply order-dimension 2 properties to construct graph connections (edges).
   // //
   // // This version does not work for graphs with a single node.
   // #[allow(clippy::missing_panics_doc)]
   // #[allow(clippy::missing_errors_doc)]
   // pub fn connect_graph(&mut self) -> Result<(), &'static str> {
   //    let node_count: usize = self.nodes.len();
   //    let final_node_index = node_count - 1;
   //    let mut scratch_parents = vec![VecDeque::<IndexType>::new(); node_count];
   //    let mut scratch_children = vec![VecDeque::<IndexType>::new(); node_count];

   //    let mut leftmost_sw_parent = vec![final_node_index; node_count];
   //    // Sentinel leftmost_ne_child == children.last() when set, but always valid.
   //    let mut leftmost_ne_child = vec![final_node_index; node_count];
   //    // Potentially sentinel-like may be valid, since all sorts end with final node.
   //    let mut sorted_next = vec![final_node_index; node_count];
   //    // A block's sort root is in the lowest index for the block.  Initialized as N separate
   //    // size-1 sorts.
   //    let mut sorted_roots: Vec<IndexType> = (0..node_count).collect();

   //    let iter = MinusPlusShift::new(node_count);
   //    for merge_step in iter {
   //       for c in (merge_step.middle..merge_step.upper).rev() {
   //          let child_obverse: RankType = self.nodes[c].obverse_rank;
   //          let mut max_parent_obverse = if scratch_parents[c].is_empty() {
   //             RankType::MIN
   //          } else {
   //             self.nodes[*scratch_parents[c].back().unwrap()].obverse_rank
   //          }; // For each node, keep max parent rank.
   //          for p in (merge_step.lower..merge_step.middle).rev() {
   //             // If there are two nodes with same prime, the highest obverse should be encountered
   //             // first.
   //             let parent_obverse = self.nodes[p].obverse_rank;
   //             if (parent_obverse <= child_obverse) && (parent_obverse > max_parent_obverse) {
   //                max_parent_obverse = parent_obverse;
   //                scratch_parents[c].push_back(p);
   //             }
   //          }
   //       }

   //       match merge_step.singles_to_add {
   //          2 => {
   //             // Technically can be handled in general routine.
   //             if self.nodes[merge_step.lower].obverse_rank
   //                > self.nodes[merge_step.middle].obverse_rank
   //             {
   //                sorted_roots[merge_step.lower] = merge_step.middle;
   //                sorted_next[merge_step.middle] = merge_step.lower;
   //                leftmost_sw_parent[merge_step.middle] = merge_step.lower;
   //             } else {
   //                sorted_roots[merge_step.lower] = merge_step.lower;
   //                sorted_next[merge_step.lower] = merge_step.middle;
   //                leftmost_ne_child[merge_step.lower] = merge_step.middle;
   //             }
   //          }
   //          _ => {
   //             let mut right_tracer = sorted_roots[merge_step.middle];
   //             let mut left_tracer = sorted_roots[merge_step.lower];
   //             let mut right_rank = self.nodes[right_tracer].obverse_rank;
   //             let mut left_rank = self.nodes[left_tracer].obverse_rank;

   //             // TODO: skip everything on R if all below L.

   //             // Advance R so that R tracer is "immediately" above a L node.
   //             if left_rank > right_rank {
   //                sorted_roots[merge_step.lower] = right_tracer;
   //                let mut next_right_tracer = leftmost_sw_parent[right_tracer];
   //                let mut next_right_rank = self.nodes[next_right_tracer].obverse_rank;
   //                while left_rank > next_right_rank {
   //                   right_tracer = next_right_tracer;
   //                   next_right_tracer = leftmost_sw_parent[right_tracer];
   //                   next_right_rank = self.nodes[next_right_tracer].obverse_rank;
   //                }

   //                // Replumb R chain to L tracer.
   //                leftmost_sw_parent[right_tracer] = left_tracer;
   //                next_right_tracer = leftmost_ne_child[right_tracer];
   //                next_right_rank = self.nodes[next_right_tracer].obverse_rank;
   //                while left_rank > next_right_rank {
   //                   right_tracer = next_right_tracer;
   //                   leftmost_sw_parent[right_tracer] = left_tracer;
   //                   next_right_tracer = leftmost_ne_child[right_tracer];
   //                   next_right_rank = self.nodes[next_right_tracer].obverse_rank;
   //                }
   //                // At this point R is just below first L. Stitch sort while advancing one on R.
   //                next_right_tracer = sorted_next[right_tracer];
   //                sorted_next[right_tracer] = left_tracer;
   //                right_tracer = next_right_tracer;
   //                right_rank = self.nodes[right_tracer].obverse_rank;
   //             }
   //             // else:
   //             // Already sorted_roots[merge_step.lower] == left_tracer.

   //             let mut next_left_tracer = sorted_next[left_tracer];
   //             let mut next_left_rank = self.nodes[next_left_tracer].obverse_rank;
   //             assert!(next_left_rank < right_rank);
   //             // Loop one less than L size.
   //             for _i in merge_step.lower..merge_step.middle - 1 {
   //                if next_left_rank < right_rank {
   //                   // Move 1 on L until L is below R but next L is above R.
   //                   left_tracer = next_left_tracer;
   //                   next_left_tracer = sorted_next[left_tracer];
   //                   next_left_rank = self.nodes[next_left_tracer].obverse_rank;
   //                   continue;
   //                }
   //                left_rank = self.nodes[left_tracer].obverse_rank;
   //                assert!(next_left_rank > right_rank);
   //                assert!(left_rank < right_rank);
   //             }
   //          }
   //       }
   //    }

   //    for c in (0..node_count).rev() {
   //       for p in &scratch_parents[c] {
   //          scratch_children[*p].push_back(c);
   //       }
   //    }

   //    // Create efficient shrink-wrapped vector edge structures.  In the long run we may find
   //    // that building directly in place will work.
   //    //
   //    // The preferred final ordering is for children to be "left-to-right" and parents
   //    // "right-to-left", so that the relationships are independent of a 180-degree rotation of
   //    // the graph.
   //    for i in (0..node_count).rev() {
   //       let mut children = Vec::<IndexType>::from(scratch_children.pop().unwrap());
   //       children.reverse();
   //       self.nodes[i].children = children;
   //       self.nodes[i].children.shrink_to_fit();
   //       self.nodes[i].parents = Vec::<IndexType>::from(scratch_parents.pop().unwrap());
   //       self.nodes[i].parents.shrink_to_fit();
   //    }

   //    Ok(())
   // }

   // Build "less than" shaped block on R.
   //
   // On entry: L tracer would be just below current R tracer.  Next L (next in unmerged L) is
   // past current R. R next is considered invalid.
   //
   // On exit: R current is advanced to end of block. R next is the next, but R current may
   // point to the end of the R input.  Next L is just after R current.
   #[inline]
   fn build_right_block(
      &self,
      aux: &mut AuxData,
      left_next_rank: RankType,
      right_current: &mut IndexType,
   ) {
      aux.right_block_low = *right_current;

      // OPT: First traverse WN.  The last will be min_prime.   Then go sort-sortwise.
      // OPT: Cache WN traversal to save work later.

      let mut right_next = aux.sorted_next[*right_current];
      let mut right_mid = *right_current;
      let mut min_prime = *right_current;
      while (self.nodes[right_next].obverse_rank < left_next_rank)
         && (*right_current != aux.right_terminal)
      {
         *right_current = right_next;
         right_next = aux.sorted_next[*right_current];

         if *right_current < min_prime {
            min_prime = *right_current;
            right_mid = *right_current;
         }
      }
      assert!(self.nodes[*right_current].obverse_rank < left_next_rank);
      assert!(
         (self.nodes[right_next].obverse_rank >= left_next_rank)
            || (*right_current == aux.right_terminal)
      );

      aux.right_block_mid = right_mid;
      aux.right_block_high = *right_current;
   }

   // Variation for terminal block, but where L is effectively invalid, so consume remainder of
   // R.
   #[inline]
   #[allow(clippy::unused_self)]
   fn build_right_block_terminal(&self, aux: &mut AuxData, right_current: &mut IndexType) {
      aux.right_block_low = *right_current;

      // OPT: First traverse WN.  The last will be min_prime.   Then jump to terminal.
      // OPT: Cache WN traversal to save work later.

      let right_terminal = aux.right_terminal;
      let mut min_prime = *right_current;
      while *right_current != right_terminal {
         *right_current = aux.sorted_next[*right_current];
         if *right_current < min_prime {
            min_prime = *right_current;
         }
      }

      aux.right_block_mid = min_prime;
      aux.right_block_high = right_terminal;
   }

   #[inline]
   fn build_left_block(
      &self,
      aux: &mut AuxData,
      right_next_rank: RankType,
      left_current: &mut IndexType,
   ) {
      aux.left_block_low = *left_current;

      // OPT: First traverse last child.  The last will be max_prime.  Then go sort-sortwise.
      // OPT: Cache child traversal to save work later.

      let mut left_next = aux.sorted_next[*left_current];
      let mut max_prime = *left_current;
      while (self.nodes[left_next].obverse_rank <= right_next_rank)
         && (*left_current != aux.left_terminal)
      {
         *left_current = left_next;
         left_next = aux.sorted_next[*left_current];

         if *left_current >= max_prime {
            max_prime = *left_current;
         }
      }
      assert!(self.nodes[*left_current].obverse_rank <= right_next_rank);
      assert!(
         (self.nodes[left_next].obverse_rank > right_next_rank)
            || (*left_current == aux.left_terminal)
      );

      aux.left_block_mid = max_prime;
      aux.left_block_high = *left_current;
   }

   // Variation for terminal block, but where R is effectively invalid, so consume remainder of
   // L.
   #[inline]
   #[allow(clippy::unused_self)]
   fn build_left_block_terminal(&self, aux: &mut AuxData, left_current: &mut IndexType) {
      aux.left_block_low = *left_current;

      // OPT: First traverse last child.  The last will be max_prime.  Then jump to terminal.
      // OPT: Cache child traversal to save work later.

      let left_terminal = aux.left_terminal;
      let mut max_prime = *left_current;
      while *left_current != left_terminal {
         *left_current = aux.sorted_next[*left_current];
         if *left_current >= max_prime {
            max_prime = *left_current; // NOT really max_prime, since not the prime_rank.
         }
      }

      aux.left_block_mid = max_prime;
      aux.left_block_high = left_terminal;
   }

   #[inline]
   fn stitch_right_below_left(
      aux: &mut AuxData,
      right_tracer: &mut IndexType,
      left_tracer: IndexType,
   ) {
      let right_next = aux.sorted_next[*right_tracer];
      aux.sorted_next[*right_tracer] = left_tracer;
      aux.sorted_prev[left_tracer] = *right_tracer;
      // aux.right_merge_prev = aux.right_block_high;
      *right_tracer = right_next;
   }

   #[inline]
   fn stitch_left_below_right(
      aux: &mut AuxData,
      left_tracer: &mut IndexType,
      right_tracer: IndexType,
   ) {
      let left_next = aux.sorted_next[*left_tracer];
      aux.sorted_next[*left_tracer] = right_tracer;
      aux.sorted_prev[right_tracer] = *left_tracer;
      // aux.left_merge_prev = aux.left_block_high;
      *left_tracer = left_next;
   }

   #[inline]
   fn stitch_right_between_left(
      aux: &mut AuxData,
      right_tracer: &mut IndexType,
      left_above_tracer: IndexType,
   ) {
      assert!(*right_tracer == aux.right_block_high);
      let right_next = aux.sorted_next[*right_tracer];
      // let low = aux.right_block_low;
      let high = aux.right_block_high;
      // let left_below = aux.left_merge_prev;
      // This may duplicate work with the following stitch_left_between_right.
      // aux.sorted_next[left_below] = low;
      // aux.sorted_prev[low] = left_below;
      aux.sorted_next[high] = left_above_tracer;
      aux.sorted_prev[left_above_tracer] = high;
      // aux.right_merge_prev = high;
      *right_tracer = right_next;
   }

   #[inline]
   fn stitch_left_between_right(
      aux: &mut AuxData,
      left_tracer: &mut IndexType,
      right_above_tracer: IndexType,
   ) {
      assert!(*left_tracer == aux.left_block_high);
      let left_next = aux.sorted_next[*left_tracer];
      // let low = aux.left_block_low;
      let high = aux.left_block_high;
      // let right_below = aux.right_merge_prev;
      // aux.sorted_next[right_below] = low;
      // aux.sorted_prev[low] = right_below;
      aux.sorted_next[high] = right_above_tracer;
      aux.sorted_prev[right_above_tracer] = high;
      // aux.left_merge_prev = high;
      *left_tracer = left_next;
   }

   // Updates of cross links for each node in parent-child in block's chains, with R block below
   // L block.
   #[inline]
   fn apply_downwards_updates(aux: &mut AuxData) {
      {
         let mut current_right = aux.right_block_high;
         aux.cross_wn_links[current_right] = aux.left_block_low;
         while current_right != aux.right_block_mid {
            current_right = *aux.accum_parents[current_right].back().unwrap();
            aux.cross_wn_links[current_right] = aux.left_block_low;
         }
      }
      {
         let mut current_left = aux.left_block_low;
         aux.cross_es_links[current_left] = aux.right_block_high;
         while current_left != aux.left_block_mid {
            current_left = *aux.accum_children[current_left].back().unwrap();
            aux.cross_es_links[current_left] = aux.right_block_high;
         }
      }
   }

   // Append parents and children between blocks, R above L.
   #[inline]
   fn apply_upwards_appending(&self, aux: &mut AuxData, merge_step: &MergeStep) {
      // Build stack of parents on L, using sorted_roots as scratch space.
      let mut left_topmost = merge_step.lower;
      {
         let mut current_left = aux.left_block_high;
         aux.sorted_roots[left_topmost] = current_left;
         left_topmost += 1;
         while current_left != aux.left_block_mid {
            current_left = aux.cross_es_links[current_left];
            aux.sorted_roots[left_topmost] = current_left;
            left_topmost += 1;
         }
      }

      // Build stack of children on R, using sorted_roots as scratch space.  This stack grows
      // downwards.  We push the maximum set required, which is that for the rightmost L parent.
      let mut right_bottom = merge_step.upper;
      {
         let index_of_max_left = aux.accum_children[aux.left_block_mid].back();
         let obverse_limit =
            index_of_max_left.map_or(RankType::MAX, |i| self.nodes[*i].obverse_rank);
         let mut current_right = aux.right_block_low;
         right_bottom -= 1;
         aux.sorted_roots[right_bottom] = current_right;

         while (aux.cross_wn_links[current_right] != IndexType::MAX)
            && (self.nodes[current_right].obverse_rank < obverse_limit)
         {
            current_right = aux.cross_wn_links[current_right];
            right_bottom -= 1;
            aux.sorted_roots[right_bottom] = current_right;
         }
      }

      for i in (merge_step.lower..left_topmost).rev() {
         let parent_i = aux.sorted_roots[i];
         // OPT: After the first one (for the L block mid), there must be a child.
         let index_of_max_left = aux.accum_children[parent_i].back();
         let obverse_limit =
            index_of_max_left.map_or(RankType::MAX, |i| self.nodes[*i].obverse_rank);
         // First pop any unneeded children.  (Those above the existing child of current parent).
         // OPT: For the first one none are ever popped.
         while self.nodes[aux.sorted_roots[right_bottom]].obverse_rank >= obverse_limit {
            right_bottom += 1;
         }

         for j in right_bottom..merge_step.upper {
            let child_j = aux.sorted_roots[j];
            aux.accum_children[parent_i].push_back(child_j);
            aux.accum_parents[child_j].push_back(parent_i);
         }
      }
   }

   // Apply order-dimension 2 properties to construct graph connections (edges).
   //
   // This version does not work for graphs with a single node.
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::too_many_lines)]
   #[allow(clippy::cognitive_complexity)]
   pub fn connect_graph(&mut self) -> Result<(), &'static str> {
      let node_count: usize = self.nodes.len();
      let final_node_index = node_count - 1;

      // AAAA dbg!(self.nodes.iter().map(|n| n.obverse_rank).collect::<Vec<_>>());
      let mut aux = AuxData {
         // Journalling within merge steps.
         left_root: 0,
         right_root: 0,
         left_terminal: 0,
         right_terminal: 0,
         final_root: 0, // Finals depend on ordering of sub-sort ends.
         final_terminal: 0,
         // left_merge_prev: IndexType::MAX,
         // right_merge_prev: IndexType::MAX,
         // partial_rev: vec![final_node_index; node_count],
         left_block_low: 0,
         left_block_mid: 0,
         left_block_high: 0,
         right_block_low: 0,
         right_block_mid: 0,
         right_block_high: 0,

         // Construction of graph, perhaps in different storage to final form.
         scratch_parents: vec![VecDeque::<IndexType>::new(); node_count],
         scratch_children: vec![VecDeque::<IndexType>::new(); node_count],
         accum_parents: vec![VecDeque::<IndexType>::new(); node_count],
         accum_children: vec![VecDeque::<IndexType>::new(); node_count],
         cross_wn_links: vec![IndexType::MAX; node_count],
         cross_es_links: vec![IndexType::MAX; node_count],

         // Journalling across merge steps.
         sorted_next: vec![final_node_index; node_count],
         // reverse_links: vec![0_usize; node_count],
         sorted_prev: vec![final_node_index; node_count],
         // A sub-sort's sort root is in the lowest index for the block.  A sub-sort's sort
         // terminus is in the highest index for the block.  A block's Initialized as N separate
         // size-1 sorts.  May be used as scratch space during a merge step.
         sorted_roots: (0..node_count).collect::<Vec<IndexType>>(),
      };
      // let mut leftmost_sw_parent = vec![final_node_index; node_count];
      // // Sentinel leftmost_ne_child == children.last() when set, but always valid.
      // let mut leftmost_ne_child = vec![final_node_index; node_count];
      // // Potentially sentinel-like may be valid, since all sorts end with final node.

      let iter = MinusPlusShift::new(node_count);
      for merge_step in iter {
         #[allow(clippy::needless_range_loop)]
         for p in merge_step.lower..merge_step.middle {
            let parent_obverse: RankType = self.nodes[p].obverse_rank;
            let mut min_child_obverse = if aux.scratch_children[p].is_empty() {
               RankType::MAX
            } else {
               self.nodes[*aux.scratch_children[p].back().unwrap()].obverse_rank
            }; // For each node, keep max child rank.
            for c in merge_step.middle..merge_step.upper {
               // If there are two nodes with same prime, the highest obverse should be encountered
               // first.
               let child_obverse = self.nodes[c].obverse_rank;
               if (child_obverse >= parent_obverse) && (child_obverse < min_child_obverse) {
                  min_child_obverse = child_obverse;
                  aux.scratch_children[p].push_back(c);
               }
            }
         }

         // AAAA dbg!((merge_step.lower, merge_step.middle, merge_step.upper));

         if merge_step.singles_to_add == 2 {
            // Technically can be handled in general routine.
            if self.nodes[merge_step.lower].obverse_rank
               > self.nodes[merge_step.middle].obverse_rank
            {
               aux.sorted_roots[merge_step.lower] = merge_step.middle;
               aux.sorted_roots[merge_step.middle] = merge_step.lower; // Terminal.
               aux.sorted_next[merge_step.middle] = merge_step.lower;
               aux.sorted_prev[merge_step.lower] = merge_step.middle;
               aux.cross_wn_links[merge_step.middle] = merge_step.lower;
               aux.cross_es_links[merge_step.lower] = merge_step.middle;
               // leftmost_sw_parent[merge_step.middle] = merge_step.lower;
            } else {
               aux.sorted_roots[merge_step.lower] = merge_step.lower;
               aux.sorted_roots[merge_step.middle] = merge_step.middle;
               aux.sorted_next[merge_step.lower] = merge_step.middle;
               aux.sorted_prev[merge_step.middle] = merge_step.lower;
               aux.accum_children[merge_step.lower].push_back(merge_step.middle);
               aux.accum_parents[merge_step.middle].push_back(merge_step.lower);
               // leftmost_ne_child[merge_step.lower] = merge_step.middle;
            }
         } else {
            aux.left_root = aux.sorted_roots[merge_step.lower];
            aux.right_root = aux.sorted_roots[merge_step.middle];
            aux.left_terminal = aux.sorted_roots[merge_step.middle - 1];
            aux.right_terminal = aux.sorted_roots[merge_step.upper - 1];
            let mut left_tracer = aux.left_root;
            let mut right_tracer = aux.right_root;

            'core_step_work: {
               let left_root_rank = self.nodes[aux.left_root].obverse_rank;
               let right_root_rank = self.nodes[aux.right_root].obverse_rank;

               if left_root_rank > right_root_rank {
                  aux.final_root = aux.right_root;
               } else {
                  aux.final_root = aux.left_root;
               }
               if self.nodes[aux.left_terminal].obverse_rank
                  > self.nodes[aux.right_terminal].obverse_rank
               {
                  aux.final_terminal = aux.left_terminal;
               } else {
                  aux.final_terminal = aux.right_terminal;
               }

               if left_root_rank > right_root_rank {
                  {
                     // Next after this block.
                     let left_rank = self.nodes[left_tracer].obverse_rank;
                     self.build_right_block(&mut aux, left_rank, &mut right_tracer);

                     // L is root, so just tuck before, with no need to adjust root.
                     // No L block, so no need to do apply_upwards_appending.
                     Self::stitch_right_below_left(&mut aux, &mut right_tracer, left_tracer);
                  }

                  {
                     // Half the main loop, in order to align two root conditions.
                     if aux.right_block_high == aux.right_terminal {
                        self.build_left_block_terminal(&mut aux, &mut left_tracer);
                        Self::apply_downwards_updates(&mut aux);

                        break 'core_step_work;
                     }
                     {
                        // Next after this block.
                        let right_rank = self.nodes[right_tracer].obverse_rank;
                        self.build_left_block(&mut aux, right_rank, &mut left_tracer);

                        Self::apply_downwards_updates(&mut aux);
                        Self::stitch_left_between_right(&mut aux, &mut left_tracer, right_tracer);
                     }
                  }
               } else {
                  {
                     // Next after this block.
                     let right_rank = self.nodes[right_tracer].obverse_rank;
                     self.build_left_block(&mut aux, right_rank, &mut left_tracer);

                     {
                        let left_next = aux.sorted_next[left_tracer];
                        assert!(self.nodes[left_tracer].obverse_rank <= right_rank);
                        assert!(
                           (self.nodes[left_next].obverse_rank > right_rank)
                              || (left_tracer == aux.left_terminal)
                        );
                     }

                     // R is root, so just tuck before, with no need to adjust root.
                     Self::stitch_left_below_right(&mut aux, &mut left_tracer, right_tracer);
                  }
               }
               // let mut counter = 0i32;

               {
                  // dbg!("Loopiness");
                  loop {
                     // dbg!("Loopy");
                     // dbg!(counter);
                     // counter += 1;
                     if aux.left_block_high == aux.left_terminal {
                        self.build_right_block_terminal(&mut aux, &mut right_tracer);
                        self.apply_upwards_appending(&mut aux, &merge_step);

                        break 'core_step_work;
                     }

                     // dbg!("Loopy A");
                     {
                        let left_rank = self.nodes[left_tracer].obverse_rank;
                        assert!(left_rank > self.nodes[right_tracer].obverse_rank);
                        // Next after this block is left_rank.
                        self.build_right_block(&mut aux, left_rank, &mut right_tracer);

                        self.apply_upwards_appending(&mut aux, &merge_step);
                        Self::stitch_right_between_left(&mut aux, &mut right_tracer, left_tracer);
                     }

                     if aux.right_block_high == aux.right_terminal {
                        // dbg!("Loopy B");
                        self.build_left_block_terminal(&mut aux, &mut left_tracer);
                        Self::apply_downwards_updates(&mut aux);

                        break 'core_step_work;
                     }

                     // dbg!("Loopy C");
                     {
                        let right_rank = self.nodes[right_tracer].obverse_rank;

                        // Next after this block is right_rank.
                        self.build_left_block(&mut aux, right_rank, &mut left_tracer);

                        assert!(self.nodes[left_tracer].obverse_rank <= right_rank);
                        assert!(
                           (self.nodes[aux.sorted_next[left_tracer]].obverse_rank > right_rank)
                              || (left_tracer == aux.left_terminal)
                        );

                        Self::apply_downwards_updates(&mut aux);
                        Self::stitch_left_between_right(&mut aux, &mut left_tracer, right_tracer);
                     }
                  }
               }
            } // 'core_step_work.

            // // TODO: skip everything on R if all below L.

            aux.sorted_roots[merge_step.lower] = aux.final_root;
            aux.sorted_roots[merge_step.upper - 1] = aux.final_terminal;
         }

         let mut rabbit = aux.sorted_roots[merge_step.lower];
         // let mut diagnostic = vec![rabbit; 1];
         for _i in merge_step.lower..merge_step.upper - 1 {
            let prev_rabbit = rabbit;
            rabbit = aux.sorted_next[rabbit];
            assert_eq!(aux.sorted_prev[rabbit], prev_rabbit,);
            // diagnostic.push(rabbit);
         }
         // dbg!(diagnostic);
         assert_eq!(rabbit, aux.sorted_roots[merge_step.upper - 1]);

         for i in merge_step.lower..merge_step.upper {
            assert_eq!(&aux.accum_children[i], &aux.scratch_children[i]);
         }
      }

      for i in 0..node_count {
         assert_eq!(&aux.accum_children[i], &aux.scratch_children[i]);
      }
      for (p, children) in aux.scratch_children.iter().enumerate() {
         for c in children {
            aux.scratch_parents[*c].push_front(p);
         }
      }
      for i in 0..node_count {
         assert_eq!(&aux.accum_parents[i], &aux.scratch_parents[i]);
      }

      // Create efficient shrink-wrapped vector edge structures.  In the long run we may find
      // that building directly in place will work.
      //
      // The preferred final ordering is for parents to be "left-to-right" and children
      // "right-to-left", so that the relationships are independent of a 180-degree rotation of
      // the graph.
      for i in (0..node_count).rev() {
         self.nodes[i].parents = Vec::<IndexType>::from(aux.accum_parents.pop().unwrap());
         self.nodes[i].parents.shrink_to_fit();
         self.nodes[i].children = Vec::<IndexType>::from(aux.accum_children.pop().unwrap());
         self.nodes[i].children.shrink_to_fit();
      }

      Ok(())
   }
}
