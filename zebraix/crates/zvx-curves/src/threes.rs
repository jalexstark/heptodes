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

use crate::base::TEval;
use crate::rat_quad::RegularizedRatQuadPath;
use crate::{Curve, RatQuadPolyPath};
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

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
impl OneThreePath {
   pub fn create_from_ordinary(
      poly_curve: &Curve<RatQuadPolyPath>,
      tolerance: f64,
   ) -> Result<Self, &'static str> {
      let ooe_rat_quad_extracted: RatQuadOoeSubclassed =
         RatQuadOoeSubclassed::create_from_ordinary(poly_curve, tolerance).unwrap();

      Ok(ooe_rat_quad_extracted.convert_to_path())
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
