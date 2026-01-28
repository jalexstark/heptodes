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
use crate::{Curve, CurveEval};
use approx::assert_abs_diff_eq;
use zvx_base::rat_quad_expand_power;
use zvx_base::utils::CoordSliceWrapped;
use zvx_base::utils::PathWrapped;
use zvx_base::{q_reduce, rat_quad_rq_eval, RatQuadHomog, RatQuadHomogPower, RatQuadHomogWeighted};

// Done: Checklist, Eval end points.
// Done: Checklist, Eval end point derivatives.
// Done: Checklist, Eval derivatives.
// Done: Checklist, Euler diff test derivatives.
// Done: Checklist, Eval.
// TODO: Checklist, Test collapse bilinear with eval (not cubic).
// TODO: Checklist, Eval without bilinear (internal).
// TODO: Checklist, Four-point specification.
// TODO: Checklist, Three-point specification.
// TODO: Checklist, Range cut / select.
// TODO: Checklist, Test range cut, perhaps via bilinear collapse.
// TODO: Checklist, Test solvable split for three-point.
// TODO: Checklist, Test solvable split for four-point.
// Done: Checklist, Test transformation of form.
// TODO: Checklist, Test direct modify bilinear and range.
// TODO: Checklist, Test displace (method and adjust eval).
//
// Next: Eval test.
// Next: Collapse bilinear.
// Next: Look for further removal of power CurveEval and TEval.
// TODO: Fill out CurveTransform trait, both RQC and cubic.

#[allow(clippy::suboptimal_flops)]
#[allow(clippy::missing_errors_doc)]
#[allow(clippy::many_single_char_names)]
// This is really the original version, before the matrix method was created.  It serves as a
// cross-check.
fn reference_create_power_from_weighted(
   weighted: &Curve<RatQuadHomogWeighted>,
) -> Curve<RatQuadHomogPower> {
   // Get from rat_poly.sigma once confirmed working.
   let sigma = 1.0;
   let v = weighted.path.r[0];
   let w = weighted.path.r[1];
   let a;
   let b;
   let c;
   {
      let h0 = weighted.path.h.0[2][0];
      let h1 = 0.5 * sigma * weighted.path.h.0[2][1];
      let h2 = sigma * sigma * weighted.path.h.0[2][2];
      a = [
         w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
         2.0 * (-w * h0 + (w + v) * h1 - v * h2),
         h0 - 2.0 * h1 + h2,
      ];
   }
   {
      let h0 = weighted.path.h.0[0][0];
      let h1 = 0.5 * sigma * weighted.path.h.0[0][1];
      let h2 = sigma * sigma * weighted.path.h.0[0][2];
      b = [
         w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
         2.0 * (-w * h0 + (w + v) * h1 - v * h2),
         h0 - 2.0 * h1 + h2,
      ];
   }
   {
      let h0 = weighted.path.h.0[1][0];
      let h1 = 0.5 * sigma * weighted.path.h.0[1][1];
      let h2 = sigma * sigma * weighted.path.h.0[1][2];
      c = [
         w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
         2.0 * (-w * h0 + (w + v) * h1 - v * h2),
         h0 - 2.0 * h1 + h2,
      ];
   }

   Curve::<RatQuadHomogPower> {
      path: RatQuadHomogPower {
         r: weighted.path.r,
         h: RatQuadHomog([b, c, a]),
         sigma: weighted.path.sigma,
      },
   }
}

#[allow(clippy::unreadable_literal)]
fn weighted_example_0() -> RatQuadHomogWeighted {
   RatQuadHomogWeighted {
      r: [-6.0, 14.0],
      h: RatQuadHomog([
         [-2.946278254943949, 0.0, -3.9283710065919317],
         [-2.946278254943949, 2.0 * 0.6944444444444453, 3.9283710065919317],
         [1.9641855032959659, 2.0 * 1.388888888888889, 1.9641855032959659],
      ]),
      sigma: (2.0, 1.5),
   }
}

