#[cfg(test)]
use crate::ams_demo::ams_debug::write_graph_coords;
pub use crate::ams_demo::fill_sortable;
pub use crate::ams_demo::Linkable;
pub use crate::ams_demo::MergeStep;
pub use crate::ams_demo::MinusPlusShift;
pub use crate::ams_demo::SortStats;
pub use crate::ams_demo::SortStatsInit;
pub use crate::ams_demo::SortableEntity;
#[cfg(test)]
use goldenfile::Mint;
use ndarray::s;
use ndarray::Array1;
use ndarray::Array2;
use rand::Rng;
use rand::RngCore;
use std::collections::VecDeque;
#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::{BufWriter, Write};

// This should really be incorporated into pattern generation,
// although perhaps retained nonetheless as a utility.
pub fn nudge_values(data_vec: &mut [u32], rng: &mut impl RngCore, num_nudges: u32) {
   let max_index = data_vec.len() - 1;
   let max_value = max_index as u32; // Assume in this version, rather than find min/max.
   let min_value: u32 = 0;
   for _i in 0..num_nudges {
      let nudge_change: i32 = if rng.gen::<f32>() < 0.5 { -1 } else { 1 };
      let index_to_nudge = rng.gen_range(1..max_index); // Avoid beginning and end of vector.
      if (data_vec[index_to_nudge] != max_value) && (data_vec[index_to_nudge] != min_value) {
         data_vec[index_to_nudge] = (data_vec[index_to_nudge] as i32 + nudge_change) as u32;
      }
   }
}

#[derive(Copy, Clone)]
pub enum SortType {
   RANDOM,
   FORWARD,
   REVERSE,
   SWITCH,
}

impl Default for SortType {
   fn default() -> Self {
      SortType::RANDOM
   }
}

#[derive(Copy, Clone)]
pub enum DisruptionType {
   NONE,
   DISPLACE,
   SHUFFLE,
}

impl Default for DisruptionType {
   fn default() -> Self {
      DisruptionType::NONE
   }
}

#[derive(Copy, Clone)]
pub struct PermutationConfig {
   pub inter_block_strategy: SortType,
   pub intra_block_strategy: SortType,
   pub num_blocks_rows: u32,
   pub num_blocks_cols: u32,
   pub reversal_switch_rate: f32,
   pub disruption_type: DisruptionType,
   pub disruption_rate: f32,
}

impl Default for PermutationConfig {
   fn default() -> Self {
      PermutationConfig {
         inter_block_strategy: SortType::FORWARD,
         intra_block_strategy: Default::default(),
         num_blocks_rows: 1,
         num_blocks_cols: 1,
         reversal_switch_rate: 0.5,
         disruption_type: Default::default(),
         disruption_rate: Default::default(),
      }
   }
}
// Constraints:
// inter_block_strategy not SWITCH.
// If inter_block_strategy not RANDOM, num_blocks_row == num_blocks_col.
// If inter_block_strategy RANDOM, num_blocks_row <= num_blocks_col.
//
// Since rejection sampling is used with DisruptionType;:Displace,
// disruption_rate should be small, preferrably <= 0.05.
pub struct BlockPermutation {
   pub sort_size: u32,
   pub permutation_config: PermutationConfig,
   // pub inter_block_strategy: SortType,
   // pub intra_block_strategy: SortType,
   // pub num_blocks_rows: u32,
   // pub num_blocks_cols: u32,
   // pub reversal_switch_rate: f32,
   // pub disruption_type: DisruptionType,
   // pub disruption_rate: f32,
   pub block_types: Array2<SortType>,
   pub principal_dividers: Array1<u32>, // Last entry is sort_size.

   // pub permutation_of_principals: Array1<u32>,
   pub order_of_cols: Array1<u32>,
   pub permutation: Array1<u32>,
}

