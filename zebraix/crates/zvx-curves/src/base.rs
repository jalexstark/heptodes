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

#[derive(Serialize, Deserialize, Default, Debug, Copy, Clone, PartialEq)]
pub enum RatQuadOoeSubclassed {
   #[default]
   Nothing,
   // Elliptical to custom OOE.
   Elliptical(RegularizedRatQuadRepr),
   // Perhaps change to cubilinear form
   Parabolic(RegularizedRatQuadRepr),
   Hyperbolic(RegularizedRatQuadRepr),
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
   #[allow(clippy::suboptimal_flops)]
   #[must_use]
   #[allow(clippy::many_single_char_names)]
   pub fn rq_apply_bilinear_unranged(
      &self,
      mut w: f64,
      mut x: f64,
      mut y: f64,
      mut z: f64,
   ) -> Self {
      let norm = 1.0 / ((w * w + x * x) * (y * y + z * z)).sqrt().sqrt();
      w *= norm;
      x *= norm;
      y *= norm;
      z *= norm;

      // println!("w: {}, x: {} y: {}, z: {}", w, x, y, z);

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

      // println!("c[0]: {}, c[1]: {} c[2]: {}", c[0], c[1], c[2]);

      Self { a, b, c, r: self.r, sigma: self.sigma }
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

      // assert!(matches!(self, BaseRatQuad::OffsetOddEven { .. }));
      // if let BaseRatQuad::OffsetOddEven(rat_ooe) = self {
      //    assert!(matches!(rat_ooe, RatQuadOoeSubtype::Parabolic { .. }));
      //    if let RatQuadOoeSubtype::Parabolic(rat_poly) = rat_ooe {
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
      //    }
      // }
      (x, y)
   }
}

