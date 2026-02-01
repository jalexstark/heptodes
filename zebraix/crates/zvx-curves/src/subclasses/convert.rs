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

// #[cfg(test)]
// mod tests;
use crate::rat_quad::power_characterize_endpoints;
use crate::rat_quad::rq_power_collapse_bilinear;
use crate::subclasses::threes::RatQuadOoeSubclassed;
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use zvx_base::{
   default_unit_ratio, is_default_unit_ratio, CubicHomog, CubicPath, HyperbolicPath, RatQuadHomog,
   RatQuadHomogPower, RatQuadHomogWeighted,
};

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

trait PowerExtras {
   #[must_use]
   fn rq_overwrite_bilinear(&self, sigma_ratio: (f64, f64)) -> Self;

   #[must_use]
   fn figure_symmetric_range_rat_quad(&self) -> Self;
}

impl PowerExtras for RatQuadHomogPower {
   // Internal bilinear transform.
   fn rq_overwrite_bilinear(&self, sigma_ratio: (f64, f64)) -> Self {
      let mut retval = self.clone();
      retval.sigma.0 = sigma_ratio.0;
      retval.sigma.1 = sigma_ratio.1;
      retval
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::many_single_char_names)]
   fn figure_symmetric_range_rat_quad(&self) -> Self {
      // Replace t with t - d.
      let d = 0.5 * (self.r[0] + self.r[1]);
      let r_half = 0.5 * (self.r[1] - self.r[0]);

      let a = [
         d * (d * self.h.0[2][2] + self.h.0[2][1]) + self.h.0[2][0],
         2.0 * d * self.h.0[2][2] + self.h.0[2][1],
         self.h.0[2][2],
      ];
      let b = [
         d * (d * self.h.0[0][2] + self.h.0[0][1]) + self.h.0[0][0],
         2.0 * d * self.h.0[0][2] + self.h.0[0][1],
         self.h.0[0][2],
      ];
      let c = [
         d * (d * self.h.0[1][2] + self.h.0[1][1]) + self.h.0[1][0],
         2.0 * d * self.h.0[1][2] + self.h.0[1][1],
         self.h.0[1][2],
      ];

      let r = [-r_half, r_half];
      Self { r, h: RatQuadHomog([b, c, a]), sigma: self.sigma }
   }
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

impl RegularizedRatQuadPath {
   #[allow(clippy::suboptimal_flops)]
   #[must_use]
   pub fn convert_to_parabolic(&self) -> CubicPath {
      let (ends, deltas) = power_characterize_endpoints(&RatQuadHomogPower::from(self));
      let f = 3.0;
      let four_c = [
         [ends[0][0], f * ends[0][0] + deltas[0][0], f * ends[1][0] - deltas[1][0], ends[1][0]],
         [ends[0][1], f * ends[0][1] + deltas[0][1], f * ends[1][1] - deltas[1][1], ends[1][1]],
      ];

      CubicPath {
         r: [-self.range_bound, self.range_bound],
         h: CubicHomog(four_c),
         sigma: self.sigma,
      }
   }

   // At present there is no proper testing of s. Manual inspection verifies that negating all
   // a, b and c in the input leaves the output invariant.
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_panics_doc)]
   #[must_use]
   pub fn convert_to_hyperbolic(&self) -> HyperbolicPath {
      let s = self.a_0.signum();

      let lambda = (s * self.a_0).sqrt();
      assert!(-s * self.a_2 > 0.0);
      let mu = (-s * self.a_2).sqrt();
      let r_lambda = 1.0 / lambda;
      let r_mu = 1.0 / mu;
      let r_a_2 = 1.0 / self.a_2;

      let offset = [self.b[2] * r_a_2, self.c[2] * r_a_2];

      let f = 0.5 * s;
      let plus_partial = [
         f * (self.b[0] * r_lambda + (-self.b[1] + lambda * r_mu * self.b[2]) * r_mu),
         f * (self.c[0] * r_lambda + (-self.c[1] + lambda * r_mu * self.c[2]) * r_mu),
      ];
      let minus_partial = [
         f * (self.b[0] * r_lambda + (self.b[1] + lambda * r_mu * self.b[2]) * r_mu),
         f * (self.c[0] * r_lambda + (self.c[1] + lambda * r_mu * self.c[2]) * r_mu),
      ];

      HyperbolicPath {
         range: (-self.range_bound, self.range_bound),
         lambda,
         mu,
         offset,
         plus_partial,
         minus_partial,
         sigma: self.sigma,
      }
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::unnecessary_wraps)]
   #[allow(clippy::missing_errors_doc)]
   fn create_by_raising_to_regularized_symmetric(
      rat_poly_extracted: &RatQuadHomogPower,
   ) -> Result<Self, &'static str> {
      let rat_poly = rat_poly_extracted.figure_symmetric_range_rat_quad();

      let r_both = rat_poly.r[1];
      let a_s = rat_poly.h.0[2][2] * r_both * r_both + rat_poly.h.0[2][0];
      // let a_d = rat_poly.h.0[2][2] * r * r - rat_poly.h.0[2][0];
      let combo_s = a_s + rat_poly.h.0[2][1] * r_both;
      let combo_d = a_s - rat_poly.h.0[2][1] * r_both;

      let sigma_ratio = (combo_d.abs().sqrt(), combo_s.abs().sqrt());

      let intermediate_rat_poly = rat_poly.rq_overwrite_bilinear(sigma_ratio);
      let scratchy_rat_poly = rq_power_collapse_bilinear(&intermediate_rat_poly);

      let check_poly = scratchy_rat_poly;
      assert!(check_poly.h.0[2][1].abs() < 0.001);
      Ok(Self {
         range_bound: check_poly.r[1],
         a_0: check_poly.h.0[2][0],
         a_2: check_poly.h.0[2][2],
         b: check_poly.h.0[0],
         c: check_poly.h.0[1],
         sigma: check_poly.sigma,
      })
   }
}