#[test]
#[allow(clippy::unreadable_literal)]
fn weighted_power_conversion_test() {
   // The data in these tests are not deeply thought out.
   let weighted = Curve { path: weighted_example_0() };

   let reference_powered = reference_create_power_from_weighted(&weighted).path;
   let powered = Curve {
      path: RatQuadHomogPower {
         r: [-6.0, 14.0],
         h: RatQuadHomog([
            [-718.8918942063235, 35.35533905932739, -6.874649261535881],
            [-319.38251506503764, 140.7473543286449, -0.40679613724090746],
            [689.0243700979975, -9.204745830513225, 1.1505932288141536],
         ]),
         sigma: (2.0, 1.5),
      },
   };

   // Test in power form.
   assert_abs_diff_eq!(
      &PathWrapped::from(&reference_powered),
      &PathWrapped::from(&powered.path),
      epsilon = 1.0e-5
   );

   // Test in power form.
   assert_abs_diff_eq!(
      &PathWrapped::from(&powered.path),
      &PathWrapped::from(&RatQuadHomogPower::from(&weighted.path)),
      epsilon = 1.0e-5
   );

   // Test in weighted form.
   assert_abs_diff_eq!(
      &PathWrapped::from(&weighted.path.normalize()),
      &PathWrapped::from(&RatQuadHomogWeighted::from(&powered.path).normalize()),
      epsilon = 1.0e-5
   );

   // Collapse while in power form, tested in weighted.
   let direct = rq_weighted_collapse_bilinear(&weighted.path).normalize();
   let indirect =
      RatQuadHomogWeighted::from(&rq_power_collapse_bilinear(&powered.path)).normalize();
   assert_abs_diff_eq!(
      &PathWrapped::from(&direct),
      &PathWrapped::from(&indirect),
      epsilon = 1.0e-5
   );
   assert_abs_diff_eq!(direct.sigma.0, 1.0, epsilon = 1.0e-5);
   assert_abs_diff_eq!(direct.sigma.1, 1.0, epsilon = 1.0e-5);
   assert_abs_diff_eq!(indirect.sigma.0, 1.0, epsilon = 1.0e-5);
   assert_abs_diff_eq!(indirect.sigma.1, 1.0, epsilon = 1.0e-5);
}

// This uses the power form eval, and can be used as alternative route to cross-check weighted.
// There is some underlying code in common.
fn reference_eval_with_bilinear(weighted: &RatQuadHomogWeighted, t: &[f64]) -> Vec<[f64; 2]> {
   let scratchy_rat_weighted =
      RatQuadHomogWeighted::from(&rq_power_collapse_bilinear(&RatQuadHomogPower::from(weighted)));

   q_reduce(&rat_quad_rq_eval(
      &RatQuadHomogPower::from(&scratchy_rat_weighted).h.0,
      &rat_quad_expand_power(t),
   ))
}

