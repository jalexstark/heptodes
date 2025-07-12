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

use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use zvx_base::default_unit_f64;
use zvx_base::is_default;
use zvx_base::is_default_unit_f64;

const fn scale_3(x: &[f64; 3], s: f64) -> [f64; 3] {
   [s * x[0], s * x[1], s * x[2]]
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum ZebraixAngle {
   Quadrant(f64),
   Radians(f64),
   TanHalf(f64),
}

impl ZebraixAngle {
   #[inline]
   #[must_use]
   pub fn in_radians(&self) -> f64 {
      match self {
         Self::Quadrant(q) => 0.5 * q * std::f64::consts::PI,
         Self::Radians(r) => *r,
         Self::TanHalf(t) => 2.0 * t.atan(),
      }
   }

   // This is really not good. We should deal with half the opening angle, or otherwise we get
   // strangeness as regards interpretation of angles (such as subtracting 2 pi from angle.
   #[inline]
   #[must_use]
   pub fn cos(&self) -> f64 {
      match self {
         Self::Quadrant(_) => self.in_radians().cos(),
         Self::Radians(r) => r.cos(),
         Self::TanHalf(t) => (1.0 - t * t) / (1.0 + t * t),
      }
   }
}

impl Default for ZebraixAngle {
   fn default() -> Self {
      Self::Quadrant(1.0)
   }
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct FourPointRatQuad {
   pub r: [f64; 2], // Range.
   pub p: [[f64; 2]; 4],
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct RatQuadRepr {
   pub r: [f64; 2], // Range.
   pub a: [f64; 3], // Denominator, as a[2] * t^2 + a[1] * t... .
   pub b: [f64; 3], // Numerator or O-O-E coefficients for x component.
   pub c: [f64; 3], // Numerator or O-O-E coefficients for y component.
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct RegularizedRatQuadRepr {
   pub r_bound: f64, // Range is [-r_bound, r_bound].
   pub a_0: f64,     // Denominator, as a[2] * t^2 + a[1] * t... .
   pub a_2: f64,
   pub b: [f64; 3], // Numerator or O-O-E coefficients for x component.
   pub c: [f64; 3], // Numerator or O-O-E coefficients for y component.
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

// Path is:
//
// offset + minus_partial / (lambda - mu * t) + plus_partial / (lambda + mu * t).
#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct HyperbolicRatQuadRepr {
   pub r_bound: f64, // Range is [-r_bound, r_bound].
   pub lambda: f64,
   pub mu: f64,
   pub offset: [f64; 2],
   pub minus_partial: [f64; 2],
   pub plus_partial: [f64; 2],
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

// TODO: Migrate to Cubilinear form.
#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct ParabolicRatQuadRepr {
   pub r: [f64; 2], // Range.
   pub a_0: f64,    // Denominator, as a[2] * t^2 + a[1] * t... .
   pub a_2: f64,
   pub b: [f64; 3], // Numerator or O-O-E coefficients for x component.
   pub c: [f64; 3], // Numerator or O-O-E coefficients for y component.
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct ThreePointAngleRepr {
   pub r: [f64; 2], // Range.
   pub p: [[f64; 2]; 3],
   #[serde(skip_serializing_if = "is_default")]
   pub angle: ZebraixAngle,
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

#[derive(Debug, Deserialize, Serialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct FourPointCubiLinearRepr {
   pub r: [f64; 2], // Range.
   pub x: [f64; 4],
   pub y: [f64; 4],
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

#[derive(Debug, Deserialize, Serialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct MidDiffCubiLinearRepr {
   pub r: [f64; 2], // Range.
   pub x: [f64; 4],
   pub y: [f64; 4],
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq)]
pub enum CubiLinear {
   #[default]
   Nothing,
   FourPoint(FourPointCubiLinearRepr),
   MidDiff(MidDiffCubiLinearRepr),
}

#[derive(Serialize, Deserialize, Default, Debug, Copy, Clone, PartialEq)]
pub enum RatQuadOoeSubclassed {
   #[default]
   Nothing,
   // Elliptical to custom OOE.
   Elliptical(RegularizedRatQuadRepr),
   // Perhaps change to cubilinear form
   Parabolic(ParabolicRatQuadRepr),
   Hyperbolic(HyperbolicRatQuadRepr),
}

#[derive(Debug, Serialize, Default, PartialEq, Copy, Clone)]
pub enum BaseRatQuad {
   #[default]
   Nothing,
   RationalPoly(RatQuadRepr),
   RegularizedSymmetric(RegularizedRatQuadRepr), // SymmetricRange with zero middle denominator coefficient.
   FourPoint(FourPointRatQuad),                  // Like cubic.
   ThreePointAngle(ThreePointAngleRepr),         // Form p, angle, sigma.
   RationalWeighted(RatQuadRepr),                // Polynomial-like, by  difference from end points.
}

#[derive(Serialize, Debug, Default, Copy, Clone, PartialEq)]
pub enum SpecifiedRatQuad {
   #[default]
   None, // For, say, polynomial directly specified.
   FourPoint(FourPointRatQuad),
   ThreePointAngle(ThreePointAngleRepr),
}

impl RatQuadRepr {
   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::many_single_char_names)]
   fn rq_apply_bilinear_unranged(&self, w: f64, x: f64, y: f64, z: f64) -> Self {
      // let norm = 1.0 / ((w * w + x * x) * (y * y + z * z)).sqrt().sqrt();
      // w *= norm;
      // x *= norm;
      // y *= norm;
      // z *= norm;

      let a = [
         self.a[0] * z * z + self.a[1] * x * z + self.a[2] * x * x,
         2.0 * self.a[0] * y * z + self.a[1] * (x * y + w * z) + 2.0 * self.a[2] * w * x,
         self.a[0] * y * y + self.a[1] * w * y + self.a[2] * w * w,
      ];
      let b = [
         self.b[0] * z * z + self.b[1] * x * z + self.b[2] * x * x,
         2.0 * self.b[0] * y * z + self.b[1] * (x * y + w * z) + 2.0 * self.b[2] * w * x,
         self.b[0] * y * y + self.b[1] * w * y + self.b[2] * w * w,
      ];
      let c = [
         self.c[0] * z * z + self.c[1] * x * z + self.c[2] * x * x,
         2.0 * self.c[0] * y * z + self.c[1] * (x * y + w * z) + 2.0 * self.c[2] * w * x,
         self.c[0] * y * y + self.c[1] * w * y + self.c[2] * w * w,
      ];

      let f = 1.0 / (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).abs().sqrt();

      Self { a: scale_3(&a, f), b: scale_3(&b, f), c: scale_3(&c, f), r: self.r, sigma: self.sigma }
   }

   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::many_single_char_names)]
   pub fn rq_apply_bilinear(&self, sigma_n: f64, sigma_d: f64) -> Self {
      let p = -self.r[0];
      let q = self.r[1];

      self.rq_apply_bilinear_unranged(
         sigma_n * q + sigma_d * p,
         (sigma_n - sigma_d) * p * q,
         sigma_n - sigma_d,
         sigma_d * q + sigma_n * p,
      )
   }

   #[must_use]
   pub fn rq_eval_quad(&self, t: &[f64]) -> Vec<[f64; 2]> {
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());

      for item in t {
         let denom_reciprocal = 1.0 / self.a[2].mul_add(*item, self.a[1]).mul_add(*item, self.a[0]);
         ret_val.push([
            self.b[2].mul_add(*item, self.b[1]).mul_add(*item, self.b[0]) * denom_reciprocal,
            self.c[2].mul_add(*item, self.c[1]).mul_add(*item, self.c[0]) * denom_reciprocal,
         ]);
      }

      ret_val
   }

   // Only used at present to characterize for case of OOE subtype parabola.
   #[inline]
   #[must_use]
   pub fn rq_characterize_endpoints(&self) -> ([f64; 4], [f64; 4]) {
      let mut x = [0.0; 4];
      let mut y = [0.0; 4];

      let speed_scale = self.r[1] - self.r[0];
      for (outer, inner, t) in [(0, 1, self.r[0]), (3, 2, self.r[1])] {
         let recip_a = 1.0 / self.a[2].mul_add(t, self.a[1]).mul_add(t, self.a[0]);
         let b = self.b[2].mul_add(t, self.b[1]).mul_add(t, self.b[0]);
         let c = self.c[2].mul_add(t, self.c[1]).mul_add(t, self.c[0]);
         let da = self.a[2].mul_add(2.0 * t, self.a[1]) * speed_scale;
         let db = self.b[2].mul_add(2.0 * t, self.b[1]) * speed_scale;
         let dc = self.c[2].mul_add(2.0 * t, self.c[1]) * speed_scale;
         x[outer] = b * recip_a;
         y[outer] = c * recip_a;
         x[inner] = (-b * da).mul_add(recip_a, db) * recip_a;
         y[inner] = (-c * da).mul_add(recip_a, dc) * recip_a;
      }
      (x, y)
   }

   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::many_single_char_names)]
   pub fn figure_symmetric_range_rat_quad(&self) -> Self {
      // Replace t with t - d.
      let d = 0.5 * (self.r[0] + self.r[1]);
      let r_half = 0.5 * (self.r[1] - self.r[0]);

      let a =
         [d * (d * self.a[2] + self.a[1]) + self.a[0], 2.0 * d * self.a[2] + self.a[1], self.a[2]];
      let b =
         [d * (d * self.b[2] + self.b[1]) + self.b[0], 2.0 * d * self.b[2] + self.b[1], self.b[2]];
      let c =
         [d * (d * self.c[2] + self.c[1]) + self.c[0], 2.0 * d * self.c[2] + self.c[1], self.c[2]];

      let r = [-r_half, r_half];
      Self { r, a, b, c, sigma: self.sigma }
   }
}

