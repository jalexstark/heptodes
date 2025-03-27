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

use serde::{Deserialize, Serialize};

pub struct LineParameters {
   pub line_width: f64,
   pub dashes: Box<[f64]>,
   pub dash_offset: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum LineChoice {
   #[default]
   Ordinary,
   Light,
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum PointChoice {
   #[default]
   Circle,
   Dot,
   Plus,
   Times,
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TextAnchorHorizontal {
   #[default]
   Center,
   Left,
   Right,
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TextAnchorVertical {
   #[default]
   Middle,
   Bottom,
   Top,
}

// Normal vs annotation vs title.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TextSizeChoice {
   #[default]
   Normal,
   Large,
   Small,
}

// Directions (horizontal, vertical) over which to offset anchoring.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TextOffsetChoice {
   #[default]
   None,
   Diagram,
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TextAnchorChoice {
   #[default]
   Centered, // Really undesirable, but cleanly handled as default sub-field.
   ThreeByThree(TextAnchorHorizontal, TextAnchorVertical),
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ColorChoice {
   #[default]
   DefaultBlack,
   Black,
   Gray,
   DarkGray,
   LightGray,
   BrightRed,
   BrightGreen,
   BrightBlue,
   BrightYellow,
   BrightCyan,
   BrightMagenta,
   Red,
   Green,
   Blue,
   YellowBrown,
   BlueGreen,
   BlueRed,
   RedRedGreen,
   GreenGreenRed,
   BlueBlueGreen,
   GreenGreenBlue,
   RedRedBlue,
   BlueBlueRed,
}

impl LineChoice {
   // Diagram-specific simple values.
   const LIGHT_DASH_LENGTH: f64 = 10.0;
   const LIGHT_DASH_SEPARATION: f64 = 7.0;
}

impl ColorChoice {
   #[must_use]
   pub const fn to_rgb(&self) -> (f64, f64, f64) {
      match self {
         Self::DefaultBlack | Self::Black => (0.0, 0.0, 0.0),
         Self::Gray => (0.55, 0.55, 0.55),
         Self::DarkGray => (0.35, 0.35, 0.35),
         Self::LightGray => (0.7, 0.7, 0.7),
         Self::BrightRed => (1.0, 0.0, 0.0),
         Self::BrightGreen => (0.0, 1.0, 0.0),
         Self::BrightBlue => (0.0, 0.0, 1.0),
         Self::BrightYellow => (1.0, 1.0, 0.0),
         Self::BrightCyan => (0.0, 1.0, 1.0),
         Self::BrightMagenta => (1.0, 0.0, 1.0),
         Self::Red => (0.6, 0.0, 0.0),
         Self::Green => (0.0, 0.4, 0.0),
         Self::Blue => (0.0, 0.0, 0.65),
         Self::YellowBrown => (0.37, 0.28, 0.0),
         Self::BlueGreen => (0.0, 0.3, 0.3),
         Self::BlueRed => (0.35, 0.0, 0.5),
         Self::RedRedGreen => (0.45, 0.18, 0.0),
         Self::GreenGreenRed => (0.24, 0.32, 0.0),
         Self::BlueBlueGreen => (0.0, 0.18, 0.45),
         Self::GreenGreenBlue => (0.0, 0.36, 0.18),
         Self::RedRedBlue => (0.47, 0.0, 0.34),
         Self::BlueBlueRed => (0.23, 0.0, 0.55),
      }
   }
}

impl LineChoice {
   #[must_use]
   pub fn to_line_parameters(&self, diagram_choices: &DiagramChoices) -> LineParameters {
      match self {
         Self::Ordinary => LineParameters {
            line_width: diagram_choices.line_width,
            dashes: Box::new([]),
            dash_offset: 0.0,
         },
         Self::Light => LineParameters {
            line_width: diagram_choices.line_width * diagram_choices.annotation_linear_scale,
            dashes: Box::new([
               Self::LIGHT_DASH_LENGTH * diagram_choices.annotation_linear_scale,
               Self::LIGHT_DASH_SEPARATION * diagram_choices.annotation_linear_scale,
            ]),
            dash_offset: 0.0,
         },
      }
   }
}

#[derive(Debug, Default)]
pub struct CanvasLayout {
   pub scale: [f64; 2],
   pub offset: [f64; 2],
   pub canvas_size: [f64; 2],
}

#[derive(Debug, Default)]
pub struct DiagramChoices {
   pub annotation_linear_scale: f64,
   pub annotation_area_scale: f64,

   pub font_size: f64,
   pub point_size: f64,
   pub line_width: f64,
   pub annotation_offset_absolute: [f64; 2], // Horiz and vert text offsets, relative to font size.
}

// impl TextOffsetChoice {

// }
