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

use crate::{
   Curve, CurveTransform, FourPointRatQuad, RatQuadPolyPath, SpecifiedRatQuad, ThreePointAngleRepr,
};
use serde::Serialize;
use serde_default::DefaultFromSerde;

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct ManagedRatQuad {
   pub poly: Curve<RatQuadPolyPath>,
   // How originally specified, FourPoint or ThreePointAngle, for plotting and diagnostics only.
   pub specified: SpecifiedRatQuad,
   pub canvas_range: [f64; 4],
}

// ========== Now really ratquad-managed.  Cubic-managed is in cubic.rs.

const fn extract_x_from_4(p: &[[f64; 2]; 4]) -> [f64; 4] {
   [p[0][0], p[1][0], p[2][0], p[3][0]]
}

const fn extract_y_from_4(p: &[[f64; 2]; 4]) -> [f64; 4] {
   [p[0][1], p[1][1], p[2][1], p[3][1]]
}

const fn extract_x_from_3(p: &[[f64; 2]; 3]) -> [f64; 3] {
   [p[0][0], p[1][0], p[2][0]]
}

const fn extract_y_from_3(p: &[[f64; 2]; 3]) -> [f64; 3] {
   [p[0][1], p[1][1], p[2][1]]
}

#[allow(clippy::missing_panics_doc)]
impl ManagedRatQuad {
   #[must_use]
   pub fn create_from_polynomial(poly: &Curve<RatQuadPolyPath>, canvas_range: [f64; 4]) -> Self {
      Self { poly: poly.clone(), canvas_range, ..Default::default() }
   }

   #[must_use]
   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::similar_names)]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::neg_multiply)]
   pub fn create_from_four_points(four_points: &FourPointRatQuad, canvas_range: [f64; 4]) -> Self {
      let x = extract_x_from_4(&four_points.p);
      let y = extract_y_from_4(&four_points.p);
      let delta_x = (x[2] - x[3]) * (y[1] - y[0]);
      let delta_y = (y[2] - y[3]) * (x[1] - x[0]);
      let w_b = delta_x - delta_y;
      let w_b_x_m = (y[3] - y[0]) * (x[2] - x[3]) * (x[1] - x[0]) - x[3] * delta_y + x[0] * delta_x;
      // If we exchange all x and y then we also negate, by implication, w_b.
      let w_b_y_m =
         -1.0 * ((x[3] - x[0]) * (y[2] - y[3]) * (y[1] - y[0]) - y[3] * delta_x + y[0] * delta_y);
      let w_a = 2.0 / 3.0 * (x[0] * (y[2] - y[3]) + x[2] * (y[3] - y[0]) + x[3] * (y[0] - y[2]));
      let w_c = -2.0 / 3.0 * (y[0] * (x[2] - x[3]) + y[2] * (x[3] - x[0]) + y[3] * (x[0] - x[2]));

      let b = [w_a * x[0], 2.0 * w_b_x_m, w_c * x[3]];
      let c = [w_a * y[0], 2.0 * w_b_y_m, w_c * y[3]];
      let a = [w_a, 2.0 * w_b, w_c];
      let rat_quad = Curve::<RatQuadPolyPath> {
         path: RatQuadPolyPath { r: four_points.r, a, b, c },
         sigma: four_points.sigma,
      };
      Self {
         poly: Curve::<RatQuadPolyPath>::create_from_weighted(&rat_quad).unwrap(),
         specified: SpecifiedRatQuad::FourPoint(four_points.clone()),
         canvas_range,
      }
   }

   #[allow(clippy::missing_errors_doc)]
   pub fn create_from_three_points(
      three_point_rat_quad: &ThreePointAngleRepr,
      canvas_range: [f64; 4],
   ) -> Result<Self, &'static str> {
      let xs = extract_x_from_3(&three_point_rat_quad.p);
      let ys = extract_y_from_3(&three_point_rat_quad.p);
      let f_mult_1p5 = three_point_rat_quad.angle.cos();
      // Can construct as four-point rat quad with these values.
      // let x = [xs[0], f * xs[1] + (1.0 - f) * xs[0], f * xs[1] + (1.0 - f) * xs[2], xs[2]];
      // let y = [ys[0], f * ys[1] + (1.0 - f) * ys[0], f * ys[1] + (1.0 - f) * ys[2], ys[2]];

      let b = [xs[0], 2.0 * f_mult_1p5 * xs[1], xs[2]];
      let c = [ys[0], 2.0 * f_mult_1p5 * ys[1], ys[2]];
      let a = [1.0, 2.0 * f_mult_1p5, 1.0];
      let rat_quad = Curve::<RatQuadPolyPath> {
         path: RatQuadPolyPath { r: three_point_rat_quad.r, a, b, c },
         // TODO: Figure out preferred sigma.
         sigma: three_point_rat_quad.sigma,
      };
      Ok(Self {
         poly: Curve::<RatQuadPolyPath>::create_from_weighted(&rat_quad).unwrap(),
         specified: SpecifiedRatQuad::ThreePointAngle(three_point_rat_quad.clone()),
         canvas_range,
      })
   }

   #[allow(clippy::missing_errors_doc)]
   pub fn get_poly_rat_quad_repr(&self) -> Result<Curve<RatQuadPolyPath>, &'static str> {
      Ok(self.poly.clone())
   }

   #[allow(clippy::missing_errors_doc)]
   // Velocity at beginning multiplied by sigma, and velocity at end divided by sigma.
   pub fn apply_bilinear(&mut self, sigma_ratio: (f64, f64)) -> Result<(), &'static str> {
      self.poly = self.poly.rq_apply_bilinear(sigma_ratio);
      Ok(())
   }

   // Remove as bilinear is properly applied.
   #[allow(clippy::suboptimal_flops)]
   pub fn patch_up_poly_symmetric(&mut self) {
      let rat_poly = self.poly.figure_symmetric_range_rat_quad();

      let r_both = rat_poly.path.r[1];
      let a_s = rat_poly.path.a[2] * r_both * r_both + rat_poly.path.a[0];
      // let a_d = rat_poly.path.a[2] * r * r - rat_poly.path.a[0];
      let combo_s = a_s + rat_poly.path.a[1] * r_both;
      let combo_d = a_s - rat_poly.path.a[1] * r_both;

      let sigma_ratio = (combo_d.abs().sqrt(), combo_s.abs().sqrt());

      self.poly = rat_poly.rq_apply_bilinear(sigma_ratio);
   }

   pub fn raw_change_range(&mut self, new_range: [f64; 2]) {
      self.poly.raw_change_range(new_range);
   }

   // These should account for sigma.
   pub fn select_range(&mut self, new_range: [f64; 2]) {
      self.poly.select_range(new_range);
   }
}
