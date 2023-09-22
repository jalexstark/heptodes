pub use crate::ams_demo::ams_sortable::SortableEntity;
use ndarray::Array1;
use ndarray::Array2;
use std::io;

pub struct SortStatsInit {
   pub num_bins: u32,
   pub sort_size: usize,
   pub overstretch: f64,
   // For classic merge sort set shifted_limit = ceil(sort_size_log2).
   pub shifted_limit: u32,
   pub sort_size_log2: f64,
}

pub enum SortStatsCounts {
   NWork,
   NLogN,
}

pub struct SortStats {
   // Configuration.
   pub num_bins: u32,
   pub sort_size: usize,
   pub overstretch: f64,
   pub shifted_limit: u32,
   pub sort_size_log2: f64,
   //
   // Post-init commissioning.
   pub small_range_base: i64, // Small range is intended for 0..N.
   pub small_range_scale: f64,
   pub large_range_base: i64, // Large range is intended for 0..NlogN storage.
   pub large_range_scale: f64,
   // Data gathering.  Note that "nlogn" refers to the complexity category.
   pub subsort_count_by_k: Array1<u32>,
   pub sample_count: u32,
   //
   pub nlogn_by_k: Array2<u32>, // Stored as small range (..N).
   pub nlogn_by_k_total: Array1<u64>,
   pub nlogn_all: Array1<u32>, // Stored as large range (..NlogN).
   pub nlogn_all_total: u64,
   //
   pub n_work_by_k: Array2<u32>, // Stored as small range (..N).
   pub n_work_by_k_total: Array1<u64>,
   pub n_work_all: Array1<u32>, // Stored as large range (..NlogN).
   pub n_work_all_total: u64,
   //
   pub combo_by_k: Array2<u32>, // Stored as small range (..N).
   pub combo_by_k_total: Array1<u64>,
   pub combo_all: Array1<u32>, // Stored as large range (..NlogN).
   pub combo_all_total: u64,
   //
   pub accum_nlogn_by_k: Array1<u32>,
   pub accum_n_work_by_k: Array1<u32>,
   // Data on finalization.
}

impl SortStats {
   pub fn new(init_struct: SortStatsInit) -> SortStats {
      SortStats {
         sort_size: init_struct.sort_size,
         shifted_limit: init_struct.shifted_limit,
         overstretch: init_struct.overstretch,
         num_bins: init_struct.num_bins,
         sort_size_log2: init_struct.sort_size_log2,
         // Junk numbers to ensure meaningless results without start /
         // finish.
         small_range_base: 452672,
         small_range_scale: 345.7547,
         large_range_base: 8652486,
         large_range_scale: 6365.885,
         //
         subsort_count_by_k: Array1::zeros(0),
         sample_count: 4854673,
         //
         nlogn_by_k: Array2::zeros((0, 0)),
         nlogn_by_k_total: Array1::zeros(0),
         nlogn_all: Array1::zeros(0),
         nlogn_all_total: 523827,
         //
         n_work_by_k: Array2::zeros((0, 0)),
         n_work_by_k_total: Array1::zeros(0),
         n_work_all: Array1::zeros(0),
         n_work_all_total: 752382,
         //
         combo_by_k: Array2::zeros((0, 0)),
         combo_by_k_total: Array1::zeros(0),
         combo_all: Array1::zeros(0),
         combo_all_total: 238725,
         // Working counters during single sort.
         accum_nlogn_by_k: Array1::zeros(0),
         accum_n_work_by_k: Array1::zeros(0),
      }
   }

   #[inline]
   pub fn cmp_sortable(
      &mut self,
      a: &SortableEntity,
      b: &SortableEntity,
      count: SortStatsCounts,
      shifted_level: u32,
   ) -> bool {
      match count {
         SortStatsCounts::NLogN => {
            self.increment_nlogn(shifted_level);
         }
         SortStatsCounts::NWork => {
            self.increment_n_work(shifted_level);
         }
      }
      a.value <= b.value
   }

   #[inline]
   pub fn bin_by_small_range(&self, n_value: f64) -> u32 {
      ((n_value - self.small_range_base as f64) * self.small_range_scale)
         .clamp(0.0, self.num_bins as f64 - 1.0) as u32
   }

