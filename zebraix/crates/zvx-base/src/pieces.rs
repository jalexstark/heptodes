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

// pub type CubicPathUnranged = [[f64; 2]; 4];

// Four-point "standard" form.
//
// r[0] is the value of t at p[0], and r[1] is value of t at p[3].
#[derive(Debug, Serialize, Deserialize, Clone, DefaultFromSerde, PartialEq)]
pub struct CubicPath {
   pub r: [f64; 2], // Range.
   pub p: [[f64; 2]; 4],
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
}

#[derive(Serialize, Debug, Default, PartialEq)]
pub enum OneOfSegment {
   #[default]
   Nothing,
   Arc(ArcPath),
   Cubic(CubicPath),
   Hyperbolic(HyperbolicPath),
   Polyline(PolylinePath),
}
