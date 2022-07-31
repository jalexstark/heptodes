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

use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::result::Result;

// Traits, especially Serde and defaults.
pub use crate::jaywalk_graph::jaywalk_foundation::Anchorage;
pub use crate::jaywalk_graph::jaywalk_foundation::ArrowType;
pub use crate::jaywalk_graph::jaywalk_foundation::Bidirection;
pub use crate::jaywalk_graph::jaywalk_foundation::Coord;
pub use crate::jaywalk_graph::jaywalk_foundation::Finish;
pub use crate::jaywalk_graph::jaywalk_foundation::JKey;
pub use crate::jaywalk_graph::jaywalk_foundation::JVec;
pub use crate::jaywalk_graph::jaywalk_foundation::JaywalkAffine;
pub use crate::jaywalk_graph::jaywalk_foundation::LineStyle;
pub use crate::jaywalk_graph::jaywalk_foundation::LineType;
pub use crate::jaywalk_graph::jaywalk_foundation::Octant;
pub use crate::jaywalk_graph::jaywalk_foundation::Shape;
pub use crate::jaywalk_graph::jaywalk_foundation::StateMark;
pub use crate::jaywalk_graph::jaywalk_foundation::TMatrix;
pub use crate::jaywalk_graph::jaywalk_foundation::Yna;
pub use crate::jaywalk_graph::jaywalk_foundation::Yon;
use crate::jaywalk_graph::jaywalk_foundation::ADDITIVE_ID_F64;
use crate::jaywalk_graph::jaywalk_foundation::FLOAT_MISSING_VAL;
use crate::jaywalk_graph::jaywalk_foundation::INT32_MISSING_VAL;
use crate::jaywalk_graph::jaywalk_foundation::MULTIPLICATIVE_ID_F64;

pub fn absent_int32() -> i32 {
   return INT32_MISSING_VAL;
}

pub fn absent_f64() -> f64 {
   return FLOAT_MISSING_VAL;
}

pub fn add_ident_f64() -> f64 {
   return ADDITIVE_ID_F64;
}

pub fn mult_ident_f64() -> f64 {
   return MULTIPLICATIVE_ID_F64;
}

impl Default for JKey {
   fn default() -> Self {
      return JKey(INT32_MISSING_VAL);
   }
}

impl Serialize for JKey {
   fn serialize<S>(
      &self,
      serializer: S,
   ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
   where
      S: Serializer,
   {
      (self.0 as i32).serialize(serializer)
   }
}

impl<'de> Deserialize<'de> for JKey {
   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
   where
      D: Deserializer<'de>,
   {
      let result = i32::deserialize(deserializer);
      match result {
         Ok(v) => return Ok(JKey(v)),
         Err(e) => return Err(e),
      }
   }
}

impl<T> Default for JVec<T> {
   fn default() -> Self {
      return JVec(<Vec<T> as Default>::default());
   }
}

impl<T: Serialize> Serialize for JVec<T> {
   fn serialize<S>(
      &self,
      serializer: S,
   ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
   where
      S: Serializer,
   {
      (&self.0 as &Vec<T>).serialize(serializer)
   }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for JVec<T> {
   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
   where
      D: Deserializer<'de>,
   {
      let result = Vec::<T>::deserialize(deserializer);
      match result {
         Ok(v) => return Ok(JVec::<T>(v)),
         Err(e) => return Err(e),
      }
   }
}

impl<T> JVec<T> {
   pub fn len(&self) -> usize {
      return self.0.len();
   }
}

impl Default for Coord {
   fn default() -> Self {
      return Coord(ADDITIVE_ID_F64, ADDITIVE_ID_F64);
   }
}

impl Default for TMatrix {
   fn default() -> Self {
      return TMatrix(
         MULTIPLICATIVE_ID_F64,
         ADDITIVE_ID_F64,
         ADDITIVE_ID_F64,
         MULTIPLICATIVE_ID_F64,
      );
   }
}

impl Default for Yna {
   fn default() -> Self {
      return Yna::Auto;
   }
}

impl Default for Yon {
   fn default() -> Self {
      return Yon::No;
   }
}

impl Default for StateMark {
   fn default() -> Self {
      return StateMark::Unfit;
   }
}

impl Default for Bidirection {
   fn default() -> Self {
      return Bidirection::Auto;
   }
}

// This is pretty ridiculous, but somehow one has to program defensively.
impl JaywalkAffine {
   #[inline]
   pub fn default_value_offset() -> f64 {
      add_ident_f64()
   }
   #[inline]
   pub fn default_value_scale() -> f64 {
      mult_ident_f64()
   }
}

impl Default for JaywalkAffine {
   fn default() -> Self {
      return JaywalkAffine {
         offset: JaywalkAffine::default_value_offset(),
         scale: JaywalkAffine::default_value_scale(),
         value: f64::default(),
      };
   }
}

impl Default for Finish {
   fn default() -> Self {
      return Finish::Auto;
   }
}

impl Default for LineType {
   fn default() -> Self {
      return LineType::Auto;
   }
}

impl Default for LineStyle {
   fn default() -> Self {
      return LineStyle {
         line_type: LineType::default(),
         pattern_length: JaywalkAffine::default(),
      };
   }
}

impl Default for Octant {
   fn default() -> Self {
      return Octant::Auto;
   }
}

// This is pretty ridiculous, but somehow one has to program defensively.
impl Anchorage {
   #[inline]
   pub fn default_value_degrees() -> f64 {
      absent_f64()
   }
}

impl Default for Anchorage {
   fn default() -> Self {
      return Anchorage {
         octant: Octant::default(),
         orig_degrees: Option::<f64>::default(),
         degrees: Anchorage::default_value_degrees(),
      };
   }
}

impl Default for Shape {
   fn default() -> Self {
      return Shape::Auto;
   }
}

impl Default for ArrowType {
   fn default() -> Self {
      return ArrowType::Auto;
   }
}

// impl Default for TransformType {
//     fn default() -> Self {
//         return TransformType::Auto;
//     }
// }
