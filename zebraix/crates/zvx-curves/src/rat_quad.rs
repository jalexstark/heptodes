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
use crate::threes::RatQuadOoeSubclassed;
use crate::{Curve, CurveEval, CurveTransform, ZebraixAngle};
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use zvx_base::{
   default_unit_ratio, is_default, is_default_unit_ratio, q_reduce, rat_quad_expand_weighted,
   rat_quad_rq_eval, CubicHomog, CubicPath, CurveMatrix, HyperbolicPath, QMat, RatQuadHomog,
   RatQuadHomogPower, RatQuadHomogWeighted,
};

// Done: Checklist, Eval end points.
// Done: Checklist, Eval end point derivatives.
// Done: Checklist, Eval derivatives.
// Done: Checklist, Euler diff test derivatives.
// Done: Checklist, Eval.
// TODO: Checklist, Test collapse bilinear with eval (not cubic).
// TODO: Checklist, Eval without bilinear (internal).
// TODO: Checklist, Four-point specification.
// TODO: Checklist, Three-point specification.
// TODO: Checklist, Range cut / select.
// TODO: Checklist, Test range cut, perhaps via bilinear collapse.
// TODO: Checklist, Test solvable split for three-point.
// TODO: Checklist, Test solvable split for four-point.
// Done: Checklist, Test transformation of form.
// TODO: Checklist, Test direct modify bilinear and range.
//
// Next: Eval test.
// Next: Collapse bilinear.
// Next: Look for further removal of power CurveEval and TEval.
// TODO: Fill out CurveTransform trait, both RQC and cubic.

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

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct RegularizedRatQuadPath {
   pub range_bound: f64, // Range is [-range_bound, range_bound].
   pub a_0: f64,         // Denominator, as a[2] * t^2 + a[1] * t... .
   pub a_2: f64,
   pub b: [f64; 3], // Numerator for x component.
   pub c: [f64; 3], // Numerator for y component.
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
fn rq_power_collapse_bilinear(power: &RatQuadHomogPower) -> RatQuadHomogPower {
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

impl Curve<RatQuadHomogPower> {
   // Internal bilinear transform.
   #[must_use]
   fn rq_overwrite_bilinear(&self, sigma_ratio: (f64, f64)) -> Self {
      let mut retval = self.clone();
      retval.path.sigma.0 = sigma_ratio.0;
      retval.path.sigma.1 = sigma_ratio.1;
      retval
   }

   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::many_single_char_names)]
   fn figure_symmetric_range_rat_quad(&self) -> Self {
      // Replace t with t - d.
      let d = 0.5 * (self.path.r[0] + self.path.r[1]);
      let r_half = 0.5 * (self.path.r[1] - self.path.r[0]);

      let a = [
         d * (d * self.path.h.0[2][2] + self.path.h.0[2][1]) + self.path.h.0[2][0],
         2.0 * d * self.path.h.0[2][2] + self.path.h.0[2][1],
         self.path.h.0[2][2],
      ];
      let b = [
         d * (d * self.path.h.0[0][2] + self.path.h.0[0][1]) + self.path.h.0[0][0],
         2.0 * d * self.path.h.0[0][2] + self.path.h.0[0][1],
         self.path.h.0[0][2],
      ];
      let c = [
         d * (d * self.path.h.0[1][2] + self.path.h.0[1][1]) + self.path.h.0[1][0],
         2.0 * d * self.path.h.0[1][2] + self.path.h.0[1][1],
         self.path.h.0[1][2],
      ];

      let r = [-r_half, r_half];
      Self { path: RatQuadHomogPower { r, h: RatQuadHomog([b, c, a]), sigma: self.path.sigma } }
   }
}

