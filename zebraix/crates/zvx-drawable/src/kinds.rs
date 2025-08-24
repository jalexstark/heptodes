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

use crate::choices::{
   ColorChoice, ContinuationChoice, LineChoice, LineClosureChoice, PathCompletion, PointChoice,
   TextAnchorChoice, TextOffsetChoice, TextSizeChoice,
};
use serde::Serialize;
use serde_default::DefaultFromSerde;
use zvx_base::{is_default, ArcPath, CubicPath, HyperbolicPath, OneOfSegment, PolylinePath};

#[derive(Serialize, Debug, Clone, DefaultFromSerde, PartialEq, Eq)]
pub struct SegmentChoices {
   #[serde(skip_serializing_if = "is_default")]
   pub continuation: ContinuationChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub closure: LineClosureChoice,
}

#[derive(Serialize, Debug, Clone, DefaultFromSerde, PartialEq, Eq)]
pub struct PathChoices {
   #[serde(skip_serializing_if = "is_default")]
   pub line_choice: LineChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub color: ColorChoice,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Serialize, Default, PartialEq)]
pub struct Strokeable<T: Default + PartialEq> {
   #[serde(skip_serializing_if = "is_default")]
   pub path: T,
   #[serde(skip_serializing_if = "is_default")]
   pub path_choices: PathChoices,
}

// Single-segment lines are more naturally one-stage polylines rather than single-element lines
// sets.
//
// Outer-product of an optional set of offsets and a set of lines.
#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct LinesSetSet {
   #[serde(skip_serializing_if = "is_default")]
   pub coords: Vec<([f64; 2], [f64; 2])>,
   // If offsets is empty, draw single line with no offset.
   #[serde(skip_serializing_if = "is_default")]
   pub offsets: Option<Vec<[f64; 2]>>,
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct CirclesSet {
   #[serde(skip_serializing_if = "is_default")]
   pub radius: f64,
   #[serde(skip_serializing_if = "is_default")]
   pub centers: Vec<[f64; 2]>,
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct PointsDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub point_choice: PointChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub color_choice: ColorChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub centers: Vec<[f64; 2]>,
}

// Length of start and end must match.
#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct TextSingle {
   #[serde(skip_serializing_if = "is_default")]
   pub content: String,
   #[serde(skip_serializing_if = "is_default")]
   pub location: [f64; 2],
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct TextDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub size_choice: TextSizeChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub offset_choice: TextOffsetChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub anchor_choice: TextAnchorChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub color_choice: ColorChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub texts: Vec<TextSingle>,
}

// #[derive(Serialize, Debug, Default, PartialEq)]
// pub enum OneOfSegment {
//    #[default]
//    Nothing,
//    Arc(ArcPath),
//    Cubic(CubicPath),
//    Hyperbolic(HyperbolicPath),
//    Polyline(PolylinePath),
// }

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct SegmentSequence {
   #[serde(skip_serializing_if = "is_default")]
   pub completion: PathCompletion,
   #[serde(skip_serializing_if = "is_default")]
   pub path_choices: PathChoices,
   #[serde(skip_serializing_if = "is_default")]
   pub segments: Vec<OneOfSegment>,
}

#[derive(Serialize, Debug, Default, PartialEq)]
pub enum OneOfDrawable {
   #[default]
   Nothing,
   Arc(Strokeable<ArcPath>),
   Cubic(Strokeable<CubicPath>),
   Hyperbolic(Strokeable<HyperbolicPath>),
   Polyline(Strokeable<PolylinePath>),
   Lines(Strokeable<LinesSetSet>),
   Circles(Strokeable<CirclesSet>),
   Points(PointsDrawable),
   Text(TextDrawable),
   SegmentSequence(SegmentSequence),
}

// Layer is logically a cross-drawable / path choice, but we want to make it trivial to be able
// to sort drawables by layer before further processing.
#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct QualifiedDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub layer: i32,
   #[serde(skip_serializing_if = "is_default")]
   pub drawable: OneOfDrawable,
}
