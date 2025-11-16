// Copyright 2025 Google LLC
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

use crate::{Curve, CurveEval, CurveTransform};
use serde::Serialize;
use serde_default::DefaultFromSerde;
use zvx_base::CubicPath;

#[cfg(test)]
use crate::{CubicHomogWrapped, F64SliceWrapped};
#[cfg(test)]
use approx::assert_abs_diff_eq;
#[cfg(test)]
use zvx_base::CubicHomog;

const fn displace_4(p: &mut [[f64; 4]; 2], d: [f64; 2]) {
   p[0][0] += d[0];
   p[0][1] += d[0];
   p[0][2] += d[0];
   p[0][3] += d[0];
   p[1][0] += d[1];
   p[1][1] += d[1];
   p[1][2] += d[1];
   p[1][3] += d[1];
}

// // As yet not used.
// #[derive(Debug, Deserialize, Serialize, DefaultFromSerde, PartialEq, Clone)]
// pub struct MidDiffCubiLinearRepr {
//    pub r: [f64; 2], // Range.
//    pub x: [f64; 4],
//    pub y: [f64; 4],
//    #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
//    pub sigma: f64,
// }

// Recreate as specified Cubic or SpecifiedCubiLinear when reworking managed curves.
//
// #[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
// pub enum SpecifiedCubic {
//    #[default]
//    Nothing,
//    FourPoint(Curve<CubicPath>),
//    MidDiff(MidDiffCubiLinearRepr),
// }

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct ManagedCubic {
   pub four_point: Curve<CubicPath>,
   // How originally specified, FourPoint or MidDiff, for plotting and diagnostics only.
   // pub specified: SpecifiedCubic,
   pub canvas_range: [f64; 4],
}

#[allow(clippy::missing_errors_doc)]
impl Curve<CubicPath> {
   #[inline]
   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::suboptimal_flops)]
   #[must_use]
   fn eval_part(b: f64, a: f64, coeffs: &[f64; 4], multiplier: f64) -> f64 {
      multiplier
         * (b * b * b * coeffs[0]
            + b * b * a * coeffs[1]
            + b * a * a * coeffs[2]
            + a * a * a * coeffs[3])
   }

   #[inline]
   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::suboptimal_flops)]
   #[must_use]
   fn eval_part_quad(b: f64, a: f64, coeffs: &[f64; 3], multiplier: f64) -> f64 {
      multiplier * (b * b * coeffs[0] + b * a * coeffs[1] + a * a * coeffs[2])
   }

   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::many_single_char_names)]
   pub fn eval_derivative_scaled(&self, t: &[f64], scale: f64) -> Vec<[f64; 2]> {
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
      for item in t {
         let a = self.sigma.0 * (*item - self.path.r[0]);
         let b = self.sigma.1 * (self.path.r[1] - *item);
         let f0 = 1.0 / (b + a);
         let w_minus_v = self.path.r[1] - self.path.r[0];
         let recip_denom = scale * f0 * f0 * f0 * f0 * w_minus_v * self.sigma.0 * self.sigma.1;
         // let recip_denom = scale * f0 * f0 / w_minus_v;
         let in_x = [
            3.0 * self.path.h.0[0][1] - 3.0 * self.path.h.0[0][0],
            3.0 * 2.0 * (self.path.h.0[0][2] - self.path.h.0[0][1]),
            3.0 * self.path.h.0[0][3] - 3.0 * self.path.h.0[0][2],
         ];
         let in_y = [
            3.0 * self.path.h.0[1][1] - 3.0 * self.path.h.0[1][0],
            3.0 * 2.0 * (self.path.h.0[1][2] - self.path.h.0[1][1]),
            3.0 * self.path.h.0[1][3] - 3.0 * self.path.h.0[1][2],
         ];
         let x = Self::eval_part_quad(b, a, &in_x, recip_denom);
         let y = Self::eval_part_quad(b, a, &in_y, recip_denom);
         ret_val.push([x, y]);
      }
      ret_val
   }
}

impl CurveEval for Curve<CubicPath> {
   fn eval_no_bilinear(&self, _t: &[f64]) -> Vec<[f64; 2]> {
      unimplemented!("It takes time.");
      // self.path.eval_no_bilinear(t)
   }