impl HyperbolicRatQuadRepr {
   const fn convert_to_regularized(&self) -> RegularizedRatQuadRepr {
      let lambda = self.lambda;
      let mu = self.mu;
      let b = [
         lambda * (self.offset[0] * lambda + self.minus_partial[0] + self.plus_partial[0]),
         mu * (self.minus_partial[0] - self.plus_partial[0]),
         -self.offset[0] * mu * mu,
      ];
      let c = [
         lambda * (self.offset[1] * lambda + self.minus_partial[1] + self.plus_partial[1]),
         mu * (self.minus_partial[1] - self.plus_partial[1]),
         -self.offset[1] * mu * mu,
      ];
      let a_0 = lambda * lambda;
      let a_2 = -mu * mu;

      RegularizedRatQuadRepr { r_bound: self.r_bound, a_0, a_2, b, c, sigma: self.sigma }
   }
}

impl RegularizedRatQuadRepr {
   const fn convert_to_parabolic(&self) -> ParabolicRatQuadRepr {
      ParabolicRatQuadRepr {
         r: [-self.r_bound, self.r_bound],
         a_0: self.a_0,
         a_2: self.a_2,
         b: self.b,
         c: self.c,
         sigma: self.sigma,
      }
   }

   // At present there is no proper testing of s. Manual inspection verifies that negating all
   // a, b and c in the input leaves the output invariant.
   #[allow(clippy::suboptimal_flops)]
   fn convert_to_hyperbolic(&self) -> HyperbolicRatQuadRepr {
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

      HyperbolicRatQuadRepr {
         r_bound: self.r_bound,
         lambda,
         mu,
         offset,
         plus_partial,
         minus_partial,
         sigma: self.sigma,
      }
   }

   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   pub fn classify_offset_odd_even(&self, tolerance: f64) -> RatQuadOoeSubclassed {
      let orig_rat_poly = *self;
      let mut rat_poly = *self;

      let r = rat_poly.r_bound;
      if (rat_poly.a_2.abs() * r * r) < (rat_poly.a_0.abs() * tolerance) {
         RatQuadOoeSubclassed::Parabolic(orig_rat_poly.convert_to_parabolic())
      } else if rat_poly.a_2.signum() == rat_poly.a_0.signum() {
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

            RatQuadOoeSubclassed::Parabolic(orig_rat_poly.convert_to_parabolic())
         } else {
            // Only outcome that actually uses OOE form.
            RatQuadOoeSubclassed::Elliptical(rat_poly)
         }
      } else {
         // {
         let hyperbolic_form = orig_rat_poly.convert_to_hyperbolic();
         let poly = orig_rat_poly;
         let reconstructed = hyperbolic_form.convert_to_regularized();
         //      Self {
         //    r: orig_rat_poly.r,
         //    a_0: orig_rat_poly.a_0,
         //    a_2: orig_rat_poly.a_2,
         //    // a: [orig_rat_poly.a[0], 0.0, orig_rat_poly.a[2]],
         //    b: orig_rat_poly.b,
         //    c: orig_rat_poly.c,
         //    sigma: orig_rat_poly.sigma,
         // };
         println!("a: [{}, {}, {}]", poly.a_0, 0.0, poly.a_2);
         println!("b: [{}, {}, {}]", poly.b[0], poly.b[1], poly.b[2]);
         println!("c: [{}, {}, {}]", poly.c[0], poly.c[1], poly.c[2]);

         println!("a: [{}, {}, {}]", reconstructed.a_0, 0.0, reconstructed.a_2);
         println!("b: [{}, {}, {}]", reconstructed.b[0], reconstructed.b[1], reconstructed.b[2]);
         println!("c: [{}, {}, {}]", reconstructed.c[0], reconstructed.c[1], reconstructed.c[2]);

         // let reconstructed_b = [
         //    offset[0] * beta * beta + minus_fraction[0] * beta + plus_fraction[0] * beta,
         //    minus_fraction[0] * gamma - plus_fraction[0] * gamma,
         //    -offset[0] * gamma * gamma,
         // ];
         // let reconstructed_c = [
         //    offset[1] * beta * beta + minus_fraction[1] * beta + plus_fraction[1] * beta,
         //    minus_fraction[1] * gamma - plus_fraction[1] * gamma,
         //    -offset[1] * gamma * gamma,
         // ];
         // println!("recon a: [{}, {}, {}]", beta * beta, 0.0, -gamma * gamma);
         // println!(
         //    "recon b: [{}, {}, {}]",
         //    reconstructed_b[0], reconstructed_b[1], reconstructed_b[2]
         // );
         // println!(
         //    "recon c: [{}, {}, {}]",
         //    reconstructed_c[0], reconstructed_c[1], reconstructed_c[2]
         // );
         // }

         RatQuadOoeSubclassed::Hyperbolic(hyperbolic_form)
      }
   }
}

