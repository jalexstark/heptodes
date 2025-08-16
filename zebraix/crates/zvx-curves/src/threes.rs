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

// Three kinds of path, each of which has 3 components: offset and two complementary
// components. The components can be even-odd (sine-cosine) for elliptical, positive-negative
// for hyperbolic, and (implied) linear-quadratic for parabolic.

use crate::base::{Curve, RegularizedRatQuadPath};
use serde::Serialize;
use zvx_base::{ArcPath, CubicPath, HyperbolicPath};

#[derive(Serialize, Default, Debug, Clone, PartialEq)]
pub enum RatQuadOoeSubclassed {
   #[default]
   Nothing,
   // TODO: Elliptical to custom OOE.
   Elliptical(Curve<RegularizedRatQuadPath>),
   Parabolic(Curve<CubicPath>),
   Hyperbolic(Curve<HyperbolicPath>),
}

#[derive(Serialize, Default, Debug, Clone, PartialEq)]
pub enum OneThreePath {
   #[default]
   Nothing,
   Arc(ArcPath),
   Cubic(CubicPath),
   Hyperbolic(HyperbolicPath),
}

#[must_use]
#[allow(clippy::suboptimal_flops)]
#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
pub fn classify_offset_odd_even(
   rat_quad: &Curve<RegularizedRatQuadPath>,
   tolerance: f64,
) -> RatQuadOoeSubclassed {
   let mut rat_poly = rat_quad.clone();
   let orig_rat_poly = rat_quad;

   let r = rat_poly.path.range_bound;
   if (rat_poly.path.a_2.abs() * r * r) < (rat_poly.path.a_0.abs() * tolerance) {
      RatQuadOoeSubclassed::Parabolic(orig_rat_poly.convert_to_parabolic())
   } else if rat_poly.path.a_2.signum() == rat_poly.path.a_0.signum() {
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

         RatQuadOoeSubclassed::Parabolic(orig_rat_poly.convert_to_parabolic())
      } else {
         // Only outcome that actually uses OOE form.
         RatQuadOoeSubclassed::Elliptical(rat_poly)
      }
   } else {
      // {
      let hyperbolic_form = orig_rat_poly.convert_to_hyperbolic();
      let poly = orig_rat_poly;
      let reconstructed: Curve<RegularizedRatQuadPath> = From::from(&hyperbolic_form);
      //      Rat_Quad {
      //    r: orig_rat_poly.r,
      //    a_0: orig_rat_poly.a_0,
      //    a_2: orig_rat_poly.a_2,
      //    // a: [orig_rat_poly.a[0], 0.0, orig_rat_poly.a[2]],
      //    b: orig_rat_poly.b,
      //    c: orig_rat_poly.c,
      //    sigma: orig_rat_poly.sigma,
      // };
      println!("a: [{}, {}, {}]", poly.path.a_0, 0.0, poly.path.a_2);
      println!("b: [{}, {}, {}]", poly.path.b[0], poly.path.b[1], poly.path.b[2]);
      println!("c: [{}, {}, {}]", poly.path.c[0], poly.path.c[1], poly.path.c[2]);

      println!("a: [{}, {}, {}]", reconstructed.path.a_0, 0.0, reconstructed.path.a_2);
      println!(
         "b: [{}, {}, {}]",
         reconstructed.path.b[0], reconstructed.path.b[1], reconstructed.path.b[2]
      );
      println!(
         "c: [{}, {}, {}]",
         reconstructed.path.c[0], reconstructed.path.c[1], reconstructed.path.c[2]
      );

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

impl RatQuadOoeSubclassed {
   #[must_use]
   pub fn convert_to_path(&self) -> OneThreePath {
      match self {
         Self::Nothing => OneThreePath::Nothing,
         Self::Elliptical(ooe_rat_quad) => {
            let r = ooe_rat_quad.path.range_bound;
            let s = 1.0 / ooe_rat_quad.path.a_2.sqrt();
            let mx = ooe_rat_quad.path.b[0];
            let my = ooe_rat_quad.path.c[0];
            let (sx, sy) = (0.5 * s * ooe_rat_quad.path.b[1], 0.5 * s * ooe_rat_quad.path.c[1]);
            let (cx, cy) = (ooe_rat_quad.path.b[2], ooe_rat_quad.path.c[2]);

            // The arc range is [-angle_range, angle_range].
            let angle_range =
               2.0 * (r * (ooe_rat_quad.path.a_2 / ooe_rat_quad.path.a_0).sqrt()).atan();

            OneThreePath::Arc(ArcPath {
               angle_range: [-angle_range, angle_range],
               center: [mx, my],
               transform: [cx, cy, sx, sy],
            })
         }

         Self::Parabolic(four_point) => OneThreePath::Cubic(four_point.path.clone()),

         // Since hyperbolic is not supported in SVG, we do a simple polyline approximation.
         Self::Hyperbolic(hyper_rat_quad) => {
            // let t_int: Vec<i32> = (0..num_segments_hyperbolic).collect();
            // let mut t = Vec::<f64>::with_capacity(t_int.len());
            // let scale = 2.0 * hyper_rat_quad.path.range_bound / f64::from(num_segments_hyperbolic);
            // let offset = -hyper_rat_quad.path.range_bound;
            // for item in &t_int {
            //    t.push(f64::from(*item).mul_add(scale, offset));
            // }

            // let pattern_vec = hyper_rat_quad.eval(&t);

            OneThreePath::Hyperbolic(hyper_rat_quad.path.clone())
         }
      }
   }
}

pub trait TEval {
   fn eval_no_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]>;
}

impl TEval for HyperbolicPath {
   #[must_use]
   #[allow(clippy::suboptimal_flops)]
   fn eval_no_bilinear(&self, t: &[f64]) -> Vec<[f64; 2]> {
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());

      let lambda = self.lambda;
      let mu = self.mu;

      for item in t {
         let x = self.offset[0]
            + self.minus_partial[0] / (lambda - mu * *item)
            + self.plus_partial[0] / (lambda + mu * *item);
         let y = self.offset[1]
            + self.minus_partial[1] / (lambda - mu * *item)
            + self.plus_partial[1] / (lambda + mu * *item);
         ret_val.push([x, y]);
      }

      ret_val
   }
}
