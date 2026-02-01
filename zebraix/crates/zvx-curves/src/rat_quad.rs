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

pub mod managed;
#[cfg(test)]
mod tests;

use crate::{CurveEval, CurveTransform, ZebraixAngle};
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use zvx_base::{
   default_unit_ratio, is_default, is_default_unit_ratio, q_reduce, rat_quad_expand_weighted,
   rat_quad_rq_eval, CurveMatrix, QMat, RatQuadHomog, RatQuadHomogPower, RatQuadHomogWeighted,
};

// Update by adding 3x1 vector multiplied by scalar.
const fn mul_add_3_1_1(p: &mut [f64; 3], v: &[f64; 3], m: f64) {
   p[0] += v[0] * m;
   p[1] += v[1] * m;
   p[2] += v[2] * m;
}

#[inline]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::suboptimal_flops)]
#[must_use]
fn eval_part_quad(b: f64, a: f64, coeffs: &[f64; 3]) -> f64 {
   b * b * coeffs[0] + b * a * coeffs[1] + a * a * coeffs[2]
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq, Clone)]
pub struct FourPointRatQuad {
   pub r: [f64; 2], // Range.
   pub p: [[f64; 2]; 4],
   #[serde(skip_serializing_if = "is_default_unit_ratio", default = "default_unit_ratio")]
   pub sigma: (f64, f64),
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct ThreePointAngleRepr {
   pub r: [f64; 2], // Range.
   pub p: [[f64; 2]; 3],
   #[serde(skip_serializing_if = "is_default")]
   pub angle: ZebraixAngle,
   #[serde(skip_serializing_if = "is_default_unit_ratio", default = "default_unit_ratio")]
   pub sigma: (f64, f64),
}

#[derive(Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub enum SpecifiedRatQuad {
   #[default]
   None, // For, say, polynomial directly specified.
   FourPoint,
   ThreePointAngle,
}

// New trait, applied to RatQuadHomogWeighted and Cubilinear:
//
// displace
// bilinear_transform
// overwrite_bilinear
// overwrite_range
// recut_range
// eval (with_bilinear)
// eval_derivative_scaled
// eval_endpoints
//
// Extras for RQC trait:
// Collapse bilinear
//
// Not exported, or maybe for extras trait
// eval no bilinear (do we even want?)

// Internal bilinear transform.  Really only for testing.
#[must_use]
#[allow(clippy::many_single_char_names)]
pub fn rq_weighted_collapse_bilinear(weighted: &RatQuadHomogWeighted) -> RatQuadHomogWeighted {
   let (p, q) = weighted.sigma;
   let (e, f, g) = (q * q, p * q, p * p);

   let wh = weighted.h.0;
   let h = RatQuadHomog([
      [e * wh[0][0], f * wh[0][1], g * wh[0][2]],
      [e * wh[1][0], f * wh[1][1], g * wh[1][2]],
      [e * wh[2][0], f * wh[2][1], g * wh[2][2]],
   ]);

   RatQuadHomogWeighted { r: weighted.r, h, sigma: (1.0, 1.0) }
}

impl CurveEval for RatQuadHomogWeighted {
   fn eval_with_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]> {
      q_reduce(&rat_quad_rq_eval(&self.h.0, &rat_quad_expand_weighted(t, self.sigma, self.r)))
   }

   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::suboptimal_flops)]
   fn eval_derivative_scaled(&self, t: &[f64], scale: f64) -> Vec<[f64; 2]> {
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
      for item in t {
         let p = self.sigma.0 * (*item - self.r[0]);
         let q = self.sigma.1 * (self.r[1] - *item);
         let b = &self.h.0[0];
         let c = &self.h.0[1];
         let a = &self.h.0[2];
         let expansion_b = [
            a[0] * b[1] - a[1] * b[0],
            2.0 * (a[0] * b[2] - a[2] * b[0]),
            a[1] * b[2] - a[2] * b[1],
         ];
         let expansion_c = [
            a[0] * c[1] - a[1] * c[0],
            2.0 * (a[0] * c[2] - a[2] * c[0]),
            a[1] * c[2] - a[2] * c[1],
         ];
         let rb = eval_part_quad(q, p, &expansion_b);
         let rc = eval_part_quad(q, p, &expansion_c);
         let ra = eval_part_quad(q, p, a);
         let w_minus_v = self.r[1] - self.r[0];
         let div_factor = self.sigma.0 * self.sigma.1 * w_minus_v * scale / ra / ra;

         ret_val.push([rb * div_factor, rc * div_factor]);
      }
      ret_val
   }

   #[inline]
   #[allow(clippy::suboptimal_flops)]
   fn characterize_endpoints(&self) -> ([[f64; 2]; 2], [[f64; 2]; 2]) {
      let b = &self.h.0[0];
      let c = &self.h.0[1];
      let a = &self.h.0[2];
      let (sigma_a, sigma_b) = self.sigma;
      let factor_up = sigma_a / sigma_b / a[0] / a[0];
      let factor_down = sigma_b / sigma_a / a[2] / a[2];
      (
         [[b[0] / a[0], c[0] / a[0]], [b[2] / a[2], c[2] / a[2]]],
         [
            [factor_up * (a[0] * b[1] - a[1] * b[0]), factor_up * (a[0] * c[1] - a[1] * c[0])],
            [factor_down * (a[1] * b[2] - a[2] * b[1]), factor_down * (a[1] * c[2] - a[2] * c[1])],
         ],
      )
   }
}

