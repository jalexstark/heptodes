// Copyright 2022 Google LLC
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

use crate::jaywalk_graph::absent_f64;
use serde::{Deserialize, Serialize};

pub const INT32_MISSING_VAL: i32 = i32::MIN;
pub const FLOAT_MISSING_VAL: f64 = f64::NAN;

pub const MULTIPLICATIVE_ID_F64: f64 = 1.0f64;
pub const ADDITIVE_ID_F64: f64 = 0.0f64;

pub struct JKey(pub i32);
pub struct JVec<T>(pub Vec<T>);

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub struct Coord(pub f64, pub f64);

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub struct TMatrix(pub f64, pub f64, pub f64, pub f64);

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub enum StateMark {
   Unfit = 0,
   Dirty,
   Derived,
   Fit,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub enum Yna {
   Auto = 0,
   Yes,
   No,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub enum Yon {
   No = 0,
   Other,
   Yes,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub enum Bidirection {
   Auto = 0,
   Forward,
   Backward,
   Neither,
   Both,
}

// Cascade is [m_2 & c_2 \\ 0 & 1] * [m_1 & c_1 \\ 0 & 1], and so
// combined scale = m_2 * m_1 and combined offset = m_2 * c_1 + c_2.
// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub struct JaywalkAffine {
   #[serde(default = "JaywalkAffine::default_value_scale")]
   pub scale: f64,
   #[serde(default = "JaywalkAffine::default_value_offset")]
   pub offset: f64,

   // The value is calculated based off the parent value.
   #[serde(skip)]
   // DANGER: Field default is tied to implementation of struct default.
   #[serde(default)]
   pub value: f64,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub enum Finish {
   Auto,
   Open, // Transparent / no-fill, open outline.
   FG,   // Closed outline, foreground fill.
   BG,   // Closed outline, background fill.
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub enum LineType {
   Auto,
   Solid,
   Dashed,
   Dotted,
   Chain,
}

// For now, just fixed patterns. In future a pattern vector could override.
// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize, Default)]
pub struct LineStyle {
   // DANGER: Field default is tied to implementation of struct default.
   #[serde(default)]
   pub line_type: LineType,
   // DANGER: Field default is tied to implementation of struct default.
   #[serde(default)]
   pub pattern_length: JaywalkAffine, // Affine calculation based on resolved line width.
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub enum Octant {
   Auto = 0,
   N = 1,
   NW = 2,
   W = 3,
   SW = 4,
   S = 5,
   SE = 6,
   E = 7,
   NE = 8,
   // Also consider base, mid-line, other height alignments.
   //   // Also CL or ML for centre (mid-line) left, and CR or MR on right. These
   //   // would align with, say the centre of "+" and "=".
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub struct Anchorage {
   // DANGER: Field default is tied to implementation of struct default.
   #[serde(default)]
   pub octant: Octant,

   // DANGER: Field default is tied to implementation of struct default.
   pub orig_degrees: Option<f64>,

   #[serde(default = "absent_f64")]
   pub degrees: f64,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub enum Shape {
   Auto = 0,
   Circle = 1,
   Rectangle = 2,
   // Vanish = 3,
}

// Mark as: Not yet completely migrated.
#[derive(Serialize, Deserialize)]
pub enum ArrowType {
   Auto,
   Simple, // Simple triangle / wedge.
   Curly,  // Single curly.
}

// Mark as: Not yet completely migrated.
// #[derive(Serialize, Deserialize)]
// pub enum TransformType {
//     Auto,
//     // The most standard, with alpha-beta parameterization.
//     Diagonally,
//     SquareUp, // A size-dependent stretch in the 45-degree direction.
// }
