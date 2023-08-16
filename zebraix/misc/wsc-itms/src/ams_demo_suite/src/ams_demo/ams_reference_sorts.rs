#[cfg(test)]
use crate::ams_demo::ams_debug::check_lozenge;
#[cfg(test)]
use crate::ams_demo::ams_debug::check_p_dfs;
#[cfg(test)]
use crate::ams_demo::ams_debug::check_sort;
#[cfg(test)]
use crate::ams_demo::ams_debug::write_auxiliary_edges;
// #[cfg(test)]
// use crate::ams_demo::ams_debug::write_graph_coords;
pub use crate::ams_demo::fill_sortable;
#[cfg(test)]
use crate::ams_demo::nudge_values;
pub use crate::ams_demo::Linkable;
pub use crate::ams_demo::MergeStep;
pub use crate::ams_demo::MinusPlusShift;
pub use crate::ams_demo::SortStats;
pub use crate::ams_demo::SortStatsCounts;
pub use crate::ams_demo::SortStatsInit;
pub use crate::ams_demo::SortableEntity;
#[cfg(test)]
use goldenfile::Mint;
#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::BufWriter;

// Used to ensure ceiling of log works.  This can be eliminated when
// we have standardized next-power-of-2 in Rust.  Note that
// lg2(10^6+1)-lg2(10^6) > CEIL_LOG_FIDDLE.
pub const CEIL_LOG_FIDDLE: f64 = 0.0000001;

#[cfg(test)]
#[inline]
fn sort_with_builtin(sorting_data: &mut [SortableEntity]) -> usize {
   sorting_data.sort_by(|a, b| a.value.cmp(&b.value));

   let size: usize = sorting_data.len();
   // Rust unable to optimize: for i in 0..size - 1_usize {
   for (i, item) in sorting_data.iter_mut().enumerate().take(size - 1_usize) {
      item.set_forward_link(Some(i + 1));
   }
   sorting_data[size - 1].set_forward_link(None);
   0
}

pub struct SimpleSubSort {
   pub head: usize,
   pub tail: usize, // Points to actual end, not one-past.
                    // pub p_dfs_start: usize, // Highest unparented.
                    // pub r_dfs_start: usize, // Highest unparented.
}

pub struct DfsSubSort {
   pub has_reverse: bool, // True iff reverse sort chain is created.
   pub head: usize,
   pub tail: usize,                // Points to actual end, not one-past.
   pub p_dfs_start: Option<usize>, // Highest unparented.
   pub c_dfs_start: Option<usize>, // Lowest unparented, 180-deg-rotated pDFS.
}

pub struct DfsSortConfig {
   pub final_reverse: bool,
   pub upper_lozenge: bool,
   pub dfs_tree: bool,
}