impl BaseRatQuad {
   #[allow(clippy::match_same_arms)]
   #[allow(clippy::missing_errors_doc)]
   pub const fn get_poly(&self) -> Result<RatQuadRepr, &'static str> {
      match self {
         Self::RationalPoly(repr) => Ok(*repr),
         Self::RegularizedSymmetric(symm) => Ok(RatQuadRepr {
            r: [-symm.r_bound, symm.r_bound],
            a: [symm.a_0, 0.0, symm.a_2],
            b: symm.b,
            c: symm.c,
            sigma: symm.sigma,
         }),
         Self::Nothing
         | Self::FourPoint(_)
         | Self::ThreePointAngle(_)
         | Self::RationalWeighted(_) => Err("QR not  proper rational poly."),
      }
   }

   #[must_use]
   #[allow(clippy::missing_panics_doc)]
   pub fn eval_quad(&self, t: &[f64]) -> Vec<[f64; 2]> {
      let ret_val = Vec::<[f64; 2]>::with_capacity(t.len());

      assert!(matches!(self, Self::RationalPoly { .. }));
      if let Self::RationalPoly(rat_poly) = self {
         rat_poly.rq_eval_quad(t)
      } else {
         ret_val
      }
   }

   #[inline]
   // Applies bilinear transformation with factor sigma, preserving the range.
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   pub fn apply_bilinear(&mut self, sigma_n: f64, sigma_d: f64) -> Result<(), &'static str> {
      match self {
         Self::Nothing => unimplemented!("Nothing form is invalid from construction."),
         Self::RegularizedSymmetric(_) => {
            Err("Applying bilinear to regularized will downgrade it.")
         }
         Self::RationalPoly(rat_poly) => {
            *self = Self::RationalPoly(rat_poly.rq_apply_bilinear(sigma_n, sigma_d));
            Ok(())
         }
         Self::FourPoint(_) => {
            Err("Bilinear is applicable to four-point form, but not implemented.")
         }
         Self::ThreePointAngle(_) => {
            Err("Bilinear is applicable to three-point-angle form, but not implemented.")
         }
         Self::RationalWeighted(_) => {
            Err("Bilinear is applicable to rational-weighted form, but not implemented.")
         }
      }
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_regularized_symmetric(&mut self) -> Result<(), &'static str> {
      if let Self::RationalPoly(rat_poly_extracted) = self {
         let rat_poly = rat_poly_extracted.figure_symmetric_range_rat_quad();

         let r_both = rat_poly.r[1];
         let a_s = rat_poly.a[2] * r_both * r_both + rat_poly.a[0];
         // let a_d = rat_poly.a[2] * r * r - rat_poly.a[0];
         let combo_s = a_s + rat_poly.a[1] * r_both;
         let combo_d = a_s - rat_poly.a[1] * r_both;

         let sigma_n = combo_d.abs().sqrt();
         let sigma_d = combo_s.abs().sqrt();

         *self = Self::RationalPoly(rat_poly);
         self.apply_bilinear(sigma_n, sigma_d)?;

         if let Self::RationalPoly(check_poly) = self {
            assert!(check_poly.a[1].abs() < 0.001);
            *self = Self::RegularizedSymmetric(RegularizedRatQuadRepr {
               r_bound: check_poly.r[1],
               a_0: check_poly.a[0],
               a_2: check_poly.a[2],
               b: check_poly.b,
               c: check_poly.c,
               sigma: check_poly.sigma,
            });
         } else {
            panic!("Unreachable");
         }

         Ok(())
      } else {
         Err("Can only raise from rat-poly to regularized-symmetric form.")
      }
   }

   // Only used at present to characterize for case of OOE subtype parabola.
   #[inline]
   #[must_use]
   #[allow(clippy::missing_panics_doc)]
   pub fn characterize_endpoints(&self) -> ([f64; 4], [f64; 4]) {
      let x = [0.0; 4];
      let y = [0.0; 4];

      assert!(matches!(self, Self::RationalPoly { .. }));
      if let Self::RationalPoly(rat_poly) = self {
         rat_poly.rq_characterize_endpoints()
      } else {
         (x, y)
      }
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   pub fn weighted_to_polynomial(&mut self) -> Result<(), &'static str> {
      if let Self::RationalWeighted(rat_poly) = self {
         // Get from rat_poly.sigma once confirmed working.
         let sigma = 1.0;
         let v = rat_poly.r[0];
         let w = rat_poly.r[1];
         {
            let h0 = rat_poly.a[0];
            let h1 = sigma * rat_poly.a[1];
            let h2 = sigma * sigma * rat_poly.a[2];
            rat_poly.a = [
               w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
               2.0 * (-w * h0 + (w + v) * h1 - v * h2),
               h0 - 2.0 * h1 + h2,
            ];
         }
         {
            let h0 = rat_poly.b[0];
            let h1 = sigma * rat_poly.b[1];
            let h2 = sigma * sigma * rat_poly.b[2];
            rat_poly.b = [
               w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
               2.0 * (-w * h0 + (w + v) * h1 - v * h2),
               h0 - 2.0 * h1 + h2,
            ];
         }
         {
            let h0 = rat_poly.c[0];
            let h1 = sigma * rat_poly.c[1];
            let h2 = sigma * sigma * rat_poly.c[2];
            rat_poly.c = [
               w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
               2.0 * (-w * h0 + (w + v) * h1 - v * h2),
               h0 - 2.0 * h1 + h2,
            ];
         }

         *self = Self::RationalPoly(*rat_poly);
         Ok(())
      } else {
         Err("Attempted conversion from rational-weighted when not in that state.")
      }
   }
}

