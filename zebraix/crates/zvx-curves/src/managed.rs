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

use crate::base::BaseRatQuad;
use crate::base::CubiLinear;
use crate::base::CubicForm;
use crate::base::FourPointRatQuad;
use crate::base::RatQuadState;
use crate::base::SpecifiedRatQuad;
use serde::Serialize;
use serde_default::DefaultFromSerde;

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct ManagedRatQuad {
   pub ooe: BaseRatQuad,
   pub poly: BaseRatQuad,
   pub specified: SpecifiedRatQuad, // FourPoint or ThreePointAngle.
   pub canvas_range: [f64; 4],
}

#[allow(clippy::missing_panics_doc)]
impl ManagedRatQuad {
   #[must_use]
   pub fn create_from_polynomial(poly: &BaseRatQuad, canvas_range: [f64; 4]) -> Self {
      assert!(poly.state == RatQuadState::RationalPoly);
      Self { poly: *poly, ooe: BaseRatQuad::default(), canvas_range, ..Default::default() }
   }

   #[must_use]
   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::similar_names)]
   #[allow(clippy::suboptimal_flops)]
   pub fn create_from_four_points(four_points: &FourPointRatQuad, canvas_range: [f64; 4]) -> Self {
      assert!(four_points.state == RatQuadState::FourPoint);
      let x = &four_points.x;
      let y = &four_points.y;
      let delta_x = (x[2] - x[3]) * (y[1] - y[0]);
      let delta_y = (y[2] - y[3]) * (x[1] - x[0]);
      let w_b = delta_x - delta_y;
      let w_b_x_m = (y[3] - y[0]) * (x[2] - x[3]) * (x[1] - x[0]) - x[3] * delta_y + x[0] * delta_x;
      // If we exchange all x and y then we also negate, by implication, w_b.
      let w_b_y_m =
         -1.0 * ((x[3] - x[0]) * (y[2] - y[3]) * (y[1] - y[0]) - y[3] * delta_x + y[0] * delta_y);
      let w_a = 2.0 / 3.0 * (x[0] * (y[2] - y[3]) + x[2] * (y[3] - y[0]) + x[3] * (y[0] - y[2]));
      let w_c = -2.0 / 3.0 * (y[0] * (x[2] - x[3]) + y[2] * (x[3] - x[0]) + y[3] * (x[0] - x[2]));

      let b = [w_a * x[0], w_b_x_m, w_c * x[3]];
      let c = [w_a * y[0], w_b_y_m, w_c * y[3]];
      let a = [w_a, w_b, w_c];
      let mut rat_quad = BaseRatQuad {
         state: RatQuadState::RationalWeighted,
         r: four_points.r,
         a,
         b,
         c,
         ..Default::default()
      };
      rat_quad.weighted_to_polynomial().unwrap();
      Self {
         poly: rat_quad,
         ooe: BaseRatQuad::default(),
         specified: SpecifiedRatQuad::FourPoint(*four_points),
         canvas_range,
         // ..Default::default()
      }
   }
   #[must_use]
   pub fn create_from_three_points(
      three_point_rat_quad: &BaseRatQuad,
      canvas_range: [f64; 4],
   ) -> Self {
      assert!(three_point_rat_quad.state == RatQuadState::ThreePointAngle);
      let xs = &three_point_rat_quad.b;
      let ys = &three_point_rat_quad.c;
      let f_mult_1p5 = three_point_rat_quad.angle.cos();
      // Can construct as four-point rat quad with these values.
      // let x = [xs[0], f * xs[1] + (1.0 - f) * xs[0], f * xs[1] + (1.0 - f) * xs[2], xs[2]];
      // let y = [ys[0], f * ys[1] + (1.0 - f) * ys[0], f * ys[1] + (1.0 - f) * ys[2], ys[2]];

      let b = [xs[0], f_mult_1p5 * xs[1], xs[2]];
      let c = [ys[0], f_mult_1p5 * ys[1], ys[2]];
      let a = [1.0, f_mult_1p5, 1.0];
      let mut rat_quad = BaseRatQuad {
         state: RatQuadState::RationalWeighted,
         r: three_point_rat_quad.r,
         a,
         b,
         c,
         ..Default::default()
      };
      rat_quad.weighted_to_polynomial().unwrap();
      Self {
         poly: rat_quad,
         ooe: BaseRatQuad::default(),
         specified: SpecifiedRatQuad::ThreePointAngle(*three_point_rat_quad),
         canvas_range,
         // ..Default::default()
      }
   }

   #[must_use]
   pub const fn get_ooe_rat_quad(&self) -> &BaseRatQuad {
      &self.ooe
   }
   #[must_use]
   pub const fn get_poly_rat_quad(&self) -> &BaseRatQuad {
      &self.poly
   }

   #[allow(clippy::missing_errors_doc)]
   // Velocity at beginning multiplied by sigma, and velocity at end divided by sigma.
   pub fn apply_bilinear(&mut self, sigma: f64) -> Result<(), &'static str> {
      self.poly.apply_bilinear(sigma)
   }

   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_symmetric_range(&mut self) -> Result<(), &'static str> {
      self.poly.raise_to_symmetric_range()
   }
   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_regularized_symmetric(&mut self) -> Result<(), &'static str> {
      self.poly.raise_to_regularized_symmetric()
   }
   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_offset_odd_even(&mut self) -> Result<(), &'static str> {
      self.ooe.raise_to_offset_odd_even(&self.poly, 0.01)
   }
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct ManagedCubic {
   pub four_point: CubiLinear,
   pub canvas_range: [f64; 4],
}

#[allow(clippy::missing_panics_doc)]
impl ManagedCubic {
   #[must_use]
   pub const fn create_from_control_points(
      control_points: &CubiLinear,
      canvas_range: [f64; 4],
   ) -> Self {
      let mut ret_val = Self { four_point: *control_points, canvas_range };
      ret_val.four_point.form = CubicForm::FourPoint;
      ret_val
   }

   #[must_use]
   pub const fn get_form(&self) -> CubicForm {
      self.four_point.form
   }

   #[must_use]
   pub const fn get_four_point(&self) -> &CubiLinear {
      &self.four_point
   }

   pub fn displace(&mut self, d: [f64; 2]) {
      self.four_point.displace(d);
   }

   pub fn bilinear_transform(&mut self, sigma: f64) {
      self.four_point.bilinear_transform(sigma);
   }

   pub fn adjust_range(&mut self, new_range: [f64; 2]) {
      self.four_point.adjust_range(new_range);
   }

   pub fn select_range(&mut self, new_range: [f64; 2]) {
      self.four_point.select_range(new_range);
   }
}
