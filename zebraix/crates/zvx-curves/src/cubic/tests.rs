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

use super::*;
use crate::bilinear_transform_timepoints;
use crate::Curve;
use approx::assert_abs_diff_eq;
use zvx_base::utils::CoordSliceWrapped;
use zvx_base::utils::PathWrapped;
use zvx_base::CubicHomog;
use zvx_base::CubicPath;

// Progress: Checklist, Eval end points.
// TODO: Checklist, Eval end point derivatives.
// TODO: Checklist, Eval derivatives.
// TODO: Checklist, Euler diff test derivatives.
// TODO: Checklist, Eval.
// TODO: Checklist, Eval without bilinear (internal).
// TODO: Checklist, Four-point specification.
// TODO: Checklist, Three-point specification.
// TODO: Checklist, Range cut / select.
// TODO: Checklist, Test range cut, perhaps via bilinear collapse.
// TODO: Checklist, Test solvable split for three-point.
// TODO: Checklist, Test solvable split for four-point.
// TODO: Checklist, Test transformation of form.
// TODO: Checklist, Test direct modify bilinear and range.
// TODO: Checklist, Test displace (method and adjust eval).

#[allow(clippy::unreadable_literal)]
fn clc_example_0() -> CubicPath {
   CubicPath {
      r: [-4.5, 13.5],
      h: CubicHomog([[4.0, 3.0 * 3.5, 3.0 * 4.5, 3.0], [-1.5, 3.0 * -2.0, 3.0 * 1.5, 2.0]]),
      // TODO: Check eval consistency when sigma.1 neq 1.0.
      sigma: (3.6, 1.2),
   }
}

#[allow(clippy::many_single_char_names)]
fn reference_eval_no_bilinear(clc: &CubicPath, t: &[f64]) -> Vec<[f64; 2]> {
   let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
   for item in t {
      let a = *item - clc.r[0];
      let b = clc.r[1] - *item;
      let f0 = 1.0 / (clc.r[1] - clc.r[0]);
      let recip_denom = f0 * f0 * f0;
      let x = cubic_eval_part(b, a, &clc.h.0[0], recip_denom);
      let y = cubic_eval_part(b, a, &clc.h.0[1], recip_denom);
      ret_val.push([x, y]);
   }
   ret_val
}

fn reference_eval_with_bilinear(clc: &CubicPath, t: &[f64]) -> Vec<[f64; 2]> {
   reference_eval_no_bilinear(clc, &bilinear_transform_timepoints(t, clc.sigma, clc.r))
}

#[test]
#[allow(clippy::unreadable_literal)]
fn eval_test() {
   const NUM_SEGMENTS: i32 = 20;
   let clc = clc_example_0();

   let t_int: Vec<i32> = (0..=NUM_SEGMENTS).collect();
   let mut t = Vec::<f64>::with_capacity(t_int.len());
   let scale = (clc.r[1] - clc.r[0]) / f64::from(NUM_SEGMENTS);

   let offset = clc.r[0];
   for item in &t_int {
      t.push(f64::from(*item).mul_add(scale, offset));
   }

   {
      let points = clc.eval_with_bilinear(&t);
      let reference_points = reference_eval_with_bilinear(&clc, &t);

      assert_abs_diff_eq!(
         &CoordSliceWrapped::from(&points[..]),
         &CoordSliceWrapped::from(&reference_points[..]),
         epsilon = 1.0e-5
      );
   }
}

#[must_use]
#[allow(clippy::suboptimal_flops)]
#[allow(clippy::many_single_char_names)]
fn euler_reference_derivative_scaled(
   curve: &CubicPath,
   t: &[f64],
   scale: f64,
   delta: f64,
) -> Vec<[f64; 2]> {
   let mut t_plus = Vec::<f64>::with_capacity(t.len());
   for item in t {
      t_plus.push(*item + delta);
   }

   let points = curve.eval_with_bilinear(t);
   let points_plus = curve.eval_with_bilinear(&t_plus);

   let factor = scale / delta;
   let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
   for (a, b) in points.iter().zip(&points_plus[..]) {
      ret_val.push([factor * (b[0] - a[0]), factor * (b[1] - a[1])]);
   }
   ret_val
}

#[test]
#[allow(clippy::unreadable_literal)]
fn derivative_scaled_test() {
   const EULER_DELTA: f64 = 0.0001;
   const SCALE_NUDGE: f64 = 1.5;

   const NUM_SEGMENTS: i32 = 12;
   let clc = clc_example_0();

   let t_int: Vec<i32> = (0..=NUM_SEGMENTS).collect();
   let mut t = Vec::<f64>::with_capacity(t_int.len());
   let scale = SCALE_NUDGE * (clc.r[1] - clc.r[0]) / f64::from(NUM_SEGMENTS);

   let offset = clc.r[0];
   for item in &t_int {
      t.push(f64::from(*item).mul_add(scale, offset));
   }

   let derivatives = clc.eval_derivative_scaled(&t, scale);
   let reference_derivatives = euler_reference_derivative_scaled(&clc, &t, scale, EULER_DELTA);

   assert_abs_diff_eq!(
      &CoordSliceWrapped::from(&derivatives[..]),
      &CoordSliceWrapped::from(&reference_derivatives[..]),
      epsilon = 1.0e-4
   );
}