#[allow(clippy::missing_errors_doc)]
impl FourPointCubiLinearRepr {
   #[inline]
   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::suboptimal_flops)]
   #[must_use]
   pub fn eval_part(b: f64, a: f64, coeffs: &[f64; 4], multiplier: f64) -> f64 {
      multiplier
         * (b * b * b * coeffs[0]
            + 3.0 * b * b * a * coeffs[1]
            + 3.0 * b * a * a * coeffs[2]
            + a * a * a * coeffs[3])
   }

   #[must_use]
   #[allow(clippy::many_single_char_names)]
   pub fn eval(&self, t: &[f64]) -> Vec<[f64; 2]> {
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
      for item in t {
         let a = self.sigma * (*item - self.r[0]);
         let b = self.r[1] - *item;
         let f0 = 1.0 / (b + a);
         let recip_denom = f0 * f0 * f0;
         let x = Self::eval_part(b, a, &self.x, recip_denom);
         let y = Self::eval_part(b, a, &self.y, recip_denom);
         ret_val.push([x, y]);
      }
      ret_val
   }

   #[allow(clippy::similar_names)]
   #[allow(clippy::suboptimal_flops)]
   pub fn select_range(&mut self, new_range: [f64; 2]) {
      let mut new_x = [0.0; 4];
      let mut new_y = [0.0; 4];

      let a_k = self.sigma * (new_range[0] - self.r[0]);
      let b_k = self.r[1] - new_range[0];
      let a_l = self.sigma * (new_range[1] - self.r[0]);
      let b_l = self.r[1] - new_range[1];
      let f0_k = 1.0 / (b_k + a_k);
      let recip_denom_k = f0_k * f0_k * f0_k;
      let f0_l = 1.0 / (b_l + a_l);
      let recip_denom_l = f0_l * f0_l * f0_l;
      new_x[0] = Self::eval_part(b_k, a_k, &self.x, recip_denom_k);
      new_y[0] = Self::eval_part(b_k, a_k, &self.y, recip_denom_k);
      new_x[3] = Self::eval_part(b_l, a_l, &self.x, recip_denom_l);
      new_y[3] = Self::eval_part(b_l, a_l, &self.y, recip_denom_l);
      let kl_numerator_k = self.sigma * self.r[1] * (new_range[0] - self.r[0])
         + self.r[0] * (self.r[1] - new_range[0]);
      let kl_numerator_l = self.sigma * self.r[1] * (new_range[1] - self.r[0])
         + self.r[0] * (self.r[1] - new_range[1]);
      // This is [k, l] bilinearly transformed.
      let selected_range_bilineared = kl_numerator_l / (a_l + b_l) - kl_numerator_k / (a_k + b_k);
      let fudge_k = selected_range_bilineared / (self.r[1] - self.r[0]);
      let fudge_l = selected_range_bilineared / (self.r[1] - self.r[0]);
      // assert_eq!(1.0 / f0_k, 0.0);
      let dx_1 = fudge_k
         * f0_k
         * f0_k
         * (b_k * b_k * (self.x[1] - self.x[0])
            + 2.0 * b_k * a_k * (self.x[2] - self.x[1])
            + a_k * a_k * (self.x[3] - self.x[2]));
      new_x[1] = new_x[0] + dx_1;
      let dy_1 = fudge_k
         * f0_k
         * f0_k
         * (b_k * b_k * (self.y[1] - self.y[0])
            + 2.0 * b_k * a_k * (self.y[2] - self.y[1])
            + a_k * a_k * (self.y[3] - self.y[2]));
      new_y[1] = new_y[0] + dy_1;
      let dx_1 = fudge_l
         * f0_l
         * f0_l
         * (b_l * b_l * (self.x[1] - self.x[0])
            + 2.0 * b_l * a_l * (self.x[2] - self.x[1])
            + a_l * a_l * (self.x[3] - self.x[2]));
      new_x[2] = new_x[3] - dx_1;
      let dy_1 = fudge_l
         * f0_l
         * f0_l
         * (b_l * b_l * (self.y[1] - self.y[0])
            + 2.0 * b_l * a_l * (self.y[2] - self.y[1])
            + a_l * a_l * (self.y[3] - self.y[2]));
      new_y[2] = new_y[3] - dy_1;

      self.sigma = (a_l + b_l) / (a_k + b_k);
      self.x = new_x;
      self.y = new_y;
      self.r = new_range;
   }

   pub fn displace(&mut self, d: [f64; 2]) {
      self.x[0] += d[0];
      self.x[1] += d[0];
      self.x[2] += d[0];
      self.x[3] += d[0];
      self.y[0] += d[1];
      self.y[1] += d[1];
      self.y[2] += d[1];
      self.y[3] += d[1];
   }

   pub fn bilinear_transform(&mut self, sigma: f64) {
      self.sigma *= sigma;
   }

   pub fn adjust_range(&mut self, new_range: [f64; 2]) {
      self.r = new_range;
   }
}

#[allow(clippy::missing_errors_doc)]
impl CubiLinear {
   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_panics_doc)]
   pub fn eval(&self, t: &[f64]) -> Result<Vec<[f64; 2]>, &'static str> {
      assert!(matches!(self, Self::FourPoint { .. }));
      if let Self::FourPoint(four_point) = self {
         Ok(four_point.eval(t))
      } else {
         Err("Can only evaluate cubilinear curves in four-point form.")
      }
   }
}
