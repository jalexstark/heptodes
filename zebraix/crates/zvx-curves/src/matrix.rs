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

/// Curve matrix mini library.
// #[cfg(test)]
use approx::AbsDiffEq;
// use serde::{Deserialize, Serialize};
// use serde_default::DefaultFromSerde;
use zvx_base::{CubicHomog, RatQuadHomog};

// Transforms are row-major, that is each row nested.
//
// Curve-point vectors are column-major, with a fixed column size nested in a flexible vector.

// Mainly for quadratic curves.
pub type QMat = [[f64; 3]; 3];
pub type QVec = Vec<[f64; 3]>;
// Mainly for cubic curves.
pub type CMat = [[f64; 4]; 4];
pub type CVec = Vec<[f64; 4]>;

#[inline]
#[must_use]
pub fn rat_quad_expand_power(t: &[f64]) -> Vec<[f64; 3]> {
   let mut ret_val = Vec::<[f64; 3]>::with_capacity(t.len());

   for item in t {
      ret_val.push([1.0, *item, *item * *item]);
   }
   ret_val
}

#[inline]
#[must_use]
pub fn cubic_expand_power(t: &[f64]) -> CVec {
   let mut ret_val = CVec::with_capacity(t.len());
   for item in t {
      let sq = *item * *item;
      ret_val.push([1.0, *item, sq, sq * *item]);
   }
   ret_val
}

#[inline]
#[must_use]
#[allow(clippy::suboptimal_flops)]
fn q_power_eval_single(c: &QMat, t: &[f64; 3]) -> [f64; 3] {
   [
      c[0][0] * t[0] + c[0][1] * t[1] + c[0][2] * t[2],
      c[1][0] * t[0] + c[1][1] * t[1] + c[1][2] * t[2],
      c[2][0] * t[0] + c[2][1] * t[1] + c[2][2] * t[2],
   ]
}

#[inline]
#[must_use]
pub fn rat_quad_power_eval(power_curve: &QMat, t: &QVec) -> QVec {
   let mut power_points = QVec::with_capacity(t.len());
   for item in t {
      power_points.push(q_power_eval_single(power_curve, item));
   }
   power_points
}

#[inline]
#[must_use]
pub fn q_reduce(v: &QVec) -> Vec<[f64; 2]> {
   let mut ret_val = Vec::<[f64; 2]>::with_capacity(v.len());
   for item in v {
      let recip = 1.0 / item[2];
      ret_val.push([item[0] * recip, item[1] * recip]);
   }
   ret_val
}

#[derive(PartialEq, Debug)]
pub struct F64SliceWrapped<'a, const N: usize> {
   v: &'a [f64; N],
}

impl<'a, const N: usize> From<&'a [f64; N]> for F64SliceWrapped<'a, N> {
   fn from(unwrapped: &'a [f64; N]) -> Self {
      F64SliceWrapped::<N> { v: unwrapped }
   }
}

// #[cfg(test)]
#[allow(clippy::elidable_lifetime_names)]
impl<'a, const N: usize> AbsDiffEq for F64SliceWrapped<'a, N> {
   type Epsilon = f64;

   fn default_epsilon() -> f64 {
      1.0e-06
   }

   fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
      for i in 0..N {
         if !f64::abs_diff_eq(&self.v[i], &other.v[i], epsilon) {
            return false;
         }
      }
      true
   }
}

#[derive(PartialEq, Debug)]
pub struct RatQuadHomogWrapped<'a> {
   v: &'a RatQuadHomog,
}

impl<'a> From<&'a RatQuadHomog> for RatQuadHomogWrapped<'a> {
   fn from(unwrapped: &'a RatQuadHomog) -> Self {
      RatQuadHomogWrapped { v: unwrapped }
   }
}

// #[cfg(test)]
#[allow(clippy::elidable_lifetime_names)]
impl<'a> AbsDiffEq for RatQuadHomogWrapped<'a> {
   type Epsilon = f64;

   fn default_epsilon() -> f64 {
      1.0e-06
   }

   fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
      for k in 0..3 {
         if !F64SliceWrapped::<3>::abs_diff_eq(
            &F64SliceWrapped::<3>::from(&self.v.0[k]),
            &F64SliceWrapped::<3>::from(&other.v.0[k]),
            epsilon,
         ) {
            return false;
         }
      }
      true
   }
}