// If this is no longer needed for converting curves to subclasses, then move to test code.
#[inline]
#[allow(clippy::suboptimal_flops)]
fn power_characterize_endpoints(power: &RatQuadHomogPower) -> ([[f64; 2]; 2], [[f64; 2]; 2]) {
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

#[allow(clippy::suboptimal_flops)]
impl From<&RegularizedRatQuadPath> for RatQuadHomogPower {
   fn from(regular: &RegularizedRatQuadPath) -> Self {
      Self {
         r: [-regular.range_bound, regular.range_bound],
         h: RatQuadHomog([regular.b, regular.c, [regular.a_0, 0.0, regular.a_2]]),
         sigma: regular.sigma,
      }
   }
}

impl Curve<RegularizedRatQuadPath> {
   #[allow(clippy::suboptimal_flops)]
   #[must_use]
   pub fn convert_to_parabolic(&self) -> Curve<CubicPath> {
      let (ends, deltas) = power_characterize_endpoints(&RatQuadHomogPower::from(&self.path));
      let f = 3.0;
      let four_c = [
         [ends[0][0], f * ends[0][0] + deltas[0][0], f * ends[1][0] - deltas[1][0], ends[1][0]],
         [ends[0][1], f * ends[0][1] + deltas[0][1], f * ends[1][1] - deltas[1][1], ends[1][1]],
      ];

      Curve::<CubicPath> {
         path: CubicPath {
            r: [-self.path.range_bound, self.path.range_bound],
            h: CubicHomog(four_c),
            sigma: self.path.sigma,
         },
      }
   }

   // At present there is no proper testing of s. Manual inspection verifies that negating all
   // a, b and c in the input leaves the output invariant.
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_panics_doc)]
   #[must_use]
   pub fn convert_to_hyperbolic(&self) -> Curve<HyperbolicPath> {
      let s = self.path.a_0.signum();

      let lambda = (s * self.path.a_0).sqrt();
      assert!(-s * self.path.a_2 > 0.0);
      let mu = (-s * self.path.a_2).sqrt();
      let r_lambda = 1.0 / lambda;
      let r_mu = 1.0 / mu;
      let r_a_2 = 1.0 / self.path.a_2;

      let offset = [self.path.b[2] * r_a_2, self.path.c[2] * r_a_2];

      let f = 0.5 * s;
      let plus_partial = [
         f * (self.path.b[0] * r_lambda
            + (-self.path.b[1] + lambda * r_mu * self.path.b[2]) * r_mu),
         f * (self.path.c[0] * r_lambda
            + (-self.path.c[1] + lambda * r_mu * self.path.c[2]) * r_mu),
      ];
      let minus_partial = [
         f * (self.path.b[0] * r_lambda + (self.path.b[1] + lambda * r_mu * self.path.b[2]) * r_mu),
         f * (self.path.c[0] * r_lambda + (self.path.c[1] + lambda * r_mu * self.path.c[2]) * r_mu),
      ];

      Curve::<HyperbolicPath> {
         path: HyperbolicPath {
            range: (-self.path.range_bound, self.path.range_bound),
            lambda,
            mu,
            offset,
            plus_partial,
            minus_partial,
            sigma: self.path.sigma,
         },
      }
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::unnecessary_wraps)]
   #[allow(clippy::missing_errors_doc)]
   fn create_by_raising_to_regularized_symmetric(
      rat_poly_extracted: &Curve<RatQuadHomogPower>,
   ) -> Result<Self, &'static str> {
      let rat_poly = rat_poly_extracted.figure_symmetric_range_rat_quad();

      let r_both = rat_poly.path.r[1];
      let a_s = rat_poly.path.h.0[2][2] * r_both * r_both + rat_poly.path.h.0[2][0];
      // let a_d = rat_poly.path.h.0[2][2] * r * r - rat_poly.path.h.0[2][0];
      let combo_s = a_s + rat_poly.path.h.0[2][1] * r_both;
      let combo_d = a_s - rat_poly.path.h.0[2][1] * r_both;

      let sigma_ratio = (combo_d.abs().sqrt(), combo_s.abs().sqrt());

      let intermediate_rat_poly = rat_poly.rq_overwrite_bilinear(sigma_ratio);
      let scratchy_rat_poly = rq_power_collapse_bilinear(&intermediate_rat_poly.path);

      let check_poly = scratchy_rat_poly;
      assert!(check_poly.h.0[2][1].abs() < 0.001);
      Ok(Self {
         path: RegularizedRatQuadPath {
            range_bound: check_poly.r[1],
            a_0: check_poly.h.0[2][0],
            a_2: check_poly.h.0[2][2],
            b: check_poly.h.0[0],
            c: check_poly.h.0[1],
            sigma: check_poly.sigma,
         },
      })
   }
}

#[allow(clippy::suboptimal_flops)]
impl RatQuadOoeSubclassed {
   fn create_elliptical_or_parabolic(
      poly_curve: &Curve<RatQuadHomogPower>,
      tolerance: f64,
   ) -> Result<Self, &'static str> {
      let reg_curve =
         Curve::<RegularizedRatQuadPath>::create_by_raising_to_regularized_symmetric(poly_curve)?;

