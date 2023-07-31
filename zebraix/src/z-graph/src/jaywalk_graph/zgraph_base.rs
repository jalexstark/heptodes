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
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

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

// #[derive(Serialize, Deserialize)]
// pub enum ZDataByType {
//    Void = 0,
//    Dirty,
//    Derived,
//    Fit,
// }

pub type PortDataVec = Rc<RefCell<Vec<ZPiece>>>;
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ZUnit {
   // Em,
   // Ex,
   In,
   Mm,
   Pt,
}

impl Default for ZUnit {
   fn default() -> Self {
      ZUnit::Pt
   }
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ZColor {
   Rgb(f64, f64, f64),
   Cmyk(f64, f64, f64, f64),
}

impl Default for ZColor {
   fn default() -> Self {
      ZColor::Rgb(0.5, 0.5, 0.5)
   }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ZFontStyle {
   #[serde(skip_serializing_if = "is_default")]
   pub size: f64,
   #[serde(skip_serializing_if = "String::is_empty")]
   pub family: String,
   #[serde(skip_serializing_if = "ZOptionBox::is_default")]
   pub language: ZOptionBox, // Example: "en-US".
}

impl Default for ZFontStyle {
   fn default() -> Self {
      ZFontStyle {
         size: 10.0,
         family: "monospace".to_string(),
         language: ZOptionBox { v: Some(Box::new(ZPiece::Text("en-US".to_string()))) },
      }
   }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ZTextStyle {
   #[serde(skip_serializing_if = "is_default")]
   pub font_style: ZFontStyle,
   #[serde(skip_serializing_if = "is_default")]
   pub color: ZColor,
}

impl Default for ZTextStyle {
   fn default() -> Self {
      ZTextStyle { font_style: ZFontStyle::default(), color: ZColor::default() }
   }
}

// context.set_source_rgb(0.5, 0.0, 0.0);
// Centre 160,120, radius 30

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CoordReal2D(pub f64, pub f64);

//  context.move_to(120.0, 60.0);
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum ZBigData {
   Color(ZColor),
   FontStyle(ZFontStyle),
   TextStyle(ZTextStyle),
}

//  context.move_to(120.0, 60.0);
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ZTupleData {
   Coord2D(CoordReal2D),
}

impl Default for CoordReal2D {
   fn default() -> Self {
      CoordReal2D(0.5, 0.5)
   }
}

// Pieces are small, but can indirect to bigger.
//
// Pieces should be mutually exclusive, so deserializable from untagged.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ZPiece {
   Void,
   Integer(i64), // Integer before Real, because then serde attempts to match it first.
   Real(f64),
   Unit(ZUnit),
   Text(String),
   //
   Big(ZBigData),
   Tuple(ZTupleData),
   //
   OptionBox(ZOptionBox),
   //
   // Renderer / library data will be box-dyn-any here.
}

impl Default for ZPiece {
   fn default() -> Self {
      ZPiece::Real(f64::default())
   }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ZOptionBox {
   pub v: Option<Box<ZPiece>>,
}

impl PartialEq for ZOptionBox {
   fn eq(&self, other: &Self) -> bool {
      if self.v.is_some() == other.v.is_some() {
         if self.v.is_some() {
            self.v.as_ref().unwrap() == other.v.as_ref().unwrap()
         } else {
            true
         }
      } else {
         false
      }
   }
}
impl Eq for ZOptionBox {}

impl ZOptionBox {
   pub fn is_default(&self) -> bool {
      self.v.is_none()
   }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum ZPieceType {
   Void,
   Integer,
   Real,
   Unit,
   Text,
   //
   Color,
   FontStyle,
   TextStyle,
   //
   Coord2D,
   //
   OptionBox,
   //
   // Renderer / library data will be box-dyn-any here.
}

impl Default for ZPieceType {
   fn default() -> Self {
      ZPieceType::Real
   }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PortPieceTyped(pub String, pub ZPieceType);

// #[derive(Clone, Serialize, Deserialize)]
// pub struct PortTyped {
//    pub name: String,
//    pub piece_type: ZPieceType,
// }

impl ZPieceType {
   pub fn get_piece_type_from_data(piece: &ZPiece) -> ZPieceType {
      match piece {
         ZPiece::Void => ZPieceType::Void,
         ZPiece::Integer(_) => ZPieceType::Integer,
         ZPiece::Real(_) => ZPieceType::Real,
         ZPiece::Unit(_) => ZPieceType::Unit,
         ZPiece::Text(_) => ZPieceType::Text,
         //
         ZPiece::Big(big_data) => match big_data {
            ZBigData::Color(_) => ZPieceType::Color,
            ZBigData::FontStyle(_) => ZPieceType::FontStyle,
            ZBigData::TextStyle(_) => ZPieceType::TextStyle,
         },
         ZPiece::Tuple(tuple_data) => match tuple_data {
            ZTupleData::Coord2D(_) => ZPieceType::Coord2D,
         },
         ZPiece::OptionBox(_) => ZPieceType::OptionBox,
      }
   }
}

impl ZPiece {
   pub fn piece_data_default_for_piece_type(piece_type: &ZPieceType) -> ZPiece {
      match piece_type {
         ZPieceType::Void => ZPiece::Void,
         ZPieceType::Integer => ZPiece::Integer(0),
         ZPieceType::Real => ZPiece::Real(0.0),
         ZPieceType::Unit => ZPiece::Unit(ZUnit::default()),
         ZPieceType::Text => ZPiece::Text("".to_string()),
         //
         ZPieceType::Color => ZPiece::Big(ZBigData::Color(ZColor::default())),
         ZPieceType::FontStyle => ZPiece::Big(ZBigData::FontStyle(ZFontStyle::default())),
         ZPieceType::TextStyle => ZPiece::Big(ZBigData::TextStyle(ZTextStyle::default())),
         //
         ZPieceType::Coord2D => ZPiece::Tuple(ZTupleData::Coord2D(CoordReal2D::default())),
         //
         ZPieceType::OptionBox => ZPiece::OptionBox(ZOptionBox { v: None }),
      }
   }
}

// These are horrible, requiring the caller to check something that we
// insist should be a contractual entry assurance.
impl ZPiece {
   pub fn get_real(&self) -> Option<f64> {
      match self {
         ZPiece::Real(v) => Some(*v),
         _default => None,
      }
   }

