#[cfg(test)]
use ams_demo_package::ams_demo::ams_debug::{check_block_list, check_lozenge, check_sort};
#[cfg(test)]
use ams_demo_package::ams_demo::ams_patterns::BlockPermutation;
#[cfg(test)]
use ams_demo_package::ams_demo::ams_patterns::DisruptionType;
#[cfg(test)]
use ams_demo_package::ams_demo::ams_patterns::PermutationConfig;
#[cfg(test)]
use ams_demo_package::ams_demo::ams_patterns::SortType;
pub use ams_demo_package::ams_demo::fill_sortable;
#[cfg(test)]
pub use ams_demo_package::ams_demo::nudge_values;
pub use ams_demo_package::ams_demo::sort_standard_interlink;
pub use ams_demo_package::ams_demo::Linkable;
pub use ams_demo_package::ams_demo::MergeStep;
pub use ams_demo_package::ams_demo::MinusPlusShift;
pub use ams_demo_package::ams_demo::SortStats;
pub use ams_demo_package::ams_demo::SortStatsCounts;
pub use ams_demo_package::ams_demo::SortStatsInit;
pub use ams_demo_package::ams_demo::SortableEntity;
pub use ams_demo_package::ams_demo::CEIL_LOG_FIDDLE;
use goldenfile::Mint;

// #[cfg(test)]
// use std::io::Write;

use std::fs::File;
use std::io::BufWriter;
// use std::io::Write;

pub struct WscSubSort {
   pub has_reverse: bool, // True iff reverse sort chain is created.
   pub head: usize,
   pub tail: usize,                // Points to actual end, not one-past.
   pub p_dfs_start: Option<usize>, // Highest unparented.
   pub c_dfs_start: Option<usize>, // Lowest unparented, 180-deg-rotated pDFS.
}

#[derive(Copy, Clone, PartialEq)]
pub enum WscSortType {
   Classic,
   AnchorClassic,
   AnchorSkipless,
   AnchorSkipper,
   Legacy1,
}

impl Default for WscSortType {
   fn default() -> Self {
      WscSortType::Classic
   }
}

pub struct WscSortConfig {
   pub main_sort_type: WscSortType,
   pub lower_sort_type: WscSortType,
   pub final_reverse: bool,
   pub unused_skip_upper_lozenge: bool,
   pub check_block_chain: bool,
   pub type_switch_level: u32,
}

impl Default for WscSortConfig {
   fn default() -> Self {
      WscSortConfig {
         main_sort_type: WscSortType::Legacy1,
         lower_sort_type: WscSortType::Classic,
         final_reverse: false,
         unused_skip_upper_lozenge: false,
         check_block_chain: true,
         type_switch_level: 0,
      }
   }
}

#[derive(PartialEq)]
enum BlockSkipFsm {
   AtRoot,
   JustBeyond,
   Stepwise,
}

pub enum EvalOutputStyle {
   HUMAN,
   CSVM,
}

#[derive(Copy, Clone)]
pub enum EvalSortChoice {
   AMS,
   BASELINE,
}

pub enum EvalDestination<'a> {
   GoldenFile(),
   Writer(&'a mut dyn std::io::Write),
   // StdOut,
}

type PatternModOp = fn(&mut BlockPermutation) -> ();

fn nop_pattern_mod_op(_block_permutation: &mut BlockPermutation) {}

#[derive(Clone)]
pub struct SortEvalConfig<'a> {
   base_name: &'a str,
   num_samples: u32,
   sort_size: u32,
   type_switch_level: u32,
   permutation_config: PermutationConfig,
   randomizer_offset: u64,
   sort_choice: EvalSortChoice,
   pattern_mod_op: PatternModOp,
}

struct StructuralAnalysis {
   left_head_right_head: bool,

   left_anchor_right_head: bool,
   // left_tail_right_head: bool,
   left_head_right_anchor: bool,
   // left_head_right_tail: bool,
   left_anchor_right_tail: bool,
   left_tail_right_anchor: bool,
   left_anchor_right_anchor: bool,

   has_anchoring: bool,

   curr_l: usize,
   curr_r: usize,
   //
   consume_left: bool,
   curr_head: usize,
   consumed_left_east: bool,
   consumed_right_west: bool,
   left_next_block: usize,
   right_next_block: usize,
   //
   left_chain_state: BlockSkipFsm,
   right_chain_state: BlockSkipFsm,

   trimming_se: Option<usize>,
   trimming_sw: Option<usize>,
   parents_head: Option<usize>,
   children_head: Option<usize>,

   // Required only for non-classic merge stage.
   restore_se: Option<usize>,
   restore_sw: Option<usize>,

   // Retained for longer for diagnostics.
   all_se_appended: bool,
   all_se_overlapped: bool,
   all_sw_appended: bool,
   all_sw_overlapped: bool,

   right_c_dfs_start: Option<usize>,
   left_p_dfs_start: Option<usize>,
}

impl StructuralAnalysis {
   pub fn new(// sorting_data: &[SortableEntity],
      // right_sub: &WscSubSort,
      // left_sub: &WscSubSort,
      // merge_step: &MergeStep,
      // sorting_stats: &mut SortStats,
   ) -> StructuralAnalysis {
      StructuralAnalysis {
         left_head_right_head: Default::default(),
         left_anchor_right_head: Default::default(),
         // left_tail_right_head: Default::default(),
         left_head_right_anchor: Default::default(),
         // left_head_right_tail: Default::default(),
         left_anchor_right_tail: Default::default(),
         left_tail_right_anchor: Default::default(),
         left_anchor_right_anchor: Default::default(),

         has_anchoring: Default::default(),

         curr_l: Default::default(),
         curr_r: Default::default(),
         //
         consume_left: Default::default(),
         curr_head: Default::default(),
         consumed_left_east: Default::default(),
         consumed_right_west: Default::default(),
         left_next_block: Default::default(),
         right_next_block: Default::default(),
         //
         left_chain_state: BlockSkipFsm::AtRoot,
         right_chain_state: BlockSkipFsm::AtRoot,

         trimming_se: Default::default(),
         trimming_sw: Default::default(),
         parents_head: Default::default(),
         children_head: Default::default(),
         restore_se: Default::default(),
         restore_sw: Default::default(),
         all_se_appended: Default::default(),
         all_se_overlapped: Default::default(),
         all_sw_appended: Default::default(),
         all_sw_overlapped: Default::default(),

         right_c_dfs_start: Default::default(),
         left_p_dfs_start: Default::default(),
      }
   }

   pub fn pre_work_minimal(
      &mut self,
      sorting_data: &[SortableEntity],
      right_sub: &WscSubSort,
      left_sub: &WscSubSort,
      merge_step: &MergeStep,
      sorting_stats: &mut SortStats,
   ) {
      let right_head = right_sub.head;
      let left_head = left_sub.head;

      self.left_head_right_head = sorting_stats.cmp_sortable(
         &sorting_data[left_head],
         &sorting_data[right_head],
         SortStatsCounts::NLogN,
         merge_step.shifted_level,
      );
   }

   pub fn pre_work_anchor(
      &mut self,
      sorting_data: &[SortableEntity],
      right_sub: &WscSubSort,
      left_sub: &WscSubSort,
      merge_step: &MergeStep,
      sorting_stats: &mut SortStats,
   ) {
      let right_head = right_sub.head;
      let left_head = left_sub.head;
      let right_tail = right_sub.tail;
      let left_tail = left_sub.tail;
      let right_anchor = merge_step.middle;
      let left_anchor = merge_step.middle - 1;

      self.left_anchor_right_anchor = sorting_stats.cmp_sortable(
         &sorting_data[left_anchor],
         &sorting_data[right_anchor],
         SortStatsCounts::NLogN, // YYYY
         merge_step.shifted_level,
      );

      if self.left_anchor_right_anchor {
         self.left_anchor_right_tail = true;
         self.left_head_right_anchor = true;

         if right_head == right_anchor {
            self.left_anchor_right_head = self.left_anchor_right_anchor;
         } else {
            self.left_anchor_right_head = sorting_stats.cmp_sortable(
               &sorting_data[left_anchor],
               &sorting_data[right_head],
               SortStatsCounts::NWork, // YYYY
               merge_step.shifted_level,
            );
         }
         if left_tail == left_anchor {
            self.left_tail_right_anchor = self.left_anchor_right_anchor;
         } else {
            self.left_tail_right_anchor = sorting_stats.cmp_sortable(
               &sorting_data[left_tail],
               &sorting_data[right_anchor],
               SortStatsCounts::NWork, // YYYY
               merge_step.shifted_level,
            );
         }
      } else {
         if right_tail == right_anchor {
            self.left_anchor_right_tail = self.left_anchor_right_anchor;
         } else {
            self.left_anchor_right_tail = sorting_stats.cmp_sortable(
               &sorting_data[left_anchor],
               &sorting_data[right_tail],
               SortStatsCounts::NWork, // YYYY
               merge_step.shifted_level,
            );
         }
         if left_head == left_anchor {
            self.left_head_right_anchor = self.left_anchor_right_anchor;
         } else {
            self.left_head_right_anchor = sorting_stats.cmp_sortable(
               &sorting_data[left_head],
               &sorting_data[right_anchor],
               SortStatsCounts::NWork, // YYYY
               merge_step.shifted_level,
            );
         }

         self.left_anchor_right_head = false;
         self.left_tail_right_anchor = false;
      }

      if !self.left_head_right_anchor {
         self.left_head_right_head = false;
      } else if self.left_anchor_right_head {
         self.left_head_right_head = true;
      } else {
         if left_head == left_anchor {
            self.left_head_right_head = self.left_anchor_right_head;
         } else if right_head == right_anchor {
            self.left_head_right_head = self.left_head_right_anchor;
         } else {
            self.left_head_right_head = sorting_stats.cmp_sortable(
               &sorting_data[left_head],
               &sorting_data[right_head],
               SortStatsCounts::NWork, // YYYY
               merge_step.shifted_level,
            );
         }
      }

      self.has_anchoring = true;

      assert!(
         self.left_head_right_head
            == (sorting_data[left_head].value <= sorting_data[right_head].value)
      );
      assert!(
         self.left_anchor_right_anchor
            == (sorting_data[left_anchor].value <= sorting_data[right_anchor].value)
      );
      assert!(
         self.left_anchor_right_head
            == (sorting_data[left_anchor].value <= sorting_data[right_head].value)
      );
      assert!(
         self.left_anchor_right_tail
            == (sorting_data[left_anchor].value <= sorting_data[right_tail].value)
      );
      assert!(
         self.left_head_right_anchor
            == (sorting_data[left_head].value <= sorting_data[right_anchor].value)
      );
      assert!(
         self.left_tail_right_anchor
            == (sorting_data[left_tail].value <= sorting_data[right_anchor].value)
      );
   }

   pub fn pre_work_full(
      &mut self,
      sorting_data: &[SortableEntity],
      right_sub: &WscSubSort,
      left_sub: &WscSubSort,
      merge_step: &MergeStep,
      sorting_stats: &mut SortStats,
   ) {
      let right_head = right_sub.head;
      let left_head = left_sub.head;

      // The conditions logic was not comprehensively analysed.
      self.left_head_right_head = sorting_stats.cmp_sortable(
         &sorting_data[left_head],
         &sorting_data[right_head],
         SortStatsCounts::NLogN,
         merge_step.shifted_level,
      );

      let right_head = right_sub.head;
      let left_head = left_sub.head;
      // let right_tail = right_sub.tail;
      // let left_tail = left_sub.tail;
      // let right_p_dfs_start = right_sub.p_dfs_start;
      // let left_c_dfs_start = left_sub.c_dfs_start;

      if self.left_head_right_head {
         // self.left_head_right_tail = true;
         self.left_head_right_anchor = true;
         // self.left_tail_right_head = sorting_data[left_tail] <= sorting_data[right_head];

         // if self.left_tail_right_head {
         //    self.left_anchor_right_head = true;
         // } else {
         self.left_anchor_right_head = sorting_stats.cmp_sortable(
            &sorting_data[merge_step.middle - 1],
            &sorting_data[right_head],
            SortStatsCounts::NWork,
            merge_step.shifted_level,
         );
         // }
      } else {
         self.left_anchor_right_head = false; // Works if left_anchor == left_head.

         // self.left_tail_right_head = false;
         // self.left_head_right_tail = sorting_data[left_head] <= sorting_data[right_tail];

         // if !self.left_head_right_tail {
         //    self.left_head_right_anchor = false;
         // } else {
         self.left_head_right_anchor = sorting_stats.cmp_sortable(
            &sorting_data[left_head],
            &sorting_data[merge_step.middle],
            SortStatsCounts::NWork,
            merge_step.shifted_level,
         );
         // }
      }
      self.left_anchor_right_tail = if self.left_anchor_right_head {
         true
      } else {
         sorting_stats.cmp_sortable(
            &sorting_data[merge_step.middle - 1],
            &sorting_data[right_head],
            SortStatsCounts::NWork,
            merge_step.shifted_level,
         )
      };

      self.left_tail_right_anchor = if !self.left_head_right_anchor {
         false
      } else {
         sorting_stats.cmp_sortable(
            &sorting_data[left_head],
            &sorting_data[merge_step.middle],
            SortStatsCounts::NWork,
            merge_step.shifted_level,
         )
      };

      // Various conditions that can be used.
      //
      if self.left_head_right_head {
         // assert!(self.left_head_right_tail);
         assert!(self.left_head_right_anchor);
         // if self.left_tail_right_head {
         //    assert!(self.left_anchor_right_head);
         // }
         // if !self.left_anchor_right_head {
         //    assert!(!self.left_tail_right_head);
         // }
      } else {
         // assert!(!self.left_tail_right_head);
         assert!(!self.left_anchor_right_head);

         // // Knowing left_head_right_tail does not seem to yield much in average performance.
         // if !self.left_head_right_tail {
         //    assert!(!self.left_head_right_anchor);
         // }
         // if self.left_head_right_anchor {
         //    assert!(self.left_head_right_tail);
         // }
      }
      if self.left_anchor_right_head {
         assert!(self.left_anchor_right_tail);
      }
      if !self.left_head_right_anchor {
         assert!(!self.left_tail_right_anchor);
      }

      // // Required by skipless.

      // let right_anchor = merge_step.middle;
      // let left_anchor = merge_step.middle - 1;

      // assert!(self.left_head_right_head == (sorting_data[left_head] <= sorting_data[right_head]));
      // assert!(
      //    self.left_anchor_right_anchor == (sorting_data[left_anchor] <= sorting_data[right_anchor])
      // );
      // assert!(
      //    self.left_anchor_right_head == (sorting_data[left_anchor] <= sorting_data[right_head])
      // );
      // assert!(
      //    self.left_anchor_right_tail == (sorting_data[left_anchor] <= sorting_data[right_tail])
      // );
      // assert!(
      //    self.left_head_right_anchor == (sorting_data[left_head] <= sorting_data[right_anchor])
      // );
      // assert!(
      //    self.left_tail_right_anchor == (sorting_data[left_tail] <= sorting_data[right_anchor])
      // );
   }