impl BaseRatQuad {
   #[allow(clippy::match_same_arms)]
   #[allow(clippy::missing_errors_doc)]
   pub const fn get_poly(&self) -> Result<RatQuadRepr, &'static str> {
      match self {
         // => Ok(repr),
         Self::RationalPoly(repr) => Ok(*repr),
         Self::RegularizedSymmetric(symm) => Ok(RatQuadRepr {
            r: symm.r,
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
   // pub fn new(r: [f64; 2], a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Self {
   //    Self { r, a, b, c, form: RatQuadState::RationalPoly }
   // }

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
   // Applies bilinear substitution of the form (wt + x) / (yt + z) with normalization.
   //
   // This function should be applied by a knowledgeable caller, that is one that handles the
   // state of the RatQuad.
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   pub fn apply_bilinear_unranged(
      &mut self,
      w: f64,
      x: f64,
      y: f64,
      z: f64,
   ) -> Result<(), &'static str> {
      match self {
         Self::Nothing => unimplemented!("Nothing form is invalid from construction."),
         Self::RegularizedSymmetric(_) => {
            Err("Applying bilinear to regularized will downgrade it.")
         }
         Self::RationalPoly(rat_poly) => {
            *self = Self::RationalPoly(rat_poly.rq_apply_bilinear_unranged(w, x, y, z));
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

   #[inline]
   // Applies bilinear transformation with factor sigma, preserving the range.
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   pub fn apply_bilinear(&mut self, sigma: f64) -> Result<(), &'static str> {
      match self {
         Self::Nothing => unimplemented!("Nothing form is invalid from construction."),
         // Self::OffsetOddEven(_) => {
         //    Err("Unable to convert offset-even-odd form to symmetric-range form.")
         // }
         Self::RegularizedSymmetric(_) => {
            Err("Applying bilinear to regularized will downgrade it.")
         }
         // Self::SymmetricRange(rat_poly) => {
         //    let r = rat_poly.r[1];
         //    self
         //       .apply_bilinear_unranged(
         //          (sigma + 1.0) * r,
         //          (sigma - 1.0) * r * r,
         //          sigma - 1.0,
         //          (sigma + 1.0) * r,
         //       )
         //       .expect("No more restrictive than caller");
         //    Ok(())
         // }
         Self::RationalPoly(rat_poly) => {
            let p = rat_poly.r[0];
            let q = rat_poly.r[1];

            // println!("p: {}, q: {}", p, q);

            self
               .apply_bilinear_unranged(
                  sigma * q - p,
                  -(sigma - 1.0) * p * q,
                  sigma - 1.0,
                  q - sigma * p,
               )
               .expect("No more restrictive than caller");

            // if let Self::RationalPoly(rat_poly_revised) = self {
            //    println!("r[0]: {}, r[1]: {}", rat_poly_revised.r[0], rat_poly_revised.r[1]);
            //    println!(
            //       "a[0]: {}, a[1]: {} a[2]: {}",
            //       rat_poly_revised.a[0], rat_poly_revised.a[1], rat_poly_revised.a[2]
            //    );
            //    println!(
            //       "c[0]: {}, c[1]: {} c[2]: {}",
            //       rat_poly_revised.c[0], rat_poly_revised.c[1], rat_poly_revised.c[2]
            //    );
            // }

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
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::many_single_char_names)]
   pub fn figure_symmetric_range(rat_poly: &RatQuadRepr) -> Result<RatQuadRepr, &'static str> {
      // TODO: Remove result, always succeeds.

      // if let Self::RationalPoly(rat_poly) = self {
      // Replace t with t - d.
      let d = 0.5 * (rat_poly.r[0] + rat_poly.r[1]);
      let r_half = 0.5 * (rat_poly.r[1] - rat_poly.r[0]);

      let a = [
         d * (d * rat_poly.a[2] + rat_poly.a[1]) + rat_poly.a[0],
         2.0 * d * rat_poly.a[2] + rat_poly.a[1],
         rat_poly.a[2],
      ];
      let b = [
         d * (d * rat_poly.b[2] + rat_poly.b[1]) + rat_poly.b[0],
         2.0 * d * rat_poly.b[2] + rat_poly.b[1],
         rat_poly.b[2],
      ];
      let c = [
         d * (d * rat_poly.c[2] + rat_poly.c[1]) + rat_poly.c[0],
         2.0 * d * rat_poly.c[2] + rat_poly.c[1],
         rat_poly.c[2],
      ];

      let r = [-r_half, r_half];
      Ok(RatQuadRepr { r, a, b, c, sigma: rat_poly.sigma })
      // } else {
      //    Err("Unable to convert offset-even-odd form to symmetric-range form.")
      // }
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_regularized_symmetric(&mut self) -> Result<(), &'static str> {
      if let Self::RationalPoly(rat_poly_extracted) = self {
         let rat_poly = Self::figure_symmetric_range(rat_poly_extracted).unwrap();

         let r_both = rat_poly.r[1];
         let a_s = rat_poly.a[2] * r_both * r_both + rat_poly.a[0];
         // let a_d = rat_poly.a[2] * r * r - rat_poly.a[0];
         let combo_s = a_s + rat_poly.a[1] * r_both;
         let combo_d = a_s - rat_poly.a[1] * r_both;

         let sigma = combo_d.abs().sqrt() / combo_s.abs().sqrt();

         *self = Self::RationalPoly(rat_poly);
         self.apply_bilinear(sigma)?;

         if let Self::RationalPoly(check_poly) = self {
            assert!(check_poly.a[1].abs() < 0.001);
            *self = Self::RegularizedSymmetric(RegularizedRatQuadRepr {
               r: check_poly.r,
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
   #[allow(clippy::missing_panics_doc)]
   pub fn classify_offset_odd_even(
      // &mut self,
      poly: &Self,
      tolerance: f64,
   ) -> Result<RatQuadOoeSubclassed, &'static str> {
      // TODO: Take reg. symmetric directly.
      assert!(matches!(poly, Self::RegularizedSymmetric { .. }));
      if let Self::RegularizedSymmetric(rat_poly_extracted) = poly {
         let orig_rat_poly = *rat_poly_extracted;
         let mut rat_poly = *rat_poly_extracted;

         let r = rat_poly.r[1];
         if (rat_poly.a_2.abs() * r * r) < (rat_poly.a_0.abs() * tolerance) {
            Ok(RatQuadOoeSubclassed::Parabolic(orig_rat_poly))
         } else if rat_poly.a_2.signum() == rat_poly.a_0.signum() {
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

               Ok(RatQuadOoeSubclassed::Parabolic(orig_rat_poly))
            } else {
               // Only outcome that actually uses OOE form.
               Ok(RatQuadOoeSubclassed::Elliptical(rat_poly))
            }
         } else {
            Ok(RatQuadOoeSubclassed::Hyperbolic(orig_rat_poly))
         }
      } else {
         Err("Can only raise from regularized-symmetric form to offset-odd-even form.")
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
         // assert_eq!(w, 100.0);
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

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum CubicForm {
   #[default]
   FourPoint,
   MidDiff,
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct CubiLinear {
   pub form: CubicForm,
   pub r: [f64; 2], // Range.
   pub x: [f64; 4],
   pub y: [f64; 4],
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

#[allow(clippy::missing_errors_doc)]
impl CubiLinear {
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

   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::suboptimal_flops)]
   pub fn eval(&self, t: &[f64]) -> Result<Vec<[f64; 2]>, &'static str> {
      if self.form != CubicForm::FourPoint {
         return Err("Can only evaluate cubilinear curves in four-point form.");
      }
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
      Ok(ret_val)
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