// Sort that switches sub-sorts so that one is guaranteed to finish
// first in main mergesort loop.
//
// This sort is not stable.
#[cfg(test)]
#[inline]
fn sort_standard_switch(
   sorting_data: &mut [SortableEntity],
   sorting_stats: &mut SortStats,
) -> usize {
   // The stack basically needs to hold sub-sort info of the form 8,
   // 4, 2, 1, 1.  That is, there are two sorts of the same size for
   // only the smallest.  Some implementations may not even push in
   // such cases.
   let stack_size = std::mem::size_of::<usize>() * 8 + 1;
   let sort_size = sorting_data.len();

   // sorting_data.sort();

   let mut subsort_stack = Vec::<SimpleSubSort>::new(); // vec![SimpleSubSort { head: 0 }; stack_size];
   subsort_stack.reserve_exact(stack_size);
   let iter = MinusPlusShift::new(sort_size);

   sorting_stats.start_one();
   for merge_step in iter {
      sorting_stats.start_subsort(merge_step.shifted_level);

      if merge_step.singles_to_add > 0 {
         if merge_step.singles_to_add == 2 {
            subsort_stack.push(SimpleSubSort { head: merge_step.lower, tail: merge_step.lower });
            sorting_data[merge_step.lower].set_forward_link(None);
         }
         subsort_stack.push(SimpleSubSort { head: merge_step.middle, tail: merge_step.middle });
         sorting_data[merge_step.middle].set_forward_link(None);
      }
      assert!(subsort_stack.len() <= stack_size);
      // println!("Stack size {}, {}", subsort_stack.len(), merge_step.shifted_level);
      // assert!(subsort_stack.len() <= (merge_step.shifted_level as usize + 2));

      let right_sub = subsort_stack.pop().unwrap();
      let left_sub = subsort_stack.pop().unwrap();
      let right_tail = right_sub.tail;
      let left_tail = left_sub.tail;
      // println!("{}, {} / {}, {}", left_sub.head, left_sub.tail, right_sub.head, right_sub.tail);

      // Consider these assignments fake. Really Rust should allow
      // guaranteed assignment in conditional that follows.
      let mut curr_b: usize;
      let mut curr_a: usize;
      let final_head: usize;
      let final_tail: usize;
      let mut count_a: usize;

      if !sorting_stats.cmp_sortable(
         &sorting_data[right_tail],
         &sorting_data[left_tail],
         SortStatsCounts::NLogN,
         merge_step.shifted_level,
      ) {
         curr_b = right_sub.head;
         curr_a = left_sub.head;
         final_tail = right_sub.tail;
         count_a = merge_step.middle - merge_step.lower;
      } else {
         // Switch so that right subsort finishes combined sort.
         curr_b = left_sub.head;
         curr_a = right_sub.head;
         final_tail = left_sub.tail;
         count_a = merge_step.upper - merge_step.middle;
      }

      // assert!(sorting_data[curr_b] > sorting_data[curr_a]);

      if !sorting_stats.cmp_sortable(
         &sorting_data[curr_b],
         &sorting_data[curr_a],
         SortStatsCounts::NLogN,
         merge_step.shifted_level,
      ) {
         final_head = curr_a;
         curr_a = sorting_data[curr_a].get_forward_link().unwrap_or(usize::MAX);
         count_a -= 1;
         assert!((count_a == 0) || (curr_a != usize::MAX));
      } else {
         final_head = curr_b;
         // Impossible to reach this point if size of R subsort is 1.
         curr_b = sorting_data[curr_b].get_forward_link().unwrap();
      }

      // This is Rust-restricted inefficient code.  This is because we
      // cannot assert to Rust that we know that two indices into a
      // vector are different, which makes mutably borrowing safe.
      let mut curr_head = final_head;
      assert!(final_head != curr_b);
      // Cannot maintain this: let mut curr_head_data: &mut
      // SortableEntity = &mut sorting_data[final_head];
      if count_a > 0 {
         // Cannot maintain this: let mut curr_b_data: &mut
         // SortableEntity = &mut sorting_data[curr_b];

         // Cannot maintain this: let mut curr_a_data: &mut
         // SortableEntity = &mut sorting_data[curr_a];
         for i in 0..count_a {
            while !sorting_stats.cmp_sortable(
               &sorting_data[curr_a],
               &sorting_data[curr_b],
               SortStatsCounts::NLogN,
               merge_step.shifted_level,
            ) {
               sorting_data[curr_head].set_forward_link(Some(curr_b));
               curr_head = curr_b;
               curr_b = sorting_data[curr_b].get_forward_link().unwrap();
            }
            sorting_data[curr_head].set_forward_link(Some(curr_a));
            curr_head = curr_a;
            curr_a = sorting_data[curr_a].get_forward_link().unwrap_or(usize::MAX);
            assert!((i == count_a - 1) || (curr_a != usize::MAX));
         }
      }
      // Append remaining R.
      sorting_data[curr_head].set_forward_link(Some(curr_b));
      // curr_head = curr_b;
      // curr_b = sorting_data[curr_b].get_forward_link().unwrap();

      // let mut curr = final_head;
      // while true {
      //    print!("{:2}, ", sorting_data[curr].value);
      //    let next = sorting_data[curr].get_forward_link();
      //    if next.is_none() {
      //       break;
      //    }
      //    curr = next.unwrap();
      // }
      // println!("");

      subsort_stack.push(SimpleSubSort { head: final_head, tail: final_tail });
      sorting_stats.finish_subsort(merge_step.shifted_level);
   }

   sorting_stats.finish_one();

   // Final subsort is the completed sort.
   subsort_stack[0].head
}

