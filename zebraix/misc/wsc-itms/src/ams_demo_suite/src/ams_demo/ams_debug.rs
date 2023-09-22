use crate::ams_demo::ams_sortable::Linkable;
use crate::ams_demo::MergeStep;
use crate::ams_demo::SortableEntity;
use std::io::Write;

pub fn write_graph_coords(obverse_by_principal: &[u32], writer: &mut impl Write) {
   let size = obverse_by_principal.len();
   for (i, vertex) in obverse_by_principal.iter().enumerate().take(size) {
      writeln!(writer, "{},{}", i, vertex).unwrap();
   }
}

pub fn check_sort(sortable: &[SortableEntity], head_item: usize, check_backwards: bool) {
   let size: usize = sortable.len();
   let mut prev_link: Option<usize> = Some(head_item);
   let mut prev: &SortableEntity = &sortable[head_item];
   let mut next_link: Option<usize> = prev.get_forward_link();

   if check_backwards {
      assert!(prev.get_backward_link() == None);
   }

   let mut count = 0;
   while next_link.is_some() {
      let next: &SortableEntity = &sortable[next_link.unwrap()];
      assert!(
         (prev.value < next.value)
            || ((prev.value == next.value) && (prev.original_index < next.original_index))
      );
      if check_backwards {
         assert!(next.get_backward_link().unwrap() == prev_link.unwrap());
      }
      //
      prev_link = next_link;
      prev = next;
      count += 1;
      next_link = next.get_forward_link();
   }
   assert_eq!(count, size - 1);
}

pub fn check_lozenge(sortable: &[SortableEntity], head_item: usize) {
   let size: usize = sortable.len();
   {
      // Forward, left lozenge paths, secondary.
      let mut check_secondary: Vec<Option<usize>> = vec![None; size];

      let mut curr_max_value = sortable[0].value;
      let mut curr_min_value = sortable[0].value;
      let mut index_of_max = 0;
      let mut index_of_min = 0;
      for i in 1..size {
         let new_value = sortable[i].value;
         if new_value >= curr_max_value {
            check_secondary[index_of_max] = Some(i);
            curr_max_value = new_value;
            index_of_max = i;
         } else if new_value < curr_min_value {
            check_secondary[i] = Some(index_of_min);
            curr_min_value = new_value;
            index_of_min = i;
         }
      }
      assert!(index_of_min == head_item);

      for i in 0..size {
         assert!(
            check_secondary[i] == None || (sortable[i].get_secondary_link() == check_secondary[i])
         );
      }
   }
   {
      // Reverse, right  lozenge paths, tertiary.
      let mut check_tertiary: Vec<Option<usize>> = vec![None; size];

      let mut curr_max_value = sortable[size - 1].value;
      let mut curr_min_value = sortable[size - 1].value;
      let mut index_of_max = size - 1;
      let mut index_of_min = size - 1;
      for i in (0..size - 1).rev() {
         let new_value = sortable[i].value;
         if new_value > curr_max_value {
            check_tertiary[index_of_max] = Some(i);
            curr_max_value = new_value;
            index_of_max = i;
         } else if new_value <= curr_min_value {
            check_tertiary[i] = Some(index_of_min);
            curr_min_value = new_value;
            index_of_min = i;
         }
      }
      assert!(index_of_min == head_item);

      for i in 0..size {
         assert!(
            check_tertiary[i] == None || (sortable[i].get_tertiary_link() == check_tertiary[i])
         );
      }
   }
}

pub fn check_p_dfs(sortable: &[SortableEntity], head_item: usize) {
   let size: usize = sortable.len();
   let mut roots_stack: Vec<usize> = vec![0; 0];
   let mut check_secondary: Vec<Option<usize>> = vec![None; size];

   for i in (0..size).rev() {
      while !roots_stack.is_empty()
         && (sortable[i].value <= sortable[roots_stack[roots_stack.len() - 1]].value)
      {
         let popped_index = roots_stack.pop().unwrap();
         check_secondary[popped_index] = Some(i);
      }
      roots_stack.push(i);
   }

   assert!(roots_stack[0] == head_item);

   for i in 0..roots_stack.len() - 1 {
      check_secondary[roots_stack[i]] = Some(roots_stack[i + 1]);
   }

   for i in 0..size {
      assert!(sortable[i].get_secondary_link() == check_secondary[i]);
   }
}

