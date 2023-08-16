pub struct MergeStep {
   pub lower: usize,
   pub middle: usize,
   pub upper: usize,
   pub shifted_level: u32,
   pub singles_to_add: u32,
}

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
   pub fn new(merge_size: usize) -> MinusPlusShift {
      assert!(merge_size >= 2);
      // The initial state is based on a phoney "previous" state.
      let phoney_shifted_level = usize::MAX.count_ones() - 1;
      assert!((!usize::MAX + 1).is_power_of_two());
      assert!(merge_size < usize::MAX);
      MinusPlusShift {
         size: merge_size,
         shifted: 1 << phoney_shifted_level,
         negation: 1 << phoney_shifted_level,
         shifted_level: phoney_shifted_level as u32,
         high_sort_mark: 0,
      }
   }
}

// Be sure to test a few iterations with max size.
impl Iterator for MinusPlusShift {
   type Item = MergeStep;

   fn next(&mut self) -> Option<Self::Item> {
      // For now do not count, but check directly in iterator, to be sure of correctness.
      if (self.shifted >= ((self.size + 1) / 2)) && (self.shifted != self.negation) {
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
      let stop_sort_size: usize = 20;

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
}
