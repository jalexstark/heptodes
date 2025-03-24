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

use zvx_base::is_default;
use zvx_base::default_unit_f64;
use zvx_base::is_default_unit_f64;
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;

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
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum RatQuadState {
   #[default]
   RationalPoly,
   SymmetricRange,       // RationalPoly[nomial] with symmetric range.
   RegularizedSymmetric, // SymmetricRange with zero middle denominator coefficient.
   OffsetOddEven,        // O-O-E weightings of RegularizedSymmetric.

   FourPoint,        // Like cubic.
   ThreePointAngle,  // Form a,b,angle, sigma.
   RationalWeighted, // Polynomial-like, by  difference from end points.
}

#[derive(Debug, Serialize, PartialEq, Copy, Clone)]
pub struct FourPointRatQuad {
   pub state: RatQuadState,
   pub r: [f64; 2], // Range.
   pub x: [f64; 4],
   pub y: [f64; 4],
}

impl Default for FourPointRatQuad {
   fn default() -> Self {
      Self {
         state: RatQuadState::FourPoint,
         r: [0.0, 0.0],
         x: [0.0, 0.0, 0.0, 0.0],
         y: [0.0, 0.0, 0.0, 0.0],
      }
   }
}

// #[derive(Debug, Serialize, PartialEq, Copy, Clone)]
// pub struct ThreePointAngleRatQuad {
//    pub state: RatQuadState,
//    pub r: [f64; 2], // Range.
//    pub x: [f64; 3],
//    pub y: [f64; 3],
//    pub angle: ZebraixAngle,
//    pub sigma: f64,
// }