impl BlockPermutation {
   pub fn new(sort_size: u32, permutation_config: PermutationConfig) -> BlockPermutation {
      BlockPermutation {
         sort_size,
         permutation_config,
         permutation: Array1::from_iter(0..sort_size),

         // inter_block_strategy: permutation_config.inter_block_strategy,
         // intra_block_strategy: permutation_config.intra_block_strategy,

         // permutation_config.num_blocks_rows,
         // permutation_config.num_blocks_cols,
         // reversal_switch_rate: 0.0,
         // permutation_config.disruption_type,
         // permutation_config.disruption_rate,
         principal_dividers: Array1::zeros(permutation_config.num_blocks_cols as usize),
         block_types: Array2::default((
            permutation_config.num_blocks_rows as usize,
            permutation_config.num_blocks_cols as usize,
         )),

         // permutation_of_principals: Array1::from_iter(0..sort_size),
         order_of_cols: Array1::from_iter(0..permutation_config.num_blocks_cols),
      }
   }

   pub fn commission(&mut self) {
      // sort_size: u32,
      // inter_block_strategy: SortType,
      // intra_block_strategy: SortType,
      // num_blocks_rows: u32,
      // num_blocks_cols: u32,
      // reversal_switch_rate: f32,

      for i in 0..self.permutation_config.num_blocks_rows as usize {
         for j in 0..self.permutation_config.num_blocks_cols as usize {
            self.block_types[[i, j]] = self.permutation_config.intra_block_strategy;
         }
      }

      for i in 0..self.permutation_config.num_blocks_cols as usize {
         self.principal_dividers[i] = (((i + 1) as f64) * self.sort_size as f64
            / self.permutation_config.num_blocks_cols as f64)
            .round() as u32;
      }

      // permutation_of_principals: Array1<u32>,
      // order_of_cols: Array1<u32>,
      // permutation: Array1<u32>,
      // let die = Uniform::from(1..7);
   }