impl CurveTransform for RatQuadHomogWeighted {
   // Not yet tested.
   fn displace(&mut self, d: [f64; 2]) {
      let (b, remainder) = self.h.0.split_at_mut(1);
      let (c, a) = remainder.split_at_mut(1);
      mul_add_3_1_1(&mut b[0], &a[0], d[0]);
      mul_add_3_1_1(&mut c[0], &a[0], d[1]);
   }

   // Remove after cubic reworked.
   fn bilinear_transform(&mut self, sigma_ratio: (f64, f64)) {
      self.sigma.0 *= sigma_ratio.0;
      self.sigma.1 *= sigma_ratio.1;
   }

   fn raw_change_range(&mut self, new_range: [f64; 2]) {
      self.r = new_range;
   }

   // when revising, change to return Self.
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

      // This appears slightly less accurate than the reference, which converts to power form
      // and doing a simple cut there.  Differences may be more substantial when recutting
      // elliptical arcs to wider ranges.
      let selection_transform: QMat = [
         [alpha * alpha, 2.0 * alpha * gamma, gamma * gamma],
         [alpha * beta, alpha * delta + beta * gamma, gamma * delta],
         [beta * beta, 2.0 * beta * delta, delta * delta],
      ];

      let selected_rq: RatQuadHomog = self.h.apply_q_mat(&selection_transform);

      // It would be good to power-2 normalize.
      self.sigma = (a_l + b_l, a_k + b_k);
      self.h = selected_rq;
      self.r = new_range;
   }
}

// Internal bilinear transform.
#[must_use]
#[allow(clippy::suboptimal_flops)]
#[allow(clippy::many_single_char_names)]
fn rq_power_collapse_bilinear_unranged(
   power: &RatQuadHomogPower,
   w: f64,
   x: f64,
   y: f64,
   z: f64,
) -> RatQuadHomogPower {
   // let norm = 1.0 / ((w * w + x * x) * (y * y + z * z)).sqrt().sqrt();
   // w *= norm;
   // x *= norm;
   // y *= norm;
   // z *= norm;

   let input_path = &power;

   let tran_q_mat: QMat =
      [[z * z, 2.0 * y * z, y * y], [x * z, x * y + w * z, w * y], [x * x, 2.0 * w * x, w * w]];
   let output_homog = input_path.h.apply_q_mat(&tran_q_mat);

   let mut homog_path = RatQuadHomogPower { h: output_homog, r: power.r, sigma: (1.0, 1.0) };
   homog_path.h.normalize();

   homog_path
}

// Internal bilinear transform.
#[must_use]
#[allow(clippy::suboptimal_flops)]
#[allow(clippy::many_single_char_names)]
pub fn rq_power_collapse_bilinear(power: &RatQuadHomogPower) -> RatQuadHomogPower {
   let sigma_n = power.sigma.0;
   let sigma_d = power.sigma.1;
   let p = -power.r[0];
   let q = power.r[1];

   rq_power_collapse_bilinear_unranged(
      power,
      sigma_n * q + sigma_d * p,
      (sigma_n - sigma_d) * p * q,
      sigma_n - sigma_d,
      sigma_d * q + sigma_n * p,
   )
}

// If this is no longer needed for converting curves to subclasses, then move to test code.
#[inline]
#[allow(clippy::suboptimal_flops)]
#[must_use]
pub fn power_characterize_endpoints(power: &RatQuadHomogPower) -> ([[f64; 2]; 2], [[f64; 2]; 2]) {
   let mut x = [0.0; 4];
   let mut y = [0.0; 4];
   let (sigma_a, sigma_b) = power.sigma;
   let factor_up = sigma_a / sigma_b;
   let factor_down = sigma_b / sigma_a;

   let speed_scale = power.r[1] - power.r[0];
   for (outer, inner, t) in [(0, 1, power.r[0]), (3, 2, power.r[1])] {
      let recip_a = 1.0 / ((power.h.0[2][2] * t + power.h.0[2][1]) * t + power.h.0[2][0]);
      let b = (power.h.0[0][2] * t + power.h.0[0][1]) * t + power.h.0[0][0];
      let c = (power.h.0[1][2] * t + power.h.0[1][1]) * t + power.h.0[1][0];
      let da = (power.h.0[2][2] * 2.0 * t + power.h.0[2][1]) * speed_scale;
      let db = (power.h.0[0][2] * 2.0 * t + power.h.0[0][1]) * speed_scale;
      let dc = (power.h.0[1][2] * 2.0 * t + power.h.0[1][1]) * speed_scale;
      x[outer] = b * recip_a;
      y[outer] = c * recip_a;
      x[inner] = (-b * da).mul_add(recip_a, db) * recip_a;
      y[inner] = ((-c * da) * recip_a + dc) * recip_a;
   }
   (
      [[x[0], y[0]], [x[3], y[3]]],
      [[factor_up * x[1], factor_up * y[1]], [factor_down * x[2], factor_down * y[2]]],
   )
}