   #[inline]
   pub fn bin_by_large_range(&self, n_value: f64) -> u32 {
      ((n_value - self.large_range_base as f64) * self.large_range_scale)
         .clamp(0.0, self.num_bins as f64 - 1.0) as u32
   }

   pub fn start_gather(&mut self) {
      // First, commission.
      self.small_range_base = 0;
      self.small_range_scale = (self.sort_size as i64 - self.small_range_base) as f64
         / self.num_bins as f64
         / self.overstretch;
      self.large_range_base = 0;
      self.large_range_scale = (self.sort_size as f64 * self.sort_size_log2
         - self.large_range_base as f64) as f64
         / self.num_bins as f64
         / self.overstretch;

      // Second, create accumulators.
      self.subsort_count_by_k = Array1::zeros(self.shifted_limit as usize);
      self.sample_count = 0;

      self.nlogn_by_k = Array2::zeros((self.shifted_limit as usize, self.num_bins as usize));
      self.nlogn_by_k_total = Array1::zeros(self.shifted_limit as usize); // Unbinned total for mean calc.
      self.nlogn_all = Array1::zeros(self.num_bins as usize);
      self.nlogn_all_total = 0;

      self.n_work_by_k = Array2::zeros((self.shifted_limit as usize, self.num_bins as usize));
      self.n_work_by_k_total = Array1::zeros(self.shifted_limit as usize); // Unbinned total for mean calc.
      self.n_work_all = Array1::zeros(self.num_bins as usize);
      self.n_work_all_total = 0;

      self.combo_by_k = Array2::zeros((self.shifted_limit as usize, self.num_bins as usize));
      self.combo_by_k_total = Array1::zeros(self.shifted_limit as usize); // Unbinned total for mean calc.
      self.combo_all = Array1::zeros(self.num_bins as usize);
      self.combo_all_total = 0;
   }

   pub fn start_one(&mut self) {
      self.accum_nlogn_by_k = Array1::zeros(self.shifted_limit as usize);
      self.accum_n_work_by_k = Array1::zeros(self.shifted_limit as usize);
   }

   pub fn increment_nlogn(&mut self, shifted_level: u32) {
      assert!(shifted_level < self.shifted_limit);
      self.accum_nlogn_by_k[shifted_level as usize] += 1;
   }

   pub fn increment_n_work(&mut self, shifted_level: u32) {
      assert!(shifted_level < self.shifted_limit);
      self.accum_n_work_by_k[shifted_level as usize] += 1;
   }

   pub fn start_subsort(&mut self, _shifted_level: u32) {}

   pub fn finish_subsort(&mut self, shifted_level: u32) {
      self.subsort_count_by_k[shifted_level as usize] += 1
   }

   pub fn finish_one(&mut self) {
      for i in 0..self.shifted_limit as usize {
         let bin_nlogn: u32 = self.bin_by_small_range(self.accum_nlogn_by_k[i] as f64);
         self.nlogn_by_k[[i, bin_nlogn as usize]] += 1;
         self.nlogn_by_k_total[i] += self.accum_nlogn_by_k[i] as u64;

         let bin_n_work: u32 = self.bin_by_small_range(self.accum_n_work_by_k[i] as f64);
         self.n_work_by_k[[i, bin_n_work as usize]] += 1;
         self.n_work_by_k_total[i] += self.accum_n_work_by_k[i] as u64;

         let accum_combo_by_k = self.accum_nlogn_by_k[i] as u64 + self.accum_n_work_by_k[i] as u64;
         let bin_combo: u32 = self.bin_by_small_range(accum_combo_by_k as f64);
         self.combo_by_k[[i, bin_combo as usize]] += 1;
         self.combo_by_k_total[i] += accum_combo_by_k;
      }

      let count_nlogn_all: u32 = self.accum_nlogn_by_k.sum();
      let binned_nlogn_all: u32 = self.bin_by_large_range(count_nlogn_all as f64);
      self.nlogn_all[binned_nlogn_all as usize] += 1;
      self.nlogn_all_total += count_nlogn_all as u64;

      let count_n_work_all: u32 = self.accum_n_work_by_k.sum();
      let binned_n_work_all: u32 = self.bin_by_large_range(count_n_work_all as f64);
      self.n_work_all[binned_n_work_all as usize] += 1;
      self.n_work_all_total += count_n_work_all as u64;

      let count_combo_all: u32 = count_nlogn_all + count_n_work_all;
      let binned_combo_all: u32 = self.bin_by_large_range(count_combo_all as f64);
      self.combo_all[binned_combo_all as usize] += 1;
      self.combo_all_total += count_combo_all as u64;

      self.sample_count += 1;
   }

