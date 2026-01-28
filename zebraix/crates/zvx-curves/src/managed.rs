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

use crate::{Curve, FourPointRatQuad, SpecifiedRatQuad, ThreePointAngleRepr};
use serde::Serialize;
use serde_default::DefaultFromSerde;
use zvx_base::{RatQuadHomog, RatQuadHomogWeighted};

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct ManagedRatQuad {
   pub rq_curve: Curve<RatQuadHomogWeighted>,
   // How originally specified, FourPoint or ThreePointAngle, for plotting and diagnostics only.
   pub specified: SpecifiedRatQuad,
   // Used as desired, by renders, for clipping and curve approximation.
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
}
