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

use crate::choices::ColorChoice;
use crate::choices::ContinuationChoice;
use crate::choices::LineChoice;
use crate::choices::LineClosureChoice;
use crate::choices::PointChoice;
use crate::choices::TextAnchorChoice;
use crate::choices::TextOffsetChoice;
use crate::choices::TextSizeChoice;
use serde::Serialize;
use serde_default::DefaultFromSerde;
use zvx_base::is_default;

#[derive(Serialize, Debug, Copy, Clone, DefaultFromSerde, PartialEq, Eq)]
pub struct SegmentChoices {
   #[serde(skip_serializing_if = "is_default")]
   pub line_choice: LineChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub color: ColorChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub continuation: ContinuationChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub closure: LineClosureChoice,
}

// Single-segment lines are more naturally one-stage polylines rather than single-element lines
// sets.
#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct LinesDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub line_choice: LineChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub color_choice: ColorChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub coords: Vec<([f64; 2], [f64; 2])>,
   // If offsets is empty, draw single line with no offset.
   #[serde(skip_serializing_if = "is_default")]
   pub offsets: Option<Vec<[f64; 2]>>,
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct ArcDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub segment_choices: SegmentChoices,
   #[serde(skip_serializing_if = "is_default")]
   pub angle_range: [f64; 2],
   #[serde(skip_serializing_if = "is_default")]
   pub center: [f64; 2],
   // Elliptical transform matrix.  Zero angle is in direction of x axis.
   #[serde(skip_serializing_if = "is_default")]
   pub transform: [f64; 4],
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct CubicDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub segment_choices: SegmentChoices,
   #[serde(skip_serializing_if = "is_default")]
   pub x: [f64; 4],
   #[serde(skip_serializing_if = "is_default")]
   pub y: [f64; 4],
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
   pub line_choice: LineChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub color_choice: ColorChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub radius: f64,
   #[serde(skip_serializing_if = "is_default")]
   pub centers: Vec<[f64; 2]>,
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct PolylineDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub segment_choices: SegmentChoices,
   #[serde(skip_serializing_if = "is_default")]
   pub locations: Vec<[f64; 2]>,
}

#[derive(Serialize, Debug, Default, PartialEq)]
pub enum OneOfSegment {
   #[default]
   Nothing,
   Arc(ArcDrawable),
   Cubic(CubicDrawable),
   Polyline(PolylineDrawable),
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct SegmentSequence {
   #[serde(skip_serializing_if = "is_default")]
   pub segments: Vec<OneOfSegment>,
}

#[derive(Serialize, Debug, Default, PartialEq)]
pub enum OneOfDrawable {
   #[default]
   Nothing,
   Lines(LinesDrawable),
   Arc(ArcDrawable),
   Cubic(CubicDrawable),
   Points(PointsDrawable),
   Text(TextDrawable),
   Circles(CirclesDrawable),
   Polyline(PolylineDrawable),
   SegmentSequence(SegmentSequence),
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct QualifiedDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub layer: i32,
   #[serde(skip_serializing_if = "is_default")]
   pub drawable: OneOfDrawable,
}