   pub fn finish_gather(&mut self) {}

   pub fn write_golden_info(&mut self, writer: &mut impl io::Write) {
      writeln!(writer, "nlogn_by_logn = {};", self.nlogn_by_k).unwrap();
   }

   pub fn write_summary_info(&mut self, writer: &mut dyn io::Write) {
      writeln!(writer, "count of samples = {};", self.sample_count).unwrap();
      writeln!(
         writer,
         "mean nlogn work = {} = n * {};",
         self.nlogn_all_total as f64 / self.sample_count as f64,
         self.nlogn_all_total as f64 / self.sample_count as f64 / self.sort_size as f64
      )
      .unwrap();
      writeln!(
         writer,
         "mean n-work = {} = n * {};",
         self.n_work_all_total as f64 / self.sample_count as f64,
         self.n_work_all_total as f64 / self.sample_count as f64 / self.sort_size as f64
      )
      .unwrap();
      writeln!(
         writer,
         "mean combined work = {} = n * {};",
         self.combo_all_total as f64 / self.sample_count as f64,
         self.combo_all_total as f64 / self.sample_count as f64 / self.sort_size as f64
      )
      .unwrap();
   }

   pub fn write_summary_info_by_k(&mut self, writer: &mut dyn io::Write) {
      writeln!(writer, "count of samples = {};", self.sample_count).unwrap();
      writeln!(writer, "mean n-work + nlogn work vs combined, by stage...").unwrap();
      for i in 0..self.shifted_limit as usize {
         writeln!(
            writer,
            "i = [{}]: {} + {} = n * ({} + {}) = n * {};",
            i,
            self.n_work_by_k_total[i] as f64 / self.sample_count as f64,
            self.nlogn_by_k_total[i] as f64 / self.sample_count as f64,
            self.n_work_by_k_total[i] as f64 / self.sample_count as f64 / self.sort_size as f64,
            self.nlogn_by_k_total[i] as f64 / self.sample_count as f64 / self.sort_size as f64,
            // self.combo_by_k_total[i] as f64 / self.sample_count as f64,
            self.combo_by_k_total[i] as f64 / self.sample_count as f64 / self.sort_size as f64,
         )
         .unwrap();
      }
   }

   pub fn write_summary_info_csvm(&mut self, base_name: &str, writer: &mut dyn io::Write) {
      pub const MIN_SHIFT_ROWS: usize = 16;

      writeln!(writer, "function {}_data = {}()", base_name, base_name).unwrap();
      writeln!(writer, "{}_data = [", base_name).unwrap();

      for i in 0..self.shifted_limit as usize {
         writeln!(
            writer,
            "{},{},{},{},{},{}",
            i,
            self.n_work_by_k_total[i] as f64 / self.sample_count as f64,
            self.nlogn_by_k_total[i] as f64 / self.sample_count as f64,
            self.n_work_by_k_total[i] as f64 / self.sample_count as f64 / self.sort_size as f64,
            self.nlogn_by_k_total[i] as f64 / self.sample_count as f64 / self.sort_size as f64,
            self.combo_by_k_total[i] as f64 / self.sample_count as f64 / self.sort_size as f64,
         )
         .unwrap();
      }
      for i in self.shifted_limit as usize..MIN_SHIFT_ROWS {
         writeln!(writer, "{},-1,-1,-1,-1,-1", i,).unwrap();
      }
      writeln!(
         writer,
         "{},{},{},{},{},{}",
         self.sample_count,
         self.n_work_all_total as f64 / self.sample_count as f64,
         self.nlogn_all_total as f64 / self.sample_count as f64,
         self.n_work_all_total as f64 / self.sample_count as f64 / self.sort_size as f64,
         self.nlogn_all_total as f64 / self.sample_count as f64 / self.sort_size as f64,
         self.combo_all_total as f64 / self.sample_count as f64 / self.sort_size as f64,
      )
      .unwrap();
      writeln!(writer, "];").unwrap();
      writeln!(writer, "endfunction").unwrap();
   }
}
