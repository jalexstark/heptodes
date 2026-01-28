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

#[cfg(test)]
mod tests;
use crate::{Curve, CurveEval, CurveTransform};
use serde::Serialize;
use serde_default::DefaultFromSerde;
use zvx_base::matrix::CMat;
use zvx_base::CubicHomog;
use zvx_base::{CubicFourPoint, CubicPath, CurveCubicMatrix};

const fn displace_4(p: &mut [[f64; 4]; 2], d: [f64; 2]) {
   p[0][0] += d[0];
   p[0][1] += 3.0 * d[0];
   p[0][2] += 3.0 * d[0];
   p[0][3] += d[0];
   p[1][0] += d[1];
   p[1][1] += 3.0 * d[1];
   p[1][2] += 3.0 * d[1];
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

#[inline]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::suboptimal_flops)]
#[must_use]
fn cubic_eval_part(b: f64, a: f64, coeffs: &[f64; 4], multiplier: f64) -> f64 {
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
fn cubic_eval_part_quad(b: f64, a: f64, coeffs: &[f64; 3], multiplier: f64) -> f64 {
   multiplier * (b * b * coeffs[0] + b * a * coeffs[1] + a * a * coeffs[2])
}

impl CurveEval for CubicPath {
   #[allow(clippy::many_single_char_names)]
   fn eval_with_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]> {
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
      for item in t {
         let a = self.sigma.0 * (*item - self.r[0]);
         let b = self.sigma.1 * (self.r[1] - *item);
         let f0 = 1.0 / (b + a);
         let recip_denom = f0 * f0 * f0;
         let x = cubic_eval_part(b, a, &self.h.0[0], recip_denom);
         let y = cubic_eval_part(b, a, &self.h.0[1], recip_denom);
         ret_val.push([x, y]);
      }
      ret_val
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::many_single_char_names)]
   fn eval_derivative_scaled(&self, t: &[f64], scale: f64) -> Vec<[f64; 2]> {
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
      for item in t {
         let a = self.sigma.0 * (*item - self.r[0]);
         let b = self.sigma.1 * (self.r[1] - *item);
         let f0 = 1.0 / (b + a);
         let w_minus_v = self.r[1] - self.r[0];
         let recip_denom = scale * f0 * f0 * f0 * f0 * w_minus_v * self.sigma.0 * self.sigma.1;
         let in_x = [
            self.h.0[0][1] - 3.0 * self.h.0[0][0],
            2.0 * (self.h.0[0][2] - self.h.0[0][1]),
            3.0 * self.h.0[0][3] - self.h.0[0][2],
         ];
         let in_y = [
            self.h.0[1][1] - 3.0 * self.h.0[1][0],
            2.0 * (self.h.0[1][2] - self.h.0[1][1]),
            3.0 * self.h.0[1][3] - self.h.0[1][2],
         ];
         let x = cubic_eval_part_quad(b, a, &in_x, recip_denom);
         let y = cubic_eval_part_quad(b, a, &in_y, recip_denom);
         ret_val.push([x, y]);
      }
      ret_val
   }

   #[allow(clippy::suboptimal_flops)]
   fn characterize_endpoints(&self) -> ([[f64; 2]; 2], [[f64; 2]; 2]) {
      let (sigma_a, sigma_b) = self.sigma;
      let factor_up = sigma_a / sigma_b;
      let factor_down = sigma_b / sigma_a;
      (
         [[self.h.0[0][0], self.h.0[1][0]], [self.h.0[0][3], self.h.0[1][3]]],
         [
            [
               factor_up * (self.h.0[0][1] - 3.0 * self.h.0[0][0]),
               factor_up * (self.h.0[1][1] - 3.0 * self.h.0[1][0]),
            ],
            [
               factor_down * (3.0 * self.h.0[0][3] - self.h.0[0][2]),
               factor_down * (3.0 * self.h.0[1][3] - self.h.0[1][2]),
            ],
         ],
      )
   }
}

impl CurveTransform for CubicPath {
   fn displace(&mut self, d: [f64; 2]) {
      displace_4(&mut self.h.0, d);
   }

   fn bilinear_transform(&mut self, sigma_ratio: (f64, f64)) {
      self.sigma.0 *= sigma_ratio.0;
      self.sigma.1 *= sigma_ratio.1;
   }

   fn raw_change_range(&mut self, new_range: [f64; 2]) {
      self.r = new_range;
   }

   #[allow(clippy::similar_names)]
   #[allow(clippy::suboptimal_flops)]
   fn select_range(&mut self, new_range: [f64; 2]) {
      // Does not check that range is valid (no asymptote in bilinear within range).
      let a_k = self.sigma.0 * (new_range[0] - self.r[0]);
      let b_k = self.sigma.1 * (self.r[1] - new_range[0]);
      let a_l = self.sigma.0 * (new_range[1] - self.r[0]);
      let b_l = self.sigma.1 * (self.r[1] - new_range[1]);

      let alpha = b_k / (a_k + b_k);
      let beta = 1.0 - alpha;
      let gamma = b_l / (a_l + b_l);
      let delta = 1.0 - gamma;

      let selection_transform: CMat = [
         [
            alpha * alpha * alpha,
            3.0 * alpha * alpha * gamma,
            3.0 * alpha * gamma * gamma,
            gamma * gamma * gamma,
         ],
         [
            alpha * alpha * beta,
            2.0 * alpha * beta * gamma + alpha * alpha * delta,
            2.0 * alpha * gamma * delta + beta * gamma * gamma,
            gamma * gamma * delta,
         ],
         [
            alpha * beta * beta,
            2.0 * alpha * beta * delta + beta * beta * gamma,
            2.0 * beta * gamma * delta + alpha * delta * delta,
            gamma * delta * delta,
         ],
         [
            beta * beta * beta,
            3.0 * beta * beta * delta,
            3.0 * beta * delta * delta,
            delta * delta * delta,
         ],
      ];

      let selected_cubic: CubicHomog = self.h.apply_c_mat(&selection_transform);

      // It would be good to power-2 normalize.
      self.sigma = (a_l + b_l, a_k + b_k);
      self.h = selected_cubic;
      self.r = new_range;
   }
}

#[allow(clippy::missing_panics_doc)]
impl ManagedCubic {
   #[must_use]
   pub fn create_from_control_points(
      control_points: &CubicFourPoint,
      canvas_range: [f64; 4],
   ) -> Self {
      Self {
         four_point: Curve {
            path: CubicPath {
               r: control_points.r,
               h: CubicHomog([
                  [
                     control_points.h.0[0][0],
                     3.0 * control_points.h.0[0][1],
                     3.0 * control_points.h.0[0][2],
                     control_points.h.0[0][3],
                  ],
                  [
                     control_points.h.0[1][0],
                     3.0 * control_points.h.0[1][1],
                     3.0 * control_points.h.0[1][2],
                     control_points.h.0[1][3],
                  ],
               ]),
               sigma: control_points.sigma,
            },
         },
         canvas_range,
      }
   }

   pub fn displace(&mut self, d: [f64; 2]) {
      self.four_point.path.displace(d);
   }

   pub fn bilinear_transform(&mut self, sigma_ratio: (f64, f64)) {
      self.four_point.path.bilinear_transform(sigma_ratio);
   }

   pub fn select_range(&mut self, new_range: [f64; 2]) {
      self.four_point.path.select_range(new_range);
   }
}