      // fn create_from_regularized(reg_curve: &Curve<RegularizedRatQuadPath>, tolerance: f64) -> Self {
      let mut rat_poly = reg_curve.clone();
      let orig_rat_poly = reg_curve;

      let r = rat_poly.path.range_bound;
      if (rat_poly.path.a_2.abs() * r * r) < (rat_poly.path.a_0.abs() * tolerance) {
         Ok(Self::Parabolic(orig_rat_poly.convert_to_parabolic()))
      } else {
         // Rust clippy effectively makes this check impossible.
         // assert_eq!(rat_poly.path.a_2.signum(), rat_poly.path.a_0.signum());

         // TODO: Better handle cases where s or f might be infinite.
         let s = 1.0 / rat_poly.path.a_0;
         let f = 1.0 / rat_poly.path.a_2;
         rat_poly.path.a_0 = 1.0;
         rat_poly.path.a_2 *= s;

         {
            let offset = 0.5 * (s * rat_poly.path.b[0] + f * rat_poly.path.b[2]);
            let even = 0.5 * (s * rat_poly.path.b[0] - f * rat_poly.path.b[2]);
            let odd = rat_poly.path.b[1] * s;
            rat_poly.path.b = [offset, odd, even];
         }
         {
            let offset = 0.5 * (s * rat_poly.path.c[0] + f * rat_poly.path.c[2]);
            let even = 0.5 * (s * rat_poly.path.c[0] - f * rat_poly.path.c[2]);
            let odd = rat_poly.path.c[1] * s;
            rat_poly.path.c = [offset, odd, even];
         }

         let sss = 1.0 / rat_poly.path.a_2.sqrt();
         let (sx, sy) = (0.5 * sss * rat_poly.path.b[1], 0.5 * sss * rat_poly.path.c[1]);
         let (cx, cy) = (rat_poly.path.b[2], rat_poly.path.c[2]);
         let determinant = sx * cy - cx * sy;
         let frobenius_squared = sx * sx + sy * sy + cx * cx + cy * cy;
         if determinant.abs() < (frobenius_squared * tolerance) {
            // From the plotting point of view this is not a degenerate case, but renderers may
            // want the transformation to be invertible.
            //
            // If one singular value is much larger than the other, the frobenius norm
            // (squared) will be approximately the square of larger.  The determinant is their
            // product, and so the condition effectively compares their magnitude (for small
            // tolerances).

            Ok(Self::Parabolic(orig_rat_poly.convert_to_parabolic()))
         } else {
            // Only outcome that actually uses OOE form.
            Ok(Self::Elliptical(rat_poly))
         }
      }
   }

   fn create_hyperbolic_or_parabolic(
      poly_curve: &Curve<RatQuadHomogPower>,
      tolerance: f64,
   ) -> Result<Self, &'static str> {
      let reg_curve =
         Curve::<RegularizedRatQuadPath>::create_by_raising_to_regularized_symmetric(poly_curve)?;

      // fn create_from_regularized(reg_curve: &Curve<RegularizedRatQuadPath>, tolerance: f64) -> Self {
      let rat_poly = reg_curve;

      let r = rat_poly.path.range_bound;
      if (rat_poly.path.a_2.abs() * r * r) < (rat_poly.path.a_0.abs() * tolerance) {
         Ok(Self::Parabolic(rat_poly.convert_to_parabolic()))
      } else {
         // Rust clippy effectively makes this check impossible.
         // assert_ne!(rat_poly.path.a_2.signum(), rat_poly.path.a_0.signum());

         let hyperbolic_form = rat_poly.convert_to_hyperbolic();

         Ok(Self::Hyperbolic(hyperbolic_form))
      }
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   pub fn create_from_ordinary(
      weighted_curve: &Curve<RatQuadHomogWeighted>,
      tolerance: f64,
   ) -> Result<Self, &'static str> {
      let poly_curve: Curve<RatQuadHomogPower> = Curve::<RatQuadHomogPower>::from(weighted_curve);
      // First test "b^2-4ac" to see if denominator has real roots. If it does, create either
      // hyperbolic or parabolic. If no real roots, then elliptical or parabolic.
      if (poly_curve.path.h.0[2][1] * poly_curve.path.h.0[2][1])
         < (4.0 * poly_curve.path.h.0[2][0] * poly_curve.path.h.0[2][2])
      {
         Self::create_elliptical_or_parabolic(&poly_curve, tolerance)
      } else {
         Self::create_hyperbolic_or_parabolic(&poly_curve, tolerance)
      }
   }
}