   pub fn generate(&mut self, perm_values: &mut [u32], rng: &mut impl RngCore) {
      let mut fy = shuffle::fy::FisherYates::default();
      use shuffle::shuffler::Shuffler; // For fy.shuffle.

      // sort_size: u32,
      // inter_block_strategy: SortType,
      // intra_block_strategy: SortType,
      // num_blocks_rows: u32,
      // num_blocks_cols: u32,
      // reversal_switch_rate: f32,

      // block_types: Array2<SortType>,
      // principal_dividers: Array1<u32>, // Last entry is sort_size.

      // let mut principal_shuffle: Vec<u32> = (0..self.sort_size).collect();
      // assert!(fy.shuffle(&mut principal_shuffle, rng).is_ok());
      // self.permutation_of_principals = Array1::from_iter(principal_shuffle);

      match self.permutation_config.inter_block_strategy {
         SortType::RANDOM => {
            let mut column_shuffle: Vec<u32> =
               (0..self.permutation_config.num_blocks_cols).collect();
            assert!(fy.shuffle(&mut column_shuffle, rng).is_ok());
            self.order_of_cols = Array1::from_iter(column_shuffle);
         }
         SortType::FORWARD => {
            self.order_of_cols = Array1::from_iter(0..self.permutation_config.num_blocks_cols)
         }
         SortType::REVERSE => {
            self.order_of_cols =
               Array1::from_iter((0..self.permutation_config.num_blocks_cols).rev())
         }
         SortType::SWITCH => {
            panic!("Inter-block layout cannot have switch type.");
         }
      }

      // let mut block_sizes = Array1::<Dim<[u32; 1]>>::zeros(self.num_blocks_cols);
      let mut block_sizes = Array1::zeros(self.permutation_config.num_blocks_cols as usize);
      let mut count = 0;
      for i in 0..self.permutation_config.num_blocks_cols as usize {
         block_sizes[i as usize] = self.principal_dividers[i as usize] - count;
         count = self.principal_dividers[i];
      }
      // let mut block_sizes = Array1::<u32>::zeros(num_blocks_cols);
      // for i in 0..self.num_blocks_cols {
      //    block_sizes[i] = block_sizes[self.order_of_cols[i]];
      // }
      assert_eq!(count, self.sort_size);

      let mut col_count: u32 = 0;
      let mut principals_covered: u32 = 0;
      let mut obverse_covered: u32 = 0;
      for row in 0..self.permutation_config.num_blocks_rows {
         let double_target_consumption: u32 = (((row + 1) as f64) * self.sort_size as f64
            / self.permutation_config.num_blocks_rows as f64
            * 2.0)
            .round() as u32;

         let mut col_block_fifo: VecDeque<u32> =
            VecDeque::with_capacity(self.permutation_config.num_blocks_cols as usize);
         let mut total_elements_this_row = 0;

         while (col_count < self.permutation_config.num_blocks_cols)
            && (((principals_covered + total_elements_this_row) * 2
               + block_sizes[self.order_of_cols[col_count as usize] as usize])
               <= double_target_consumption)
         {
            total_elements_this_row += block_sizes[self.order_of_cols[col_count as usize] as usize];
            col_block_fifo.push_back(self.order_of_cols[col_count as usize]);
            col_count += 1;
         }
         if col_block_fifo.is_empty() {
            continue; // Occurs if more rows than columns.
         }

         let obverse_low = obverse_covered;
         let obverse_high = obverse_covered + total_elements_this_row;
         let mut obverse_shuffle: Vec<u32> = (obverse_low..obverse_high).collect();
         assert!(fy.shuffle(&mut obverse_shuffle, rng).is_ok());
         let permutation_of_obverse = Array1::from_iter(obverse_shuffle);

         let mut obverse_done = 0;
         for _i in 0..col_block_fifo.len() {
            let col_block = col_block_fifo.pop_front().unwrap();
            let block_size = block_sizes[col_block as usize];
            let mut block_of_obverse: Vec<u32> = permutation_of_obverse
               .slice(s![obverse_done as usize..obverse_done as usize + block_size as usize])
               .to_vec();

            match self.block_types[[row as usize, col_block as usize]] {
               SortType::RANDOM => {}
               SortType::FORWARD => {
                  block_of_obverse.sort_unstable();
               }
               SortType::REVERSE => {
                  block_of_obverse.sort_unstable();
                  block_of_obverse.reverse();
               }
               SortType::SWITCH => {
                  assert!(self.permutation_config.reversal_switch_rate > 0.0);
                  if rng.gen::<f32>() < self.permutation_config.reversal_switch_rate {
                     block_of_obverse.sort_unstable();
                     block_of_obverse.reverse();
                  } else {
                     block_of_obverse.sort_unstable();
                  }
               }
            }
            // Assign a vector into array slice.
            for (a, b) in self
               .permutation
               .slice_mut(s![self.principal_dividers[col_block as usize] as usize
                  - block_size as usize
                  ..self.principal_dividers[col_block as usize] as usize])
               .into_iter()
               .zip(block_of_obverse.into_iter())
            {
               *a = b;
            }
            obverse_done += block_size;
         }

         // let principals_low = principals_covered;
         // let principals_high = principals_covered + self.principal_dividers[i];
         principals_covered += total_elements_this_row;
         obverse_covered += total_elements_this_row;
      }

      // Final shuffle.
      if self.permutation_config.disruption_rate > 0.0 {
         let target_shuffle_count = self.permutation_config.disruption_rate * self.sort_size as f32;
         let mut shuffle_count = target_shuffle_count.floor() as usize;
         if rng.gen::<f32>() < (target_shuffle_count - shuffle_count as f32) {
            shuffle_count += 1;
         }
         match self.permutation_config.disruption_type {
            DisruptionType::DISPLACE => {
               // Create a vector of principal indices, and randomly
               // displace shuffle_count of them.
               let mut shuffle_values: Vec<f64> = (0..self.sort_size).map(f64::from).collect();
               for i in 0..shuffle_count as usize {
                  let mut target_shuffle = rng.gen_range(0..self.sort_size as usize);
                  while shuffle_values[target_shuffle].fract() != 0.0 {
                     // Reject and retry if target has already been selected.
                     target_shuffle = rng.gen_range(0..self.sort_size as usize);
                  }
                  let mut destination_shuffle =
                     rng.gen_range(0..self.sort_size as usize - 1) as i64;
                  if destination_shuffle < (target_shuffle as i64) {
                     destination_shuffle -= 1;
                  } else {
                     destination_shuffle += 1;
                  }
                  shuffle_values[target_shuffle] =
                     destination_shuffle as f64 + (i as f64 + 1.0) / (shuffle_count as f64 + 2.0);
               }
               let mut shuffle_principals: Vec<usize> = (0..self.sort_size as usize).collect();
               let permutation_copy = self.permutation.clone();

               shuffle_principals.sort_unstable_by(|a, b| {
                  shuffle_values[*a].partial_cmp(&shuffle_values[*b]).unwrap()
               });
               for i in 0..self.sort_size as usize {
                  self.permutation[i] = shuffle_principals[permutation_copy[i] as usize] as u32;
               }
            }
            DisruptionType::SHUFFLE => {
               // Create a vector of principal indices, and randomly
               // shuffle into the first shuffle_count of them.
               let mut shuffle_principals: Vec<usize> = (0..self.sort_size as usize).collect();
               for i in 0..shuffle_count as usize {
                  let target_shuffle = rng.gen_range(i..self.sort_size as usize);
                  shuffle_principals.swap(target_shuffle, i);
               }
               // Rotate obverse indices around ring of principals given in shuffle_principals.
               let mut source_obverse = self.permutation[shuffle_principals[shuffle_count - 1]];
               for item in shuffle_principals.iter_mut().take(shuffle_count) {
                  std::mem::swap(&mut self.permutation[*item], &mut source_obverse);
               }
            }

            DisruptionType::NONE => {}
         }
      }
      // Provide permutation in output.
      for (a, b) in perm_values.iter_mut().zip(self.permutation.to_vec().into_iter()) {
         *a = b as u32;
      }
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn plain_random() {
      let sort_size: u32 = 32;
      let seed_state = 0x67349a9b7998d87c;
      let seed_stream = 0xc78563468eed87;

      let mut perm_values: Vec<u32> = (0..sort_size).collect();

      let mut rng = rand_pcg::Pcg32::new(seed_state, seed_stream);
      let mut fy = shuffle::fy::FisherYates::default();

      use shuffle::shuffler::Shuffler; // For fy.shuffle.
      assert!(fy.shuffle(&mut perm_values, &mut rng).is_ok());

      let mut mint = Mint::new("tests/golden-outputs");
      let out_file = mint.new_goldenfile("plain_random.csv").unwrap();
      let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

      for (i, perm_item) in perm_values.iter().enumerate().take(sort_size as usize) {
         writeln!(&mut out_writer, "{},{}", i, perm_item).unwrap();
      }
   }

   #[test]
   fn four_by_four_patterns() {
      let sort_size: u32 = 32;
      let seed_state = 0x430956a9b7d87c;
      let seed_stream = 0x8eedc785634687;

      let mut perm_values: Vec<u32> = (0..sort_size).collect();

      let mut rng = rand_pcg::Pcg32::new(seed_state, seed_stream);

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::RANDOM,
               intra_block_strategy: SortType::RANDOM,
               num_blocks_rows: 4,
               num_blocks_cols: 4,
               ..Default::default()
            },
         );

         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_random.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::RANDOM,
               intra_block_strategy: SortType::FORWARD,
               num_blocks_rows: 4,
               num_blocks_cols: 4,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_forward.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::RANDOM,
               intra_block_strategy: SortType::SWITCH,
               num_blocks_rows: 4,
               num_blocks_cols: 4,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_switch.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::FORWARD,
               intra_block_strategy: SortType::RANDOM,
               num_blocks_rows: 4,
               num_blocks_cols: 4,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_diagonal.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::REVERSE,
               intra_block_strategy: SortType::RANDOM,
               num_blocks_rows: 4,
               num_blocks_cols: 4,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_anti.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::FORWARD,
               intra_block_strategy: SortType::FORWARD,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_presort.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::RANDOM,
               intra_block_strategy: SortType::FORWARD,
               num_blocks_rows: 1,
               num_blocks_cols: 4,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_merge.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::FORWARD,
               intra_block_strategy: SortType::RANDOM,
               num_blocks_rows: 1,
               num_blocks_cols: 2,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.block_types[[0, 0]] = SortType::FORWARD;
         block_layout.principal_dividers[0] = sort_size as u32 * 3 / 4;
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_append.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::FORWARD,
               intra_block_strategy: SortType::SWITCH,
               num_blocks_rows: 1,
               num_blocks_cols: 4,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_zigzag.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }
   }

   #[test]
   fn four_by_four_with_disruption() {
      let sort_size: u32 = 128;
      let seed_state = 0x6a390b79d584c7;
      let seed_stream = 0xec7885634d6e78;

      let mut perm_values: Vec<u32> = (0..sort_size).collect();

      let mut rng = rand_pcg::Pcg32::new(seed_state, seed_stream);

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::RANDOM,
               intra_block_strategy: SortType::FORWARD,
               num_blocks_rows: 4,
               num_blocks_cols: 4,
               disruption_rate: 6.0 / 128.0,
               disruption_type: DisruptionType::SHUFFLE,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_forward_shuffle.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::RANDOM,
               intra_block_strategy: SortType::SWITCH,
               num_blocks_rows: 4,
               num_blocks_cols: 4,
               disruption_type: DisruptionType::SHUFFLE,
               disruption_rate: 6.0 / 128.0,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_switch_shuffle.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::RANDOM,
               intra_block_strategy: SortType::RANDOM,
               num_blocks_rows: 4,
               num_blocks_cols: 4,
               disruption_type: DisruptionType::DISPLACE,
               disruption_rate: 6.0 / 128.0,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_random_displace.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::RANDOM,
               intra_block_strategy: SortType::FORWARD,
               num_blocks_rows: 4,
               num_blocks_cols: 4,
               disruption_type: DisruptionType::DISPLACE,
               disruption_rate: 6.0 / 128.0,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_forward_displace.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::RANDOM,
               intra_block_strategy: SortType::SWITCH,
               num_blocks_rows: 4,
               num_blocks_cols: 4,
               disruption_type: DisruptionType::DISPLACE,
               disruption_rate: 6.0 / 128.0,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_switch_displace.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::FORWARD,
               intra_block_strategy: SortType::RANDOM,
               num_blocks_rows: 4,
               num_blocks_cols: 4,
               disruption_type: DisruptionType::DISPLACE,
               disruption_rate: 6.0 / 128.0,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_diagonal_displace.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::REVERSE,
               intra_block_strategy: SortType::RANDOM,
               num_blocks_rows: 4,
               num_blocks_cols: 4,
               disruption_type: DisruptionType::DISPLACE,
               disruption_rate: 6.0 / 128.0,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_anti_displace.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::FORWARD,
               intra_block_strategy: SortType::FORWARD,
               disruption_type: DisruptionType::DISPLACE,
               disruption_rate: 6.0 / 128.0,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_presort_displace.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::RANDOM,
               intra_block_strategy: SortType::FORWARD,
               num_blocks_rows: 1,
               num_blocks_cols: 4,
               disruption_type: DisruptionType::DISPLACE,
               disruption_rate: 6.0 / 128.0,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_merge_displace.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::FORWARD,
               intra_block_strategy: SortType::RANDOM,
               num_blocks_rows: 1,
               num_blocks_cols: 2,
               disruption_type: DisruptionType::DISPLACE,
               disruption_rate: 6.0 / 128.0,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.block_types[[0, 0]] = SortType::FORWARD;
         block_layout.principal_dividers[0] = sort_size as u32 * 3 / 4;
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_append_displace.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }

      {
         let mut block_layout = BlockPermutation::new(
            sort_size as u32,
            PermutationConfig {
               inter_block_strategy: SortType::FORWARD,
               intra_block_strategy: SortType::SWITCH,
               num_blocks_rows: 1,
               num_blocks_cols: 4,
               disruption_type: DisruptionType::DISPLACE,
               disruption_rate: 6.0 / 128.0,
               ..Default::default()
            },
         );
         block_layout.commission();
         block_layout.generate(&mut perm_values, &mut rng);

         let mut mint = Mint::new("tests/golden-outputs");
         let out_file = mint.new_goldenfile("four_by_four_zigzag_displace.csv").unwrap();
         let mut out_writer: BufWriter<File> = BufWriter::new(out_file);

         write_graph_coords(&perm_values, &mut out_writer);
      }
   }
}
