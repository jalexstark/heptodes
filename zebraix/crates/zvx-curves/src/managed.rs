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

use crate::{Curve, CurveTransform, FourPointRatQuad, SpecifiedRatQuad, ThreePointAngleRepr};
use serde::Serialize;
use serde_default::DefaultFromSerde;
use zvx_base::{RatQuadHomog, RatQuadHomogPower, RatQuadHomogWeighted};

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct ManagedRatQuad {
   pub rq_curve: Curve<RatQuadHomogWeighted>,
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
   pub fn create_from_weighted(
      rq_curve: &Curve<RatQuadHomogWeighted>,
      canvas_range: [f64; 4],
   ) -> Self {
      Self { rq_curve: rq_curve.clone(), canvas_range, ..Default::default() }
   }

   // #[must_use]
   // #[allow(clippy::many_single_char_names)]
   // #[allow(clippy::similar_names)]
   // #[allow(clippy::suboptimal_flops)]
   // #[allow(clippy::neg_multiply)]
   // fn create_from_four_points_broken(
   //    four_points: &FourPointRatQuad,
   //    canvas_range: [f64; 4],
   // ) -> Self {
   //    let x = extract_x_from_4(&four_points.p);
   //    let y = extract_y_from_4(&four_points.p);
   //    let delta_x = (x[2] - x[3]) * (y[1] - y[0]);
   //    let delta_y = (y[2] - y[3]) * (x[1] - x[0]);
   //    let w_b = delta_x - delta_y;
   //    let w_b_x_m = (y[3] - y[0]) * (x[2] - x[3]) * (x[1] - x[0]) - x[3] * delta_y + x[0] * delta_x;
   //    // If we exchange all x and y then we also negate, by implication, w_b.
   //    let w_b_y_m =
   //       -1.0 * ((x[3] - x[0]) * (y[2] - y[3]) * (y[1] - y[0]) - y[3] * delta_x + y[0] * delta_y);
   //    let w_a = 2.0 / 3.0 * (x[0] * (y[2] - y[3]) + x[2] * (y[3] - y[0]) + x[3] * (y[0] - y[2]));
   //    let w_c = -2.0 / 3.0 * (y[0] * (x[2] - x[3]) + y[2] * (x[3] - x[0]) + y[3] * (x[0] - x[2]));

   //    let b = [w_a * x[0], 2.0 * w_b_x_m, w_c * x[3]];
   //    let c = [w_a * y[0], 2.0 * w_b_y_m, w_c * y[3]];
   //    let a = [w_a, 2.0 * w_b, w_c];
   //    let rat_quad = Curve::<RatQuadPolyPathPower> {
   //       path: RatQuadPolyPathPower { r: four_points.r, a, b, c },
   //       sigma: four_points.sigma,
   //    };
   //    Self {
   //       rq_curve: Curve::<RatQuadPolyPathPower>::create_from_weighted(&rat_quad).unwrap(),
   //       specified: SpecifiedRatQuad::FourPoint(four_points.clone()),
   //       canvas_range,
   //    }
   // }

   #[must_use]
   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::similar_names)]
   #[allow(clippy::suboptimal_flops)]
   #[allow(clippy::neg_multiply)]
   pub fn create_from_four_points(four_points: &FourPointRatQuad, canvas_range: [f64; 4]) -> Self {
      let x = extract_x_from_4(&four_points.p);
      let y = extract_y_from_4(&four_points.p);

      let d_x = x[0] - x[3];
      let d_y = y[0] - y[3];
      let q_x0 = 3.0 * (x[1] - x[0]);
      let q_x2 = 3.0 * (x[3] - x[2]);
      let q_y0 = 3.0 * (y[1] - y[0]);
      let q_y2 = 3.0 * (y[3] - y[2]);

      let a_2 = -(q_y0 * d_x - q_x0 * d_y);
      let a_0 = q_y2 * d_x - q_x2 * d_y;
      let a_1 = -(a_0 * (q_x0 * d_x + q_y0 * d_y) + a_2 * (q_x2 * d_x + q_y2 * d_y))
         / (d_x * d_x + d_y * d_y);

      let b_0 = a_0 * x[0];
      let b_2 = a_2 * x[3];
      let c_0 = a_0 * y[0];
      let c_2 = a_2 * y[3];
      let b_1 = a_0 * q_x0 + a_1 / a_0 * b_0;
      let c_1 = a_0 * q_y0 + a_1 / a_0 * c_0;

      let b = [b_0, b_1, b_2];
      let c = [c_0, c_1, c_2];
      let a = [a_0, a_1, a_2];
      let rat_quad = Curve::<RatQuadHomogWeighted> {
         path: RatQuadHomogWeighted {
            r: four_points.r,
            h: RatQuadHomog([b, c, a]),
            sigma: four_points.sigma,
         },
      };

      Self { rq_curve: rat_quad, specified: SpecifiedRatQuad::FourPoint, canvas_range }
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
      let rat_quad = Curve::<RatQuadHomogWeighted> {
         path: RatQuadHomogWeighted {
            r: three_point_rat_quad.r,
            h: RatQuadHomog([b, c, a]),
            sigma: three_point_rat_quad.sigma,
         },
      };
      Ok(Self { rq_curve: rat_quad, specified: SpecifiedRatQuad::ThreePointAngle, canvas_range })
   }

   #[allow(clippy::missing_errors_doc)]
   // Velocity at beginning multiplied by sigma, and velocity at end divided by sigma.
   pub fn apply_bilinear(&mut self, sigma_ratio: (f64, f64)) -> Result<(), &'static str> {
      self.rq_curve = self.rq_curve.rq_apply_bilinear(sigma_ratio);
      Ok(())
   }

   // Only used in one test.  Perhaps change to apply sigma such that velocities match.
   //
   // Remove as bilinear is properly applied.
   #[allow(clippy::suboptimal_flops)]
   pub fn patch_up_poly_symmetric(&mut self) {
      let rat_poly =
         Curve::<RatQuadHomogPower>::from(&self.rq_curve).figure_symmetric_range_rat_quad();

      let r_both = rat_poly.path.r[1];
      let a_s = rat_poly.path.h.0[2][2] * r_both * r_both + rat_poly.path.h.0[2][0];
      // let a_d = rat_poly.path.a[2] * r * r - rat_poly.path.a[0];
      let combo_s = a_s + rat_poly.path.h.0[2][1] * r_both;
      let combo_d = a_s - rat_poly.path.h.0[2][1] * r_both;

      let sigma_ratio = (combo_d.abs().sqrt(), combo_s.abs().sqrt());

      self.rq_curve = self.rq_curve.rq_apply_bilinear(sigma_ratio);
   }

   pub fn raw_change_range(&mut self, new_range: [f64; 2]) {
      self.rq_curve.raw_change_range(new_range);
   }

   // These should account for sigma.
   pub fn select_range(&mut self, new_range: [f64; 2]) {
      self.rq_curve.select_range(new_range);
   }
}