   pub fn classic_loop(
      &mut self,
      sorting_data: &mut [SortableEntity],
      right_sub: &WscSubSort,
      left_sub: &WscSubSort,
      merge_step: &MergeStep,
      sorting_stats: &mut SortStats,
   ) {
      let direct_merge_complete = true;
      let block_head_test: bool = self.left_head_right_head;

      let right_head = right_sub.head;
      let left_head = left_sub.head;
      let right_tail = right_sub.tail;
      let left_tail = left_sub.tail;

      self.left_anchor_right_head = false;
      self.left_head_right_anchor = true;
      self.curr_l = left_head;
      self.curr_r = right_head;
      self.consume_left = block_head_test;
      self.curr_head = if self.consume_left { right_tail } else { left_tail };
      self.consumed_left_east = false;
      self.consumed_right_west = false;
      self.trimming_se = sorting_data[left_head].get_tertiary_link();
      self.trimming_sw = sorting_data[right_head].get_secondary_link();
      self.left_next_block = left_head;
      self.right_next_block = right_head;

      let mut consumed_some_left = false;
      let mut consumed_some_right = false;

      sorting_data[left_tail].set_tertiary_link(Some(right_head));
      sorting_data[right_tail].set_secondary_link(Some(left_head));

      loop {
         if self.consume_left {
            if direct_merge_complete {
               sorting_data[self.curr_head].set_forward_link(Some(self.curr_l));
               sorting_data[self.curr_l].set_backward_link(Some(self.curr_head));
            }
            consumed_some_left = true;
            if self.curr_l == merge_step.middle - 1 {
               self.consumed_left_east = true;
               self.left_next_block =
                  sorting_data[merge_step.middle - 1].get_tertiary_link().unwrap();
               if !consumed_some_right {
                  self.trimming_se = Some(self.curr_l);
                  self.left_anchor_right_head = true;
               }
            } else if self.consumed_left_east {
               if self.left_next_block == self.curr_l {
                  self.left_next_block =
                     sorting_data[self.left_next_block].get_tertiary_link().unwrap();
               }
            } else if !consumed_some_right
               && sorting_data[self.curr_l].get_tertiary_link() == self.trimming_se
            {
               self.trimming_se = Some(self.curr_l);
            }
            self.curr_head = self.curr_l;
            self.curr_l = sorting_data[self.curr_l].get_forward_link().unwrap();
         } else {
            if direct_merge_complete {
               sorting_data[self.curr_head].set_forward_link(Some(self.curr_r));
               sorting_data[self.curr_r].set_backward_link(Some(self.curr_head));
            }
            consumed_some_right = true;
            if self.curr_r == merge_step.middle {
               self.consumed_right_west = true;
               self.right_next_block =
                  sorting_data[merge_step.middle].get_secondary_link().unwrap();
               if !consumed_some_left {
                  self.trimming_sw = Some(self.curr_r);
                  self.left_head_right_anchor = false;
               }
            } else if self.consumed_right_west {
               if self.right_next_block == self.curr_r {
                  self.right_next_block =
                     sorting_data[self.right_next_block].get_secondary_link().unwrap();
               }
            } else if !consumed_some_left
               && sorting_data[self.curr_r].get_secondary_link() == self.trimming_sw
            {
               self.trimming_sw = Some(self.curr_r);
            }

            self.curr_head = self.curr_r;
            self.curr_r = sorting_data[self.curr_r].get_forward_link().unwrap();
         }

         if (self.curr_l == right_head) || (self.curr_r == left_head) {
            break;
         }

         self.consume_left = sorting_stats.cmp_sortable(
            &sorting_data[self.curr_l],
            &sorting_data[self.curr_r],
            SortStatsCounts::NLogN,
            merge_step.shifted_level,
         );
      }
      sorting_data[left_tail].set_tertiary_link(None);
      sorting_data[right_tail].set_secondary_link(None);
   }

   pub fn anchor_classic_loop(
      &mut self,
      sorting_data: &mut [SortableEntity],
      right_sub: &WscSubSort,
      left_sub: &WscSubSort,
      merge_step: &MergeStep,
      sorting_stats: &mut SortStats,
   ) {
      let direct_merge_complete = true;
      let block_head_test: bool = self.left_head_right_head;

      let right_head = right_sub.head;
      let left_head = left_sub.head;
      let right_tail = right_sub.tail;
      let left_tail = left_sub.tail;
      let right_anchor = merge_step.middle;
      let left_anchor = merge_step.middle - 1;

      // self.left_anchor_right_head = false;
      // self.left_head_right_anchor = true;
      self.curr_l = left_head;
      self.curr_r = right_head;

      self.consume_left = block_head_test;
      self.curr_head = if self.consume_left { right_tail } else { left_tail };

      self.consumed_left_east = false;
      self.consumed_right_west = false;
      self.trimming_se = sorting_data[left_head].get_tertiary_link();
      self.trimming_sw = sorting_data[right_head].get_secondary_link();
      self.left_next_block = left_head;
      self.right_next_block = right_head;

      let mut consumed_some_left = false;
      let mut consumed_some_right = false;

      if self.left_anchor_right_head && (left_head != left_anchor) {
         self.curr_l = left_anchor;
         consumed_some_left = true;
         self.left_next_block = left_anchor;
         self.consumed_left_east = true;
         self.trimming_se = Some(self.curr_l);
         self.curr_head = sorting_data[self.curr_l].get_backward_link().unwrap();
      }
      if !self.left_head_right_anchor {
         self.curr_r = right_anchor;
         consumed_some_right = true;
         self.right_next_block = right_anchor;
         self.consumed_right_west = true;
         self.trimming_sw = Some(self.curr_r);
         self.curr_head = sorting_data[self.curr_r].get_backward_link().unwrap();
      }

      assert!(
         self.left_head_right_head
            == (sorting_data[left_head].value <= sorting_data[right_head].value)
      );
      assert!(
         self.left_anchor_right_anchor
            == (sorting_data[left_anchor].value <= sorting_data[right_anchor].value)
      );
      assert!(
         self.left_anchor_right_head
            == (sorting_data[left_anchor].value <= sorting_data[right_head].value)
      );
      assert!(
         self.left_anchor_right_tail
            == (sorting_data[left_anchor].value <= sorting_data[right_tail].value)
      );
      assert!(
         self.left_head_right_anchor
            == (sorting_data[left_head].value <= sorting_data[right_anchor].value)
      );
      assert!(
         self.left_tail_right_anchor
            == (sorting_data[left_tail].value <= sorting_data[right_anchor].value)
      );

      sorting_data[left_tail].set_tertiary_link(Some(right_head));
      sorting_data[right_tail].set_secondary_link(Some(left_head));

      loop {
         if self.consume_left {
            if direct_merge_complete {
               sorting_data[self.curr_head].set_forward_link(Some(self.curr_l));
               sorting_data[self.curr_l].set_backward_link(Some(self.curr_head));
            }
            consumed_some_left = true;
            if self.curr_l == merge_step.middle - 1 {
               self.consumed_left_east = true;
               self.left_next_block =
                  sorting_data[merge_step.middle - 1].get_tertiary_link().unwrap();
               if !consumed_some_right {
                  self.trimming_se = Some(self.curr_l);
                  self.left_anchor_right_head = true;
               }
            } else if self.consumed_left_east {
               if self.left_next_block == self.curr_l {
                  self.left_next_block =
                     sorting_data[self.left_next_block].get_tertiary_link().unwrap();
               }
            } else if !consumed_some_right
               && sorting_data[self.curr_l].get_tertiary_link() == self.trimming_se
            {
               self.trimming_se = Some(self.curr_l);
            }
            self.curr_head = self.curr_l;
            self.curr_l = sorting_data[self.curr_l].get_forward_link().unwrap();
            assert!(self.curr_head != self.curr_l);
         } else {
            if direct_merge_complete {
               sorting_data[self.curr_head].set_forward_link(Some(self.curr_r));
               sorting_data[self.curr_r].set_backward_link(Some(self.curr_head));
            }
            consumed_some_right = true;
            if self.curr_r == merge_step.middle {
               self.consumed_right_west = true;
               self.right_next_block =
                  sorting_data[merge_step.middle].get_secondary_link().unwrap();
               if !consumed_some_left {
                  self.trimming_sw = Some(self.curr_r);
                  self.left_head_right_anchor = false;
               }
            } else if self.consumed_right_west {
               if self.right_next_block == self.curr_r {
                  self.right_next_block =
                     sorting_data[self.right_next_block].get_secondary_link().unwrap();
               }
            } else if !consumed_some_left
               && sorting_data[self.curr_r].get_secondary_link() == self.trimming_sw
            {
               self.trimming_sw = Some(self.curr_r);
            }

            self.curr_head = self.curr_r;
            self.curr_r = sorting_data[self.curr_r].get_forward_link().unwrap();
            assert!(self.curr_head != self.curr_r);
         }

         if (self.curr_l == right_head) || (self.curr_r == left_head) {
            break;
         }

         if self.curr_l == left_anchor {
            if self.curr_r == right_anchor {
               self.consume_left = self.left_anchor_right_anchor;
            } else if self.curr_r == right_tail {
               self.consume_left = self.left_anchor_right_tail;
            } else if self.curr_r == right_head {
               self.consume_left = self.left_anchor_right_head;
            } else {
               self.consume_left = sorting_stats.cmp_sortable(
                  &sorting_data[self.curr_l],
                  &sorting_data[self.curr_r],
                  SortStatsCounts::NLogN, // YYYY
                  merge_step.shifted_level,
               );
            }
         } else if self.curr_r == right_anchor {
            if self.curr_l == left_tail {
               self.consume_left = self.left_tail_right_anchor;
            } else if self.curr_l == left_head {
               self.consume_left = self.left_head_right_anchor;
            } else {
               self.consume_left = sorting_stats.cmp_sortable(
                  &sorting_data[self.curr_l],
                  &sorting_data[self.curr_r],
                  SortStatsCounts::NLogN, // YYYY
                  merge_step.shifted_level,
               );
            }
         } else {
            self.consume_left = sorting_stats.cmp_sortable(
               &sorting_data[self.curr_l],
               &sorting_data[self.curr_r],
               SortStatsCounts::NLogN, // YYYY
               merge_step.shifted_level,
            );
         }
      }

      sorting_data[left_tail].set_tertiary_link(None);
      sorting_data[right_tail].set_secondary_link(None);
   }

