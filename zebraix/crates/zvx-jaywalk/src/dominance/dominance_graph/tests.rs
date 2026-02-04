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

use super::*;

use permutohedron::Heap;
use rand_pcg::Pcg32;
use shuffle::fy::FisherYates;
use shuffle::shuffler::Shuffler;
use std::iter::zip;

trait ReferenceDominanceGraph {
   fn reference_flesh_out_graph_nodes(&mut self) -> Result<(), &'static str>;
   fn reference_connect_graph(&mut self) -> Result<(), &'static str>;
   // Check children <-> parent consistency and ordering.
   fn check_children_parent(&self) -> Result<(), String>;

   fn recurse_children_forward(&self, index: IndexType, visited: &mut VecDeque<IndexType>);
   fn recurse_children_reverse(&self, index: IndexType, visited: &mut VecDeque<IndexType>);
   fn recurse_parents_forward(&self, index: IndexType, visited: &mut VecDeque<IndexType>);
   fn recurse_parents_reverse(&self, index: IndexType, visited: &mut VecDeque<IndexType>);

   // Check tree consistency with depth-first recovery of rank sorts.
   fn check_descents(&self) -> Result<(), String>;
}

impl ReferenceDominanceGraph for DominanceGraph {
   // Create source and sink nodes as required. Sort nodes in increasing order of
   // prime rank.
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::missing_errors_doc)]
   fn reference_flesh_out_graph_nodes(&mut self) -> Result<(), &'static str> {
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
   fn reference_connect_graph(&mut self) -> Result<(), &'static str> {
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

   fn check_children_parent(&self) -> Result<(), String> {
      for (parent, node) in self.nodes[0..self.nodes.len() - 1].iter().enumerate() {
         let prev_child = IndexType::MIN;
         for child in &node.children {
            if *child <= prev_child {
               return Err(format!(
                  "Child {} is not in monotonic sequence {:?}",
                  *child, &node.children
               ));
            }
            if !self.nodes[*child].parents.contains(&parent) {
               return Err(format!(
                  "Child {}'s parents does not have parent {}, but instead {:?}",
                  *child, parent, self.nodes[*child].parents
               ));
            }
         }
      }
      for (child_minus_one, node) in self.nodes[1..].iter().enumerate() {
         let child = child_minus_one + 1;
         let prev_parent = IndexType::MAX;
         for parent in &node.parents {
            if *parent >= prev_parent {
               return Err(format!(
                  "Parent {} is not in monotonic sequence {:?}",
                  *parent, &node.parents
               ));
            }
            if !self.nodes[*parent].children.contains(&child) {
               return Err(format!(
                  "Parent {}'s children does not have child {}, but instead {:?}",
                  *parent, child, self.nodes[*parent].children
               ));
            }
         }
      }

      Ok(())
   }

   fn recurse_children_forward(&self, index: IndexType, visited: &mut VecDeque<IndexType>) {
      if visited.contains(&index) {
         return;
      }
      for child in &self.nodes[index].children {
         self.recurse_children_forward(*child, visited);
      }
      visited.push_front(index);
   }

   fn recurse_children_reverse(&self, index: IndexType, visited: &mut VecDeque<IndexType>) {
      if visited.contains(&index) {
         return;
      }
      for child in self.nodes[index].children.clone().into_iter().rev() {
         self.recurse_children_reverse(child, visited);
      }
      visited.push_front(index);
   }

   fn recurse_parents_forward(&self, index: IndexType, visited: &mut VecDeque<IndexType>) {
      if visited.contains(&index) {
         return;
      }
      for parent in &self.nodes[index].parents {
         self.recurse_parents_forward(*parent, visited);
      }
      visited.push_back(index);
   }

   fn recurse_parents_reverse(&self, index: IndexType, visited: &mut VecDeque<IndexType>) {
      if visited.contains(&index) {
         return;
      }
      for parent in self.nodes[index].parents.clone().into_iter().rev() {
         self.recurse_parents_reverse(parent, visited);
      }
      visited.push_back(index);
   }

   // Perform the 4 variations on depth-first searching, treating the graph as a tree,
   // descending from source through children and from sink through parents.  Convert results to
   // prime or obverse rank to show that the nodes are correctly sorted.
   //
   // This check only works on dominance graphs that have source and sink roots.
   fn check_descents(&self) -> Result<(), String> {
      let num_nodes = self.nodes.len();
      let mut straight_prime: Vec<RankType> =
         self.nodes.clone().into_iter().map(|node| node.prime_rank).collect();
      straight_prime.sort_unstable();
      let mut straight_obverse: Vec<RankType> =
         self.nodes.clone().into_iter().map(|node| node.obverse_rank).collect();
      straight_obverse.sort_unstable();

      {
         let mut children_forward: VecDeque<IndexType> = VecDeque::default();
         self.recurse_children_forward(0, &mut children_forward);
         let obverse_forward: Vec<RankType> =
            children_forward.into_iter().map(|index| self.nodes[index].obverse_rank).collect();
         if obverse_forward != straight_obverse {
            return Err(format!("Forward-obverse-children mismatch for {obverse_forward:?}"));
         }
      }

      {
         let mut children_reverse: VecDeque<IndexType> = VecDeque::default();
         self.recurse_children_reverse(0, &mut children_reverse);
         let prime_reverse: Vec<RankType> =
            children_reverse.into_iter().map(|index| self.nodes[index].prime_rank).collect();
         if prime_reverse != straight_prime {
            return Err(format!("Reverse-prime-children mismatch for {prime_reverse:?}"));
         }
      }

      {
         let mut parents_forward: VecDeque<IndexType> = VecDeque::default();
         self.recurse_parents_forward(num_nodes - 1, &mut parents_forward);
         let obverse_forward: Vec<RankType> =
            parents_forward.into_iter().map(|index| self.nodes[index].obverse_rank).collect();
         if obverse_forward != straight_obverse {
            return Err(format!("Forward-obverse-parents mismatch for {obverse_forward:?}"));
         }
      }

      {
         let mut parents_reverse: VecDeque<IndexType> = VecDeque::default();
         self.recurse_parents_reverse(num_nodes - 1, &mut parents_reverse);
         let prime_reverse: Vec<RankType> =
            parents_reverse.into_iter().map(|index| self.nodes[index].prime_rank).collect();
         if prime_reverse != straight_prime {
            return Err(format!("Reverse-prime-parents mismatch for {prime_reverse:?}"));
         }
      }

      Ok(())
   }
}