   pub fn get_mut_real(&mut self) -> Option<&mut f64> {
      match self {
         ZPiece::Real(v) => Some(v),
         _default => None,
      }
   }

   pub fn get_integer(&self) -> Option<i64> {
      match self {
         ZPiece::Integer(v) => Some(*v),
         _default => None,
      }
   }

   // pub fn get_mut_integer(&self) -> Option<&mut i64> {
   //    match self {
   //       ZPiece::Integer(v) => Some(&mut v),
   //       _default => None,
   //    }
   // }

   pub fn get_unit(&self) -> Option<ZUnit> {
      match self {
         ZPiece::Unit(v) => Some(v.clone()),
         _default => None,
      }
   }

   // pub fn get_mut_unit(&self) -> Option<&mut ZUnit> {
   //    match self {
   //       ZPiece::Unit(v) => Some(&mut v),
   //       _default => None,
   //    }
   // }

   pub fn get_text(&self) -> Option<&String> {
      match self {
         ZPiece::Text(s) => Some(s),
         _default => None,
      }
   }

   pub fn get_mut_text(&mut self) -> Option<&mut String> {
      match self {
         ZPiece::Text(s) => Some(s),
         _default => None,
      }
   }

   pub fn get_color(&self) -> Option<&ZColor> {
      match self {
         ZPiece::Big(ZBigData::Color(c)) => Some(c),
         _default => None,
      }
   }

   pub fn get_mut_color(&mut self) -> Option<&mut ZColor> {
      match self {
         ZPiece::Big(ZBigData::Color(c)) => Some(c),
         _default => None,
      }
   }

   pub fn get_font_style(&self) -> Option<&ZFontStyle> {
      match self {
         ZPiece::Big(ZBigData::FontStyle(c)) => Some(c),
         _default => None,
      }
   }

   pub fn get_mut_font_style(&mut self) -> Option<&mut ZFontStyle> {
      match self {
         ZPiece::Big(ZBigData::FontStyle(c)) => Some(c),
         _default => None,
      }
   }

   pub fn get_text_style(&self) -> Option<&ZTextStyle> {
      match self {
         ZPiece::Big(ZBigData::TextStyle(c)) => Some(c),
         _default => None,
      }
   }

   pub fn get_mut_text_style(&mut self) -> Option<&mut ZTextStyle> {
      match self {
         ZPiece::Big(ZBigData::TextStyle(c)) => Some(c),
         _default => None,
      }
   }

   pub fn get_option_box(&self) -> Option<&ZOptionBox> {
      match self {
         ZPiece::OptionBox(c) => Some(c),
         _default => None,
      }
   }

   pub fn get_mut_option_box(&mut self) -> Option<&mut ZOptionBox> {
      match self {
         ZPiece::OptionBox(c) => Some(c),
         _default => None,
      }
   }

   pub fn get_coord2d(&self) -> Option<&CoordReal2D> {
      match self {
         ZPiece::Tuple(ZTupleData::Coord2D(c)) => Some(c),
         _default => None,
      }
   }

   pub fn get_mut_coord2d(&mut self) -> Option<&mut CoordReal2D> {
      match self {
         ZPiece::Tuple(ZTupleData::Coord2D(c)) => Some(c),
         _default => None,
      }
   }
}