   pub fn lozenge_pre_work(
      &mut self,
      sorting_data: &mut [SortableEntity],
      right_sub: &WscSubSort,
      left_sub: &WscSubSort,
      merge_step: &MergeStep,
      sorting_stats: &mut SortStats,
   ) {
      // let block_head_test: bool = self.left_head_right_head;

      let right_head = right_sub.head;
      let left_head = left_sub.head;
      let right_tail = right_sub.tail;
      let left_tail = left_sub.tail;
      let right_p_dfs_start = right_sub.p_dfs_start;
      let left_c_dfs_start = left_sub.c_dfs_start;

      let mut scan_ne = Some(merge_step.middle - 1);
      // IDEA: Perhaps can do "at or below bottom or R" and still maintain stable sort.
      let mut lozenge_east_ext = Some(merge_step.middle - 1); // Highest of NE in L block, below bottom of R, None if all above.
                                                              // let used_left_tail_right_head = left_tail_right_head;
      let used_left_tail_right_head = false;

      if used_left_tail_right_head {
         // Resurrect?

         // Note that the loop will automatically set these, but we can avoid the (on average apparently tiny) work.
         // scan_ne = None;
         lozenge_east_ext = Some(left_tail);
      } else if self.left_anchor_right_head {
         lozenge_east_ext = scan_ne;
         scan_ne = sorting_data[scan_ne.unwrap()].get_tertiary_link();
         while (scan_ne != None)
            && sorting_stats.cmp_sortable(
               &sorting_data[scan_ne.unwrap()],
               &sorting_data[right_head],
               SortStatsCounts::NWork,
               merge_step.shifted_level,
            )
         {
            lozenge_east_ext = scan_ne;
            scan_ne = sorting_data[scan_ne.unwrap()].get_tertiary_link();
         }
      }
      // assert!((scan_ne == None) == self.left_tail_right_head);
      // if self.left_tail_right_head {
      //    assert!(lozenge_east_ext == Some(left_tail));
      // }
      //

      let mut scan_nw = Some(merge_step.middle);
      let mut lozenge_west_ext = Some(merge_step.middle); // Highest of NW in R block, below bottom of L, None if all above.
                                                          // let used_left_head_right_tail = left_head_right_tail;
      let used_left_head_right_tail = true;

      // XXXX
      if !used_left_head_right_tail {
         // scan_nw = None;
         lozenge_west_ext = Some(right_tail);
      } else if !self.left_head_right_anchor {
         lozenge_west_ext = scan_nw;
         scan_nw = sorting_data[scan_nw.unwrap()].get_secondary_link();
         while (scan_nw != None)
            && !sorting_stats.cmp_sortable(
               &sorting_data[left_head],
               &sorting_data[scan_nw.unwrap()],
               SortStatsCounts::NWork,
               merge_step.shifted_level,
            )
         {
            lozenge_west_ext = scan_nw;
            scan_nw = sorting_data[scan_nw.unwrap()].get_secondary_link();
         }
      }
      // assert!((scan_nw == None) != self.left_head_right_tail);
      // if !self.left_head_right_tail {
      //    assert!(lozenge_west_ext == Some(right_tail));
      // }

      //

      let mut lozenge_se;
      // XXXX
      if !self.left_anchor_right_head {
         self.trimming_se = left_c_dfs_start;
         lozenge_se = Some(merge_step.middle - 1);
         assert!(lozenge_east_ext != None);

         if self.left_head_right_head {
            while !sorting_stats.cmp_sortable(
               &sorting_data[self.trimming_se.unwrap()],
               &sorting_data[right_head],
               SortStatsCounts::NWork,
               merge_step.shifted_level,
            ) {
               let new_trimming_se = sorting_data[self.trimming_se.unwrap()].get_tertiary_link();
               sorting_data[self.trimming_se.unwrap()].set_tertiary_link(lozenge_se);

               lozenge_se = self.trimming_se;
               self.trimming_se = new_trimming_se;
            }
         } else {
            while self.trimming_se != None {
               let new_trimming_se = sorting_data[self.trimming_se.unwrap()].get_tertiary_link();
               sorting_data[self.trimming_se.unwrap()].set_tertiary_link(lozenge_se);

               lozenge_se = self.trimming_se;
               self.trimming_se = new_trimming_se;
            }
         }
      } else {
         self.trimming_se = Some(merge_step.middle - 1);
         lozenge_se = None;
      }

      self.all_se_appended = self.trimming_se == Some(merge_step.middle - 1);
      self.all_se_overlapped = self.trimming_se == None;

      //

      let mut lozenge_sw;
      if self.left_head_right_anchor {
         self.trimming_sw = right_p_dfs_start;
         lozenge_sw = Some(merge_step.middle);
         if !self.left_head_right_head {
            while sorting_stats.cmp_sortable(
               &sorting_data[left_head],
               &sorting_data[self.trimming_sw.unwrap()],
               SortStatsCounts::NWork,
               merge_step.shifted_level,
            ) {
               let new_trimming_sw = sorting_data[self.trimming_sw.unwrap()].get_secondary_link();
               sorting_data[self.trimming_sw.unwrap()].set_secondary_link(lozenge_sw);

               lozenge_sw = self.trimming_sw;
               self.trimming_sw = new_trimming_sw;
            }
         } else {
            while self.trimming_sw != None {
               let new_trimming_sw = sorting_data[self.trimming_sw.unwrap()].get_secondary_link();
               sorting_data[self.trimming_sw.unwrap()].set_secondary_link(lozenge_sw);

               lozenge_sw = self.trimming_sw;
               self.trimming_sw = new_trimming_sw;
            }
         }
      } else {
         self.trimming_sw = Some(merge_step.middle);
         lozenge_sw = None;
      }

      self.all_sw_appended = self.trimming_sw == Some(merge_step.middle);
      self.all_sw_overlapped = self.trimming_sw == None;
      assert!(self.all_sw_overlapped == self.left_head_right_head);

      assert!(self.all_sw_overlapped || self.all_se_overlapped);
      assert!(!self.all_sw_overlapped || !self.all_se_overlapped);

      //

      if self.all_se_appended {
         self.parents_head = lozenge_east_ext;
      } else if self.trimming_se == None {
         // Not even the joining (merge_step.middle - 1) remains in the chain.
         self.parents_head = lozenge_se;
      } else {
         self.parents_head = self.trimming_se;
      }
      assert!(self.parents_head != None);

      if self.all_sw_appended {
         self.children_head = lozenge_west_ext;
      } else if self.trimming_sw == None {
         // Not even the joining (merge_step.middle - 1) remains in the chain.
         self.children_head = lozenge_sw;
      } else {
         self.children_head = self.trimming_sw;
      }
      assert!(self.children_head != None);

      // Saves can probably be absorbed into preceding S lozenge traversals.
      if self.trimming_se != None {
         self.restore_se = sorting_data[self.trimming_se.unwrap()].get_tertiary_link();
         if self.trimming_se.unwrap() != (merge_step.middle - 1) {
            sorting_data[self.trimming_se.unwrap()].set_tertiary_link(lozenge_se);
         }
      } else {
         self.restore_se = None;
      }
      if self.trimming_sw != None {
         self.restore_sw = sorting_data[self.trimming_sw.unwrap()].get_secondary_link();
         if self.trimming_sw.unwrap() != (merge_step.middle) {
            sorting_data[self.trimming_sw.unwrap()].set_secondary_link(lozenge_sw);
         }
      } else {
         self.restore_sw = None;
      }
   }

   pub fn main_full_loop(
      &mut self,
      sorting_data: &mut [SortableEntity],
      right_sub: &WscSubSort,
      left_sub: &WscSubSort,
      merge_step: &MergeStep,
      sorting_stats: &mut SortStats,
   ) {
      let block_head_test: bool = self.left_head_right_head;

      let right_head = right_sub.head;
      let left_head = left_sub.head;
      let right_tail = right_sub.tail;
      let left_tail = left_sub.tail;
      let right_anchor = merge_step.middle;
      let left_anchor = merge_step.middle - 1;

      sorting_data[left_tail].set_tertiary_link(self.children_head);
      sorting_data[right_tail].set_secondary_link(self.parents_head);

      self.curr_l = self.parents_head.unwrap();
      self.curr_r = self.children_head.unwrap();

      self.left_chain_state = BlockSkipFsm::AtRoot;
      self.right_chain_state = BlockSkipFsm::AtRoot;
      self.left_next_block = self.curr_l; // Fake but "valid" value: Not properly set when in AtRoot.
      self.right_next_block = self.curr_r;
      self.consumed_left_east = self.left_anchor_right_head;
      self.consumed_right_west = !self.left_head_right_anchor;
      self.consume_left = block_head_test;
      self.curr_head = if self.consume_left {
         sorting_data[self.curr_l].get_backward_link().unwrap()
      } else {
         sorting_data[self.curr_r].get_backward_link().unwrap()
      };

      // self.curr_head is basically the last-appended in the sort, so we add just after that.

      // Invariant made into more specific assurance:
      assert!((self.curr_l != right_head) && (self.curr_r != left_head));

      if self.consume_left {
         self.left_chain_state = BlockSkipFsm::Stepwise;
         self.left_next_block = sorting_data[self.curr_l].get_tertiary_link().unwrap();
      } else {
         self.right_chain_state = BlockSkipFsm::Stepwise;
         self.right_next_block = sorting_data[self.curr_r].get_secondary_link().unwrap();
      }

      loop {
         if self.consume_left {
            let prev_l = self.curr_l;
            sorting_data[self.curr_head].set_forward_link(Some(self.curr_l));
            sorting_data[self.curr_l].set_backward_link(Some(self.curr_head));
            self.curr_head = self.curr_l;
            self.curr_l = sorting_data[self.curr_l].get_forward_link().unwrap();

            match self.left_chain_state {
               BlockSkipFsm::AtRoot => {
                  self.left_next_block = sorting_data[prev_l].get_tertiary_link().unwrap();

                  if self.curr_l != self.left_next_block {
                     self.left_chain_state = BlockSkipFsm::JustBeyond;
                  }
                  if prev_l == merge_step.middle - 1 {
                     self.consumed_left_east = true;
                  }
               }
               BlockSkipFsm::JustBeyond => {
                  if self.curr_l == self.left_next_block {
                     self.left_chain_state = BlockSkipFsm::AtRoot;
                  } else {
                     let one_before_next =
                        sorting_data[self.left_next_block].get_backward_link().unwrap();
                     // The following dual lookahead is only slightly better than single lookahead.
                     if (one_before_next != self.curr_l)
                        && (one_before_next
                           != sorting_data[self.curr_l].get_forward_link().unwrap())
                     {
                        let consume_block = sorting_stats.cmp_sortable(
                           &sorting_data[one_before_next],
                           &sorting_data[self.curr_r],
                           SortStatsCounts::NLogN,
                           merge_step.shifted_level,
                        );
                        if consume_block {
                           self.left_chain_state = BlockSkipFsm::AtRoot;
                           self.curr_head = one_before_next;
                           self.curr_l = sorting_data[one_before_next].get_forward_link().unwrap();
                        } else {
                           self.left_chain_state = BlockSkipFsm::Stepwise;
                        }
                     }
                  }
               }
               BlockSkipFsm::Stepwise => {
                  if self.curr_l == self.left_next_block {
                     self.left_chain_state = BlockSkipFsm::AtRoot;
                  }
               }
            }

            if (self.curr_l == right_head) || (self.curr_r == left_head) {
               break;
            }

            if !self.has_anchoring {
               self.consume_left = sorting_stats.cmp_sortable(
                  &sorting_data[self.curr_l],
                  &sorting_data[self.curr_r],
                  SortStatsCounts::NLogN, // YYYY
                  merge_step.shifted_level,
               );
            } else if self.curr_l == left_anchor {
               if self.curr_r == right_anchor {
                  self.consume_left = self.left_anchor_right_anchor;
               } else if self.curr_r == right_tail {
                  self.consume_left = self.left_anchor_right_tail;
               } else if self.curr_r == right_head {
                  self.consume_left = self.left_anchor_right_head;
               } else {
                  self.consume_left = sorting_stats.cmp_sortable(
                     &sorting_data[self.curr_l],
                     &sorting_data[self.curr_r],
                     SortStatsCounts::NLogN, // YYYY
                     merge_step.shifted_level,
                  );
               }
            } else if self.curr_r == right_anchor {
               if self.curr_l == left_tail {
                  self.consume_left = self.left_tail_right_anchor;
               } else if self.curr_l == left_head {
                  self.consume_left = self.left_head_right_anchor;
               } else {
                  self.consume_left = sorting_stats.cmp_sortable(
                     &sorting_data[self.curr_l],
                     &sorting_data[self.curr_r],
                     SortStatsCounts::NLogN, // YYYY
                     merge_step.shifted_level,
                  );
               }
            } else {
               self.consume_left = sorting_stats.cmp_sortable(
                  &sorting_data[self.curr_l],
                  &sorting_data[self.curr_r],
                  SortStatsCounts::NLogN, // YYYY
                  merge_step.shifted_level,
               );
            }
         } else {
            let prev_r = self.curr_r;
            sorting_data[self.curr_head].set_forward_link(Some(self.curr_r));
            sorting_data[self.curr_r].set_backward_link(Some(self.curr_head));
            self.curr_head = self.curr_r;
            self.curr_r = sorting_data[self.curr_r].get_forward_link().unwrap();

            match self.right_chain_state {
               BlockSkipFsm::AtRoot => {
                  self.right_next_block = sorting_data[prev_r].get_secondary_link().unwrap();

                  if self.curr_r != self.right_next_block {
                     self.right_chain_state = BlockSkipFsm::JustBeyond;
                  }
                  if prev_r == merge_step.middle {
                     self.consumed_right_west = true;
                  }
               }
               BlockSkipFsm::JustBeyond => {
                  if self.curr_r == self.right_next_block {
                     self.right_chain_state = BlockSkipFsm::AtRoot;
                  } else {
                     let one_before_next =
                        sorting_data[self.right_next_block].get_backward_link().unwrap();
                     // The following dual lookahead is only slightly better than single lookahead.
                     if (one_before_next != self.curr_r)
                        && (one_before_next
                           != sorting_data[self.curr_r].get_forward_link().unwrap())
                     {
                        let consume_block = !sorting_stats.cmp_sortable(
                           &sorting_data[self.curr_l],
                           &sorting_data[one_before_next],
                           SortStatsCounts::NLogN,
                           merge_step.shifted_level,
                        );
                        if consume_block {
                           self.right_chain_state = BlockSkipFsm::AtRoot;
                           self.curr_head = one_before_next;
                           self.curr_r = sorting_data[one_before_next].get_forward_link().unwrap();
                        } else {
                           self.right_chain_state = BlockSkipFsm::Stepwise;
                        }
                     }
                  }
               }
               BlockSkipFsm::Stepwise => {
                  if self.curr_r == self.right_next_block {
                     self.right_chain_state = BlockSkipFsm::AtRoot;
                  }
               }
            }

            if (self.curr_l == right_head) || (self.curr_r == left_head) {
               break;
            }
            if !self.has_anchoring {
               self.consume_left = sorting_stats.cmp_sortable(
                  &sorting_data[self.curr_l],
                  &sorting_data[self.curr_r],
                  SortStatsCounts::NLogN, // YYYY
                  merge_step.shifted_level,
               );
            } else if self.curr_l == left_anchor {
               if self.curr_r == right_anchor {
                  self.consume_left = self.left_anchor_right_anchor;
               } else if self.curr_r == right_tail {
                  self.consume_left = self.left_anchor_right_tail;
               } else if self.curr_r == right_head {
                  self.consume_left = self.left_anchor_right_head;
               } else {
                  self.consume_left = sorting_stats.cmp_sortable(
                     &sorting_data[self.curr_l],
                     &sorting_data[self.curr_r],
                     SortStatsCounts::NLogN, // YYYY
                     merge_step.shifted_level,
                  );
               }
            } else if self.curr_r == right_anchor {
               if self.curr_l == left_tail {
                  self.consume_left = self.left_tail_right_anchor;
               } else if self.curr_l == left_head {
                  self.consume_left = self.left_head_right_anchor;
               } else {
                  self.consume_left = sorting_stats.cmp_sortable(
                     &sorting_data[self.curr_l],
                     &sorting_data[self.curr_r],
                     SortStatsCounts::NLogN, // YYYY
                     merge_step.shifted_level,
                  );
               }
            } else {
               self.consume_left = sorting_stats.cmp_sortable(
                  &sorting_data[self.curr_l],
                  &sorting_data[self.curr_r],
                  SortStatsCounts::NLogN, // YYYY
                  merge_step.shifted_level,
               );
            }
         }

         // Invariant:
         assert!(
            (self.consume_left && (self.curr_r != left_head))
               || (!self.consume_left && (self.curr_l != right_head))
         );
      }
   }
   pub fn post_work(
      &mut self,
      sorting_data: &mut [SortableEntity],
      right_sub: &WscSubSort,
      left_sub: &WscSubSort,
      merge_step: &MergeStep,
      _sorting_stats: &mut SortStats,
   ) {
      let block_head_test: bool = self.left_head_right_head;

      let right_head = right_sub.head;
      let left_head = left_sub.head;
      let right_tail = right_sub.tail;
      let left_tail = left_sub.tail;
      let right_p_dfs_start = right_sub.p_dfs_start;
      let left_c_dfs_start = left_sub.c_dfs_start;

      // let curr_head: usize = self.curr_head;
      let curr_l: usize = self.curr_l;
      let curr_r: usize = self.curr_r;

      // let consume_left: bool = self.consume_left;
      let consumed_left_east: bool = self.consumed_left_east;
      let consumed_right_west: bool = self.consumed_right_west;
      let left_next_block: usize = self.left_next_block;
      let right_next_block: usize = self.right_next_block;

      // Trailing NE and NW lozenge traversal was performed during
      // main loop. Recover from comparisons.
      let trimming_ne = if curr_l == right_head {
         None
      } else if !consumed_left_east {
         Some(merge_step.middle - 1)
      } else {
         Some(left_next_block)
      };

      let trimming_nw = if curr_r == left_head {
         None
      } else if !consumed_right_west {
         Some(merge_step.middle)
      } else {
         Some(right_next_block)
      };

      // ========================================
      // Surgery on fronts.

      assert!((trimming_ne != None) || (trimming_nw != None));
      assert!((trimming_ne == None) || (trimming_nw == None));

      //

      if self.left_anchor_right_head {
         // When appending, the appended part needs to be complete chain.
         sorting_data[merge_step.middle - 1].set_tertiary_link(left_c_dfs_start);
      }

      if !self.left_head_right_anchor {
         // When appending, the appended part needs to be complete chain.
         sorting_data[merge_step.middle].set_secondary_link(right_p_dfs_start);
      }

      // Append L NE to R NE, and R NW to L NW.
      sorting_data[right_tail].set_tertiary_link(trimming_ne);
      sorting_data[left_tail].set_secondary_link(trimming_nw);

      // Append L SE to R SE.

      self.right_c_dfs_start = right_sub.c_dfs_start;
      if block_head_test {
         if self.right_c_dfs_start == None {
            self.right_c_dfs_start = self.trimming_se;
         } else {
            sorting_data[right_head].set_tertiary_link(self.trimming_se);
         }
      }

      // Append R SW to L SW.

      self.left_p_dfs_start = left_sub.p_dfs_start;
      if !block_head_test {
         if self.left_p_dfs_start == None {
            self.left_p_dfs_start = self.trimming_sw;
         } else {
            sorting_data[left_head].set_secondary_link(self.trimming_sw);
         }
      }

      // Done with surgery on fronts.
      // ========================================
   }