#[allow(clippy::suboptimal_flops)]
impl RatQuadOoeSubclassed {
   fn create_elliptical_or_parabolic(
      poly_curve: &RatQuadHomogPower,
      tolerance: f64,
   ) -> Result<Self, &'static str> {
      let reg_curve =
         RegularizedRatQuadPath::create_by_raising_to_regularized_symmetric(poly_curve)?;

      let mut rat_poly = reg_curve;
      let orig_rat_poly = reg_curve;

      let r = rat_poly.range_bound;
      if (rat_poly.a_2.abs() * r * r) < (rat_poly.a_0.abs() * tolerance) {
         Ok(Self::Parabolic(orig_rat_poly.convert_to_parabolic()))
      } else {
         // Rust clippy effectively makes this check impossible.
         // assert_eq!(rat_poly.a_2.signum(), rat_poly.a_0.signum());

         // TODO: Better handle cases where s or f might be infinite.
         let s = 1.0 / rat_poly.a_0;
         let f = 1.0 / rat_poly.a_2;
         rat_poly.a_0 = 1.0;
         rat_poly.a_2 *= s;

         {
            let offset = 0.5 * (s * rat_poly.b[0] + f * rat_poly.b[2]);
            let even = 0.5 * (s * rat_poly.b[0] - f * rat_poly.b[2]);
            let odd = rat_poly.b[1] * s;
            rat_poly.b = [offset, odd, even];
         }
         {
            let offset = 0.5 * (s * rat_poly.c[0] + f * rat_poly.c[2]);
            let even = 0.5 * (s * rat_poly.c[0] - f * rat_poly.c[2]);
            let odd = rat_poly.c[1] * s;
            rat_poly.c = [offset, odd, even];
         }

         let sss = 1.0 / rat_poly.a_2.sqrt();
         let (sx, sy) = (0.5 * sss * rat_poly.b[1], 0.5 * sss * rat_poly.c[1]);
         let (cx, cy) = (rat_poly.b[2], rat_poly.c[2]);
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
      poly_curve: &RatQuadHomogPower,
      tolerance: f64,
   ) -> Result<Self, &'static str> {
      let reg_curve =
         RegularizedRatQuadPath::create_by_raising_to_regularized_symmetric(poly_curve)?;

      let rat_poly = reg_curve;

      let r = rat_poly.range_bound;
      if (rat_poly.a_2.abs() * r * r) < (rat_poly.a_0.abs() * tolerance) {
         Ok(Self::Parabolic(rat_poly.convert_to_parabolic()))
      } else {
         // Rust clippy effectively makes this check impossible.
         // assert_ne!(rat_poly.a_2.signum(), rat_poly.a_0.signum());

         let hyperbolic_form = rat_poly.convert_to_hyperbolic();

         Ok(Self::Hyperbolic(hyperbolic_form))
      }
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   pub fn create_from_ordinary(
      weighted_curve: &RatQuadHomogWeighted,
      tolerance: f64,
   ) -> Result<Self, &'static str> {
      let poly_curve: RatQuadHomogPower = RatQuadHomogPower::from(weighted_curve);
      // First test "b^2-4ac" to see if denominator has real roots. If it does, create either
      // hyperbolic or parabolic. If no real roots, then elliptical or parabolic.
      if (poly_curve.h.0[2][1] * poly_curve.h.0[2][1])
         < (4.0 * poly_curve.h.0[2][0] * poly_curve.h.0[2][2])
      {
         Self::create_elliptical_or_parabolic(&poly_curve, tolerance)
      } else {
         Self::create_hyperbolic_or_parabolic(&poly_curve, tolerance)
      }
   }
}
