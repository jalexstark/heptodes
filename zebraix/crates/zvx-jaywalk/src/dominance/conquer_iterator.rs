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

// Originally implemented in
// heptodes/zebraix/misc/wsc-itms/src/ams_demo_suite/src/ams_demo/ams_traversal.rs.

// #[cfg(test)]
// mod tests;

// use serde::{Deserialize, Serialize};
// use serde_default::DefaultFromSerde;
// use std::cmp;
// use std::cmp::Ordering;
// use std::collections::VecDeque;
// use zvx_base::is_default;

// A "merge" step, in the manner of merge sort, combines two contiguous ranges. The lower range
// is `[lower..middle]` and the upper is `[middle..upper]`.  The shifted_level is largely
// informational, and reflects the depth in the (implied) recursion.  Singles to add is either
// 0, 1 or 2.  It indicates which sides of the merges are singles.  If there is only one single
// to add, it is the upper range.
pub struct MergeStep {
   pub lower: usize,
   pub middle: usize,
   pub upper: usize,
   pub shifted_level: u32,
   pub singles_to_add: u32,
}

// Used to "un-mark" entries and to initialize state.
const PHONEY_SHIFTED_LEVEL: u32 = usize::MAX.count_ones() - 1;

/// An iterator which counts from one to five
pub struct MinusPlusShift {
   size: usize,
   shifted: usize,
   negation: usize,
   shifted_level: u32, // Convenience member, maintained so that we can return value.
   high_sort_mark: usize, // The highest index of data sorted so far.
}

// The stack basically needs to hold sub-sort info of the form 8,
// 4, 2, 1, 1.  That is, there are two sorts of the same size for
// only the smallest.  Some implementations may not even push in
// such cases.
//
// Should really template and then have getter for stack size, with max for static.
impl MinusPlusShift {
   #[must_use]
   #[allow(clippy::missing_panics_doc)]
   pub fn new(merge_size: usize) -> Self {
      assert!(merge_size >= 2);
      // The initial state is based on a phoney "previous" state.
      assert!((!usize::MAX + 1).is_power_of_two());
      assert!(merge_size < usize::MAX);
      Self {
         size: merge_size,
         shifted: 1 << PHONEY_SHIFTED_LEVEL,
         negation: 1 << PHONEY_SHIFTED_LEVEL,
         shifted_level: PHONEY_SHIFTED_LEVEL,
         high_sort_mark: 0,
      }
   }
}

// Be sure to test a few iterations with max size.
impl Iterator for MinusPlusShift {
   type Item = MergeStep;

