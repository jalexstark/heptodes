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

use crate::is_default;
use crate::matrix::CurveMatrix;
use crate::{
   default_unit_ratio, is_default_unit_ratio, q_mat_power_to_weighted, q_mat_weighted_to_power,
};
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;

// Zero angle is in direction of x axis.
#[derive(Debug, Serialize, Clone, DefaultFromSerde, PartialEq)]
pub struct ArcPath {
   #[serde(skip_serializing_if = "is_default")]
   pub angle_range: [f64; 2],
   #[serde(skip_serializing_if = "is_default")]
   pub center: [f64; 2],
   // Elliptical transform matrix.
   #[serde(skip_serializing_if = "is_default")]
   pub transform: [f64; 4],
}

// Homogenous representation of cubic and rational quadratic curve.
#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct CubicHomog(pub [[f64; 4]; 2]); // Denominator assumed to be a power series.
#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct RatQuadHomog(pub [[f64; 3]; 3]); // "Denominator" in third row.

// r[0] is the value of t at p[0], and r[1] is value of t at p[3].
#[derive(Debug, Serialize, Deserialize, Clone, DefaultFromSerde, PartialEq)]
pub struct CubicFourPoint {
   pub r: [f64; 2], // Range.
   pub h: CubicHomog,
   #[serde(skip_serializing_if = "is_default_unit_ratio", default = "default_unit_ratio")]
   pub sigma: (f64, f64),
}

// Standard (weighted) form.
#[derive(Debug, Serialize, Deserialize, Clone, DefaultFromSerde, PartialEq)]
pub struct CubicPath {
   pub r: [f64; 2], // Range.
   pub h: CubicHomog,
   #[serde(skip_serializing_if = "is_default_unit_ratio", default = "default_unit_ratio")]
   pub sigma: (f64, f64),
}

// TODO: This is used for both weighted and power. First step, create RatQuadPolyPathWeighted
// and RatQuadPolyPathPower, and split uses correctly. Second step, remove and replace with
// Homog versions.
#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct RatQuadPolyPathPower {
   pub r: [f64; 2], // Range.
   pub a: [f64; 3], // Denominator, as a[2] * t^2 + a[1] * t... .
   pub b: [f64; 3], // Numerator for x component.
   pub c: [f64; 3], // Numerator for y component.
   #[serde(skip_serializing_if = "is_default_unit_ratio", default = "default_unit_ratio")]
   pub sigma: (f64, f64),
}

// TODO: This is used for both weighted and power. First step, create RatQuadPolyPathWeighted
// and RatQuadPolyPathPower, and split uses correctly. Second step, remove and replace with
// Homog versions.
#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct RatQuadPolyPathWeighted {
   pub r: [f64; 2], // Range.
   pub a: [f64; 3], // Denominator, as a[2] * t^2 + a[1] * t... .
   pub b: [f64; 3], // Numerator for x component.
   pub c: [f64; 3], // Numerator for y component.
   #[serde(skip_serializing_if = "is_default_unit_ratio", default = "default_unit_ratio")]
   pub sigma: (f64, f64),
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct RatQuadHomogPower {
   pub r: [f64; 2], // Range.
   pub h: RatQuadHomog,
   #[serde(skip_serializing_if = "is_default_unit_ratio", default = "default_unit_ratio")]
   pub sigma: (f64, f64),
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct RatQuadHomogWeighted {
   pub r: [f64; 2], // Range.
   pub h: RatQuadHomog,
   #[serde(skip_serializing_if = "is_default_unit_ratio", default = "default_unit_ratio")]
   pub sigma: (f64, f64),
}

impl From<&RatQuadHomogWeighted> for RatQuadHomogPower {
   fn from(weighted: &RatQuadHomogWeighted) -> Self {
      let r = &weighted.r;

      let tran_q_mat = q_mat_weighted_to_power(r);
      let out_quad_homog = weighted.h.apply_q_mat(&tran_q_mat);

      Self { r: *r, h: out_quad_homog, sigma: weighted.sigma }
   }
}

impl From<&RatQuadHomogPower> for RatQuadHomogWeighted {
   fn from(power: &RatQuadHomogPower) -> Self {
      let r = &power.r;

      let tran_q_mat = q_mat_power_to_weighted(r);
      let out_quad_homog = power.h.apply_q_mat(&tran_q_mat);

      Self { r: *r, h: out_quad_homog, sigma: power.sigma }
   }
}

impl From<&RatQuadPolyPathPower> for RatQuadHomogPower {
   fn from(poly: &RatQuadPolyPathPower) -> Self {
      Self { r: poly.r, h: RatQuadHomog([poly.b, poly.c, poly.a]), sigma: poly.sigma }
   }
}

impl From<&RatQuadHomogPower> for RatQuadPolyPathPower {
   fn from(homog: &RatQuadHomogPower) -> Self {
      Self { r: homog.r, a: homog.h.0[2], b: homog.h.0[0], c: homog.h.0[1], sigma: homog.sigma }
   }
}

impl From<&RatQuadPolyPathWeighted> for RatQuadHomogWeighted {
   fn from(poly: &RatQuadPolyPathWeighted) -> Self {
      Self { r: poly.r, h: RatQuadHomog([poly.b, poly.c, poly.a]), sigma: poly.sigma }
   }
}

impl From<&RatQuadHomogWeighted> for RatQuadPolyPathWeighted {
   fn from(homog: &RatQuadHomogWeighted) -> Self {
      Self { r: homog.r, a: homog.h.0[2], b: homog.h.0[0], c: homog.h.0[1], sigma: homog.sigma }
   }
}

pub type PolylinePath = Vec<[f64; 2]>;

// Path is:
//
// offset + minus_partial / (lambda - mu * t) + plus_partial / (lambda + mu * t).
#[derive(Debug, Serialize, Deserialize, DefaultFromSerde, PartialEq, Clone)]
pub struct HyperbolicPath {
   pub range: (f64, f64),
   pub lambda: f64,
   pub mu: f64,
   pub offset: [f64; 2],
   pub minus_partial: [f64; 2],
   pub plus_partial: [f64; 2],
   #[serde(skip_serializing_if = "is_default_unit_ratio", default = "default_unit_ratio")]
   pub sigma: (f64, f64),
}

#[derive(Serialize, Debug, Default, PartialEq)]
pub enum OneOfSegment {
   #[default]
   Neither,
   Arc(ArcPath),
   Cubic(CubicPath),
   Hyperbolic(HyperbolicPath),
   Polyline(PolylinePath),
}
