pub trait Linkable {
   fn set_forward_link(&mut self, link: Option<usize>);
   fn set_secondary_link(&mut self, link: Option<usize>);
   fn set_tertiary_link(&mut self, link: Option<usize>);
   fn set_backward_link(&mut self, link: Option<usize>);
   //
   fn get_forward_link(&self) -> Option<usize>;
   fn get_secondary_link(&self) -> Option<usize>;
   fn get_tertiary_link(&self) -> Option<usize>;
   fn get_backward_link(&self) -> Option<usize>;
}

#[derive(Default, Clone)]
pub struct SortableEntity {
   pub value: u32,
   pub original_index: u32, // In testing used to check sort stability.
   forward_link: Option<usize>,
   secondary_link: Option<usize>,
   tertiary_link: Option<usize>,
   backward_link: Option<usize>,
}

// impl PartialEq for SortableEntity {
//    #[inline]
//    fn eq(&self, other: &Self) -> bool {
//       self.value.eq(&other.value)
//    }
// }

// impl Eq for SortableEntity {}

// impl PartialOrd for SortableEntity {
//    #[inline]
//    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//       Some(self.value.cmp(&other.value))
//    }
// }

// impl Ord for SortableEntity {
//    #[inline]
//    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//       self.value.cmp(&other.value)
//    }
// }

impl Linkable for SortableEntity {
   #[inline]
   fn set_forward_link(self: &mut SortableEntity, link: Option<usize>) {
      self.forward_link = link;
   }
   #[inline]
   fn set_secondary_link(self: &mut SortableEntity, link: Option<usize>) {
      self.secondary_link = link;
   }
   #[inline]
   fn set_tertiary_link(self: &mut SortableEntity, link: Option<usize>) {
      self.tertiary_link = link;
   }
   #[inline]
   fn set_backward_link(self: &mut SortableEntity, link: Option<usize>) {
      self.backward_link = link;
   }
   #[inline]
   fn get_forward_link(self: &SortableEntity) -> Option<usize> {
      self.forward_link
   }
   #[inline]
   fn get_secondary_link(self: &SortableEntity) -> Option<usize> {
      self.secondary_link
   }
   #[inline]
   fn get_tertiary_link(self: &SortableEntity) -> Option<usize> {
      self.tertiary_link
   }
   #[inline]
   fn get_backward_link(self: &SortableEntity) -> Option<usize> {
      self.backward_link
   }
}

pub fn fill_sortable(sortable: &mut [SortableEntity], permutation_values: &[u32]) {
   let size: usize = sortable.len();
   assert_eq!(size, permutation_values.len());

   for i in 0..size as usize {
      // For consistency:
      sortable[i] = Default::default();
      // Set data for sorting.
      sortable[i].original_index = i as u32;
      sortable[i].value = permutation_values[i] as u32;
   }
}