   fn next(&mut self) -> Option<Self::Item> {
      // For now do not count, but check directly in iterator, to be sure of correctness.
      if (self.shifted >= self.size.div_ceil(2)) && (self.shifted != self.negation) {
         return None;
      }

      if (self.negation & self.shifted) != 0 {
         self.negation ^= self.shifted;
         self.negation ^= self.shifted - 1;
         self.shifted = 1;
         self.shifted_level = 0;
      } else {
         self.shifted <<= 1;
         self.shifted_level += 1;
      }
      let mask = !self.shifted + 1; // (-self.shifted), but works in usize
      let mut mid_point: usize = (mask & (!self.negation)) - (mask & self.negation);

      // Advance if this step's mid-point is at or beyond the size.
      //
      // It is never necessary to proceed forward with a minus-to-plus
      // transition, since that can only increase the mid-point.  This
      // is maximally logN work, since there can only be one partial
      // set of work at each shift level.
      while mid_point >= self.size {
         // We actually do not need to clear the negation at the
         // shifted bit, because we never return to this shifted
         // level.
         self.shifted <<= 1;
         self.shifted_level += 1;
         let mask = !self.shifted + 1; // (-self.shifted), but works in usize
         mid_point = (mask & (!self.negation)) - (mask & self.negation);
      }
      assert!(mid_point <= self.size);

      let lower_point = mid_point - self.shifted;
      let upper_point = std::cmp::min(mid_point + self.shifted, self.size);

      // Logic (separate from preceding) for adding new single-element sub-sorts.
      //
      // Unvisited data is added as size=1 sub-sorts.
      let singles_to_add: u32;

      if mid_point > self.high_sort_mark {
         assert_eq!(mid_point, lower_point + 1);
         assert_eq!(upper_point, mid_point + 1);
         singles_to_add = 2;
      } else if upper_point > self.high_sort_mark {
         assert_eq!(upper_point, mid_point + 1);
         singles_to_add = 1;
      } else {
         singles_to_add = 0;
      }
      self.high_sort_mark = upper_point;

      // Return step details.
      Some(MergeStep {
         lower: lower_point,
         middle: mid_point,
         upper: upper_point,
         shifted_level: self.shifted_level,
         singles_to_add,
      })
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn traversal_patterns() {
      let stop_sort_size: usize = 10;

      for sort_size in 2..stop_sort_size {
         let mut check_vector = vec![1; sort_size];
         let mut final_shifted_level = 0;
         let iter = MinusPlusShift::new(sort_size);
         for merge_step in iter {
            assert!(merge_step.upper <= sort_size);
            assert_eq!(check_vector[merge_step.lower], merge_step.middle - merge_step.lower);
            // Rust unable to optimize: for i in merge_step.lower + 1..merge_step.middle {
            for item in check_vector.iter().take(merge_step.middle).skip(merge_step.lower + 1) {
               assert_eq!(*item, 0);
            }
            assert_eq!(check_vector[merge_step.middle], merge_step.upper - merge_step.middle);
            // Rust unable to optimize: for i in merge_step.middle + 1..merge_step.upper {
            for item in check_vector.iter().take(merge_step.upper).skip(merge_step.middle + 1) {
               assert_eq!(*item, 0);
            }

            check_vector[merge_step.lower] = merge_step.upper - merge_step.lower;
            check_vector[merge_step.middle] = 0;
            final_shifted_level = merge_step.shifted_level;
         }
         assert_eq!(check_vector[0], sort_size);
         // Rust unable to optimize: for i in 1..sort_size {
         for item in check_vector.iter().take(sort_size).skip(1) {
            assert_eq!(*item, 0);
         }
         // It may be that shifted_level is not useful to merge
         // sorting, especially given the non-power-of-2 data sizes.
         assert!(1 << final_shifted_level < sort_size);
         assert!(2 << final_shifted_level >= sort_size);
      }
   }

   fn mark_stretch_single(check_vector: &mut [u32], location: usize, level: u32) {
      // dbg!((check_vector[location], location, level));
      assert_eq!(check_vector[location], PHONEY_SHIFTED_LEVEL);
      check_vector[location] = level;
      // dbg!((location, level));
   }

   fn mark_stretch(
      check_vector: &mut [u32],
      lower: usize,
      middle: usize,
      upper: usize,
      level: u32,
   ) {
      // for check_index in lower..middle {
      //    assert_eq!(check_vector[check_index], level - 1);
      //    check_vector[check_index] = level;
      // }
      for item in &mut check_vector[lower..middle] {
         assert_eq!(*item, level - 1);
         *item = level;
      }
      for item in &mut check_vector[middle..upper] {
         assert!(*item < level);
         *item = level;
      }
   }

   #[test]
   fn traversal_scan() {
      let stop_sort_size: usize = 9;

      for sort_size in 2..stop_sort_size {
         let mut check_vector = vec![PHONEY_SHIFTED_LEVEL; sort_size];
         let mut final_shifted_level = 0;
         let mut final_merge_step_lower = 0;
         let mut final_merge_step_upper = 0;

         let iter = MinusPlusShift::new(sort_size);
         for merge_step in iter {
            match merge_step.singles_to_add {
               2 => {
                  assert_eq!(merge_step.middle, merge_step.lower + 1);
                  assert_eq!(merge_step.upper, merge_step.middle + 1);
                  mark_stretch_single(
                     &mut check_vector,
                     merge_step.lower,
                     merge_step.shifted_level,
                  );
                  mark_stretch_single(
                     &mut check_vector,
                     merge_step.middle,
                     merge_step.shifted_level,
                  );
               }
               1 => {
                  assert_eq!(merge_step.upper, merge_step.middle + 1);
                  mark_stretch_single(
                     &mut check_vector,
                     merge_step.middle,
                     merge_step.shifted_level,
                  );
                  mark_stretch(
                     &mut check_vector,
                     merge_step.lower,
                     merge_step.middle, // Effectively skip the upper part.
                     merge_step.middle,
                     merge_step.shifted_level,
                  );
               }
               0 => {
                  mark_stretch(
                     &mut check_vector,
                     merge_step.lower,
                     merge_step.middle,
                     merge_step.upper,
                     merge_step.shifted_level,
                  );
               }
               _ => {
                  panic!("invalid value for singles_to_add (not 0, 1 or 2).");
               }
            }

            final_shifted_level = merge_step.shifted_level;
            final_merge_step_lower = merge_step.lower;
            final_merge_step_upper = merge_step.upper;
         }

         assert_eq!(final_merge_step_lower, 0);
         assert_eq!(final_merge_step_upper, sort_size);
         for item in check_vector {
            assert_eq!(item, final_shifted_level);
         }
      }
   }
}

#[test]
fn check_small_sizes() {
   // Sizes 0, 1 are invalid.

   {
      let mut check_vector = Vec::<(usize, usize, usize, u32, u32)>::default();
      let iter = MinusPlusShift::new(2);
      for merge_step in iter {
         check_vector.push((
            merge_step.lower,
            merge_step.middle,
            merge_step.upper,
            merge_step.shifted_level,
            merge_step.singles_to_add,
         ));
      }
      assert_eq!(check_vector, vec![(0, 1, 2, 0, 2)]);
   }
   {
      let mut check_vector = Vec::<(usize, usize, usize, u32, u32)>::default();
      let iter = MinusPlusShift::new(3);
      for merge_step in iter {
         check_vector.push((
            merge_step.lower,
            merge_step.middle,
            merge_step.upper,
            merge_step.shifted_level,
            merge_step.singles_to_add,
         ));
      }
      assert_eq!(check_vector, vec![(0, 1, 2, 0, 2), (0, 2, 3, 1, 1)]);
   }
   {
      let mut check_vector = Vec::<(usize, usize, usize, u32, u32)>::default();
      let iter = MinusPlusShift::new(6);
      for merge_step in iter {
         check_vector.push((
            merge_step.lower,
            merge_step.middle,
            merge_step.upper,
            merge_step.shifted_level,
            merge_step.singles_to_add,
         ));
      }
      assert_eq!(
         check_vector,
         vec![(0, 1, 2, 0, 2), (2, 3, 4, 0, 2), (0, 2, 4, 1, 0), (4, 5, 6, 0, 2), (0, 4, 6, 2, 0)]
      );
   }
   {
      let mut check_vector = Vec::<(usize, usize, usize, u32, u32)>::default();
      let iter = MinusPlusShift::new(7);
      for merge_step in iter {
         check_vector.push((
            merge_step.lower,
            merge_step.middle,
            merge_step.upper,
            merge_step.shifted_level,
            merge_step.singles_to_add,
         ));
      }
      assert_eq!(
         check_vector,
         vec![
            (0, 1, 2, 0, 2),
            (2, 3, 4, 0, 2),
            (0, 2, 4, 1, 0),
            (4, 5, 6, 0, 2),
            (4, 6, 7, 1, 1),
            (0, 4, 7, 2, 0)
         ]
      );
   }
   {
      let mut check_vector = Vec::<(usize, usize, usize, u32, u32)>::default();
      let iter = MinusPlusShift::new(9);
      for merge_step in iter {
         check_vector.push((
            merge_step.lower,
            merge_step.middle,
            merge_step.upper,
            merge_step.shifted_level,
            merge_step.singles_to_add,
         ));
      }
      assert_eq!(
         check_vector,
         vec![
            (0, 1, 2, 0, 2),
            (2, 3, 4, 0, 2),
            (0, 2, 4, 1, 0),
            (4, 5, 6, 0, 2),
            (6, 7, 8, 0, 2),
            (4, 6, 8, 1, 0),
            (0, 4, 8, 2, 0),
            (0, 8, 9, 3, 1)
         ]
      );
   }
}
