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

pub mod base;
pub mod cubic;
pub mod managed;
pub mod matrix;
pub mod rat_quad;
pub mod threes;

pub use crate::base::{
   default_unit_sigma, is_default_unit_sigma, Curve, CurveEval, CurveTransform, ZebraixAngle,
};
pub use crate::cubic::ManagedCubic;
pub use crate::managed::ManagedRatQuad;
pub use crate::matrix::{
   q_mat_weighted_to_power, q_reduce, rat_quad_expand_power, rat_quad_power_eval,
   CubicHomogWrapped, CurveMatrix, F64SliceWrapped, QMat, RatQuadHomogWrapped,
};
pub use crate::rat_quad::{
   FourPointRatQuad, RatQuadPolyPath, SpecifiedRatQuad, ThreePointAngleRepr,
};
pub use crate::threes::RatQuadOoeSubclassed;

// #[cfg(test)]
// #[macro_use]
extern crate approx;
