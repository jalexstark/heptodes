// Copyright 2023 Google LLC
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

use crate::jaywalk_graph::jaywalk_foundation::is_default;
use crate::jaywalk_graph::jaywalk_traiting::is_mult_ident_f64;
use crate::jaywalk_graph::jaywalk_traiting::mult_ident_f64;
use serde::{Deserialize, Serialize};
use std::any::Any;

// The idea is to keep the base minimal, but that will be difficult.

#[derive(Serialize, Deserialize)]
pub enum Types {
   Void = 0,
   Integer,
   Real,
   Bool,
   Weighted,
   WeightedView,
   //
   Group,
   GraphOutput,
   GraphInput,
   BoxedText,
   Circle,
   TextLine,
}

#[derive(Serialize, Deserialize)]
pub enum ZDataByType {
   Void = 0,
   Dirty,
   Derived,
   Fit,
}

// ZData should be a set of data, a set of ports, and is passed to
// invocation functions.
#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ZData {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

#[derive(PartialEq, Eq)]
pub enum ZMachineTypestate {
   Init = 0,
   Deffed,
   Constructed,
   Calculated,
   Inked,
   Finished,
}

pub type ZNodeStateData = Option<Box<dyn Any>>;
pub type ZRendererData = Option<Box<dyn Any>>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZNodeTypeFinder {
   ByString(String),
}

#[derive(Debug)]
pub enum ZGraphError {
   IncorrectTypestateTransition,
   DuplicateGraphDef,
   MissingGraphDef,
   DuplicateRendererSetup,
   RendererForConstruction,
   FinishNoRenderer,
   UnfinishedRenderer,
}

// -----------------------------
// Typing

#[derive(Clone, Serialize, Deserialize)]
pub enum ZUnit {
   // Em,
   // Ex,
   In,
   Mm,
   Pt,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ZCanvasDirection {
   Upwards,
   Downwards,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ZCanvas {
   pub width: f64,
   pub height: f64,
   pub x_offset: f64,
   pub y_offset: f64,
   pub unit: ZUnit,
   pub direction: ZCanvasDirection,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ZColor {
   Rgb(f64, f64, f64),
   Cmyk(f64, f64, f64, f64),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ZFontStyle {
   #[serde(skip_serializing_if = "is_default")]
   pub size: f64,
   #[serde(skip_serializing_if = "String::is_empty")]
   pub family: String,
   #[serde(skip_serializing_if = "Option::is_none")]
   pub language: Option<String>, // Example: "en-US".
}

impl Default for ZFontStyle {
   fn default() -> Self {
      ZFontStyle {
         size: 10.0,
         family: "monospace".to_string(),
         language: Some("en-US".to_string()),
      }
   }
}

// context.set_source_rgb(0.5, 0.0, 0.0);
// Centre 160,120, radius 30

#[derive(Clone, Serialize, Deserialize)]
pub struct CoordReal2D(pub f64, pub f64);

//  context.move_to(120.0, 60.0);
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ZBigData {
   Color(ZColor),
   FontStyle(ZFontStyle),
}

//  context.move_to(120.0, 60.0);
#[derive(Clone, Serialize, Deserialize)]
pub enum ZTupleData {
   Coord2D(CoordReal2D),
}

// Pieces are small, but can indirect to bigger.
//
// Pieces should be mutually exclusive, so deserializable from untagged.
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZPiece {
   Real(f64),
   Integer(i32),
   Unit(ZUnit),
   Text(String),
   //
   Big(ZBigData),
   Tuple(ZTupleData),
   //
   // Renderer / library data will be box-dyn-any here.
}

impl Default for ZPiece {
   fn default() -> Self {
      ZPiece::Real(f64::default())
   }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ZPieceType {
   Real,
}

impl Default for ZPieceType {
   fn default() -> Self {
      ZPieceType::Real
   }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Port {
   pub name: String,
   pub piece_type: ZPieceType,
}