#[derive(PartialEq, Debug)]
pub struct CubicHomogWrapped<'a> {
   v: &'a CubicHomog,
}

impl<'a> From<&'a CubicHomog> for CubicHomogWrapped<'a> {
   fn from(unwrapped: &'a CubicHomog) -> Self {
      CubicHomogWrapped { v: unwrapped }
   }
}

// #[cfg(test)]
#[allow(clippy::elidable_lifetime_names)]
impl<'a> AbsDiffEq for CubicHomogWrapped<'a> {
   type Epsilon = f64;

   fn default_epsilon() -> f64 {
      1.0e-06
   }

   fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
      for k in 0..2 {
         if !F64SliceWrapped::<4>::abs_diff_eq(
            &F64SliceWrapped::<4>::from(&self.v.0[k]),
            &F64SliceWrapped::<4>::from(&other.v.0[k]),
            epsilon,
         ) {
            return false;
         }
      }
      true
   }
}

// QMat that will convert a path in weighted form into power form.
#[must_use]
pub fn q_mat_weighted_to_power(r: &[f64; 2]) -> QMat {
   let v = r[0];
   let w = r[1];
   [[w * w, -w * 2.0, 1.0], [-v * w, w + v, -1.0], [v * v, -2.0 * v, 1.0]]
}

pub trait CurveMatrix {
   fn normalize(&mut self);

   #[must_use]
   fn apply_q_mat(&self, tran_q_mat: &QMat) -> Self;
}

#[allow(clippy::suboptimal_flops)]
impl CurveMatrix for RatQuadHomog {
   fn normalize(&mut self) {
      let a = &mut self.0[2];
      let f = 1.0 / (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).abs().sqrt();

      a[0] *= f;
      a[1] *= f;
      a[2] *= f;

      let b = &mut self.0[0];
      b[0] *= f;
      b[1] *= f;
      b[2] *= f;
      let c = &mut self.0[1];
      c[0] *= f;
      c[1] *= f;
      c[2] *= f;
   }

   fn apply_q_mat(&self, tran_q_mat: &QMat) -> Self {
      let in_quad_homog = &self.0;
      Self([
         [
            in_quad_homog[0][0] * tran_q_mat[0][0]
               + in_quad_homog[0][1] * tran_q_mat[1][0]
               + in_quad_homog[0][2] * tran_q_mat[2][0],
            in_quad_homog[0][0] * tran_q_mat[0][1]
               + in_quad_homog[0][1] * tran_q_mat[1][1]
               + in_quad_homog[0][2] * tran_q_mat[2][1],
            in_quad_homog[0][0] * tran_q_mat[0][2]
               + in_quad_homog[0][1] * tran_q_mat[1][2]
               + in_quad_homog[0][2] * tran_q_mat[2][2],
         ],
         [
            in_quad_homog[1][0] * tran_q_mat[0][0]
               + in_quad_homog[1][1] * tran_q_mat[1][0]
               + in_quad_homog[1][2] * tran_q_mat[2][0],
            in_quad_homog[1][0] * tran_q_mat[0][1]
               + in_quad_homog[1][1] * tran_q_mat[1][1]
               + in_quad_homog[1][2] * tran_q_mat[2][1],
            in_quad_homog[1][0] * tran_q_mat[0][2]
               + in_quad_homog[1][1] * tran_q_mat[1][2]
               + in_quad_homog[1][2] * tran_q_mat[2][2],
         ],
         [
            in_quad_homog[2][0] * tran_q_mat[0][0]
               + in_quad_homog[2][1] * tran_q_mat[1][0]
               + in_quad_homog[2][2] * tran_q_mat[2][0],
            in_quad_homog[2][0] * tran_q_mat[0][1]
               + in_quad_homog[2][1] * tran_q_mat[1][1]
               + in_quad_homog[2][2] * tran_q_mat[2][1],
            in_quad_homog[2][0] * tran_q_mat[0][2]
               + in_quad_homog[2][1] * tran_q_mat[1][2]
               + in_quad_homog[2][2] * tran_q_mat[2][2],
         ],
      ])
   }
}
