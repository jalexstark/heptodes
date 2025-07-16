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

use crate::base::{
   BaseRatQuad, CubiLinear, FourPointCubiLinearRepr, FourPointRatQuad, RatQuadOoeSubclassed,
   RatQuadRepr, SpecifiedRatQuad,
};
use serde::Serialize;
use serde_default::DefaultFromSerde;

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct ManagedRatQuad {
   pub poly: BaseRatQuad,
   // How originally specified, FourPoint or ThreePointAngle, for plotting and diagnostics only.
   pub specified: SpecifiedRatQuad,
   pub canvas_range: [f64; 4],
}

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
   pub fn create_from_polynomial(poly: &BaseRatQuad, canvas_range: [f64; 4]) -> Self {
      assert!(matches!(poly, BaseRatQuad::RationalPoly { .. }));
      Self { poly: *poly, canvas_range, ..Default::default() }
   }

   #[must_use]
   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::similar_names)]
   #[allow(clippy::suboptimal_flops)]
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

      let b = [w_a * x[0], w_b_x_m, w_c * x[3]];
      let c = [w_a * y[0], w_b_y_m, w_c * y[3]];
      let a = [w_a, w_b, w_c];
      let mut rat_quad = BaseRatQuad::RationalWeighted(RatQuadRepr {
         r: four_points.r,
         a,
         b,
         c,
         ..Default::default()
      });
      rat_quad.weighted_to_polynomial().unwrap();
      Self { poly: rat_quad, specified: SpecifiedRatQuad::FourPoint(*four_points), canvas_range }
   }

   #[allow(clippy::missing_errors_doc)]
   pub fn create_from_three_points(
      three_point_rat_quad_base: &BaseRatQuad,
      canvas_range: [f64; 4],
   ) -> Result<Self, &'static str> {
      assert!(matches!(three_point_rat_quad_base, BaseRatQuad::ThreePointAngle { .. }));
      if let BaseRatQuad::ThreePointAngle(three_point_rat_quad) = three_point_rat_quad_base {
         let xs = extract_x_from_3(&three_point_rat_quad.p);
         let ys = extract_y_from_3(&three_point_rat_quad.p);
         let f_mult_1p5 = three_point_rat_quad.angle.cos();
         // Can construct as four-point rat quad with these values.
         // let x = [xs[0], f * xs[1] + (1.0 - f) * xs[0], f * xs[1] + (1.0 - f) * xs[2], xs[2]];
         // let y = [ys[0], f * ys[1] + (1.0 - f) * ys[0], f * ys[1] + (1.0 - f) * ys[2], ys[2]];

         let b = [xs[0], f_mult_1p5 * xs[1], xs[2]];
         let c = [ys[0], f_mult_1p5 * ys[1], ys[2]];
         let a = [1.0, f_mult_1p5, 1.0];
         let mut rat_quad = BaseRatQuad::RationalWeighted(RatQuadRepr {
            r: three_point_rat_quad.r,
            a,
            b,
            c,
            ..Default::default()
         });
         rat_quad.weighted_to_polynomial().unwrap();
         Ok(Self {
            poly: rat_quad,
            specified: SpecifiedRatQuad::ThreePointAngle(*three_point_rat_quad),
            canvas_range,
         })
      } else {
         Err("Can only create 3-point form from ThreePointAngle.")
      }
   }

   #[must_use]
   pub const fn get_regularized_rat_quad(&self) -> &BaseRatQuad {
      // TODO: Yuck!
      assert!(matches!(&self.poly, BaseRatQuad::RegularizedSymmetric { .. }));
      &self.poly
   }

   #[allow(clippy::missing_errors_doc)]
   pub const fn get_poly_rat_quad_repr(&self) -> Result<RatQuadRepr, &'static str> {
      self.poly.get_poly()
   }

   #[allow(clippy::missing_errors_doc)]
   // Velocity at beginning multiplied by sigma, and velocity at end divided by sigma.
   pub fn apply_bilinear(&mut self, sigma_n: f64, sigma_d: f64) -> Result<(), &'static str> {
      self.poly.apply_bilinear(sigma_n, sigma_d)
   }

   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_regularized_symmetric(&mut self) -> Result<(), &'static str> {
      self.poly.raise_to_regularized_symmetric()
   }

   #[allow(clippy::missing_errors_doc)]
   pub fn classify_offset_odd_even(&self) -> Result<RatQuadOoeSubclassed, &'static str> {
      if let BaseRatQuad::RegularizedSymmetric(reg_symmetric) = &self.poly {
         Ok(reg_symmetric.classify_offset_odd_even(0.01))
      } else {
         Err("Classification into odd-even must be from regularized form.")
      }
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
      control_points: &FourPointCubiLinearRepr,
      canvas_range: [f64; 4],
   ) -> Self {
      Self { four_point: CubiLinear::FourPoint(*control_points), canvas_range }
   }

   #[allow(clippy::missing_errors_doc)]
   pub const fn get_four_point(&self) -> Result<FourPointCubiLinearRepr, &'static str> {
      assert!(matches!(self.four_point, CubiLinear::FourPoint { .. }));
      if let CubiLinear::FourPoint(four_point) = self.four_point {
         Ok(four_point)
      } else {
         Err("Managed cubic must be four-point")
      }
   }

   pub fn displace(&mut self, d: [f64; 2]) {
      assert!(matches!(self.four_point, CubiLinear::FourPoint { .. }));
      if let CubiLinear::FourPoint(four_point) = &mut self.four_point {
         four_point.displace(d);
      }
   }

   pub fn bilinear_transform(&mut self, sigma: f64) {
      if let CubiLinear::FourPoint(four_point) = &mut self.four_point {
         four_point.bilinear_transform(sigma);
      }
   }

   pub fn adjust_range(&mut self, new_range: [f64; 2]) {
      if let CubiLinear::FourPoint(four_point) = &mut self.four_point {
         four_point.adjust_range(new_range);
      }
   }

   pub fn select_range(&mut self, new_range: [f64; 2]) {
      if let CubiLinear::FourPoint(four_point) = &mut self.four_point {
         four_point.select_range(new_range);
      }
   }
}