#[test]
#[allow(clippy::unreadable_literal)]
fn eval_test() {
   const NUM_SEGMENTS: i32 = 10;
   let weighted = weighted_example_0();

   let t_int: Vec<i32> = (0..=NUM_SEGMENTS).collect();
   let mut t = Vec::<f64>::with_capacity(t_int.len());
   let scale = (weighted.r[1] - weighted.r[0]) / f64::from(NUM_SEGMENTS);

   let offset = weighted.r[0];
   for item in &t_int {
      t.push(f64::from(*item).mul_add(scale, offset));
   }

   let curve = Curve::<RatQuadHomogWeighted> { path: weighted.clone() };

   {
      let points = curve.path.eval_with_bilinear(&t);
      let reference_points = reference_eval_with_bilinear(&weighted, &t);

      assert_abs_diff_eq!(
         &CoordSliceWrapped::from(&points[..]),
         &CoordSliceWrapped::from(&reference_points[..]),
         epsilon = 1.0e-5
      );
   }

   {
      // Test application of bilinear to curve structure vs applying directly to time points.
      let test_sigma = (2.5, 3.7);
      let mut bilineared_curve = curve.clone();
      bilineared_curve.path.bilinear_transform(test_sigma);
      let points = reference_eval_with_bilinear(
         &weighted,
         &bilinear_transform_timepoints(&t, test_sigma, curve.path.r),
      );
      let reference_points = reference_eval_with_bilinear(&bilineared_curve.path, &t);

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
   curve: &RatQuadHomogWeighted,
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

// This may be removed if the final implementation is very similar.
#[must_use]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::suboptimal_flops)]
fn reference_weighted_eval_derivative_scaled(
   weighted: &RatQuadHomogWeighted,
   t: &[f64],
   scale: f64,
) -> Vec<[f64; 2]> {
   let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
   for item in t {
      let p = weighted.sigma.0 * (*item - weighted.r[0]);
      let q = weighted.sigma.1 * (weighted.r[1] - *item);
      let b = &weighted.h.0[0];
      let c = &weighted.h.0[1];
      let a = &weighted.h.0[2];
      let expansion_b =
         [a[0] * b[1] - a[1] * b[0], 2.0 * (a[0] * b[2] - a[2] * b[0]), a[1] * b[2] - a[2] * b[1]];
      let expansion_c =
         [a[0] * c[1] - a[1] * c[0], 2.0 * (a[0] * c[2] - a[2] * c[0]), a[1] * c[2] - a[2] * c[1]];
      let rb = eval_part_quad(q, p, &expansion_b);
      let rc = eval_part_quad(q, p, &expansion_c);
      let ra = eval_part_quad(q, p, a);
      let w_minus_v = weighted.r[1] - weighted.r[0];
      let div_factor = weighted.sigma.0 * weighted.sigma.1 * w_minus_v * scale / ra / ra;
      // Note that deriv of sigma tran converted cubic's
      // let recip_denom = scale * f0 * f0 / w_minus_v;
      // to
      // let recip_denom = scale * f0 * f0 * f0 * f0 * w_minus_v *
      //           weighted.sigma.0 * weighted.sigma.1;
      ret_val.push([rb * div_factor, rc * div_factor]);
   }
   ret_val
}

#[test]
#[allow(clippy::unreadable_literal)]
fn derivative_scaled_test() {
   // TODO: Clarify scale.
   const EULER_DELTA: f64 = 0.001;
   const SCALE_NUDGE: f64 = 1.5;

   const NUM_SEGMENTS: i32 = 12;
   let weighted = weighted_example_0();

   let t_int: Vec<i32> = (0..=NUM_SEGMENTS).collect();
   let mut t = Vec::<f64>::with_capacity(t_int.len());
   let scale = SCALE_NUDGE * (weighted.r[1] - weighted.r[0]) / f64::from(NUM_SEGMENTS);

   let offset = weighted.r[0];
   for item in &t_int {
      t.push(f64::from(*item).mul_add(scale, offset));
   }

   let derivatives = weighted.eval_derivative_scaled(&t, scale);
   let euler_derivatives = euler_reference_derivative_scaled(&weighted, &t, scale, EULER_DELTA);
   let reference_derivatives = reference_weighted_eval_derivative_scaled(&weighted, &t, scale);
   assert_abs_diff_eq!(
      &CoordSliceWrapped::from(&derivatives[..]),
      &CoordSliceWrapped::from(&euler_derivatives[..]),
      epsilon = 1.0e-4
   );
   assert_abs_diff_eq!(
      &CoordSliceWrapped::from(&derivatives[..]),
      &CoordSliceWrapped::from(&reference_derivatives[..]),
      epsilon = 1.0e-5
   );
}

// Compare end-point characterization method against separate eval and derivative eval.
#[test]
#[allow(clippy::unreadable_literal)]
fn endpoints_test() {
   let weighted = weighted_example_0();
   let scale = weighted.r[1] - weighted.r[0];
   let t = weighted.r;

   let reference_points = weighted.eval_with_bilinear(&t[..]);
   let reference_derivatives = weighted.eval_derivative_scaled(&t, scale);

   let reference_endpoints = power_characterize_endpoints(&RatQuadHomogPower::from(&weighted));
   let endpoints = weighted.characterize_endpoints();

   assert_abs_diff_eq!(
      &CoordSliceWrapped::from(&endpoints.0[..]),
      &CoordSliceWrapped::from(&reference_endpoints.0[..]),
      epsilon = 1.0e-4
   );
   assert_abs_diff_eq!(
      &CoordSliceWrapped::from(&endpoints.1[..]),
      &CoordSliceWrapped::from(&reference_endpoints.1[..]),
      epsilon = 1.0e-4
   );
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

#[must_use]
fn reference_weighted_select_range(
   weighted: &RatQuadHomogWeighted,
   new_range: [f64; 2],
) -> RatQuadHomogWeighted {
   let mut power_curve = RatQuadHomogPower::from(&rq_weighted_collapse_bilinear(weighted));
   power_curve.r = new_range;
   RatQuadHomogWeighted::from(&power_curve)
}

// In future actually test the sigma is correct.
#[test]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn select_range_test() {
   let weighted = weighted_example_0();
   let orig_scale: f64 = 1.0; // weighted.r[1] - weighted.r[0];
   let orig_segments = (weighted.r[1] - weighted.r[0]).round() as i32;
   let orig_steps: Vec<i32> = (0..=orig_segments).collect();
   let mut t_orig = Vec::<f64>::with_capacity(orig_steps.len());
   let orig_offset = weighted.r[0];
   for item in &orig_steps {
      t_orig.push(f64::from(*item).mul_add(orig_scale, orig_offset));
   }

   let orig_points = reference_eval_with_bilinear(&weighted, &t_orig);
   let orig_derivatives = weighted.eval_derivative_scaled(&t_orig, 1.0);
   {
      // Make shorter, but starting same place to check sigma.
      const CUT_POINT: f64 = 1.0; // All ranges need to be integers for this test.

      let cut_range = [weighted.r[0], CUT_POINT];
      let cut_scale: f64 = 1.0; // cut_range[1] - cut_range[0];
      let cut_segments = (cut_range[1] - cut_range[0]).round() as i32;
      let cut_steps: Vec<i32> = (0..=cut_segments).collect();
      let mut t_cut = Vec::<f64>::with_capacity(cut_steps.len());
      let cut_offset = cut_range[0];
      for item in &cut_steps {
         t_cut.push(f64::from(*item).mul_add(cut_scale, cut_offset));
      }

      let mut weighted_cut = weighted.clone();
      weighted_cut.select_range(cut_range);
      let reference_weighted_cut = reference_weighted_select_range(&weighted, cut_range);
      let cut_points = weighted_cut.eval_with_bilinear(&t_cut);
      let cut_derivatives = weighted_cut.eval_derivative_scaled(&t_cut, 1.0);
      let reference_cut_points = reference_weighted_cut.eval_with_bilinear(&t_cut);
      let reference_cut_derivatives = reference_weighted_cut.eval_derivative_scaled(&t_cut, 1.0);

      assert_abs_diff_eq!(
         &CoordSliceWrapped::from(&cut_points[..]),
         &CoordSliceWrapped::from(&reference_cut_points[..]),
         epsilon = 1.0e-4
      );
      assert_abs_diff_eq!(
         &CoordSliceWrapped::from(&cut_derivatives[..]),
         &CoordSliceWrapped::from(&reference_cut_derivatives[..]),
         epsilon = 1.0e-4
      );
      assert_abs_diff_eq!(
         &CoordSliceWrapped::from(&cut_points[..]),
         &CoordSliceWrapped::from(&orig_points[0..=cut_segments as usize]),
         epsilon = 1.0e-4
      );
      assert_abs_diff_eq!(
         &CoordSliceWrapped::from(&cut_derivatives[..]),
         &CoordSliceWrapped::from(&orig_derivatives[0..=cut_segments as usize]),
         epsilon = 1.0e-4
      );
   }
   {
      // Make longer, but ending same place to check sigma.
      const CUT_POINT: f64 = -10.0; // All ranges need to be integers for this test.

      let cut_range = [CUT_POINT, weighted.r[1]];
      let cut_scale: f64 = 1.0; // cut_range[1] - cut_range[0];
      let cut_segments = (cut_range[1] - cut_range[0]).round() as i32;
      let cut_steps: Vec<i32> = (0..=cut_segments).collect();
      let mut t_cut = Vec::<f64>::with_capacity(cut_steps.len());
      let cut_offset = cut_range[0];
      for item in &cut_steps {
         t_cut.push(f64::from(*item).mul_add(cut_scale, cut_offset));
      }

      let mut weighted_cut = weighted.clone();
      weighted_cut.select_range(cut_range);
      let reference_weighted_cut = reference_weighted_select_range(&weighted, cut_range);
      let cut_points = weighted_cut.eval_with_bilinear(&t_cut);
      let cut_derivatives = weighted_cut.eval_derivative_scaled(&t_cut, 1.0);
      let reference_cut_points = reference_weighted_cut.eval_with_bilinear(&t_cut);
      let reference_cut_derivatives = reference_weighted_cut.eval_derivative_scaled(&t_cut, 1.0);

      assert_abs_diff_eq!(
         &CoordSliceWrapped::from(&cut_points[..]),
         &CoordSliceWrapped::from(&reference_cut_points[..]),
         epsilon = 1.0e-4
      );
      assert_abs_diff_eq!(
         &CoordSliceWrapped::from(&cut_derivatives[..]),
         &CoordSliceWrapped::from(&reference_cut_derivatives[..]),
         epsilon = 1.0e-4
      );
      assert_abs_diff_eq!(
         &CoordSliceWrapped::from(&cut_points[(cut_segments - orig_segments) as usize..]),
         &CoordSliceWrapped::from(&orig_points[..]),
         epsilon = 1.0e-4
      );
      // assert_abs_diff_eq!(
      //    &CoordSliceWrapped::from(&cut_derivatives[cut_segments..]),
      //    &CoordSliceWrapped::from(&orig_derivatives[..]),
      //    epsilon = 1.0e-4
      // );
   }
}