// impl Default for ThreePointAngleRatQuad {
//    fn default() -> Self {
//       Self {
//          state: RatQuadState::ThreePointAngle,
//          r: [0.0, 0.0],
//          x: [0.0, 0.0, 0.0],
//          y: [0.0, 0.0, 0.0],
//          angle: ZebraixAngle::Quadrant(1.0),
//          sigma: 1.0,
//       }
//    }
// }

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum RatQuadOoeSubtype {
   #[default]
   Elliptical,
   Parabolic,
   Hyperbolic,
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct BaseRatQuad {
   pub state: RatQuadState,
   pub r: [f64; 2], // Range.
   pub a: [f64; 3], // Denominator, as a[2] * t^2 + a[1] * t... .
   pub b: [f64; 3], // Numerator or O-O-E coefficients for x component.
   pub c: [f64; 3], // Numerator or O-O-E coefficients for y component.
   #[serde(skip_serializing_if = "is_default")]
   pub angle: ZebraixAngle,
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
   pub ooe_subtype: RatQuadOoeSubtype,
}

#[derive(Serialize, Debug, Default, Copy, Clone, PartialEq)]
pub enum SpecifiedRatQuad {
   #[default]
   None, // For, say, polynomial directly specified.
   // Base(BaseRatQuad), // Three-points and angle, for example.
   FourPoint(FourPointRatQuad),
   ThreePointAngle(BaseRatQuad),
   // ThreePointAngle(ThreePointAngleRatQuad),
}

impl BaseRatQuad {
   // pub fn new(r: [f64; 2], a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Self {
   //    Self { r, a, b, c, form: RatQuadState::RationalPoly }
   // }

   #[must_use]
   pub fn eval_quad(&self, t: &[f64]) -> Vec<[f64; 2]> {
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

   #[inline]
   // Applies bilinear substitution of the form (wt + x) / (yt + z) with normalization.
   //
   // This function should be applied by a knowledgeable caller, that is one that handles the
   // state of the RatQuad.
   #[allow(clippy::suboptimal_flops)]
   pub fn apply_bilinear_unranged(&mut self, mut w: f64, mut x: f64, mut y: f64, mut z: f64) {
      #[allow(clippy::suboptimal_flops)]
      let norm = 1.0 / ((w * w + x * x) * (y * y + z * z)).sqrt().sqrt();
      w *= norm;
      x *= norm;
      y *= norm;
      z *= norm;
      self.a = [
         self.a[0] * z * z + self.a[1] * x * z + self.a[2] * x * x,
         2.0 * self.a[0] * y * z + self.a[1] * (x * y + w * z) + 2.0 * self.a[2] * w * x,
         self.a[0] * y * y + self.a[1] * w * y + self.a[2] * w * w,
      ];
      self.b = [
         self.b[0] * z * z + self.b[1] * x * z + self.b[2] * x * x,
         2.0 * self.b[0] * y * z + self.b[1] * (x * y + w * z) + 2.0 * self.b[2] * w * x,
         self.b[0] * y * y + self.b[1] * w * y + self.b[2] * w * w,
      ];
      self.c = [
         self.c[0] * z * z + self.c[1] * x * z + self.c[2] * x * x,
         2.0 * self.c[0] * y * z + self.c[1] * (x * y + w * z) + 2.0 * self.c[2] * w * x,
         self.c[0] * y * y + self.c[1] * w * y + self.c[2] * w * w,
      ];
   }
   #[inline]
   // Applies bilinear transformation with factor sigma, preserving the range.
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   pub fn apply_bilinear(&mut self, sigma: f64) -> Result<(), &'static str> {
      match self.state {
         RatQuadState::OffsetOddEven => {
            Err("Unable to convert offset-even-odd form to symmetric-range form.")
         }
         RatQuadState::RegularizedSymmetric => {
            Err("Applying bilinear to regularized will downgrade it.")
         }
         RatQuadState::SymmetricRange => {
            let r = self.r[1];
            self.apply_bilinear_unranged(
               (sigma + 1.0) * r,
               (sigma - 1.0) * r * r,
               sigma - 1.0,
               (sigma + 1.0) * r,
            );
            Ok(())
         }
         RatQuadState::RationalPoly => {
            let p = self.r[0];
            let q = self.r[1];
            self.apply_bilinear_unranged(
               sigma * q - p,
               -(sigma - 1.0) * p * q,
               sigma - 1.0,
               q - sigma * p,
            );
            Ok(())
         }
         RatQuadState::FourPoint => {
            Err("Bilinear is applicable to four-point form, but not implemented.")
         }
         RatQuadState::ThreePointAngle => {
            Err("Bilinear is applicable to three-point-angle form, but not implemented.")
         }
         RatQuadState::RationalWeighted => {
            Err("Bilinear is applicable to rational-weighted form, but not implemented.")
         }
      }
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_symmetric_range(&mut self) -> Result<(), &'static str> {
      if self.state == RatQuadState::OffsetOddEven {
         return Err("Unable to convert offset-even-odd form to symmetric-range form.");
      }
      // Replace t with t - d.
      let d = 0.5 * (self.r[0] + self.r[1]);
      let r = 0.5 * (self.r[1] - self.r[0]);

      self.a =
         [d * (d * self.a[2] + self.a[1]) + self.a[0], 2.0 * d * self.a[2] + self.a[1], self.a[2]];
      self.b =
         [d * (d * self.b[2] + self.b[1]) + self.b[0], 2.0 * d * self.b[2] + self.b[1], self.b[2]];
      self.c =
         [d * (d * self.c[2] + self.c[1]) + self.c[0], 2.0 * d * self.c[2] + self.c[1], self.c[2]];

      self.r = [-r, r];
      self.state = RatQuadState::SymmetricRange;
      Ok(())
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_regularized_symmetric(&mut self) -> Result<(), &'static str> {
      if self.state != RatQuadState::SymmetricRange {
         return Err("Can only raise from symmetric-range to regularized-symmetric form.");
      }
      let r = self.r[1];
      let a_s = self.a[2] * r * r + self.a[0];
      // let a_d = self.a[2] * r * r - self.a[0];
      let combo_s = a_s + self.a[1] * r;
      let combo_d = a_s - self.a[1] * r;

      let sigma = combo_d.abs().sqrt() / combo_s.abs().sqrt();

      self.apply_bilinear(sigma)?;

      assert!(self.a[1].abs() < 0.001);
      self.state = RatQuadState::RegularizedSymmetric;

      Ok(())
   }

   #[inline]
   #[must_use]
   pub fn characterize_endpoints(&self) -> ([f64; 4], [f64; 4]) {
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

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_offset_odd_even(
      &mut self,
      poly: &Self,
      tolerance: f64,
   ) -> Result<(), &'static str> {
      if poly.state != RatQuadState::RegularizedSymmetric {
         return Err("Can only raise from regularized-symmetric form to offset-odd-even form.");
      }
      *self = *poly;

      let r = self.r[1];
      // assert_eq!(r, 10000.0);
      if (self.a[2].abs() * r * r) < (self.a[0].abs() * tolerance) {
         self.ooe_subtype = RatQuadOoeSubtype::Parabolic;
      } else if self.a[2].signum() == self.a[0].signum() {
         self.ooe_subtype = RatQuadOoeSubtype::Elliptical;

         let s = 1.0 / self.a[0];
         let f = 1.0 / self.a[2];
         self.a[0] = 1.0;
         self.a[2] *= s;

         {
            let offset = 0.5 * (s * self.b[0] + f * self.b[2]);
            let even = 0.5 * (s * self.b[0] - f * self.b[2]);
            let odd = self.b[1] * s;
            self.b = [offset, odd, even];
         }
         {
            let offset = 0.5 * (s * self.c[0] + f * self.c[2]);
            let even = 0.5 * (s * self.c[0] - f * self.c[2]);
            let odd = self.c[1] * s;
            self.c = [offset, odd, even];
         }

         let sss = 1.0 / self.a[2].sqrt();
         let (sx, sy) = (0.5 * sss * self.b[1], 0.5 * sss * self.c[1]);
         let (cx, cy) = (self.b[2], self.c[2]);
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
            *self = *poly;
            self.ooe_subtype = RatQuadOoeSubtype::Parabolic;
         }
      } else {
         self.ooe_subtype = RatQuadOoeSubtype::Hyperbolic;
      }

      self.state = RatQuadState::OffsetOddEven;

      Ok(())
   }

   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::missing_errors_doc)]
   pub fn weighted_to_polynomial(&mut self) -> Result<(), &'static str> {
      if self.state != RatQuadState::RationalWeighted {
         return Err("Attempted conversion from rational-weighted when not in that state.");
      }
      // Get from self.sigma once confirmed working.
      let sigma = 1.0;
      let v = self.r[0];
      let w = self.r[1];
      // assert_eq!(w, 100.0);
      {
         let h0 = self.a[0];
         let h1 = sigma * self.a[1];
         let h2 = sigma * sigma * self.a[2];
         self.a = [
            w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
            2.0 * (-w * h0 + (w + v) * h1 - v * h2),
            h0 - 2.0 * h1 + h2,
         ];
      }
      {
         let h0 = self.b[0];
         let h1 = sigma * self.b[1];
         let h2 = sigma * sigma * self.b[2];
         self.b = [
            w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
            2.0 * (-w * h0 + (w + v) * h1 - v * h2),
            h0 - 2.0 * h1 + h2,
         ];
      }
      {
         let h0 = self.c[0];
         let h1 = sigma * self.c[1];
         let h2 = sigma * sigma * self.c[2];
         self.c = [
            w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
            2.0 * (-w * h0 + (w + v) * h1 - v * h2),
            h0 - 2.0 * h1 + h2,
         ];
      }

      self.state = RatQuadState::RationalPoly;
      Ok(())
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