   #[allow(clippy::many_single_char_names)]
   fn eval_with_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]> {
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
      for item in t {
         let a = self.sigma.0 * (*item - self.path.r[0]);
         let b = self.sigma.1 * (self.path.r[1] - *item);
         let f0 = 1.0 / (b + a);
         let recip_denom = f0 * f0 * f0;
         let in_x = [
            self.path.h.0[0][0],
            3.0 * self.path.h.0[0][1],
            3.0 * self.path.h.0[0][2],
            self.path.h.0[0][3],
         ];
         let in_y = [
            self.path.h.0[1][0],
            3.0 * self.path.h.0[1][1],
            3.0 * self.path.h.0[1][2],
            self.path.h.0[1][3],
         ];
         let x = Self::eval_part(b, a, &in_x, recip_denom);
         let y = Self::eval_part(b, a, &in_y, recip_denom);
         ret_val.push([x, y]);
      }
      ret_val
   }

   fn characterize_endpoints(&self) -> ([[f64; 2]; 2], [[f64; 2]; 2]) {
      todo!();
   }
}

impl CurveTransform for Curve<CubicPath> {
   fn displace(&mut self, d: [f64; 2]) {
      displace_4(&mut self.path.h.0, d);
   }

   fn bilinear_transform(&mut self, sigma_ratio: (f64, f64)) {
      self.sigma.0 *= sigma_ratio.0;
      self.sigma.1 *= sigma_ratio.1;
   }

   fn raw_change_range(&mut self, new_range: [f64; 2]) {
      self.path.r = new_range;
   }

   #[allow(clippy::similar_names)]
   #[allow(clippy::suboptimal_flops)]
   fn select_range(&mut self, new_range: [f64; 2]) {
      let mut new_x = [0.0; 4];
      let mut new_y = [0.0; 4];

      // TODO: Test consistency when sigma.1 neq 1.0.
      let a_k = self.sigma.0 * (new_range[0] - self.path.r[0]);
      let b_k = self.sigma.1 * (self.path.r[1] - new_range[0]);
      let a_l = self.sigma.0 * (new_range[1] - self.path.r[0]);
      let b_l = self.sigma.1 * (self.path.r[1] - new_range[1]);
      let f0_k = 1.0 / (b_k + a_k);
      let recip_denom_k = f0_k * f0_k * f0_k;
      let f0_l = 1.0 / (b_l + a_l);
      let recip_denom_l = f0_l * f0_l * f0_l;
      let in_x = [
         self.path.h.0[0][0],
         3.0 * self.path.h.0[0][1],
         3.0 * self.path.h.0[0][2],
         self.path.h.0[0][3],
      ];
      let in_y = [
         self.path.h.0[1][0],
         3.0 * self.path.h.0[1][1],
         3.0 * self.path.h.0[1][2],
         self.path.h.0[1][3],
      ];
      new_x[0] = Self::eval_part(b_k, a_k, &in_x, recip_denom_k);
      new_y[0] = Self::eval_part(b_k, a_k, &in_y, recip_denom_k);
      new_x[3] = Self::eval_part(b_l, a_l, &in_x, recip_denom_l);
      new_y[3] = Self::eval_part(b_l, a_l, &in_y, recip_denom_l);
      let kl_numerator_k = self.sigma.0 * self.path.r[1] * (new_range[0] - self.path.r[0])
         + self.sigma.1 * self.path.r[0] * (self.path.r[1] - new_range[0]);
      let kl_numerator_l = self.sigma.0 * self.path.r[1] * (new_range[1] - self.path.r[0])
         + self.sigma.1 * self.path.r[0] * (self.path.r[1] - new_range[1]);
      // This is [k, l] bilinearly transformed.
      let selected_range_bilineared = kl_numerator_l / (a_l + b_l) - kl_numerator_k / (a_k + b_k);
      let fudge_k = selected_range_bilineared / (self.path.r[1] - self.path.r[0]);
      let fudge_l = selected_range_bilineared / (self.path.r[1] - self.path.r[0]);
      // assert_eq!(1.0 / f0_k, 0.0);
      let dx_1 = fudge_k
         * f0_k
         * f0_k
         * (b_k * b_k * (in_x[1] / 3.0 - in_x[0])
            + 2.0 * b_k * a_k * (in_x[2] / 3.0 - in_x[1] / 3.0)
            + a_k * a_k * (in_x[3] - in_x[2] / 3.0));
      new_x[1] = new_x[0] + dx_1;
      let dy_1 = fudge_k
         * f0_k
         * f0_k
         * (b_k * b_k * (in_y[1] / 3.0 - in_y[0])
            + 2.0 * b_k * a_k * (in_y[2] / 3.0 - in_y[1] / 3.0)
            + a_k * a_k * (in_y[3] - in_y[2] / 3.0));
      new_y[1] = new_y[0] + dy_1;
      let dx_1 = fudge_l
         * f0_l
         * f0_l
         * (b_l * b_l * (in_x[1] / 3.0 - in_x[0])
            + 2.0 * b_l * a_l * (in_x[2] / 3.0 - in_x[1] / 3.0)
            + a_l * a_l * (in_x[3] - in_x[2] / 3.0));
      new_x[2] = new_x[3] - dx_1;
      let dy_1 = fudge_l
         * f0_l
         * f0_l
         * (b_l * b_l * (in_y[1] / 3.0 - in_y[0])
            + 2.0 * b_l * a_l * (in_y[2] / 3.0 - in_y[1] / 3.0)
            + a_l * a_l * (in_y[3] - in_y[2] / 3.0));
      new_y[2] = new_y[3] - dy_1;

      self.sigma.0 = a_l + b_l;
      self.sigma.1 = a_k + b_k;
      self.path.h.0 = [new_x, new_y];
      // [[new_x[0], new_y[0]], [new_x[1], new_y[1]], [new_x[2], new_y[2]], [new_x[3], new_y[3]]];
      self.path.r = new_range;
   }
}