#[allow(clippy::too_many_arguments)]
pub fn check_block_list(
   sorting_data: &[SortableEntity],
   merge_step: &MergeStep,
   parents_head: Option<usize>,
   children_head: Option<usize>,
   left_tail: usize,
   right_tail: usize,
   all_se_appended: bool,
   all_sw_appended: bool,
) {
   let mut curr_block = parents_head;
   if !all_se_appended {
      while curr_block != Some(merge_step.middle - 1) {
         let next_block = sorting_data[curr_block.unwrap()].get_tertiary_link();
         assert!(next_block.unwrap() > curr_block.unwrap());
         assert!(
            sorting_data[curr_block.unwrap()].value <= sorting_data[next_block.unwrap()].value
         );
         curr_block = next_block;
      }
   }
   while curr_block != Some(left_tail) {
      let next_block = sorting_data[curr_block.unwrap()].get_tertiary_link();
      assert!(next_block.unwrap() < curr_block.unwrap());
      assert!(sorting_data[next_block.unwrap()].value > sorting_data[curr_block.unwrap()].value);
      curr_block = next_block;
   }
   curr_block = children_head;
   if !all_sw_appended {
      while curr_block != Some(merge_step.middle) {
         let next_block = sorting_data[curr_block.unwrap()].get_secondary_link();
         assert!(next_block.unwrap() < curr_block.unwrap());
         assert!(sorting_data[next_block.unwrap()].value > sorting_data[curr_block.unwrap()].value);
         curr_block = next_block;
      }
   }
   while curr_block != Some(right_tail) {
      let next_block = sorting_data[curr_block.unwrap()].get_secondary_link();
      assert!(next_block.unwrap() > curr_block.unwrap());
      assert!(sorting_data[curr_block.unwrap()].value <= sorting_data[next_block.unwrap()].value);
      curr_block = next_block;
   }
}

pub fn write_auxiliary_edges(
   sortable: &[SortableEntity],
   head_item: usize,
   function_name: &str,
   writer: &mut impl Write,
) {
   let size: usize = sortable.len();
   let mut obverse_vec = vec![0; size];
   // obverse_vec.reserve(size);
   let mut curr_link: Option<usize> = Some(head_item);

   let mut obverse: usize = 0;
   while curr_link.is_some() {
      let principal: usize = curr_link.unwrap();
      let curr: &SortableEntity = &sortable[principal];

      obverse_vec[principal] = obverse;

      obverse += 1;
      curr_link = curr.get_forward_link();
   }

   writeln!(writer, "function auxiliary_links = {}()", function_name).unwrap();
   writeln!(writer, "auxiliary_links = [").unwrap();
   for src_principal in 0..size {
      let src_obverse = obverse_vec[src_principal];
      let src_value = sortable[src_principal].value;

      let secondary_principal: i64;
      let secondary_obverse: i64;
      let secondary_value: i64;
      let secondary_link = sortable[src_principal].get_secondary_link();
      if secondary_link != None {
         secondary_principal = secondary_link.unwrap() as i64;
         secondary_obverse = obverse_vec[secondary_principal as usize] as i64;
         secondary_value = sortable[secondary_principal as usize].value as i64;
      } else {
         secondary_principal = -1;
         secondary_obverse = -1;
         secondary_value = -1;
      }

      let tertiary_principal: i64;
      let tertiary_obverse: i64;
      let tertiary_value: i64;
      let tertiary_link = sortable[src_principal].get_tertiary_link();
      if tertiary_link != None {
         tertiary_principal = tertiary_link.unwrap() as i64;
         tertiary_obverse = obverse_vec[tertiary_principal as usize] as i64;
         tertiary_value = sortable[tertiary_principal as usize].value as i64;
      } else {
         tertiary_principal = -1;
         tertiary_obverse = -1;
         tertiary_value = -1;
      }

      writeln!(
         writer,
         "[{},{},{},{},{},{},{},{},{}],",
         src_principal,
         src_obverse,
         src_value,
         secondary_principal,
         secondary_obverse,
         secondary_value,
         tertiary_principal,
         tertiary_obverse,
         tertiary_value,
      )
      .unwrap();
   }
   writeln!(writer, "];").unwrap();
   writeln!(writer, "endfunction").unwrap();
}
