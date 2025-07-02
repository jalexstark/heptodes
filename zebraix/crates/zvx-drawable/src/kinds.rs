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
use zvx_base::is_default;

#[derive(Serialize, Debug, Copy, Clone, DefaultFromSerde, PartialEq, Eq)]
pub struct SegmentChoices {
   #[serde(skip_serializing_if = "is_default")]
   pub continuation: ContinuationChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub closure: LineClosureChoice,
}

#[derive(Serialize, Debug, Copy, Clone, DefaultFromSerde, PartialEq, Eq)]
pub struct PathChoices {
   #[serde(skip_serializing_if = "is_default")]
   pub line_choice: LineChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub color: ColorChoice,
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct ArcPath {
   #[serde(skip_serializing_if = "is_default")]
   pub angle_range: [f64; 2],
   #[serde(skip_serializing_if = "is_default")]
   pub center: [f64; 2],
   // Elliptical transform matrix.  Zero angle is in direction of x axis.
   #[serde(skip_serializing_if = "is_default")]
   pub transform: [f64; 4],
}

pub type CubicPath = [[f64; 2]; 4];
// #[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
// pub struct CubicPath {
//    #[serde(skip_serializing_if = "is_default")]
//    pub c: [[f64; 2]; 4],
// }

pub type PolylinePath = Vec<[f64; 2]>;
// #[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
// pub struct PolylinePath {
//    #[serde(skip_serializing_if = "is_default")]
//    pub locations: Vec<[f64; 2]>,
// }

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct ArcDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub path_choices: PathChoices,
   #[serde(skip_serializing_if = "is_default")]
   pub path: ArcPath,
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct CubicDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub path_choices: PathChoices,
   #[serde(skip_serializing_if = "is_default")]
   pub path: CubicPath,
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct PolylineDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub path_choices: PathChoices,
   #[serde(skip_serializing_if = "is_default")]
   pub path: PolylinePath,
}

// Single-segment lines are more naturally one-stage polylines rather than single-element lines
// sets.
#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct LinesDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub path_choices: PathChoices,
   #[serde(skip_serializing_if = "is_default")]
   pub coords: Vec<([f64; 2], [f64; 2])>,
   // If offsets is empty, draw single line with no offset.
   #[serde(skip_serializing_if = "is_default")]
   pub offsets: Option<Vec<[f64; 2]>>,
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

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct CirclesDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub path_choices: PathChoices,
   #[serde(skip_serializing_if = "is_default")]
   pub radius: f64,
   #[serde(skip_serializing_if = "is_default")]
   pub centers: Vec<[f64; 2]>,
}

#[derive(Serialize, Debug, Default, PartialEq)]
pub enum OneOfSegment {
   #[default]
   Nothing,
   Arc(ArcPath),
   Cubic(CubicPath),
   Polyline(PolylinePath),
}

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
   Arc(ArcDrawable),
   Cubic(CubicDrawable),
   Polyline(PolylineDrawable),
   Lines(LinesDrawable),
   Points(PointsDrawable),
   Circles(CirclesDrawable),
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