#[cfg(test)]
#[allow(clippy::similar_names)]
#[allow(clippy::suboptimal_flops)]
// Applied to weighted.
fn select_range_reference(curve: &mut Curve<CubicPath>, new_range: [f64; 2]) {
   let mut new_x = [0.0; 4];
   let mut new_y = [0.0; 4];

   let a_k = curve.sigma.0 * (new_range[0] - curve.path.r[0]);
   let b_k = curve.sigma.1 * (curve.path.r[1] - new_range[0]);
   let a_l = curve.sigma.0 * (new_range[1] - curve.path.r[0]);
   let b_l = curve.sigma.1 * (curve.path.r[1] - new_range[1]);
   let f0_k = 1.0 / (b_k + a_k);
   let recip_denom_k = f0_k * f0_k * f0_k;
   let f0_l = 1.0 / (b_l + a_l);
   let recip_denom_l = f0_l * f0_l * f0_l;
   let in_x =
      [curve.path.h.0[0][0], curve.path.h.0[0][1], curve.path.h.0[0][2], curve.path.h.0[0][3]];
   let in_y =
      [curve.path.h.0[1][0], curve.path.h.0[1][1], curve.path.h.0[1][2], curve.path.h.0[1][3]];
   new_x[0] = Curve::eval_part(b_k, a_k, &in_x, recip_denom_k);
   new_y[0] = Curve::eval_part(b_k, a_k, &in_y, recip_denom_k);
   new_x[3] = Curve::eval_part(b_l, a_l, &in_x, recip_denom_l);
   new_y[3] = Curve::eval_part(b_l, a_l, &in_y, recip_denom_l);
   let kl_numerator_k = curve.sigma.0 * curve.path.r[1] * (new_range[0] - curve.path.r[0])
      + curve.sigma.1 * curve.path.r[0] * (curve.path.r[1] - new_range[0]);
   let kl_numerator_l = curve.sigma.0 * curve.path.r[1] * (new_range[1] - curve.path.r[0])
      + curve.sigma.1 * curve.path.r[0] * (curve.path.r[1] - new_range[1]);
   // This is [k, l] bilinearly transformed.
   let selected_range_bilineared = kl_numerator_l / (a_l + b_l) - kl_numerator_k / (a_k + b_k);
   let fudge_k = selected_range_bilineared / (curve.path.r[1] - curve.path.r[0]);
   let fudge_l = selected_range_bilineared / (curve.path.r[1] - curve.path.r[0]);
   let dx_1 = fudge_k
      * f0_k
      * f0_k
      * (b_k * b_k * (in_x[1] / 3.0 - in_x[0])
         + 2.0 * b_k * a_k * (in_x[2] / 3.0 - in_x[1] / 3.0)
         + a_k * a_k * (in_x[3] - in_x[2] / 3.0));
   new_x[1] = 3.0 * (new_x[0] + dx_1);
   let dy_1 = fudge_k
      * f0_k
      * f0_k
      * (b_k * b_k * (in_y[1] / 3.0 - in_y[0])
         + 2.0 * b_k * a_k * (in_y[2] / 3.0 - in_y[1] / 3.0)
         + a_k * a_k * (in_y[3] - in_y[2] / 3.0));
   new_y[1] = 3.0 * (new_y[0] + dy_1);
   let dx_1 = fudge_l
      * f0_l
      * f0_l
      * (b_l * b_l * (in_x[1] / 3.0 - in_x[0])
         + 2.0 * b_l * a_l * (in_x[2] / 3.0 - in_x[1] / 3.0)
         + a_l * a_l * (in_x[3] - in_x[2] / 3.0));
   new_x[2] = 3.0 * (new_x[3] - dx_1);
   let dy_1 = fudge_l
      * f0_l
      * f0_l
      * (b_l * b_l * (in_y[1] / 3.0 - in_y[0])
         + 2.0 * b_l * a_l * (in_y[2] / 3.0 - in_y[1] / 3.0)
         + a_l * a_l * (in_y[3] - in_y[2] / 3.0));
   new_y[2] = 3.0 * (new_y[3] - dy_1);

   curve.sigma.0 = a_l + b_l;
   curve.sigma.1 = a_k + b_k;
   curve.path.h.0 = [new_x, new_y];
   curve.path.r = new_range;
}