   pub fn optional_reverse(
      sorting_data: &mut [SortableEntity],
      sort_size: usize,
      final_subsort: &WscSubSort,
   ) {
      let mut lozenge_sw;
      {
         lozenge_sw = final_subsort.p_dfs_start;
         let mut reverse_vertex = Some(0); // Globally left-most vertex.
         while lozenge_sw != None {
            let prev_lozenge_sw_index = lozenge_sw;

            let lozenge_sw_index = lozenge_sw.unwrap();
            lozenge_sw = sorting_data[lozenge_sw_index].get_secondary_link();
            sorting_data[lozenge_sw_index].set_secondary_link(reverse_vertex);

            reverse_vertex = prev_lozenge_sw_index;
         }
      }

      let mut lozenge_se;
      {
         lozenge_se = final_subsort.c_dfs_start;
         let mut reverse_vertex = Some(sort_size - 1);
         while lozenge_se != None {
            let prev_lozenge_se_index = lozenge_se;

            let lozenge_se_index = lozenge_se.unwrap();
            lozenge_se = sorting_data[lozenge_se_index].get_tertiary_link();
            sorting_data[lozenge_se_index].set_tertiary_link(reverse_vertex);

            reverse_vertex = prev_lozenge_se_index;
         }
      }
   }
}

#[inline]
fn sort_standard_wip(
   sorting_data: &mut [SortableEntity],
   sorting_stats: &mut SortStats,
   config: WscSortConfig,
) -> usize {
   // let forced_type_switch_level = 1000; // Used to decide whether to do the simplified main loop.
   // let type_switch_level = 0;
   // let direct_merge_complete = false;
   let type_switch_level = config.type_switch_level;
   // let direct_merge_complete = true;
   // let forced_type_switch_level = type_switch_level;

   let stack_size = std::mem::size_of::<usize>() * 8 + 1;
   let sort_size = sorting_data.len();

   let mut subsort_stack = Vec::<WscSubSort>::new(); // vec![WscSubSort { head: 0 }; stack_size];
   subsort_stack.reserve_exact(stack_size);
   let iter = MinusPlusShift::new(sort_size);

   sorting_stats.start_one();
   for merge_step in iter {
      sorting_stats.start_subsort(merge_step.shifted_level);

      if merge_step.singles_to_add > 0 {
         if merge_step.singles_to_add == 2 {
            subsort_stack.push(WscSubSort {
               has_reverse: true,
               head: merge_step.lower,
               tail: merge_step.lower,
               p_dfs_start: None,
               c_dfs_start: None,
            });
            sorting_data[merge_step.lower].set_forward_link(None);
            sorting_data[merge_step.lower].set_backward_link(None);
            sorting_data[merge_step.lower].set_secondary_link(None);
            sorting_data[merge_step.lower].set_tertiary_link(None);
         }
         subsort_stack.push(WscSubSort {
            has_reverse: true,
            head: merge_step.middle,
            tail: merge_step.middle,
            p_dfs_start: None,
            c_dfs_start: None,
         });
         sorting_data[merge_step.middle].set_forward_link(None);
         sorting_data[merge_step.middle].set_backward_link(None);
         sorting_data[merge_step.middle].set_secondary_link(None);
         sorting_data[merge_step.middle].set_tertiary_link(None);
      }
      assert!(subsort_stack.len() <= stack_size);

      let right_sub = subsort_stack.pop().unwrap();
      let left_sub = subsort_stack.pop().unwrap();

      let right_head = right_sub.head;
      let left_head = left_sub.head;
      let right_tail = right_sub.tail;
      let left_tail = left_sub.tail;

      let mut structure = StructuralAnalysis::new();
      // StructuralAnalysis::new(sorting_data, &right_sub, &left_sub, &merge_step, sorting_stats);

      let stage_sort_type: WscSortType = if merge_step.shifted_level >= type_switch_level {
         config.main_sort_type
      } else {
         config.lower_sort_type
      };

      match stage_sort_type {
         WscSortType::Classic => structure.pre_work_minimal(
            sorting_data,
            &right_sub,
            &left_sub,
            &merge_step,
            sorting_stats,
         ),
         WscSortType::AnchorClassic | WscSortType::AnchorSkipless | WscSortType::AnchorSkipper => {
            structure.pre_work_anchor(
               sorting_data,
               &right_sub,
               &left_sub,
               &merge_step,
               sorting_stats,
            );
         }
         WscSortType::Legacy1 => {
            structure.pre_work_full(sorting_data, &right_sub, &left_sub, &merge_step, sorting_stats)
         }
      }
      // ========================================

      sorting_data[left_tail].set_forward_link(Some(right_head));
      sorting_data[right_tail].set_forward_link(Some(left_head));
      sorting_data[right_head].set_backward_link(Some(left_tail));
      sorting_data[left_head].set_backward_link(Some(right_tail));

      // ========================================

      match stage_sort_type {
         WscSortType::Classic | WscSortType::AnchorClassic => {
            structure.classic_loop(sorting_data, &right_sub, &left_sub, &merge_step, sorting_stats);
         }
         WscSortType::AnchorSkipless => {
            structure.anchor_classic_loop(
               sorting_data,
               &right_sub,
               &left_sub,
               &merge_step,
               sorting_stats,
            );
         }
         WscSortType::Legacy1 | WscSortType::AnchorSkipper => {
            // ========================================
            // Surgery on fronts.

            structure.lozenge_pre_work(
               sorting_data,
               &right_sub,
               &left_sub,
               &merge_step,
               sorting_stats,
            );

            if config.check_block_chain {
               check_block_list(
                  sorting_data,
                  &merge_step,
                  structure.parents_head,
                  structure.children_head,
                  left_tail,
                  right_tail,
                  structure.all_se_appended,
                  structure.all_sw_appended,
               );
            }

            // Done with surgery on fronts.
            // ========================================

            assert!(
               structure.left_head_right_head
                  == (sorting_data[structure.parents_head.unwrap()].value
                     <= sorting_data[structure.children_head.unwrap()].value)
            );

            // ========================================
            // Main merge loop

            structure.main_full_loop(
               sorting_data,
               &right_sub,
               &left_sub,
               &merge_step,
               sorting_stats,
            );
         }
      }

      let curr_head: usize = structure.curr_head;
      let curr_l: usize = structure.curr_l;
      let curr_r: usize = structure.curr_r;

      let consume_left: bool = structure.consume_left;

      //
      let final_head: usize = if structure.left_head_right_head { left_head } else { right_head };

      //

      // Invariant made into more specific assurance.
      assert!(
         (consume_left && (curr_r != left_head) && (curr_l == right_head))
            || (!consume_left && (curr_l != right_head) && (curr_r == left_head))
      );

      // Need to consume opposite of what was last consumed.
      let final_tail: usize = if consume_left {
         sorting_data[curr_head].set_forward_link(Some(curr_r));
         sorting_data[curr_r].set_backward_link(Some(curr_head));
         right_tail
      } else {
         sorting_data[curr_head].set_forward_link(Some(curr_l));
         sorting_data[curr_l].set_backward_link(Some(curr_head));
         left_tail
      };

      // Restore ends of linked list of sorted nodes.
      sorting_data[final_tail].set_forward_link(None);
      sorting_data[final_head].set_backward_link(None);
      sorting_data[left_tail].set_tertiary_link(None);
      sorting_data[right_tail].set_secondary_link(None);

      // End of main merge loop.
      // ========================================

      let trimming_se = structure.trimming_se;
      let trimming_sw = structure.trimming_sw;
      let restore_se = structure.restore_se;
      let restore_sw = structure.restore_sw;

      match stage_sort_type {
         WscSortType::Classic | WscSortType::AnchorClassic | WscSortType::AnchorSkipless => {
            structure.post_work(sorting_data, &right_sub, &left_sub, &merge_step, sorting_stats);
         }
         WscSortType::Legacy1 | WscSortType::AnchorSkipper => {
            if trimming_se != None {
               sorting_data[trimming_se.unwrap()].set_tertiary_link(restore_se);
            }
            if trimming_sw != None {
               sorting_data[trimming_sw.unwrap()].set_secondary_link(restore_sw);
            }

            structure.post_work(sorting_data, &right_sub, &left_sub, &merge_step, sorting_stats);
         }
      }

      subsort_stack.push(WscSubSort {
         has_reverse: true,
         head: final_head,
         tail: final_tail,
         p_dfs_start: structure.left_p_dfs_start,
         c_dfs_start: structure.right_c_dfs_start,
      });
      sorting_stats.finish_subsort(merge_step.shifted_level);
   } // Merge step iteration.

   sorting_stats.finish_one();

   let final_subsort = subsort_stack.pop().unwrap();

   // Optionally reverse final lowers.
   if config.final_reverse {
      // assert!((type_switch_level == 0) && (config.main_sort_type == WscSortType::Legacy1));
      StructuralAnalysis::optional_reverse(sorting_data, sort_size, &final_subsort);
   }
   // // Optionally reverse final lowers.
   // if (config.main_sort_type == WscSortType::Legacy1) && config.final_reverse {
   //    assert!(type_switch_level == 0);
   //    StructuralAnalysis::optional_reverse(sorting_data, sort_size, &final_subsort);
   // }

   // Final subsort is the completed sort.
   final_subsort.head
}

#[cfg(test)]
mod tests {
   use super::*;

   pub const NUM_EVAL_SAMPLES: u32 = 100;
   pub const PREFERRED_BLEND_MAIN: WscSortType = WscSortType::AnchorSkipper;
   pub const PREFERRED_BLEND_LOWER: WscSortType = WscSortType::AnchorSkipless;
   pub const PREFERRED_BLEND_SWITCH: u32 = 4;