#[test]
fn bridge_three_direct_test() {
   let bridge_three_pairs = [(0_i32, 0_i32), (1, 3), (2, 2), (3, 1), (4, 4)];

   {
      let mut graph = DominanceGraph::new_from_pairs(&bridge_three_pairs[..], true, true);
      assert_eq!(graph.nodes.len(), 5);
      assert!(graph.flesh_out_graph_nodes().is_ok());
      assert!(graph.connect_graph().is_ok());

      #[allow(clippy::unreadable_literal)]
      let expected = DominanceGraph {
         source_index: 0,
         imputed_source: true,
         sink_index: 6,
         imputed_sink: true,
         nodes: vec![
            DominanceNode {
               prime_rank: -1,
               obverse_rank: -1,
               import_index: 18446744073709551613,
               parents: vec![],
               children: vec![1],
            },
            DominanceNode {
               prime_rank: 0,
               obverse_rank: 0,
               import_index: 0,
               parents: vec![0],
               children: vec![2, 3, 4],
            },
            DominanceNode {
               prime_rank: 1,
               obverse_rank: 3,
               import_index: 1,
               parents: vec![1],
               children: vec![5],
            },
            DominanceNode {
               prime_rank: 2,
               obverse_rank: 2,
               import_index: 2,
               parents: vec![1],
               children: vec![5],
            },
            DominanceNode {
               prime_rank: 3,
               obverse_rank: 1,
               import_index: 3,
               parents: vec![1],
               children: vec![5],
            },
            DominanceNode {
               prime_rank: 4,
               obverse_rank: 4,
               import_index: 4,
               parents: vec![4, 3, 2],
               children: vec![6],
            },
            DominanceNode {
               prime_rank: 5,
               obverse_rank: 5,
               import_index: 18446744073709551614,
               parents: vec![5],
               children: vec![],
            },
         ],
      };

      assert_eq!(graph, expected);
      let parenting_check = graph.check_children_parent();
      assert!(parenting_check.is_ok(), "{parenting_check:?}");
      let descents_check = graph.check_descents();
      assert!(descents_check.is_ok(), "{descents_check:?}");
   }
   {
      let mut graph = DominanceGraph::new_from_pairs(&bridge_three_pairs[..], false, false);
      assert_eq!(graph.nodes.len(), 5);
      assert!(graph.flesh_out_graph_nodes().is_ok());
      assert!(graph.connect_graph().is_ok());

      let expected = DominanceGraph {
         source_index: 0,
         imputed_source: false,
         sink_index: 4,
         imputed_sink: false,
         nodes: vec![
            DominanceNode {
               prime_rank: 0,
               obverse_rank: 0,
               import_index: 0,
               parents: vec![],
               children: vec![1, 2, 3],
            },
            DominanceNode {
               prime_rank: 1,
               obverse_rank: 3,
               import_index: 1,
               parents: vec![0],
               children: vec![4],
            },
            DominanceNode {
               prime_rank: 2,
               obverse_rank: 2,
               import_index: 2,
               parents: vec![0],
               children: vec![4],
            },
            DominanceNode {
               prime_rank: 3,
               obverse_rank: 1,
               import_index: 3,
               parents: vec![0],
               children: vec![4],
            },
            DominanceNode {
               prime_rank: 4,
               obverse_rank: 4,
               import_index: 4,
               parents: vec![3, 2, 1],
               children: vec![],
            },
         ],
      };

      assert_eq!(graph, expected);
      let parenting_check = graph.check_children_parent();
      assert!(parenting_check.is_ok(), "{parenting_check:?}");
      let descents_check = graph.check_descents();
      assert!(descents_check.is_ok(), "{descents_check:?}");
   }
}