#[test]
#[allow(clippy::unreadable_literal)]
fn create_from_weighted_test() {
   let mut weighted = Curve {
      path: CubicPath {
         r: [-4.5, 13.5],
         h: CubicHomog([[4.0, 3.0 * 3.5, 3.0 * 4.5, 3.0], [-1.5, 3.0 * -2.0, 3.0 * 1.5, 2.0]]),
      },
      sigma: (3.0, 1.0),
   };

   let new_range = [1.5, 10.5];

   select_range_reference(&mut weighted, new_range);

   let reference_weighted = Curve {
      path: CubicPath {
         r: [1.5, 10.5],
         h: CubicHomog([
            [3.856, 3.0 * 3.80875, 3.0 * 3.658984375, 3.2529296875],
            [0.408, 3.0 * 1.00875, 3.0 * 1.58671875, 1.872802734375],
         ]),
      },
      sigma: (48.0, 30.0),
   };

   assert_abs_diff_eq!(
      &CubicHomogWrapped::from(&weighted.path.h),
      &CubicHomogWrapped::from(&reference_weighted.path.h),
      epsilon = 1.0e-5
   );
   assert_abs_diff_eq!(
      &F64SliceWrapped::from(&weighted.path.r),
      &F64SliceWrapped::from(&reference_weighted.path.r),
      epsilon = 1.0e-5
   );
   assert_abs_diff_eq!(&weighted.sigma.0, &reference_weighted.sigma.0, epsilon = 1.0e-5);
   assert_abs_diff_eq!(&weighted.sigma.1, &reference_weighted.sigma.1, epsilon = 1.0e-5);
}

#[allow(clippy::missing_panics_doc)]
impl ManagedCubic {
   #[must_use]
   pub fn create_from_control_points(
      control_points: &Curve<CubicPath>,
      canvas_range: [f64; 4],
   ) -> Self {
      Self { four_point: control_points.clone(), canvas_range }
   }

   #[allow(clippy::missing_errors_doc)]
   pub fn get_four_point(&self) -> Result<Curve<CubicPath>, &'static str> {
      Ok(self.four_point.clone())
   }

   pub fn displace(&mut self, d: [f64; 2]) {
      self.four_point.displace(d);
   }

   pub fn bilinear_transform(&mut self, sigma_ratio: (f64, f64)) {
      self.four_point.bilinear_transform(sigma_ratio);
   }

   pub fn raw_change_range(&mut self, new_range: [f64; 2]) {
      self.four_point.raw_change_range(new_range);
   }

   pub fn select_range(&mut self, new_range: [f64; 2]) {
      self.four_point.select_range(new_range);
   }
}