// Count-based classic mergesort.
#[cfg(test)]
#[inline]
fn sort_standard_count(
   sorting_data: &mut [SortableEntity],
   sorting_stats: &mut SortStats,
) -> usize {
   // The stack basically needs to hold sub-sort info of the form 8,
   // 4, 2, 1, 1.  That is, there are two sorts of the same size for
   // only the smallest.  Some implementations may not even push in
   // such cases.
   let stack_size = std::mem::size_of::<usize>() * 8 + 1;
   let sort_size = sorting_data.len();

   let mut subsort_stack = Vec::<SimpleSubSort>::new(); // vec![SimpleSubSort { head: 0 }; stack_size];
   subsort_stack.reserve_exact(stack_size);
   let iter = MinusPlusShift::new(sort_size);

   sorting_stats.start_one();
   for merge_step in iter {
      sorting_stats.start_subsort(merge_step.shifted_level);

      if merge_step.singles_to_add > 0 {
         if merge_step.singles_to_add == 2 {
            subsort_stack.push(SimpleSubSort { head: merge_step.lower, tail: merge_step.lower });
            sorting_data[merge_step.lower].set_forward_link(None);
         }
         subsort_stack.push(SimpleSubSort { head: merge_step.middle, tail: merge_step.middle });
         sorting_data[merge_step.middle].set_forward_link(None);
      }
      assert!(subsort_stack.len() <= stack_size);

      let right_sub = subsort_stack.pop().unwrap();
      let left_sub = subsort_stack.pop().unwrap();
      let right_tail = right_sub.tail;
      let left_tail = left_sub.tail;

      // Consider these assignments fake. Really Rust should allow
      // guaranteed assignment in conditional that follows.

      let final_head: usize;
      // Clippied: let final_tail: usize;

      let mut curr_b: usize = right_sub.head;
      let mut curr_a: usize = left_sub.head;
      let mut count_a: usize = merge_step.middle - merge_step.lower;
      let mut count_b: usize = merge_step.upper - merge_step.middle;

      let mut consume_left: bool = sorting_stats.cmp_sortable(
         &sorting_data[curr_a],
         &sorting_data[curr_b],
         SortStatsCounts::NLogN,
         merge_step.shifted_level,
      );

      if consume_left {
         final_head = curr_a;
         curr_a = sorting_data[curr_a].get_forward_link().unwrap_or(usize::MAX);
         count_a -= 1;
         assert!((count_a == 0) || (curr_a != usize::MAX));
         assert!(count_b > 0);
      } else {
         final_head = curr_b;
         curr_b = sorting_data[curr_b].get_forward_link().unwrap_or(usize::MAX);
         count_b -= 1;
         assert!((count_b == 0) || (curr_b != usize::MAX));
         assert!(count_a > 0);
      }

      // Invariant:
      assert!(
         (consume_left && ((count_a == 0) || (curr_a != usize::MAX)) && (count_b > 0))
            || (!consume_left && ((count_b == 0) || (curr_b != usize::MAX)) && (count_a > 0))
      );

      let mut curr_head = final_head;
      // sorting_stats.increment_nlogn(merge_step.shifted_level); // Once for false.
      while (count_a > 0) && (count_b > 0) {
         consume_left = sorting_stats.cmp_sortable(
            &sorting_data[curr_a],
            &sorting_data[curr_b],
            SortStatsCounts::NLogN,
            merge_step.shifted_level,
         );
         if consume_left {
            sorting_data[curr_head].set_forward_link(Some(curr_a));
            curr_head = curr_a;
            curr_a = sorting_data[curr_a].get_forward_link().unwrap_or(usize::MAX);
            count_a -= 1;
         } else {
            sorting_data[curr_head].set_forward_link(Some(curr_b));
            curr_head = curr_b;
            curr_b = sorting_data[curr_b].get_forward_link().unwrap_or(usize::MAX);
            count_b -= 1;
         }

         // Invariant:
         assert!(
            (consume_left && ((count_a == 0) || (curr_a != usize::MAX)) && (count_b > 0))
               || (!consume_left && ((count_b == 0) || (curr_b != usize::MAX)) && (count_a > 0))
         );
      }

      // More specific invariant.
      assert!(
         (consume_left && (count_a == 0) && (curr_a == usize::MAX) && (count_b > 0))
            || (!consume_left && (count_b == 0) && (curr_b == usize::MAX) && (count_a > 0))
      );

      // Need to consume opposite of what was last consumed.
      let final_tail: usize = if consume_left {
         sorting_data[curr_head].set_forward_link(Some(curr_b));
         right_tail
      } else {
         sorting_data[curr_head].set_forward_link(Some(curr_a));
         left_tail
      };

      subsort_stack.push(SimpleSubSort { head: final_head, tail: final_tail });
      sorting_stats.finish_subsort(merge_step.shifted_level);
   }

   sorting_stats.finish_one();

   // Final subsort is the completed sort.
   subsort_stack[0].head
}