#[test]
fn cross_simple_02_direct_test() {
   let cross_simple_02_pairs = [(1_i32, 3_i32), (2, 2), (3, 1), (4, 7), (5, 5), (6, 4), (7, 6)];

   let mut graph = DominanceGraph::new_from_pairs(&cross_simple_02_pairs[..], true, true);
   assert!(graph.flesh_out_graph_nodes().is_ok());
   assert!(graph.connect_graph().is_ok());

   #[allow(clippy::unreadable_literal)]
   let expected = DominanceGraph {
      source_index: 0,
      imputed_source: true,
      sink_index: 8,
      imputed_sink: true,
      nodes: vec![
         DominanceNode {
            prime_rank: 0,
            obverse_rank: 0,
            import_index: 18446744073709551613,
            parents: vec![],
            children: vec![1, 2, 3],
         },
         DominanceNode {
            prime_rank: 1,
            obverse_rank: 3,
            import_index: 0,
            parents: vec![0],
            children: vec![4, 5, 6],
         },
         DominanceNode {
            prime_rank: 2,
            obverse_rank: 2,
            import_index: 1,
            parents: vec![0],
            children: vec![4, 5, 6],
         },
         DominanceNode {
            prime_rank: 3,
            obverse_rank: 1,
            import_index: 2,
            parents: vec![0],
            children: vec![4, 5, 6],
         },
         DominanceNode {
            prime_rank: 4,
            obverse_rank: 7,
            import_index: 3,
            parents: vec![3, 2, 1],
            children: vec![8],
         },
         DominanceNode {
            prime_rank: 5,
            obverse_rank: 5,
            import_index: 4,
            parents: vec![3, 2, 1],
            children: vec![7],
         },
         DominanceNode {
            prime_rank: 6,
            obverse_rank: 4,
            import_index: 5,
            parents: vec![3, 2, 1],
            children: vec![7],
         },
         DominanceNode {
            prime_rank: 7,
            obverse_rank: 6,
            import_index: 6,
            parents: vec![6, 5],
            children: vec![8],
         },
         DominanceNode {
            prime_rank: 8,
            obverse_rank: 8,
            import_index: 18446744073709551614,
            parents: vec![7, 4],
            children: vec![],
         },
      ],
   };

   assert_eq!(graph, expected);
   let parenting_check = graph.check_children_parent();
   assert!(parenting_check.is_ok(), "{parenting_check:?}");
   let descents_check = graph.check_descents();
   assert!(descents_check.is_ok(), "{descents_check:?}");
}