   #[cfg(test)]
   fn run_eval_test(config: SortEvalConfig, dest: EvalDestination) {
      let sort_size_log2: f64 = (config.sort_size as f64).log2();
      // Log base-2 of power of 2 should be exact integer, but allow for a teeny error.
      let sort_size_shifted_limit: u32 = (sort_size_log2 - CEIL_LOG_FIDDLE).ceil() as u32;
      let seed_state = 0xeffcad0d01b5e5e5 + config.randomizer_offset;
      let seed_stream = 0xd02abbf7b07bca3 + 2 * config.randomizer_offset;
      let num_bins = std::cmp::min(512, config.sort_size);

      assert!(1 << sort_size_shifted_limit >= config.sort_size);
      assert!(1 << (sort_size_shifted_limit - 1) < config.sort_size);

      let filename_suffix: &str = match config.sort_choice {
         EvalSortChoice::BASELINE => "_baseline",
         EvalSortChoice::AMS => "_ams",
      };

      let output_style = match dest {
         EvalDestination::Writer(_) => EvalOutputStyle::HUMAN,
         EvalDestination::GoldenFile() => EvalOutputStyle::CSVM,
      };

      // let mut stdout_writer = io::stdout();
      // let mut out_writer: BufWriter<File>;
      // let out_file: std::fs::File;
      let mut mint: Option<goldenfile::Mint>; // = Mint::new("tests/golden-outputs");
      let mut out_writer: Option<BufWriter<File>>; // = None;

      // let writer: Option<&mut dyn io::Write> = match dest {
      let writer: &mut dyn std::io::Write = match dest {
         // EvalDestination::Writer(unwrapped_writer) => Some(unwrapped_writer),
         EvalDestination::Writer(unwrapped_writer) => unwrapped_writer,
         EvalDestination::GoldenFile() => {
            let filename = config.base_name.to_string();
            mint = Some(Mint::new("tests/golden-outputs"));
            let out_file: std::fs::File =
               mint.as_mut().unwrap().new_goldenfile(filename + filename_suffix + ".m").unwrap();

            out_writer = Some(BufWriter::new(out_file));

            out_writer.as_mut().unwrap()
         } // EvalDestination::StdOut => &mut stdout_writer,
      };

      let mut perm_values: Vec<u32> = (0..config.sort_size).collect();

      let mut sorting_data: Vec<SortableEntity> =
         vec![SortableEntity::default(); config.sort_size as usize];

      let mut rng = rand_pcg::Pcg32::new(seed_state, seed_stream);
      // let mut fy = shuffle::fy::FisherYates::default();

      let mut sorting_stats = SortStats::new(SortStatsInit {
         sort_size: config.sort_size as usize,
         sort_size_log2,
         num_bins,
         overstretch: 1.01,
         shifted_limit: sort_size_shifted_limit,
      });

      sorting_stats.start_gather();
      for _i in 0..config.num_samples {
         let mut block_layout = BlockPermutation::new(config.sort_size, config.permutation_config);
         block_layout.commission();
         (config.pattern_mod_op)(&mut block_layout);
         block_layout.generate(&mut perm_values, &mut rng);

         fill_sortable(&mut sorting_data, &perm_values);

         let sorted_head: usize = match config.sort_choice {
            EvalSortChoice::BASELINE => {
               sort_standard_interlink(&mut sorting_data, &mut sorting_stats)
            }
            EvalSortChoice::AMS => sort_standard_wip(
               &mut sorting_data,
               &mut sorting_stats,
               WscSortConfig {
                  type_switch_level: config.type_switch_level,
                  main_sort_type: PREFERRED_BLEND_MAIN,
                  lower_sort_type: PREFERRED_BLEND_LOWER,
                  ..Default::default()
               },
            ),
         };

         check_sort(&sorting_data, sorted_head, false);
      }
      sorting_stats.finish_gather();

      match output_style {
         EvalOutputStyle::HUMAN => {
            sorting_stats.write_summary_info(writer);
            sorting_stats.write_summary_info_by_k(writer);
         }
         EvalOutputStyle::CSVM => {
            sorting_stats
               .write_summary_info_csvm(&(config.base_name.to_string() + filename_suffix), writer);
         }
      }
   }

   fn standard_wip_rand_perms_unique(
      main_sort_type: WscSortType,
      lower_sort_type: WscSortType,
      type_switch_level: u32,
   ) {
      let sort_size: u32 = 64;
      let sort_size_log2: f64 = (sort_size as f64).log2();
      // Log base-2 of power of 2 should be exact integer, but allow for a teeny error.
      let sort_size_shifted_limit: u32 = (sort_size_log2 - CEIL_LOG_FIDDLE).ceil() as u32;
      let num_samples: u64 = 10000;
      let seed_state = 0xeffcad0d01b5e5e5;
      let seed_stream = 0xd02abbf7b07bca3;
      let num_bins = std::cmp::min(512, sort_size);

      assert!(1 << sort_size_shifted_limit >= sort_size);
      assert!(1 << (sort_size_shifted_limit - 1) < sort_size);

      let mut perm_values: Vec<u32> = (0..sort_size).collect();

      let mut sorting_data: Vec<SortableEntity> =
         vec![SortableEntity::default(); sort_size as usize];

      let mut permutation_count: u64 = 0;

      let mut rng = rand_pcg::Pcg32::new(seed_state, seed_stream);
      let mut fy = shuffle::fy::FisherYates::default();

      let mut sorting_stats = SortStats::new(SortStatsInit {
         sort_size: sort_size as usize,
         sort_size_log2,
         num_bins,
         overstretch: 1.01,
         shifted_limit: sort_size_shifted_limit,
      });

      sorting_stats.start_gather();
      for _i in 0..num_samples {
         use shuffle::shuffler::Shuffler; // For fy.shuffle.
         assert!(fy.shuffle(&mut perm_values, &mut rng).is_ok());

         permutation_count += 1;
         fill_sortable(&mut sorting_data, &perm_values);

         // for i in 0..sort_size as usize {
         //    println!("\t{}", sorting_data[i].value);
         // }
         let sorted_head: usize = sort_standard_wip(
            &mut sorting_data,
            &mut sorting_stats,
            WscSortConfig {
               main_sort_type,
               lower_sort_type,
               final_reverse: false,
               unused_skip_upper_lozenge: false,
               check_block_chain: true,
               type_switch_level,
            },
         );

         check_sort(&sorting_data, sorted_head, true);
      }
      sorting_stats.finish_gather();

      assert_eq!(permutation_count, num_samples);

      // let mut mint = Mint::new("tests/golden-outputs");
      // let out_file = mint.new_goldenfile("standard_wip.m").unwrap();
      // let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
      // sorting_stats.write_golden_info(&mut out_writer);
   }

   fn standard_wip_rand_perms_multi_size(
      main_sort_type: WscSortType,
      lower_sort_type: WscSortType,
      type_switch_level: u32,
   ) {
      let seed_state = 0xeffcad0d01b5e5e5;
      let seed_stream = 0xd02abbf7b07bca3;
      let mut rng = rand_pcg::Pcg32::new(seed_state, seed_stream);
      let mut fy = shuffle::fy::FisherYates::default();

      let low_sort_size: u32 = 3;
      let high_sort_size: u32 = 600;
      for sort_size in low_sort_size..high_sort_size {
         let sort_size_log2: f64 = (sort_size as f64).log2();
         // Log base-2 of power of 2 should be exact integer, but allow for a teeny error.
         let sort_size_shifted_limit: u32 = (sort_size_log2 - CEIL_LOG_FIDDLE).ceil() as u32;
         let num_bins = std::cmp::min(512, sort_size);

         assert!(1 << sort_size_shifted_limit >= sort_size);
         assert!(1 << (sort_size_shifted_limit - 1) < sort_size);

         let mut perm_values: Vec<u32> = (0..sort_size).collect();

         let mut sorting_data: Vec<SortableEntity> =
            vec![SortableEntity::default(); sort_size as usize];

         let mut sorting_stats = SortStats::new(SortStatsInit {
            sort_size: sort_size as usize,
            sort_size_log2,
            num_bins,
            overstretch: 1.01,
            shifted_limit: sort_size_shifted_limit,
         });

         sorting_stats.start_gather();
         use shuffle::shuffler::Shuffler; // For fy.shuffle.
         assert!(fy.shuffle(&mut perm_values, &mut rng).is_ok());

         fill_sortable(&mut sorting_data, &perm_values);

         // for i in 0..sort_size as usize {
         //    println!("\t{}", sorting_data[i].value);
         // }
         let sorted_head: usize = sort_standard_wip(
            &mut sorting_data,
            &mut sorting_stats,
            WscSortConfig {
               main_sort_type,
               lower_sort_type,
               final_reverse: false,
               unused_skip_upper_lozenge: false,
               check_block_chain: true,
               type_switch_level,
            },
         );

         check_sort(&sorting_data, sorted_head, true);
      }
      // sorting_stats.finish_gather();
   }

   #[test]
   fn standard_wip_rand_perms_unique_legacy1() {
      standard_wip_rand_perms_unique(WscSortType::Legacy1, WscSortType::Legacy1, 0);
   }
   #[test]
   fn standard_wip_rand_perms_unique_classic() {
      standard_wip_rand_perms_unique(WscSortType::Classic, WscSortType::Classic, 0);
   }
   #[test]
   fn standard_wip_rand_perms_unique_aclassic() {
      standard_wip_rand_perms_unique(WscSortType::AnchorClassic, WscSortType::AnchorClassic, 0);
   }
   #[test]
   fn standard_wip_rand_perms_unique_skipless() {
      standard_wip_rand_perms_unique(
         WscSortType::AnchorSkipless,
         WscSortType::AnchorSkipless,
         1000,
      );
   }
   #[test]
   fn standard_wip_rand_perms_unique_skipper() {
      standard_wip_rand_perms_unique(WscSortType::AnchorSkipper, WscSortType::AnchorSkipper, 0);
   }
   // Keep this one up-to-date as our "preferred" blend.
   #[test]
   fn standard_wip_rand_perms_unique_anchor_legacy1() {
      standard_wip_rand_perms_unique(
         PREFERRED_BLEND_MAIN,
         PREFERRED_BLEND_LOWER,
         PREFERRED_BLEND_SWITCH,
      );
   }

   #[test]
   fn standard_wip_rand_perms_multi_size_legacy1() {
      standard_wip_rand_perms_multi_size(WscSortType::Legacy1, WscSortType::Legacy1, 0);
   }
   #[test]
   fn standard_wip_rand_perms_multi_size_classic() {
      standard_wip_rand_perms_multi_size(WscSortType::Classic, WscSortType::Classic, 0);
   }
   #[test]
   fn standard_wip_rand_perms_multi_size_aclassic() {
      standard_wip_rand_perms_multi_size(WscSortType::AnchorClassic, WscSortType::AnchorClassic, 0);
   }
   #[test]
   fn standard_wip_rand_perms_multi_size_skipless() {
      standard_wip_rand_perms_multi_size(
         WscSortType::AnchorSkipless,
         WscSortType::AnchorSkipless,
         0,
      );
   }
   #[test]
   fn standard_wip_rand_perms_multi_size_skipper() {
      standard_wip_rand_perms_multi_size(WscSortType::AnchorSkipper, WscSortType::AnchorSkipper, 0);
   }
   #[test]
   fn standard_wip_rand_perms_multi_size_classic_legacy1() {
      standard_wip_rand_perms_multi_size(WscSortType::Legacy1, WscSortType::Classic, 4);
   }
   #[test]
   fn standard_wip_rand_perms_multi_size_skipless_legacy1() {
      standard_wip_rand_perms_multi_size(WscSortType::AnchorSkipless, WscSortType::Classic, 4);
   }
   #[test]
   fn standard_wip_rand_perms_multi_size_skipless_skipper() {
      standard_wip_rand_perms_multi_size(
         WscSortType::AnchorSkipless,
         WscSortType::AnchorSkipper,
         4,
      );
   }
   #[test]
   fn standard_wip_rand_perms_multi_size_anchor() {
      standard_wip_rand_perms_multi_size(
         WscSortType::AnchorClassic,
         WscSortType::AnchorClassic,
         1000,
      );
   }
   #[test]
   fn standard_wip_rand_perms_multi_size_anchor_legacy1() {
      standard_wip_rand_perms_multi_size(WscSortType::Legacy1, WscSortType::AnchorClassic, 4);
   }

   fn standard_wip_rand_perms_stable(
      main_sort_type: WscSortType,
      lower_sort_type: WscSortType,
      type_switch_level: u32,
   ) {
      let sort_size: u32 = 64;
      let number_of_nudges: u32 = 5;
      let sort_size_log2: f64 = (sort_size as f64).log2();
      // Log base-2 of power of 2 should be exact integer, but allow for a teeny error.
      let sort_size_shifted_limit: u32 = (sort_size_log2 - CEIL_LOG_FIDDLE).ceil() as u32;
      let num_samples: u64 = 1000;
      let seed_state = 0xeffcad0d01b5e5e5;
      let seed_stream = 0xd02abbf7b07bca3;
      let num_bins = std::cmp::min(512, sort_size);

      assert!(1 << sort_size_shifted_limit >= sort_size);
      assert!(1 << (sort_size_shifted_limit - 1) < sort_size);

      let mut perm_values: Vec<u32>;

      let mut sorting_data: Vec<SortableEntity> =
         vec![SortableEntity::default(); sort_size as usize];

      let mut permutation_count: u64 = 0;

      let mut rng = rand_pcg::Pcg32::new(seed_state, seed_stream);
      let mut fy = shuffle::fy::FisherYates::default();

      let mut sorting_stats = SortStats::new(SortStatsInit {
         sort_size: sort_size as usize,
         sort_size_log2,
         num_bins,
         overstretch: 1.01,
         shifted_limit: sort_size_shifted_limit,
      });

      sorting_stats.start_gather();
      for _i in 0..num_samples {
         perm_values = (0..sort_size).collect();
         use shuffle::shuffler::Shuffler; // For fy.shuffle.
         assert!(fy.shuffle(&mut perm_values, &mut rng).is_ok());
         nudge_values(&mut perm_values, &mut rng, number_of_nudges);

         permutation_count += 1;
         fill_sortable(&mut sorting_data, &perm_values);

         let sorted_head: usize = sort_standard_wip(
            &mut sorting_data,
            &mut sorting_stats,
            WscSortConfig {
               main_sort_type,
               lower_sort_type,
               final_reverse: false,
               unused_skip_upper_lozenge: false,
               check_block_chain: true,
               type_switch_level,
            },
         );

         check_sort(&sorting_data, sorted_head, false);
      }
      sorting_stats.finish_gather();

      assert_eq!(permutation_count, num_samples);
   }

   #[test]
   fn standard_wip_rand_perms_stable_unsimple() {
      standard_wip_rand_perms_stable(WscSortType::Legacy1, WscSortType::Classic, 0);
   }

   #[test]
   fn standard_wip_rand_perms_stable_simple() {
      standard_wip_rand_perms_stable(WscSortType::Classic, WscSortType::Classic, 1000);
   }