// Compare end-point characterization method against separate eval and derivative eval.
#[test]
#[allow(clippy::unreadable_literal)]
fn endpoints_test() {
   let clc = clc_example_0();
   let scale = clc.r[1] - clc.r[0];
   let t = clc.r;

   let reference_points = clc.eval_with_bilinear(&t[..]);
   let reference_derivatives = clc.eval_derivative_scaled(&t, scale);

   let endpoints = clc.characterize_endpoints();

   assert_abs_diff_eq!(
      &CoordSliceWrapped::from(&endpoints.0[..]),
      &CoordSliceWrapped::from(&reference_points[..]),
      epsilon = 1.0e-4
   );
   assert_abs_diff_eq!(
      &CoordSliceWrapped::from(&endpoints.1[..]),
      &CoordSliceWrapped::from(&reference_derivatives[..]),
      epsilon = 1.0e-4
   );
}

#[cfg(test)]
#[allow(clippy::similar_names)]
#[allow(clippy::suboptimal_flops)]
// Older version, retained as a reference implementation.
fn select_range_reference(curve: &mut Curve<CubicPath>, new_range: [f64; 2]) {
   let mut new_x = [0.0; 4];
   let mut new_y = [0.0; 4];

   let a_k = curve.path.sigma.0 * (new_range[0] - curve.path.r[0]);
   let b_k = curve.path.sigma.1 * (curve.path.r[1] - new_range[0]);
   let a_l = curve.path.sigma.0 * (new_range[1] - curve.path.r[0]);
   let b_l = curve.path.sigma.1 * (curve.path.r[1] - new_range[1]);
   let f0_k = 1.0 / (b_k + a_k);
   let recip_denom_k = f0_k * f0_k * f0_k;
   let f0_l = 1.0 / (b_l + a_l);
   let recip_denom_l = f0_l * f0_l * f0_l;
   let in_x =
      [curve.path.h.0[0][0], curve.path.h.0[0][1], curve.path.h.0[0][2], curve.path.h.0[0][3]];
   let in_y =
      [curve.path.h.0[1][0], curve.path.h.0[1][1], curve.path.h.0[1][2], curve.path.h.0[1][3]];
   new_x[0] = cubic_eval_part(b_k, a_k, &in_x, recip_denom_k);
   new_y[0] = cubic_eval_part(b_k, a_k, &in_y, recip_denom_k);
   new_x[3] = cubic_eval_part(b_l, a_l, &in_x, recip_denom_l);
   new_y[3] = cubic_eval_part(b_l, a_l, &in_y, recip_denom_l);
   let kl_numerator_k = curve.path.r[1] * a_k + curve.path.r[0] * b_k;
   let kl_numerator_l = curve.path.r[1] * a_l + curve.path.r[0] * b_l;
   // This is [k, l] bilinearly transformed.
   let selected_range_bilineared = kl_numerator_l / (a_l + b_l) - kl_numerator_k / (a_k + b_k);
   let fudge = selected_range_bilineared / (curve.path.r[1] - curve.path.r[0]);
   let dx_1 = fudge
      * f0_k
      * f0_k
      * (b_k * b_k * (in_x[1] / 3.0 - in_x[0])
         + 2.0 * b_k * a_k * (in_x[2] / 3.0 - in_x[1] / 3.0)
         + a_k * a_k * (in_x[3] - in_x[2] / 3.0));
   new_x[1] = 3.0 * (new_x[0] + dx_1);
   let dy_1 = fudge
      * f0_k
      * f0_k
      * (b_k * b_k * (in_y[1] / 3.0 - in_y[0])
         + 2.0 * b_k * a_k * (in_y[2] / 3.0 - in_y[1] / 3.0)
         + a_k * a_k * (in_y[3] - in_y[2] / 3.0));
   new_y[1] = 3.0 * (new_y[0] + dy_1);
   let dx_1 = fudge
      * f0_l
      * f0_l
      * (b_l * b_l * (in_x[1] / 3.0 - in_x[0])
         + 2.0 * b_l * a_l * (in_x[2] / 3.0 - in_x[1] / 3.0)
         + a_l * a_l * (in_x[3] - in_x[2] / 3.0));
   new_x[2] = 3.0 * (new_x[3] - dx_1);
   let dy_1 = fudge
      * f0_l
      * f0_l
      * (b_l * b_l * (in_y[1] / 3.0 - in_y[0])
         + 2.0 * b_l * a_l * (in_y[2] / 3.0 - in_y[1] / 3.0)
         + a_l * a_l * (in_y[3] - in_y[2] / 3.0));
   new_y[2] = 3.0 * (new_y[3] - dy_1);

   curve.path.sigma.0 = a_l + b_l;
   curve.path.sigma.1 = a_k + b_k;
   curve.path.h.0 = [new_x, new_y];
   curve.path.r = new_range;
}

#[test]
#[allow(clippy::unreadable_literal)]
fn select_range_test() {
   let mut clc = Curve { path: clc_example_0() };

   let new_range = [1.5, 10.5];

   let mut reference_clc = clc.clone();
   select_range_reference(&mut reference_clc, new_range);
   clc.path.select_range(new_range);

   let literal_clc = Curve {
      path: CubicPath {
         r: [1.5, 10.5],
         h: CubicHomog([
            [3.856, 3.0 * 3.80875, 3.0 * 3.658984375, 3.2529296875],
            [0.408, 3.0 * 1.00875, 3.0 * 1.58671875, 1.872802734375],
         ]),
         sigma: (57.6, 36.0),
      },
   };

   assert_abs_diff_eq!(
      &PathWrapped::from(&clc.path),
      &PathWrapped::from(&literal_clc.path),
      epsilon = 1.0e-5
   );

   assert_abs_diff_eq!(
      &PathWrapped::from(&clc.path),
      &PathWrapped::from(&reference_clc.path),
      epsilon = 1.0e-5
   );
}