#[test]
fn complicated_direct_test() {
   let complicated_pairs = [
      (0_i32, 0_i32),
      (1, 7),
      (2, 11),
      (3, 1),
      (4, 4),
      (5, 8),
      (6, 6),
      (7, 9),
      (8, 5),
      (9, 2),
      (10, 10),
      (11, 3),
      (12, 12),
   ];

   let mut graph = DominanceGraph::new_from_pairs(&complicated_pairs[..], false, false);
   assert!(graph.flesh_out_graph_nodes().is_ok());
   assert!(graph.connect_graph().is_ok());

   let expected = DominanceGraph {
      source_index: 0,
      imputed_source: false,
      sink_index: 12,
      imputed_sink: false,
      nodes: vec![
         DominanceNode {
            prime_rank: 0,
            obverse_rank: 0,
            import_index: 0,
            parents: vec![],
            children: vec![1, 3],
         },
         DominanceNode {
            prime_rank: 1,
            obverse_rank: 7,
            import_index: 1,
            parents: vec![0],
            children: vec![2, 5],
         },
         DominanceNode {
            prime_rank: 2,
            obverse_rank: 11,
            import_index: 2,
            parents: vec![1],
            children: vec![12],
         },
         DominanceNode {
            prime_rank: 3,
            obverse_rank: 1,
            import_index: 3,
            parents: vec![0],
            children: vec![4, 9],
         },
         DominanceNode {
            prime_rank: 4,
            obverse_rank: 4,
            import_index: 4,
            parents: vec![3],
            children: vec![5, 6, 8],
         },
         DominanceNode {
            prime_rank: 5,
            obverse_rank: 8,
            import_index: 5,
            parents: vec![4, 1],
            children: vec![7],
         },
         DominanceNode {
            prime_rank: 6,
            obverse_rank: 6,
            import_index: 6,
            parents: vec![4],
            children: vec![7],
         },
         DominanceNode {
            prime_rank: 7,
            obverse_rank: 9,
            import_index: 7,
            parents: vec![6, 5],
            children: vec![10],
         },
         DominanceNode {
            prime_rank: 8,
            obverse_rank: 5,
            import_index: 8,
            parents: vec![4],
            children: vec![10],
         },
         DominanceNode {
            prime_rank: 9,
            obverse_rank: 2,
            import_index: 9,
            parents: vec![3],
            children: vec![10, 11],
         },
         DominanceNode {
            prime_rank: 10,
            obverse_rank: 10,
            import_index: 10,
            parents: vec![9, 8, 7],
            children: vec![12],
         },
         DominanceNode {
            prime_rank: 11,
            obverse_rank: 3,
            import_index: 11,
            parents: vec![9],
            children: vec![12],
         },
         DominanceNode {
            prime_rank: 12,
            obverse_rank: 12,
            import_index: 12,
            parents: vec![11, 10, 2],
            children: vec![],
         },
      ],
   };
   assert_eq!(graph, expected);
   let parenting_check = graph.check_children_parent();
   assert!(parenting_check.is_ok(), "{parenting_check:?}");
   let descents_check = graph.check_descents();
   assert!(descents_check.is_ok(), "{descents_check:?}");
}

// Exhaustively traverse permutations of values, effectively permuting the obverse ordering.
//
// The testing is against reference, and with limited checking of the graph consistency.
//
// The normal test size is 5! = 120, but this can be increased when introducing new graph
// manipulation for greater testing.
#[test]
fn exhaustive_permutations_test() {
   const TEST_LENGTH: RankType = 5;

   let prime_ranks: Vec<RankType> = (0..TEST_LENGTH).collect();
   let mut obverse_ranks = prime_ranks.clone();
   let heap = Heap::new(&mut obverse_ranks);

   for obversal in heap {
      let test_pairs: Vec<(RankType, RankType)> = zip(prime_ranks.clone(), obversal).collect();

      {
         let mut graph = DominanceGraph::new_from_pairs(&test_pairs[..], false, false);
         let mut reference_graph = graph.clone();
         assert!(graph.flesh_out_graph_nodes().is_ok());
         assert!(graph.connect_graph().is_ok());
         assert!(reference_graph.reference_flesh_out_graph_nodes().is_ok());
         assert!(reference_graph.reference_connect_graph().is_ok());

         assert_eq!(graph, reference_graph);

         let parenting_check = graph.check_children_parent();
         assert!(parenting_check.is_ok(), "{parenting_check:?}");
      }

      {
         let mut graph = DominanceGraph::new_from_pairs(&test_pairs[..], true, true);
         let mut reference_graph = graph.clone();
         assert!(graph.flesh_out_graph_nodes().is_ok());
         assert!(graph.connect_graph().is_ok());
         assert!(reference_graph.reference_flesh_out_graph_nodes().is_ok());
         assert!(reference_graph.reference_connect_graph().is_ok());

         assert_eq!(graph, reference_graph);

         let parenting_check = graph.check_children_parent();
         assert!(parenting_check.is_ok(), "{parenting_check:?}");
         let descents_check = graph.check_descents();
         assert!(descents_check.is_ok(), "{descents_check:?}");
      }
   }
}