   fn standard_wip_lozenge(
      main_sort_type: WscSortType,
      lower_sort_type: WscSortType,
      type_switch_level: u32,
   ) {
      let seed_state = 0x6fcad0e0b15d55ee;
      let seed_stream = 0x02abbdf7b07c3ab;
      let mut rng = rand_pcg::Pcg32::new(seed_state, seed_stream);
      let mut fy = shuffle::fy::FisherYates::default();

      let sort_size: u32 = 32;
      let number_of_nudges: u32 = 8;
      // let sort_size: u32 = 27;
      // let number_of_nudges: u32 = 12;

      let sort_size_log2: f64 = (sort_size as f64).log2();
      // Log base-2 of power of 2 should be exact integer, but allow for a teeny error.
      let sort_size_shifted_limit: u32 = (sort_size_log2 - CEIL_LOG_FIDDLE).ceil() as u32;
      let num_bins = std::cmp::min(512, sort_size);

      assert!(1 << sort_size_shifted_limit >= sort_size);
      assert!(1 << (sort_size_shifted_limit - 1) < sort_size);

      let mut perm_values: Vec<u32>;

      let mut sorting_data: Vec<SortableEntity> =
         vec![SortableEntity::default(); sort_size as usize];

      // We do not care about stats for a single run, but still have to provide.
      let mut sorting_stats = SortStats::new(SortStatsInit {
         sort_size: sort_size as usize,
         sort_size_log2,
         num_bins,
         overstretch: 1.01,
         shifted_limit: sort_size_shifted_limit,
      });
      sorting_stats.start_gather();

      {
         perm_values = (0..sort_size).collect();
         use shuffle::shuffler::Shuffler; // For fy.shuffle.
         assert!(fy.shuffle(&mut perm_values, &mut rng).is_ok());
         nudge_values(&mut perm_values, &mut rng, number_of_nudges);

         fill_sortable(&mut sorting_data, &perm_values);
         let sorted_head: usize = sort_standard_wip(
            &mut sorting_data,
            &mut sorting_stats,
            WscSortConfig {
               main_sort_type,
               lower_sort_type,
               final_reverse: true,
               unused_skip_upper_lozenge: false,
               check_block_chain: true,
               type_switch_level,
            },
         );
         check_sort(&sorting_data, sorted_head, true);
         check_lozenge(&sorting_data, sorted_head);
      }
   }

   fn standard_wip_lozenge_sizes_stable(
      main_sort_type: WscSortType,
      lower_sort_type: WscSortType,
      type_switch_level: u32,
   ) {
      let seed_state = 0x6fcad0e0b15d55ee;
      let seed_stream = 0x02abbdf7b07c3ab;
      let mut rng = rand_pcg::Pcg32::new(seed_state, seed_stream);
      let mut fy = shuffle::fy::FisherYates::default();

      let sort_size_lower: u32 = 27;
      let sort_size_upper: u32 = 500;
      let number_of_nudges: u32 = 12;

      for sort_size in sort_size_lower..sort_size_upper {
         let sort_size_log2: f64 = (sort_size as f64).log2();
         // Log base-2 of power of 2 should be exact integer, but allow for a teeny error.
         let sort_size_shifted_limit: u32 = (sort_size_log2 - CEIL_LOG_FIDDLE).ceil() as u32;
         let num_bins = std::cmp::min(512, sort_size);

         assert!(1 << sort_size_shifted_limit >= sort_size);
         assert!(1 << (sort_size_shifted_limit - 1) < sort_size);

         let mut sorting_data: Vec<SortableEntity> =
            vec![SortableEntity::default(); sort_size as usize];

         // We do not care about stats for a single run, but still have to provide.
         let mut sorting_stats = SortStats::new(SortStatsInit {
            sort_size: sort_size as usize,
            sort_size_log2,
            num_bins,
            overstretch: 1.01,
            shifted_limit: sort_size_shifted_limit,
         });
         sorting_stats.start_gather();

         let mut perm_values: Vec<u32> = (0..sort_size).collect();
         use shuffle::shuffler::Shuffler; // For fy.shuffle.
         assert!(fy.shuffle(&mut perm_values, &mut rng).is_ok());
         nudge_values(&mut perm_values, &mut rng, number_of_nudges);

         fill_sortable(&mut sorting_data, &perm_values);
         let sorted_head: usize = sort_standard_wip(
            &mut sorting_data,
            &mut sorting_stats,
            WscSortConfig {
               main_sort_type,
               lower_sort_type,
               final_reverse: true,
               unused_skip_upper_lozenge: false,
               check_block_chain: true,
               type_switch_level,
            },
         );
         check_sort(&sorting_data, sorted_head, true);
         check_lozenge(&sorting_data, sorted_head);
      }
   }

   #[test]
   fn standard_wip_lozenge_legacy1() {
      standard_wip_lozenge(WscSortType::Legacy1, WscSortType::Classic, 0);
   }
   #[test]
   fn standard_wip_lozenge_classic() {
      standard_wip_lozenge(WscSortType::Classic, WscSortType::Classic, 0);
   }
   #[test]
   fn standard_wip_lozenge_anchor() {
      standard_wip_lozenge(WscSortType::AnchorClassic, WscSortType::AnchorClassic, 0);
   }

   #[test]
   fn standard_wip_lozenge_sizes_stable_legacy1() {
      standard_wip_lozenge_sizes_stable(WscSortType::Legacy1, WscSortType::Classic, 0);
   }
   #[test]
   fn standard_wip_lozenge_sizes_stable_classic() {
      standard_wip_lozenge_sizes_stable(WscSortType::Classic, WscSortType::Classic, 0);
   }
   #[test]
   fn standard_wip_lozenge_sizes_stable_anchor() {
      standard_wip_lozenge_sizes_stable(WscSortType::AnchorClassic, WscSortType::AnchorClassic, 0);
   }

   #[test]
   fn comparison_plain_rand_stdout() {
      let num_samples: u32 = 100;
      run_eval_test(
         SortEvalConfig {
            base_name: "plain_rand",
            num_samples,
            sort_size: 4096,
            type_switch_level: PREFERRED_BLEND_SWITCH,
            permutation_config: PermutationConfig::default(),
            randomizer_offset: 0x0,
            sort_choice: EvalSortChoice::AMS,
            pattern_mod_op: nop_pattern_mod_op,
         },
         // Output is noisy if we use std::io::stdout(), but we want
         // to check that this option at least runs.  It is really a
         // debugging feature.
         EvalDestination::Writer(&mut std::io::sink()),
      );
   }

   enum Variation {
      Base,
      Pure,
      Skipless,
      Blend,
   }

   fn run_triple_mod_sized_test(
      base_name: &str,
      variation: Variation,
      permutation_config: PermutationConfig,
      randomizer_offset: u64,
      pattern_mod_op: PatternModOp,
      sort_size: u32,
   ) {
      let type_switch_level: u32 = match variation {
         Variation::Base => 1000, // Effectively ignored.
         Variation::Pure => 0,
         Variation::Skipless => 1000,
         Variation::Blend => 4,
      };
      let sort_choice: EvalSortChoice = match variation {
         Variation::Base => EvalSortChoice::BASELINE,
         Variation::Pure | Variation::Skipless | Variation::Blend => EvalSortChoice::AMS,
      };
      let adjusted_base_name: String = "comparison_".to_string()
         + base_name
         + match variation {
            Variation::Base => "",
            Variation::Pure => "_pure",
            Variation::Skipless => "_skipless",
            Variation::Blend => "_blend",
         };
      run_eval_test(
         SortEvalConfig {
            base_name: &adjusted_base_name,
            num_samples: NUM_EVAL_SAMPLES,
            sort_size,
            type_switch_level,
            permutation_config,
            randomizer_offset,
            sort_choice,
            pattern_mod_op,
         },
         EvalDestination::GoldenFile(),
      );
   }

   fn run_triple_mod_test(
      base_name: &str,
      variation: Variation,
      permutation_config: PermutationConfig,
      randomizer_offset: u64,
      pattern_mod_op: PatternModOp,
   ) {
      run_triple_mod_sized_test(
         base_name,
         variation,
         permutation_config,
         randomizer_offset,
         pattern_mod_op,
         4096,
      );
   }

   fn run_triple_size_test(
      base_name: &str,
      variation: Variation,
      permutation_config: PermutationConfig,
      randomizer_offset: u64,
      sort_size: u32,
   ) {
      run_triple_mod_sized_test(
         base_name,
         variation,
         permutation_config,
         randomizer_offset,
         nop_pattern_mod_op,
         sort_size,
      );
   }

   fn run_triple_test(
      base_name: &str,
      variation: Variation,
      permutation_config: PermutationConfig,
      randomizer_offset: u64,
   ) {
      run_triple_mod_sized_test(
         base_name,
         variation,
         permutation_config,
         randomizer_offset,
         nop_pattern_mod_op,
         4096,
      );
   }

   // Shared uneven-4 pattern modification.

   fn uneven_4_pattern_mod_op(block_permutation: &mut BlockPermutation) {
      block_permutation.principal_dividers[0] = block_permutation.sort_size as u32 * 3 / 17;
      block_permutation.principal_dividers[1] = block_permutation.sort_size as u32 * (3 + 5) / 17;
      block_permutation.principal_dividers[2] =
         block_permutation.sort_size as u32 * (3 + 5 + 4) / 17;
      block_permutation.principal_dividers[3] = block_permutation.sort_size as u32;
   }

   fn run_triple_uneven_4(
      base_name: &str,
      variation: Variation,
      permutation_config: PermutationConfig,
      randomizer_offset: u64,
   ) {
      run_triple_mod_test(
         base_name,
         variation,
         permutation_config,
         randomizer_offset,
         uneven_4_pattern_mod_op,
      );
   }
   // Plain random.

