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

pub mod pieces;

pub use pieces::{
   ArcPath, CubicHomog, CubicPath, HyperbolicPath, OneOfSegment, PolylinePath, RatQuadHomog,
};

#[inline]
pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
   t == &T::default()
}

#[must_use]
fn is_near_float(v: f64, w: f64) -> bool {
   (v - w).abs() < 0.0001
}

#[must_use]
pub const fn default_unit_f64() -> f64 {
   1.0
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[must_use]
pub fn is_default_unit_f64(v: &f64) -> bool {
   is_near_float(*v, default_unit_f64())
}