#[test]
#[allow(clippy::unreadable_literal)]
fn random_shuffles_test() {
   const TEST_LENGTH: RankType = 5;
   const NUM_TESTS: usize = 100;
   const SEED_STATE: u64 = 0xd6acf0e0b5d5ee15;
   const SEED_STREAM: u64 = 0xabb2df070cab73b7;
   let mut rng = Pcg32::new(SEED_STATE, SEED_STREAM);
   let mut fy = FisherYates::default();

   let prime_ranks: Vec<RankType> = (0..TEST_LENGTH).collect();
   let mut obverse_ranks = prime_ranks.clone();

   for _i in 0..NUM_TESTS {
      assert!(fy.shuffle(&mut obverse_ranks, &mut rng).is_ok());
      let test_pairs: Vec<(RankType, RankType)> =
         zip(prime_ranks.clone(), obverse_ranks.clone()).collect();

      {
         let mut graph = DominanceGraph::new_from_pairs(&test_pairs[..], false, false);
         let mut reference_graph = graph.clone();
         assert!(graph.flesh_out_graph_nodes().is_ok());
         assert!(graph.connect_graph().is_ok());
         assert!(reference_graph.reference_flesh_out_graph_nodes().is_ok());
         assert!(reference_graph.reference_connect_graph().is_ok());

         assert_eq!(graph, reference_graph);

         let parenting_check = graph.check_children_parent();
         assert!(parenting_check.is_ok(), "{parenting_check:?}");
      }

      {
         let mut graph = DominanceGraph::new_from_pairs(&test_pairs[..], true, true);
         let mut reference_graph = graph.clone();
         assert!(graph.flesh_out_graph_nodes().is_ok());
         assert!(graph.connect_graph().is_ok());
         assert!(reference_graph.reference_flesh_out_graph_nodes().is_ok());
         assert!(reference_graph.reference_connect_graph().is_ok());

         assert_eq!(graph, reference_graph);

         let parenting_check = graph.check_children_parent();
         assert!(parenting_check.is_ok(), "{parenting_check:?}");
         let descents_check = graph.check_descents();
         assert!(descents_check.is_ok(), "{descents_check:?}");
      }
   }
}

#[test]
fn grid_direct_test() {
   let complicated_pairs = [(0_i32, 0_i32), (0, 4), (2, 8), (4, 1), (5, 4)];

   let mut graph = DominanceGraph::new_from_pairs(&complicated_pairs[..], false, false);
   let mut reference_graph = graph.clone();
   assert!(graph.flesh_out_graph_nodes().is_ok());
   assert!(graph.connect_graph().is_ok());
   assert!(reference_graph.reference_flesh_out_graph_nodes().is_ok());
   assert!(reference_graph.reference_connect_graph().is_ok());
   assert_eq!(graph, reference_graph);

   #[allow(clippy::unreadable_literal)]
   let expected = DominanceGraph {
      source_index: 0,
      imputed_source: false,
      sink_index: 5,
      imputed_sink: true,
      nodes: vec![
         DominanceNode {
            prime_rank: 0,
            obverse_rank: 0,
            import_index: 0,
            parents: vec![],
            children: vec![1, 3],
         },
         DominanceNode {
            prime_rank: 0,
            obverse_rank: 4,
            import_index: 1,
            parents: vec![0],
            children: vec![2, 4],
         },
         DominanceNode {
            prime_rank: 2,
            obverse_rank: 8,
            import_index: 2,
            parents: vec![1],
            children: vec![5],
         },
         DominanceNode {
            prime_rank: 4,
            obverse_rank: 1,
            import_index: 3,
            parents: vec![0],
            children: vec![4],
         },
         DominanceNode {
            prime_rank: 5,
            obverse_rank: 4,
            import_index: 4,
            parents: vec![3, 1],
            children: vec![5],
         },
         DominanceNode {
            prime_rank: 6,
            obverse_rank: 9,
            import_index: 18446744073709551614,
            parents: vec![4, 2],
            children: vec![],
         },
      ],
   };
   assert_eq!(graph, expected);
   let parenting_check = graph.check_children_parent();
   assert!(parenting_check.is_ok(), "{parenting_check:?}");
   let descents_check = graph.check_descents();
   assert!(descents_check.is_ok(), "{descents_check:?}");
}