// Like classic mergesort, but slightly emphasizing code size and
// deduplication over number of comparisions.  This can still use
// counting, and can use range testing as an alternative termination
// condition.  However, we generalize to testing specific values at
// end of chains.
//
// Also guarantees that most chain traversals are non-null.
// #[cfg(test)]
#[inline]
pub fn sort_standard_interlink(
   sorting_data: &mut [SortableEntity],
   sorting_stats: &mut SortStats,
) -> usize {
   // The stack basically needs to hold sub-sort info of the form 8,
   // 4, 2, 1, 1.  That is, there are two sorts of the same size for
   // only the smallest.  Some implementations may not even push in
   // such cases.
   let stack_size = std::mem::size_of::<usize>() * 8 + 1;
   let sort_size = sorting_data.len();

   let mut subsort_stack = Vec::<SimpleSubSort>::new(); // vec![SimpleSubSort { head: 0 }; stack_size];
   subsort_stack.reserve_exact(stack_size);
   let iter = MinusPlusShift::new(sort_size);

   sorting_stats.start_one();
   for merge_step in iter {
      sorting_stats.start_subsort(merge_step.shifted_level);

      if merge_step.singles_to_add > 0 {
         if merge_step.singles_to_add == 2 {
            subsort_stack.push(SimpleSubSort { head: merge_step.lower, tail: merge_step.lower });
            sorting_data[merge_step.lower].set_forward_link(None);
         }
         subsort_stack.push(SimpleSubSort { head: merge_step.middle, tail: merge_step.middle });
         sorting_data[merge_step.middle].set_forward_link(None);
      }
      assert!(subsort_stack.len() <= stack_size);

      let right_sub = subsort_stack.pop().unwrap();
      let left_sub = subsort_stack.pop().unwrap();
      let right_tail = right_sub.tail;
      let left_tail = left_sub.tail;

      // Consider these assignments fake. Really Rust should allow
      // guaranteed assignment in conditional that follows.

      let final_head: usize;

      let mut curr_head: usize;

      let right_head = right_sub.head;
      let left_head = left_sub.head;
      let mut curr_b: usize = right_head;
      let mut curr_a: usize = left_head;
      let mut count_a: usize = merge_step.middle - merge_step.lower;
      let mut count_b: usize = merge_step.upper - merge_step.middle;

      sorting_data[left_tail].set_forward_link(Some(right_head));
      sorting_data[right_tail].set_forward_link(Some(left_head));

      let mut consume_left: bool = sorting_stats.cmp_sortable(
         &sorting_data[curr_a],
         &sorting_data[curr_b],
         SortStatsCounts::NLogN,
         merge_step.shifted_level,
      );
      if consume_left {
         final_head = curr_a;
         curr_head = right_tail;
      } else {
         final_head = curr_b;
         curr_head = left_tail;
      }

      // Invariant made into more specific assurance:
      assert!(
         (consume_left && (curr_a != right_head) && (count_b > 0))
            || (!consume_left && (curr_b != left_head) && (count_a > 0))
      );

      // Alternative conditions:
      //   (count_a > 0) && (count_b > 0)
      //   (curr_a < merge_step.middle) && (curr_b >= merge_step.middle)
      //   (curr_a != right_head) && (curr_b != left_head)
      loop {
         if consume_left {
            sorting_data[curr_head].set_forward_link(Some(curr_a));
            curr_head = curr_a;
            curr_a = sorting_data[curr_a].get_forward_link().unwrap();
            count_a -= 1;
         } else {
            sorting_data[curr_head].set_forward_link(Some(curr_b));
            curr_head = curr_b;
            curr_b = sorting_data[curr_b].get_forward_link().unwrap();
            count_b -= 1;
         }

         // Invariant:
         assert!(
            (consume_left && ((count_a == 0) || (curr_a != right_head)) && (count_b > 0))
               || (!consume_left && ((count_b == 0) || (curr_b != left_head)) && (count_a > 0))
         );
         if (curr_a == right_head) || (curr_b == left_head) {
            break;
         }

         consume_left = sorting_stats.cmp_sortable(
            &sorting_data[curr_a],
            &sorting_data[curr_b],
            SortStatsCounts::NLogN,
            merge_step.shifted_level,
         );
      }

      // Invariant made into more specific assurance.
      assert!(
         (consume_left && (count_a == 0) && (curr_a == right_head) && (count_b > 0))
            || (!consume_left && (count_b == 0) && (curr_b == left_head) && (count_a > 0))
      );

      // Need to consume opposite of what was last consumed.
      let final_tail: usize = if consume_left {
         sorting_data[curr_head].set_forward_link(Some(curr_b));
         right_tail
      } else {
         sorting_data[curr_head].set_forward_link(Some(curr_a));
         left_tail
      };

      // Restore end of linked list of sorted nodes.
      sorting_data[final_tail].set_forward_link(None);

      subsort_stack.push(SimpleSubSort { head: final_head, tail: final_tail });
      sorting_stats.finish_subsort(merge_step.shifted_level);
   }

   sorting_stats.finish_one();

   // Final subsort is the completed sort.
   subsort_stack[0].head
}

