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

use libm::{frexp, frexpf};
use serde_core::Serialize;
use serde_json::ser::Formatter;
use serde_json::ser::PrettyFormatter;
use serde_json::Error as JsonError;
use serde_json::Serializer as JsonSerializer;
use std::io;

// Serde-JSON pretty-printing formatter with reduced floating-point precision.
//
// Fork point (update if merging future diffs): serde_json 1.0.145 / serde_json/ser.rs.
//
// The only functional change is customization of write_f64() and write_f32().

const MIN_SIGNIFICANT_F64: f64 = 1e-10;
const MIN_SIGNIFICANT_F32: f32 = 1e-10;
const PRECISION_PLACES: i32 = 9;

/// This structure pretty prints a JSON value to make it human readable.
#[derive(Clone, Debug)]
pub struct GoldenFormatter<'a> {
   pretty_formatter: PrettyFormatter<'a>,
}

#[allow(clippy::elidable_lifetime_names)]
impl<'a> GoldenFormatter<'a> {
   #[must_use]
   pub fn new() -> Self {
      Self { pretty_formatter: PrettyFormatter::with_indent(b"  ") }
   }
}

#[allow(clippy::elidable_lifetime_names)]
impl<'a> Default for GoldenFormatter<'a> {
   fn default() -> Self {
      GoldenFormatter::new()
   }
}

#[allow(clippy::elidable_lifetime_names)]
impl<'a> Formatter for GoldenFormatter<'a> {
   #[inline]
   fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
   where
      W: ?Sized + io::Write,
   {
      self.pretty_formatter.begin_array(writer)
   }

   #[inline]
   fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
   where
      W: ?Sized + io::Write,
   {
      self.pretty_formatter.end_array(writer)
   }

   #[inline]
   fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
   where
      W: ?Sized + io::Write,
   {
      self.pretty_formatter.begin_array_value(writer, first)
   }

   #[inline]
   fn end_array_value<W>(&mut self, writer: &mut W) -> io::Result<()>
   where
      W: ?Sized + io::Write,
   {
      self.pretty_formatter.end_array_value(writer)
   }

   #[inline]
   fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
   where
      W: ?Sized + io::Write,
   {
      self.pretty_formatter.begin_object(writer)
   }

   #[inline]
   fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
   where
      W: ?Sized + io::Write,
   {
      self.pretty_formatter.end_object(writer)
   }

   #[inline]
   fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
   where
      W: ?Sized + io::Write,
   {
      self.pretty_formatter.begin_object_key(writer, first)
   }

   #[inline]
   fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
   where
      W: ?Sized + io::Write,
   {
      self.pretty_formatter.begin_object_value(writer)
   }

   #[inline]
   fn end_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
   where
      W: ?Sized + io::Write,
   {
      self.pretty_formatter.end_object_value(writer)
   }

   /// Writes a floating point value like `-31.26e+12` to the specified writer.
   ///
   /// # Special cases
   ///
   /// This function **does not** check for NaN or infinity. If the input
   /// number is not a finite float, the printed representation will be some
   /// correctly formatted but unspecified numerical value.
   ///
   /// Please check [`is_finite`] yourself before calling this function, or
   /// check [`is_nan`] and [`is_infinite`] and handle those cases yourself
   /// with a different `Formatter` method.
   ///
   /// [`is_finite`]: f64::is_finite
   /// [`is_nan`]: f64::is_nan
   /// [`is_infinite`]: f64::is_infinite
   #[inline]
   fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
   where
      W: ?Sized + io::Write,
   {
      if value.abs() < MIN_SIGNIFICANT_F64 {
         writer.write_all(b"0.0")
      } else {
         let (_mantissa, power_2) = frexp(value);
         let power_10 = f64::from(power_2) * std::f64::consts::LOG10_2;
         #[allow(clippy::cast_possible_truncation)]
         #[allow(clippy::cast_sign_loss)]
         writer.write_all(
            format!("{:.*}", (PRECISION_PLACES - (power_10.round() as i32)) as usize, value)
               .trim_end_matches('0')
               .trim_end_matches('.')
               .as_bytes(),
         )
      }
   }

   #[inline]
   fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
   where
      W: ?Sized + io::Write,
   {
      if value.abs() < MIN_SIGNIFICANT_F32 {
         writer.write_all(b"0.0")
      } else {
         let (_mantissa, power_2) = frexpf(value);
         let power_10 = f64::from(power_2) * std::f64::consts::LOG10_2;
         #[allow(clippy::cast_possible_truncation)]
         #[allow(clippy::cast_sign_loss)]
         writer.write_all(
            format!("{:.*}", (PRECISION_PLACES - (power_10.round() as i32)) as usize, value)
               .trim_end_matches('0')
               .trim_end_matches('.')
               .as_bytes(),
         )
      }
   }
}

/// Serialize the given data structure as pretty-printed JSON into the I/O
/// stream.
///
/// Serialization guarantees it only feeds valid UTF-8 sequences to the writer.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub fn to_writer_pretty_reduced<W, T>(writer: W, value: &T) -> Result<(), JsonError>
where
   W: io::Write,
   T: ?Sized + Serialize,
{
   let formatter = GoldenFormatter::new();
   let mut ser = JsonSerializer::with_formatter(writer, formatter);
   value.serialize(&mut ser)
}