   fn plain_rand_pattern_gen() -> PermutationConfig {
      PermutationConfig::default()
   }
   #[test]
   #[ignore]
   fn comparison_plain_rand_baseline() {
      run_triple_test("plain_rand", Variation::Base, plain_rand_pattern_gen(), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_plain_rand_pure() {
      run_triple_test("plain_rand", Variation::Pure, plain_rand_pattern_gen(), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_plain_rand_skipless() {
      run_triple_test("plain_rand", Variation::Skipless, plain_rand_pattern_gen(), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_plain_rand_blend() {
      run_triple_test("plain_rand", Variation::Blend, plain_rand_pattern_gen(), 0x0);
   }

   // Presorted.  The non-disrupted are deterministic.

   fn presorted_pattern_gen(disruption_rate: f32) -> PermutationConfig {
      PermutationConfig {
         inter_block_strategy: SortType::FORWARD,
         intra_block_strategy: SortType::FORWARD,
         disruption_type: DisruptionType::DISPLACE,
         disruption_rate,
         ..Default::default()
      }
   }
   #[test]
   #[ignore]
   fn comparison_presort_baseline() {
      run_triple_test("presort", Variation::Base, presorted_pattern_gen(0.0), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_pure() {
      run_triple_test("presort", Variation::Pure, presorted_pattern_gen(0.0), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_skipless() {
      run_triple_test("presort", Variation::Skipless, presorted_pattern_gen(0.0), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_blend() {
      run_triple_test("presort", Variation::Blend, presorted_pattern_gen(0.0), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_1pc_baseline() {
      run_triple_test("presort_1pc", Variation::Base, presorted_pattern_gen(0.01), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_p3pc_baseline() {
      run_triple_test("presort_p3pc", Variation::Base, presorted_pattern_gen(0.003), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_p3pc_pure() {
      run_triple_test("presort_p3pc", Variation::Pure, presorted_pattern_gen(0.003), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_p3pc_skipless() {
      run_triple_test("presort_p3pc", Variation::Skipless, presorted_pattern_gen(0.003), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_p3pc_blend() {
      run_triple_test("presort_p3pc", Variation::Blend, presorted_pattern_gen(0.003), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_1pc_pure() {
      run_triple_test("presort_1pc", Variation::Pure, presorted_pattern_gen(0.01), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_1pc_skipless() {
      run_triple_test("presort_1pc", Variation::Skipless, presorted_pattern_gen(0.01), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_1pc_blend() {
      run_triple_test("presort_1pc", Variation::Blend, presorted_pattern_gen(0.01), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_3pc_baseline() {
      run_triple_test("presort_3pc", Variation::Base, presorted_pattern_gen(0.03), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_3pc_pure() {
      run_triple_test("presort_3pc", Variation::Pure, presorted_pattern_gen(0.03), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_3pc_skipless() {
      run_triple_test("presort_3pc", Variation::Skipless, presorted_pattern_gen(0.03), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_3pc_blend() {
      run_triple_test("presort_3pc", Variation::Blend, presorted_pattern_gen(0.03), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_10pc_baseline() {
      run_triple_test("presort_10pc", Variation::Base, presorted_pattern_gen(0.10), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_10pc_pure() {
      run_triple_test("presort_10pc", Variation::Pure, presorted_pattern_gen(0.10), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_10pc_skipless() {
      run_triple_test("presort_10pc", Variation::Skipless, presorted_pattern_gen(0.10), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_10pc_blend() {
      run_triple_test("presort_10pc", Variation::Blend, presorted_pattern_gen(0.10), 0x0);
   }

   // Varying sizes.

   fn sizes_pattern_gen() -> PermutationConfig {
      PermutationConfig {
         inter_block_strategy: SortType::FORWARD,
         intra_block_strategy: SortType::FORWARD,
         disruption_type: DisruptionType::DISPLACE,
         disruption_rate: 0.002,
         ..Default::default()
      }
   }
   #[test]
   #[ignore]
   fn comparison_presort_s12_baseline() {
      run_triple_test("presort_s12", Variation::Base, sizes_pattern_gen(), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s12_pure() {
      run_triple_test("presort_s12", Variation::Pure, sizes_pattern_gen(), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s12_skipless() {
      run_triple_test("presort_s12", Variation::Skipless, sizes_pattern_gen(), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s12_blend() {
      run_triple_test("presort_s12", Variation::Blend, sizes_pattern_gen(), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s11_baseline() {
      run_triple_size_test("presort_s11", Variation::Base, sizes_pattern_gen(), 0x0, 1 << 11);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s11_pure() {
      run_triple_size_test("presort_s11", Variation::Pure, sizes_pattern_gen(), 0x0, 1 << 11);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s11_skipless() {
      run_triple_size_test("presort_s11", Variation::Skipless, sizes_pattern_gen(), 0x0, 1 << 11);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s11_blend() {
      run_triple_size_test("presort_s11", Variation::Blend, sizes_pattern_gen(), 0x0, 1 << 11);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s10_baseline() {
      run_triple_size_test("presort_s10", Variation::Base, sizes_pattern_gen(), 0x0, 1 << 10);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s10_pure() {
      run_triple_size_test("presort_s10", Variation::Pure, sizes_pattern_gen(), 0x0, 1 << 10);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s10_skipless() {
      run_triple_size_test("presort_s10", Variation::Skipless, sizes_pattern_gen(), 0x0, 1 << 10);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s10_blend() {
      run_triple_size_test("presort_s10", Variation::Blend, sizes_pattern_gen(), 0x0, 1 << 10);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s13_baseline() {
      run_triple_size_test("presort_s13", Variation::Base, sizes_pattern_gen(), 0x0, 1 << 13);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s13_pure() {
      run_triple_size_test("presort_s13", Variation::Pure, sizes_pattern_gen(), 0x0, 1 << 13);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s13_skipless() {
      run_triple_size_test("presort_s13", Variation::Skipless, sizes_pattern_gen(), 0x0, 1 << 13);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s13_blend() {
      run_triple_size_test("presort_s13", Variation::Blend, sizes_pattern_gen(), 0x0, 1 << 13);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s14_baseline() {
      run_triple_size_test("presort_s14", Variation::Base, sizes_pattern_gen(), 0x0, 1 << 14);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s14_pure() {
      run_triple_size_test("presort_s14", Variation::Pure, sizes_pattern_gen(), 0x0, 1 << 14);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s14_skipless() {
      run_triple_size_test("presort_s14", Variation::Skipless, sizes_pattern_gen(), 0x0, 1 << 14);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s14_blend() {
      run_triple_size_test("presort_s14", Variation::Blend, sizes_pattern_gen(), 0x0, 1 << 14);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s15_baseline() {
      run_triple_size_test("presort_s15", Variation::Base, sizes_pattern_gen(), 0x0, 1 << 15);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s15_pure() {
      run_triple_size_test("presort_s15", Variation::Pure, sizes_pattern_gen(), 0x0, 1 << 15);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s15_skipless() {
      run_triple_size_test("presort_s15", Variation::Skipless, sizes_pattern_gen(), 0x0, 1 << 15);
   }
   #[test]
   #[ignore]
   fn comparison_presort_s15_blend() {
      run_triple_size_test("presort_s15", Variation::Blend, sizes_pattern_gen(), 0x0, 1 << 15);
   }

   // Merge-sub.

   fn mergesub_pattern_gen(num_cols: u32) -> PermutationConfig {
      PermutationConfig {
         inter_block_strategy: SortType::RANDOM,
         intra_block_strategy: SortType::FORWARD,
         num_blocks_rows: 1,
         num_blocks_cols: num_cols,
         ..Default::default()
      }
   }

   #[test]
   #[ignore]
   fn comparison_mergesub_a_baseline() {
      run_triple_test("mergesub_a", Variation::Base, mergesub_pattern_gen(2), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_a_pure() {
      run_triple_test("mergesub_a", Variation::Pure, mergesub_pattern_gen(2), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_a_skipless() {
      run_triple_test("mergesub_a", Variation::Skipless, mergesub_pattern_gen(2), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_a_blend() {
      run_triple_test("mergesub_a", Variation::Blend, mergesub_pattern_gen(2), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_b_baseline() {
      run_triple_test("mergesub_b", Variation::Base, mergesub_pattern_gen(4), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_b_pure() {
      run_triple_test("mergesub_b", Variation::Pure, mergesub_pattern_gen(4), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_b_skipless() {
      run_triple_test("mergesub_b", Variation::Skipless, mergesub_pattern_gen(4), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_b_blend() {
      run_triple_test("mergesub_b", Variation::Blend, mergesub_pattern_gen(4), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_c_baseline() {
      run_triple_test("mergesub_c", Variation::Base, mergesub_pattern_gen(15), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_c_pure() {
      run_triple_test("mergesub_c", Variation::Pure, mergesub_pattern_gen(15), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_c_skipless() {
      run_triple_test("mergesub_c", Variation::Skipless, mergesub_pattern_gen(15), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_c_blend() {
      run_triple_test("mergesub_c", Variation::Blend, mergesub_pattern_gen(15), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_d_baseline() {
      run_triple_test("mergesub_d", Variation::Base, mergesub_pattern_gen(16), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_d_pure() {
      run_triple_test("mergesub_d", Variation::Pure, mergesub_pattern_gen(16), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_d_skipless() {
      run_triple_test("mergesub_d", Variation::Skipless, mergesub_pattern_gen(16), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_d_blend() {
      run_triple_test("mergesub_d", Variation::Blend, mergesub_pattern_gen(16), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_e_baseline() {
      run_triple_uneven_4("mergesub_e", Variation::Base, mergesub_pattern_gen(4), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_e_pure() {
      run_triple_uneven_4("mergesub_e", Variation::Pure, mergesub_pattern_gen(4), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_e_skipless() {
      run_triple_uneven_4("mergesub_e", Variation::Skipless, mergesub_pattern_gen(4), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_mergesub_e_blend() {
      run_triple_uneven_4("mergesub_e", Variation::Blend, mergesub_pattern_gen(4), 0x0);
   }

   // Append.

   fn append_pattern_gen(append_size: u16) -> PermutationConfig {
      PermutationConfig {
         inter_block_strategy: SortType::FORWARD,
         intra_block_strategy: SortType::RANDOM,
         num_blocks_rows: 1,
         num_blocks_cols: 2,
         // We hide the size of the appended data in this unused field.
         reversal_switch_rate: append_size as f32,
         ..Default::default()
      }
   }

   fn appendage_pattern_mod_op(block_permutation: &mut BlockPermutation) {
      block_permutation.block_types[[0, 0]] = SortType::FORWARD;
      block_permutation.principal_dividers[0] = block_permutation.sort_size as u32
         - block_permutation.permutation_config.reversal_switch_rate as u32;
      block_permutation.principal_dividers[1] = block_permutation.sort_size as u32;
   }

   fn run_triple_appendage_test(
      base_name: &str,
      variation: Variation,
      permutation_config: PermutationConfig,
      randomizer_offset: u64,
   ) {
      run_triple_mod_test(
         base_name,
         variation,
         permutation_config,
         randomizer_offset,
         appendage_pattern_mod_op,
      );
   }
   #[test]
   #[ignore]
   fn comparison_append_a_baseline() {
      run_triple_appendage_test("append_a", Variation::Base, append_pattern_gen(64), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_a_pure() {
      run_triple_appendage_test("append_a", Variation::Pure, append_pattern_gen(64), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_a_skipless() {
      run_triple_appendage_test("append_a", Variation::Skipless, append_pattern_gen(64), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_a_blend() {
      run_triple_appendage_test("append_a", Variation::Blend, append_pattern_gen(64), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_b_baseline() {
      run_triple_appendage_test("append_b", Variation::Base, append_pattern_gen(91), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_b_pure() {
      run_triple_appendage_test("append_b", Variation::Pure, append_pattern_gen(91), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_b_skipless() {
      run_triple_appendage_test("append_b", Variation::Skipless, append_pattern_gen(91), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_b_blend() {
      run_triple_appendage_test("append_b", Variation::Blend, append_pattern_gen(91), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_c_baseline() {
      run_triple_appendage_test("append_c", Variation::Base, append_pattern_gen(128), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_c_pure() {
      run_triple_appendage_test("append_c", Variation::Pure, append_pattern_gen(128), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_c_skipless() {
      run_triple_appendage_test("append_c", Variation::Skipless, append_pattern_gen(128), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_c_blend() {
      run_triple_appendage_test("append_c", Variation::Blend, append_pattern_gen(128), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_d_baseline() {
      run_triple_appendage_test("append_d", Variation::Base, append_pattern_gen(181), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_d_pure() {
      run_triple_appendage_test("append_d", Variation::Pure, append_pattern_gen(181), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_d_skipless() {
      run_triple_appendage_test("append_d", Variation::Skipless, append_pattern_gen(181), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_d_blend() {
      run_triple_appendage_test("append_d", Variation::Blend, append_pattern_gen(181), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_e_baseline() {
      run_triple_appendage_test("append_e", Variation::Base, append_pattern_gen(71), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_e_pure() {
      run_triple_appendage_test("append_e", Variation::Pure, append_pattern_gen(71), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_e_skipless() {
      run_triple_appendage_test("append_e", Variation::Skipless, append_pattern_gen(71), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_append_e_blend() {
      run_triple_appendage_test("append_e", Variation::Blend, append_pattern_gen(71), 0x0);
   }

   // Block-diagonal.

   fn block_diag_pattern(blocks: u32, disruption_rate: f32) -> PermutationConfig {
      PermutationConfig {
         num_blocks_rows: blocks,
         num_blocks_cols: blocks,
         disruption_type: DisruptionType::DISPLACE,
         disruption_rate,
         ..Default::default()
      }
   }

   //
   // Reduce, especially for block-diagnonal, unused evals.
   //
   // Vary RNG offsets more.
   //

   #[test]
   #[ignore]
   fn comparison_block_diag_4_baseline() {
      run_triple_test("block_diag_4", Variation::Base, block_diag_pattern(4, 0.0), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_4_pure() {
      run_triple_test("block_diag_4", Variation::Pure, block_diag_pattern(4, 0.0), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_4_skipless() {
      run_triple_test("block_diag_4", Variation::Skipless, block_diag_pattern(4, 0.0), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_4_blend() {
      run_triple_test("block_diag_4", Variation::Blend, block_diag_pattern(4, 0.0), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_4_1pc_baseline() {
      run_triple_test("block_diag_4_1pc", Variation::Base, block_diag_pattern(4, 0.01), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_4_1pc_pure() {
      run_triple_test("block_diag_4_1pc", Variation::Pure, block_diag_pattern(4, 0.01), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_4_1pc_skipless() {
      run_triple_test("block_diag_4_1pc", Variation::Skipless, block_diag_pattern(4, 0.01), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_4_1pc_blend() {
      run_triple_test("block_diag_4_1pc", Variation::Blend, block_diag_pattern(4, 0.01), 0x0);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_7_baseline() {
      run_triple_test("block_diag_7", Variation::Base, block_diag_pattern(7, 0.0), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_7_pure() {
      run_triple_test("block_diag_7", Variation::Pure, block_diag_pattern(7, 0.0), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_7_skipless() {
      run_triple_test("block_diag_7", Variation::Skipless, block_diag_pattern(7, 0.0), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_7_blend() {
      run_triple_test("block_diag_7", Variation::Blend, block_diag_pattern(7, 0.0), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_7_1pc_baseline() {
      run_triple_test("block_diag_7_1pc", Variation::Base, block_diag_pattern(7, 0.01), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_7_1pc_pure() {
      run_triple_test("block_diag_7_1pc", Variation::Pure, block_diag_pattern(7, 0.01), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_7_1pc_skipless() {
      run_triple_test("block_diag_7_1pc", Variation::Skipless, block_diag_pattern(7, 0.01), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_7_1pc_blend() {
      run_triple_test("block_diag_7_1pc", Variation::Blend, block_diag_pattern(7, 0.01), 0x70);
   }

   #[test]
   #[ignore]
   fn comparison_block_diag_10_baseline() {
      run_triple_test("block_diag_10", Variation::Base, block_diag_pattern(10, 0.0), 0x100);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_10_pure() {
      run_triple_test("block_diag_10", Variation::Pure, block_diag_pattern(10, 0.0), 0x100);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_10_skipless() {
      run_triple_test("block_diag_10", Variation::Skipless, block_diag_pattern(10, 0.0), 0x100);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_10_blend() {
      run_triple_test("block_diag_10", Variation::Blend, block_diag_pattern(10, 0.0), 0x100);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_10_1pc_baseline() {
      run_triple_test("block_diag_10_1pc", Variation::Base, block_diag_pattern(10, 0.01), 0x100);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_10_1pc_pure() {
      run_triple_test("block_diag_10_1pc", Variation::Pure, block_diag_pattern(10, 0.01), 0x100);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_10_1pc_skipless() {
      run_triple_test(
         "block_diag_10_1pc",
         Variation::Skipless,
         block_diag_pattern(10, 0.01),
         0x100,
      );
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_10_1pc_blend() {
      run_triple_test("block_diag_10_1pc", Variation::Blend, block_diag_pattern(10, 0.01), 0x100);
   }

   #[test]
   #[ignore]
   fn comparison_block_diag_13_baseline() {
      run_triple_test("block_diag_13", Variation::Base, block_diag_pattern(13, 0.0), 0x130);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_13_pure() {
      run_triple_test("block_diag_13", Variation::Pure, block_diag_pattern(13, 0.0), 0x130);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_13_skipless() {
      run_triple_test("block_diag_13", Variation::Skipless, block_diag_pattern(13, 0.0), 0x130);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_13_blend() {
      run_triple_test("block_diag_13", Variation::Blend, block_diag_pattern(13, 0.0), 0x130);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_13_1pc_baseline() {
      run_triple_test("block_diag_13_1pc", Variation::Base, block_diag_pattern(13, 0.01), 0x130);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_13_1pc_pure() {
      run_triple_test("block_diag_13_1pc", Variation::Pure, block_diag_pattern(13, 0.01), 0x130);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_13_1pc_skipless() {
      run_triple_test(
         "block_diag_13_1pc",
         Variation::Skipless,
         block_diag_pattern(13, 0.01),
         0x130,
      );
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_13_1pc_blend() {
      run_triple_test("block_diag_13_1pc", Variation::Blend, block_diag_pattern(13, 0.01), 0x130);
   }

   #[test]
   #[ignore]
   fn comparison_block_diag_16_baseline() {
      run_triple_test("block_diag_16", Variation::Base, block_diag_pattern(16, 0.0), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_16_pure() {
      run_triple_test("block_diag_16", Variation::Pure, block_diag_pattern(16, 0.0), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_16_skipless() {
      run_triple_test("block_diag_16", Variation::Skipless, block_diag_pattern(16, 0.0), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_16_blend() {
      run_triple_test("block_diag_16", Variation::Blend, block_diag_pattern(16, 0.0), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_16_1pc_baseline() {
      run_triple_test("block_diag_16_1pc", Variation::Base, block_diag_pattern(16, 0.01), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_16_1pc_pure() {
      run_triple_test("block_diag_16_1pc", Variation::Pure, block_diag_pattern(16, 0.01), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_16_1pc_skipless() {
      run_triple_test(
         "block_diag_16_1pc",
         Variation::Skipless,
         block_diag_pattern(16, 0.01),
         0x160,
      );
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_16_1pc_blend() {
      run_triple_test("block_diag_16_1pc", Variation::Blend, block_diag_pattern(16, 0.01), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_uneven_baseline() {
      run_triple_uneven_4("block_diag_uneven", Variation::Base, block_diag_pattern(4, 0.0), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_uneven_pure() {
      run_triple_uneven_4("block_diag_uneven", Variation::Pure, block_diag_pattern(4, 0.0), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_uneven_skipless() {
      run_triple_uneven_4(
         "block_diag_uneven",
         Variation::Skipless,
         block_diag_pattern(4, 0.0),
         0x40,
      );
   }
   #[test]
   #[ignore]
   fn comparison_block_diag_uneven_blend() {
      run_triple_uneven_4("block_diag_uneven", Variation::Blend, block_diag_pattern(4, 0.0), 0x40);
   }

   // Rand-block-presort.

   fn rand_presort_pattern(blocks: u32) -> PermutationConfig {
      PermutationConfig {
         inter_block_strategy: SortType::RANDOM,
         intra_block_strategy: SortType::FORWARD,
         num_blocks_rows: blocks,
         num_blocks_cols: blocks,
         ..Default::default()
      }
   }

   #[test]
   #[ignore]
   fn comparison_rand_presort_4_baseline() {
      run_triple_test("rand_presort_4", Variation::Base, rand_presort_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_4_pure() {
      run_triple_test("rand_presort_4", Variation::Pure, rand_presort_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_4_skipless() {
      run_triple_test("rand_presort_4", Variation::Skipless, rand_presort_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_4_blend() {
      run_triple_test("rand_presort_4", Variation::Blend, rand_presort_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_7_baseline() {
      run_triple_test("rand_presort_7", Variation::Base, rand_presort_pattern(7), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_7_pure() {
      run_triple_test("rand_presort_7", Variation::Pure, rand_presort_pattern(7), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_7_skipless() {
      run_triple_test("rand_presort_7", Variation::Skipless, rand_presort_pattern(7), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_7_blend() {
      run_triple_test("rand_presort_7", Variation::Blend, rand_presort_pattern(7), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_11_baseline() {
      run_triple_test("rand_presort_11", Variation::Base, rand_presort_pattern(11), 0x110);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_11_pure() {
      run_triple_test("rand_presort_11", Variation::Pure, rand_presort_pattern(11), 0x110);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_11_skipless() {
      run_triple_test("rand_presort_11", Variation::Skipless, rand_presort_pattern(11), 0x110);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_11_blend() {
      run_triple_test("rand_presort_11", Variation::Blend, rand_presort_pattern(11), 0x110);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_16_baseline() {
      run_triple_test("rand_presort_16", Variation::Base, rand_presort_pattern(16), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_16_pure() {
      run_triple_test("rand_presort_16", Variation::Pure, rand_presort_pattern(16), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_16_skipless() {
      run_triple_test("rand_presort_16", Variation::Skipless, rand_presort_pattern(16), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_16_blend() {
      run_triple_test("rand_presort_16", Variation::Blend, rand_presort_pattern(16), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_uneven_baseline() {
      run_triple_uneven_4("rand_presort_uneven", Variation::Base, rand_presort_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_uneven_pure() {
      run_triple_uneven_4("rand_presort_uneven", Variation::Pure, rand_presort_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_uneven_skipless() {
      run_triple_uneven_4(
         "rand_presort_uneven",
         Variation::Skipless,
         rand_presort_pattern(4),
         0x40,
      );
   }
   #[test]
   #[ignore]
   fn comparison_rand_presort_uneven_blend() {
      run_triple_uneven_4("rand_presort_uneven", Variation::Blend, rand_presort_pattern(4), 0x40);
   }

   // Rand-block-rand.

   fn rand_rand_pattern(blocks: u32) -> PermutationConfig {
      PermutationConfig {
         inter_block_strategy: SortType::RANDOM,
         intra_block_strategy: SortType::RANDOM,
         num_blocks_rows: blocks,
         num_blocks_cols: blocks,
         ..Default::default()
      }
   }

   #[test]
   #[ignore]
   fn comparison_rand_rand_4_baseline() {
      run_triple_test("rand_rand_4", Variation::Base, rand_rand_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_4_pure() {
      run_triple_test("rand_rand_4", Variation::Pure, rand_rand_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_4_skipless() {
      run_triple_test("rand_rand_4", Variation::Skipless, rand_rand_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_4_blend() {
      run_triple_test("rand_rand_4", Variation::Blend, rand_rand_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_7_baseline() {
      run_triple_test("rand_rand_7", Variation::Base, rand_rand_pattern(7), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_7_pure() {
      run_triple_test("rand_rand_7", Variation::Pure, rand_rand_pattern(7), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_7_skipless() {
      run_triple_test("rand_rand_7", Variation::Skipless, rand_rand_pattern(7), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_7_blend() {
      run_triple_test("rand_rand_7", Variation::Blend, rand_rand_pattern(7), 0x70);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_11_baseline() {
      run_triple_test("rand_rand_11", Variation::Base, rand_rand_pattern(11), 0x110);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_11_pure() {
      run_triple_test("rand_rand_11", Variation::Pure, rand_rand_pattern(11), 0x110);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_11_skipless() {
      run_triple_test("rand_rand_11", Variation::Skipless, rand_rand_pattern(11), 0x110);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_11_blend() {
      run_triple_test("rand_rand_11", Variation::Blend, rand_rand_pattern(11), 0x110);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_16_baseline() {
      run_triple_test("rand_rand_16", Variation::Base, rand_rand_pattern(16), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_16_pure() {
      run_triple_test("rand_rand_16", Variation::Pure, rand_rand_pattern(16), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_16_skipless() {
      run_triple_test("rand_rand_16", Variation::Skipless, rand_rand_pattern(16), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_16_blend() {
      run_triple_test("rand_rand_16", Variation::Blend, rand_rand_pattern(16), 0x160);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_uneven_baseline() {
      run_triple_uneven_4("rand_rand_uneven", Variation::Base, rand_rand_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_uneven_pure() {
      run_triple_uneven_4("rand_rand_uneven", Variation::Pure, rand_rand_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_uneven_skipless() {
      run_triple_uneven_4("rand_rand_uneven", Variation::Skipless, rand_rand_pattern(4), 0x40);
   }
   #[test]
   #[ignore]
   fn comparison_rand_rand_uneven_blend() {
      run_triple_uneven_4("rand_rand_uneven", Variation::Blend, rand_rand_pattern(4), 0x40);
   }

   // Bidirection.

   fn reverse_pattern_gen(disruption_rate: f32) -> PermutationConfig {
      PermutationConfig {
         inter_block_strategy: SortType::REVERSE,
         intra_block_strategy: SortType::REVERSE,
         disruption_type: DisruptionType::DISPLACE,
         disruption_rate,
         ..Default::default()
      }
   }
   #[test]
   #[ignore]
   fn comparison_presorted_4096_baseline() {
      run_triple_size_test("forward_4096", Variation::Base, presorted_pattern_gen(0.0), 0x4, 4096);
   }
   #[test]
   #[ignore]
   fn comparison_presorted_4096_pure() {
      run_triple_size_test("forward_4096", Variation::Pure, presorted_pattern_gen(0.0), 0x4, 4096);
   }
   #[test]
   #[ignore]
   fn comparison_presorted_4096_skipless() {
      run_triple_size_test(
         "forward_4096",
         Variation::Skipless,
         presorted_pattern_gen(0.0),
         0x4,
         4096,
      );
   }
   #[test]
   #[ignore]
   fn comparison_presorted_4096_blend() {
      run_triple_size_test("forward_4096", Variation::Blend, presorted_pattern_gen(0.0), 0x4, 4096);
   }
   #[test]
   #[ignore]
   fn comparison_reverse_4096_baseline() {
      run_triple_size_test("reverse_4096", Variation::Base, reverse_pattern_gen(0.0), 0x40, 4096);
   }
   #[test]
   #[ignore]
   fn comparison_reverse_4096_pure() {
      run_triple_size_test("reverse_4096", Variation::Pure, reverse_pattern_gen(0.0), 0x40, 4096);
   }
   #[test]
   #[ignore]
   fn comparison_reverse_4096_skipless() {
      run_triple_size_test(
         "reverse_4096",
         Variation::Skipless,
         reverse_pattern_gen(0.0),
         0x40,
         4096,
      );
   }
   #[test]
   #[ignore]
   fn comparison_reverse_4096_blend() {
      run_triple_size_test("reverse_4096", Variation::Blend, reverse_pattern_gen(0.0), 0x40, 4096);
   }
   #[test]
   #[ignore]
   fn comparison_presorted_4095_baseline() {
      run_triple_size_test("forward_4095", Variation::Base, presorted_pattern_gen(0.0), 0x4, 4095);
   }
   #[test]
   #[ignore]
   fn comparison_presorted_4095_pure() {
      run_triple_size_test("forward_4095", Variation::Pure, presorted_pattern_gen(0.0), 0x4, 4095);
   }
   #[test]
   #[ignore]
   fn comparison_presorted_4095_skipless() {
      run_triple_size_test(
         "forward_4095",
         Variation::Skipless,
         presorted_pattern_gen(0.0),
         0x4,
         4095,
      );
   }
   #[test]
   #[ignore]
   fn comparison_presorted_4095_blend() {
      run_triple_size_test("forward_4095", Variation::Blend, presorted_pattern_gen(0.0), 0x4, 4095);
   }
   #[test]
   #[ignore]
   fn comparison_reverse_4095_baseline() {
      run_triple_size_test("reverse_4095", Variation::Base, reverse_pattern_gen(0.0), 0x40, 4095);
   }
   #[test]
   #[ignore]
   fn comparison_reverse_4095_pure() {
      run_triple_size_test("reverse_4095", Variation::Pure, reverse_pattern_gen(0.0), 0x40, 4095);
   }
   #[test]
   #[ignore]
   fn comparison_reverse_4095_skipless() {
      run_triple_size_test(
         "reverse_4095",
         Variation::Skipless,
         reverse_pattern_gen(0.0),
         0x40,
         4095,
      );
   }
   #[test]
   #[ignore]
   fn comparison_reverse_4095_blend() {
      run_triple_size_test("reverse_4095", Variation::Blend, reverse_pattern_gen(0.0), 0x40, 4095);
   }
   #[test]
   #[ignore]
   fn comparison_presorted_4097_baseline() {
      run_triple_size_test("forward_4097", Variation::Base, presorted_pattern_gen(0.0), 0x4, 4097);
   }
   #[test]
   #[ignore]
   fn comparison_presorted_4097_pure() {
      run_triple_size_test("forward_4097", Variation::Pure, presorted_pattern_gen(0.0), 0x4, 4097);
   }
   #[test]
   #[ignore]
   fn comparison_presorted_4097_skipless() {
      run_triple_size_test(
         "forward_4097",
         Variation::Skipless,
         presorted_pattern_gen(0.0),
         0x4,
         4097,
      );
   }
   #[test]
   #[ignore]
   fn comparison_presorted_4097_blend() {
      run_triple_size_test("forward_4097", Variation::Blend, presorted_pattern_gen(0.0), 0x4, 4097);
   }
   #[test]
   #[ignore]
   fn comparison_reverse_4097_baseline() {
      run_triple_size_test("reverse_4097", Variation::Base, reverse_pattern_gen(0.0), 0x40, 4097);
   }
   #[test]
   #[ignore]
   fn comparison_reverse_4097_pure() {
      run_triple_size_test("reverse_4097", Variation::Pure, reverse_pattern_gen(0.0), 0x40, 4097);
   }
   #[test]
   #[ignore]
   fn comparison_reverse_4097_skipless() {
      run_triple_size_test(
         "reverse_4097",
         Variation::Skipless,
         reverse_pattern_gen(0.0),
         0x40,
         4097,
      );
   }
   #[test]
   #[ignore]
   fn comparison_reverse_4097_blend() {
      run_triple_size_test("reverse_4097", Variation::Blend, reverse_pattern_gen(0.0), 0x40, 4097);
   }

   // Other.

   #[test]
   #[ignore]
   fn comparison_block_switch_16() {
      run_eval_test(
         SortEvalConfig {
            base_name: "block_switch_16",
            num_samples: NUM_EVAL_SAMPLES,
            sort_size: 4096,
            type_switch_level: 4,
            permutation_config: PermutationConfig {
               inter_block_strategy: SortType::RANDOM,
               intra_block_strategy: SortType::SWITCH,
               num_blocks_rows: 16,
               num_blocks_cols: 16,
               ..Default::default()
            },
            randomizer_offset: 0x2,
            sort_choice: EvalSortChoice::AMS,
            pattern_mod_op: nop_pattern_mod_op,
         },
         EvalDestination::GoldenFile(),
      );
   }
}