#[cfg(test)]
#[inline]
fn sort_standard_dfs(
   sorting_data: &mut [SortableEntity],
   sorting_stats: &mut SortStats,
   config: DfsSortConfig,
) -> usize {
   let stack_size = std::mem::size_of::<usize>() * 8 + 1;
   let sort_size = sorting_data.len();

   let mut subsort_stack = Vec::<DfsSubSort>::new(); // vec![DfsSubSort { head: 0 }; stack_size];
   subsort_stack.reserve_exact(stack_size);
   let iter = MinusPlusShift::new(sort_size);

   sorting_stats.start_one();
   for merge_step in iter {
      sorting_stats.start_subsort(merge_step.shifted_level);

      if merge_step.singles_to_add > 0 {
         if merge_step.singles_to_add == 2 {
            subsort_stack.push(DfsSubSort {
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
         subsort_stack.push(DfsSubSort {
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
      let right_tail = right_sub.tail;
      let left_tail = left_sub.tail;
      let right_p_dfs_start = right_sub.p_dfs_start;
      // let left_p_dfs_start = left_sub.p_dfs_start;
      // let right_c_dfs_start = right_sub.c_dfs_start;
      let left_c_dfs_start = left_sub.c_dfs_start;

      // Consider these assignments fake. Really Rust should allow
      // guaranteed assignment in conditional that follows.

      let final_head: usize;

      let mut curr_head: usize;

      let right_head = right_sub.head;
      let left_head = left_sub.head;

      // Rust cannot cope with speed and cannot allow for reasonable declaration of disjointedness.
      //
      // let left_min = &sorting_data[left_head];
      // let right_min = &sorting_data[right_head];
      // let left_max = &sorting_data[left_tail];
      // let right_max = &sorting_data[right_tail];

      // ========================================
      // Surgery on fronts.

      let mut trimming_ne;
      let mut trimming_nw;
      if config.upper_lozenge {
         trimming_ne = Some(merge_step.middle - 1);
         while (trimming_ne != None)
            && (sorting_stats.cmp_sortable(
               &sorting_data[trimming_ne.unwrap()],
               &sorting_data[right_tail],
               SortStatsCounts::NWork,
               merge_step.shifted_level,
            ))
         {
            trimming_ne = sorting_data[trimming_ne.unwrap()].get_tertiary_link();
         }

         trimming_nw = Some(merge_step.middle);
         while (trimming_nw != None)
            && (!sorting_stats.cmp_sortable(
               &sorting_data[left_tail],
               &sorting_data[trimming_nw.unwrap()],
               SortStatsCounts::NWork,
               merge_step.shifted_level,
            ))
         {
            trimming_nw = sorting_data[trimming_nw.unwrap()].get_secondary_link();
         }
      } else {
         // Unnecessary code complication.
         trimming_ne = None;
         trimming_nw = None;
      }

      //

      let mut lozenge_se;
      let mut trimming_se;
      if !sorting_stats.cmp_sortable(
         &sorting_data[merge_step.middle - 1],
         &sorting_data[right_head],
         SortStatsCounts::NWork,
         merge_step.shifted_level,
      ) {
         trimming_se = left_c_dfs_start;
         lozenge_se = Some(merge_step.middle - 1);
         while (trimming_se != None)
            && (!sorting_stats.cmp_sortable(
               &sorting_data[trimming_se.unwrap()],
               &sorting_data[right_head],
               SortStatsCounts::NWork,
               merge_step.shifted_level,
            ))
         {
            let new_trimming_se = sorting_data[trimming_se.unwrap()].get_tertiary_link();
            sorting_data[trimming_se.unwrap()].set_tertiary_link(lozenge_se);

            lozenge_se = trimming_se;
            trimming_se = new_trimming_se;
         }
      } else {
         trimming_se = Some(merge_step.middle - 1);
         lozenge_se = None;
      }

      //

      let mut lozenge_sw;
      let mut trimming_sw;
      if sorting_stats.cmp_sortable(
         &sorting_data[left_head],
         &sorting_data[merge_step.middle],
         SortStatsCounts::NWork,
         merge_step.shifted_level,
      ) {
         trimming_sw = right_p_dfs_start;
         lozenge_sw = Some(merge_step.middle);
         while (trimming_sw != None)
            && (sorting_stats.cmp_sortable(
               &sorting_data[left_head],
               &sorting_data[trimming_sw.unwrap()],
               SortStatsCounts::NWork,
               merge_step.shifted_level,
            ))
         {
            let new_trimming_sw = sorting_data[trimming_sw.unwrap()].get_secondary_link();
            sorting_data[trimming_sw.unwrap()].set_secondary_link(lozenge_sw);

            lozenge_sw = trimming_sw;
            trimming_sw = new_trimming_sw;
         }
      } else {
         trimming_sw = Some(merge_step.middle);
         lozenge_sw = None;
      }

      // Done with surgery on fronts.
      // ========================================

      // ========================================
      // Main merge loop

      let mut curr_b: usize = right_head;
      let mut curr_a: usize = left_head;

      sorting_data[left_tail].set_forward_link(Some(right_head));
      sorting_data[right_tail].set_forward_link(Some(left_head));

      let mut consume_left: bool = sorting_stats.cmp_sortable(
         &sorting_data[curr_a],
         &sorting_data[curr_b],
         SortStatsCounts::NLogN,
         merge_step.shifted_level,
      );
      if consume_left {
         final_head = curr_a;
         curr_head = right_tail;
      } else {
         final_head = curr_b;
         curr_head = left_tail;
      }

      // Invariant made into more specific assurance:
      assert!((curr_a != right_head) && (curr_b != left_head));

      while (curr_a != right_head) && (curr_b != left_head) {
         consume_left = sorting_stats.cmp_sortable(
            &sorting_data[curr_a],
            &sorting_data[curr_b],
            SortStatsCounts::NLogN,
            merge_step.shifted_level,
         );
         if consume_left {
            sorting_data[curr_head].set_forward_link(Some(curr_a));
            sorting_data[curr_a].set_backward_link(Some(curr_head));
            curr_head = curr_a;
            curr_a = sorting_data[curr_a].get_forward_link().unwrap();
         } else {
            sorting_data[curr_head].set_forward_link(Some(curr_b));
            sorting_data[curr_b].set_backward_link(Some(curr_head));
            curr_head = curr_b;
            curr_b = sorting_data[curr_b].get_forward_link().unwrap();
         }

         // Invariant:
         assert!(
            (consume_left && (curr_b != left_head)) || (!consume_left && (curr_a != right_head))
         );
      }

      // Invariant made into more specific assurance.
      assert!(
         (consume_left && (curr_b != left_head) && (curr_a == right_head))
            || (!consume_left && (curr_a != right_head) && (curr_b == left_head))
      );

      // Need to consume opposite of what was last consumed.
      let final_tail: usize = if consume_left {
         sorting_data[curr_head].set_forward_link(Some(curr_b));
         sorting_data[curr_b].set_backward_link(Some(curr_head));
         right_tail
      } else {
         sorting_data[curr_head].set_forward_link(Some(curr_a));
         sorting_data[curr_a].set_backward_link(Some(curr_head));
         left_tail
      };

      // Restore ends of linked list of sorted nodes.
      sorting_data[final_tail].set_forward_link(None);
      sorting_data[final_head].set_backward_link(None);

      // End of main merge loop.
      // ========================================

      // ========================================
      // Surgery on fronts.

      if sorting_stats.cmp_sortable(
         &sorting_data[merge_step.middle - 1],
         &sorting_data[right_head],
         SortStatsCounts::NWork,
         merge_step.shifted_level,
      ) {
         // When appending, the appended part needs to be complete chain.
         sorting_data[merge_step.middle - 1].set_tertiary_link(left_c_dfs_start);
      }

      if !sorting_stats.cmp_sortable(
         &sorting_data[left_head],
         &sorting_data[merge_step.middle],
         SortStatsCounts::NWork,
         merge_step.shifted_level,
      ) {
         // When appending, the appended part needs to be complete chain.
         sorting_data[merge_step.middle].set_secondary_link(right_p_dfs_start);
      }

      // Append L NE to R NE, and R NW to L NW.
      if config.upper_lozenge {
         sorting_data[right_tail].set_tertiary_link(trimming_ne);
         sorting_data[left_tail].set_secondary_link(trimming_nw);
      }

      // Append L SE to R SE.

      // Save R block head, and link to current. Probably just compare with NULL.
      let orig_right_px_dfs_start = sorting_data[merge_step.upper - 1].get_tertiary_link();
      sorting_data[merge_step.upper - 1].set_tertiary_link(right_sub.c_dfs_start);
      // Append, which may be to head.
      sorting_data[right_head].set_tertiary_link(trimming_se);
      // Extract current head and restore original.
      let right_c_dfs_start = sorting_data[merge_step.upper - 1].get_tertiary_link();
      sorting_data[merge_step.upper - 1].set_tertiary_link(orig_right_px_dfs_start);

      // Append R SW to L SW.

      // Save R block head, and link to current. Probably just compare with NULL.
      let orig_left_cx_dfs_start = sorting_data[merge_step.lower].get_secondary_link();
      sorting_data[merge_step.lower].set_secondary_link(left_sub.p_dfs_start);
      // Append, which may be to head.
      sorting_data[left_head].set_secondary_link(trimming_sw);
      // Extract current head and restore original.
      let left_p_dfs_start = sorting_data[merge_step.lower].get_secondary_link();
      sorting_data[merge_step.lower].set_secondary_link(orig_left_cx_dfs_start);

      //

      // Build DFS tree.
      //
      if config.dfs_tree {
         // There must be at least one vertex in each (initial / final)
         // lozenge side, which is divided between set of vertices to be
         // trimmed and to be to be handled, so they cannot both be
         // empty.
         assert!((trimming_se != None) || (lozenge_se != None));
         // Only need to add to DFS if right block overlaps.
         if lozenge_sw != None {
            if trimming_se == None {
               // The whole of the left chain overlaps. Therefore all
               // overlap in the right block will link to a vertex in
               // the left chain, and we can disregard the trimmed
               // chain.
               //
               // assert!(merge_step.middle != 16);
               trimming_se = lozenge_se;
               lozenge_se = sorting_data[lozenge_se.unwrap()].get_tertiary_link();
            }

            // If none of the left block overlaps, lozenge_se == None
            // and the next step is skipped and we just link all the
            // right block to the top of the trimmed left.

            while (lozenge_se != None) && (lozenge_sw != None) {
               while (lozenge_sw != None)
                  && (!sorting_stats.cmp_sortable(
                     &sorting_data[lozenge_se.unwrap()],
                     &sorting_data[lozenge_sw.unwrap()],
                     SortStatsCounts::NWork,
                     merge_step.shifted_level,
                  ))
               {
                  let next_lozenge_sw = sorting_data[lozenge_sw.unwrap()].get_secondary_link();
                  sorting_data[lozenge_sw.unwrap()].set_secondary_link(trimming_se);
                  lozenge_sw = next_lozenge_sw;
               }
               trimming_se = lozenge_se;
               lozenge_se = sorting_data[lozenge_se.unwrap()].get_tertiary_link();
            }
            while lozenge_sw != None {
               let next_lozenge_sw = sorting_data[lozenge_sw.unwrap()].get_secondary_link();
               sorting_data[lozenge_sw.unwrap()].set_secondary_link(trimming_se);
               lozenge_sw = next_lozenge_sw;
            }
         }
      }

      // trimming_se is just below lowest in right, or None.
      // trimming_sw is just below lowest in left, or None.

      // Done with surgery on fronts.
      // ========================================

      // println!("{} + < {}-{} >", subsort_stack.len(), merge_step.lower, merge_step.upper);
      subsort_stack.push(DfsSubSort {
         has_reverse: true,
         head: final_head,
         tail: final_tail,
         // p_dfs_start: None,
         // c_dfs_start: None,
         p_dfs_start: left_p_dfs_start,
         c_dfs_start: right_c_dfs_start,
      });
      sorting_stats.finish_subsort(merge_step.shifted_level);
   }

   sorting_stats.finish_one();

   let final_subsort = subsort_stack.pop().unwrap();

   // ========================================
   // Surgery on fronts: reverse final lowers.
   if config.final_reverse {
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

   // Done with surgery on fronts.
   // ========================================

   // Final subsort is the completed sort.
   final_subsort.head
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn classic_all_perms_unique() {
      let sort_size: u32 = 6;
      assert!(sort_size < 13); // 12! < 2^31.  Nonetheless try to support more.

      let mut perm_values: Vec<u32> = (0..sort_size).collect();

      let mut sorting_data: Vec<SortableEntity> =
         vec![SortableEntity::default(); sort_size as usize];

      let mut permutation_count: u64 = 0;

      let heap_permutator = permutohedron::Heap::new(&mut perm_values);
      for perm_values in heap_permutator {
         permutation_count += 1;
         fill_sortable(&mut sorting_data, &perm_values);

         let sorted_head: usize = sort_with_builtin(&mut sorting_data);

         check_sort(&sorting_data, sorted_head, false);
      }

      assert_eq!(permutation_count, factorial::Factorial::factorial(&(sort_size as u64)));
   }

   #[test]
   fn classic_all_perms_stable() {
      let sort_size: u32 = 6;
      let dag_cut: u32 = sort_size / 2;
      assert!(sort_size >= 3);
      assert!(sort_size < 13); // 12! < 2^31.  Nonetheless try to support more.

      let mut perm_values: Vec<u32> = (0..dag_cut).collect::<Vec<u32>>();
      perm_values.append(&mut vec![dag_cut, dag_cut]);
      perm_values.append(&mut (dag_cut..sort_size - 2).collect::<Vec<u32>>());

      let mut sorting_data: Vec<SortableEntity> =
         vec![SortableEntity::default(); sort_size as usize];

      let mut permutation_count: u64 = 0;

      let heap_permutator = permutohedron::Heap::new(&mut perm_values);
      for permutation in heap_permutator {
         permutation_count += 1;
         fill_sortable(&mut sorting_data, &permutation);

         let sorted_head: usize = sort_with_builtin(&mut sorting_data);

         check_sort(&sorting_data, sorted_head, false);
      }

      assert_eq!(permutation_count, factorial::Factorial::factorial(&(sort_size as u64)));
   }

   #[test]
   fn classic_rand_perms_unique() {
      let sort_size: u32 = 27;
      let num_samples: u64 = 1000;
      let seed_state = 0xeffcad0d01a5e5e5;
      let seed_stream = 0xd02abbf7b07bca3;

      let mut perm_values: Vec<u32> = (0..sort_size).collect();

      let mut sorting_data: Vec<SortableEntity> =
         vec![SortableEntity::default(); sort_size as usize];

      let mut permutation_count: u64 = 0;

      let mut rng = rand_pcg::Pcg32::new(seed_state, seed_stream);
      let mut fy = shuffle::fy::FisherYates::default();

      for _i in 0..num_samples {
         use shuffle::shuffler::Shuffler; // For fy.shuffle.
         assert!(fy.shuffle(&mut perm_values, &mut rng).is_ok());

         permutation_count += 1;
         fill_sortable(&mut sorting_data, &perm_values);

         let sorted_head: usize = sort_with_builtin(&mut sorting_data);

         check_sort(&sorting_data, sorted_head, false);
      }

      assert_eq!(permutation_count, num_samples);
   }

   #[test]
   fn standard_switch_rand_perms_unique() {
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
         let sorted_head: usize = sort_standard_switch(&mut sorting_data, &mut sorting_stats);

         check_sort(&sorting_data, sorted_head, false);
      }
      sorting_stats.finish_gather();

      assert_eq!(permutation_count, num_samples);

      let mut mint = Mint::new("tests/golden-outputs");
      let out_file = mint.new_goldenfile("standard_switch.m").unwrap();
      let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
      sorting_stats.write_golden_info(&mut out_writer);
   }

   #[test]
   fn standard_count_rand_perms_unique() {
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
         let sorted_head: usize = sort_standard_count(&mut sorting_data, &mut sorting_stats);

         check_sort(&sorting_data, sorted_head, false);
      }
      sorting_stats.finish_gather();

      assert_eq!(permutation_count, num_samples);

      let mut mint = Mint::new("tests/golden-outputs");
      let out_file = mint.new_goldenfile("standard_count.m").unwrap();
      let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
      sorting_stats.write_golden_info(&mut out_writer);
   }

   #[test]
   fn standard_interlink_rand_perms_unique() {
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
         let sorted_head: usize = sort_standard_interlink(&mut sorting_data, &mut sorting_stats);

         check_sort(&sorting_data, sorted_head, false);
      }
      sorting_stats.finish_gather();

      assert_eq!(permutation_count, num_samples);

      let mut mint = Mint::new("tests/golden-outputs");
      let out_file = mint.new_goldenfile("standard_interlink.m").unwrap();
      let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
      sorting_stats.write_golden_info(&mut out_writer);
   }

   #[test]
   fn standard_count_rand_perms_stable() {
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

         // for i in 0..sort_size as usize {
         //    println!("\t{}", sorting_data[i].value);
         // }
         let sorted_head: usize = sort_standard_count(&mut sorting_data, &mut sorting_stats);

         check_sort(&sorting_data, sorted_head, false);
      }
      sorting_stats.finish_gather();

      assert_eq!(permutation_count, num_samples);

      // let mut mint = Mint::new("tests/golden-outputs");
      // let out_file = mint.new_goldenfile("standard_interlink.m").unwrap();
      // let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
      // sorting_stats.write_golden_info(&mut out_writer);
   }

   #[test]
   fn standard_interlink_rand_perms_stable() {
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

         // for i in 0..sort_size as usize {
         //    println!("\t{}", sorting_data[i].value);
         // }
         let sorted_head: usize = sort_standard_interlink(&mut sorting_data, &mut sorting_stats);

         check_sort(&sorting_data, sorted_head, false);
      }
      sorting_stats.finish_gather();

      assert_eq!(permutation_count, num_samples);

      // let mut mint = Mint::new("tests/golden-outputs");
      // let out_file = mint.new_goldenfile("standard_interlink.m").unwrap();
      // let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
      // sorting_stats.write_golden_info(&mut out_writer);
   }

   #[test]
   fn standard_dfs_rand_perms_unique() {
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
         let sorted_head: usize = sort_standard_dfs(
            &mut sorting_data,
            &mut sorting_stats,
            DfsSortConfig { final_reverse: false, upper_lozenge: true, dfs_tree: false },
         );

         check_sort(&sorting_data, sorted_head, true);
      }
      sorting_stats.finish_gather();

      assert_eq!(permutation_count, num_samples);

      let mut mint = Mint::new("tests/golden-outputs");
      let out_file = mint.new_goldenfile("standard_dfs.m").unwrap();
      let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
      sorting_stats.write_golden_info(&mut out_writer);
   }

   #[test]
   fn standard_dfs_rand_perms_multi_size() {
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
         let sorted_head: usize = sort_standard_dfs(
            &mut sorting_data,
            &mut sorting_stats,
            DfsSortConfig { final_reverse: false, upper_lozenge: true, dfs_tree: false },
         );

         check_sort(&sorting_data, sorted_head, true);
      }
      // sorting_stats.finish_gather();

      // let mut mint = Mint::new("tests/golden-outputs");
      // let out_file = mint.new_goldenfile("standard_dfs.m").unwrap();
      // let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
      // sorting_stats.write_golden_info(&mut out_writer);
   }

   #[test]
   fn standard_dfs_rand_perms_stable() {
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

         // for i in 0..sort_size as usize {
         //    println!("\t{}", sorting_data[i].value);
         // }
         let sorted_head: usize = sort_standard_dfs(
            &mut sorting_data,
            &mut sorting_stats,
            DfsSortConfig { final_reverse: false, upper_lozenge: true, dfs_tree: false },
         );

         check_sort(&sorting_data, sorted_head, false);
      }
      sorting_stats.finish_gather();

      assert_eq!(permutation_count, num_samples);

      // let mut mint = Mint::new("tests/golden-outputs");
      // let out_file = mint.new_goldenfile("standard_interlink.m").unwrap();
      // let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
      // sorting_stats.write_golden_info(&mut out_writer);
   }

   #[test]
   fn standard_dfs_lozenge() {
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
         let sorted_head: usize = sort_standard_dfs(
            &mut sorting_data,
            &mut sorting_stats,
            DfsSortConfig { final_reverse: true, upper_lozenge: true, dfs_tree: false },
         );
         check_sort(&sorting_data, sorted_head, true);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("standard_lozenge.m").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
         write_auxiliary_edges(&sorting_data, sorted_head, "standard_lozenge", &mut out_writer);
      }
   }

   #[test]
   fn standard_dfs_lozenge_sizes_stable() {
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
         let sorted_head: usize = sort_standard_dfs(
            &mut sorting_data,
            &mut sorting_stats,
            DfsSortConfig { final_reverse: true, upper_lozenge: true, dfs_tree: false },
         );
         check_sort(&sorting_data, sorted_head, true);
         check_lozenge(&sorting_data, sorted_head);

         // let mut mint = Mint::new("tests/golden-outputs");
         // let out_file = mint.new_goldenfile("standard_lozenge.m").unwrap();
         // let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
         // write_auxiliary_edges(&sorting_data, sorted_head, "standard_lozenge", &mut out_writer);
      }
   }

   #[test]
   fn standard_dfs_dfs_tree() {
      let seed_state = 0x6fcad0e0b15d55ee;
      let seed_stream = 0x02abbdf7b07c3ab;
      let mut rng = rand_pcg::Pcg32::new(seed_state, seed_stream);
      let mut fy = shuffle::fy::FisherYates::default();

      let sort_size: u32 = 32;
      let number_of_nudges: u32 = 8;

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
         let sorted_head: usize = sort_standard_dfs(
            &mut sorting_data,
            &mut sorting_stats,
            DfsSortConfig { final_reverse: true, upper_lozenge: false, dfs_tree: true },
         );
         check_sort(&sorting_data, sorted_head, true);
         check_p_dfs(&sorting_data, sorted_head);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("standard_dfs_tree.m").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
         write_auxiliary_edges(&sorting_data, sorted_head, "standard_dfs_tree", &mut out_writer);
      }
   }

   #[test]
   fn standard_dfs_dfs_tree_sizes_stable() {
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
         let sorted_head: usize = sort_standard_dfs(
            &mut sorting_data,
            &mut sorting_stats,
            DfsSortConfig { final_reverse: true, upper_lozenge: false, dfs_tree: true },
         );
         check_sort(&sorting_data, sorted_head, true);
         check_p_dfs(&sorting_data, sorted_head);

         // let mut mint = Mint::new("tests/golden-outputs");
         // let out_file = mint.new_goldenfile("standard_dfs_tree.m").unwrap();
         // let mut out_writer: BufWriter<File> = BufWriter::new(out_file);
         // write_auxiliary_edges(&sorting_data, sorted_head, "standard_dfs_tree", &mut out_writer);
      }
   }
}
