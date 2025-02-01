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

//! Goldenfile tests generate one or more output files as they run. If any files
//! differ from their checked-in "golden" version, the test fails. This ensures
//! that behavioral changes are intentional, explicit, and version controlled.
//!
//! # Example
//!
//! ```rust
//! use diag_golden::JsonGoldenTest;
//!
//! let mut json_golden = JsonGoldenTest::new("tests/goldenfiles/", "json_example");
//!
//! let in_text = r#"{
//!  "top-level": {
//!      "entry": "value",
//!      "nested": {   "array": ["Tutti", "Frutty"]
//!                  }
//!          }
//! }"#;
//!    json_golden.writeln_as_bytes(&in_text);
//! ```
//!
//! # Example
//!
//! ```rust
//! use diag_golden::SvgGoldenTest;
//!
//! let mut svg_golden = SvgGoldenTest::new("tests/goldenfiles/", "svg_example");
//!
//! // Note that SVG files must not start with blank line.
//! let in_text = r#"<?xml version="1.0" encoding="UTF-8"?>
//! <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
//!  width="320" height="240" viewBox="0 0 320 240">
//!    <path fill="none" stroke-width="2" stroke-linecap="butt" stroke-linejoin="miter"
//!     stroke="rgb(0%, 0%, 0%)" stroke-opacity="1" stroke-miterlimit="10"
//!     d="M 190 120 C 190 136.570312 176.570312 150 160 150
//!        C 143.429688 150 130 136.570312 130 120 C 130 103.429688 143.429688 90 160 90
//!        C 176.570312 90 190 103.429688 190 120 "
//!     transform="matrix(1, 0, 0, 1, -15, -25)"/>
//! </svg>"#;
//!
//! svg_golden.writeln_as_bytes(&in_text);
//! ```
//!
//! When the `Mint` goes out of scope, it compares the contents of each file
//! to its checked-in golden version and fails the test if they differ. To
//! update the checked-in versions, run:
//! ```sh
//! UPDATE_GOLDENFILES=1 cargo test
//! ```

// At time of coding, Rust has a bug that cannot cope with test-only emptiness.
// #![cfg(test)]

use cairo::Context;
use cairo::Matrix;
use goldenfile::Mint;
use serde_default::DefaultFromSerde;
// use json5::from_str;
// use serde_json::from_slice;
use serde_json::to_string_pretty;
use serde_json::Value;
// use std::ffi::OsStr;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use testdir::testdir;

#[inline]
pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
   t == &T::default()
}

// Helper that handles a golden (Mint) test for SVG output.
//
// Some parts of SVG are "random", such as the surface ID.  The
// strategy is to filter the output.  The alternative, modifying the
// differ, would result in churn in golden files.
pub struct SvgGoldenTest {
   _mint_dir: String,
   _output_filename: String,
   _mint: Mint,
   pub out_stream: fs::File,
}

// SvgGoldenWriteable may change, but will have trait Write.
pub type SvgGoldenWriteable = Vec<u8>;
// pub type SvgGoldenResult = Result<Box<dyn core::any::Any>, cairo::StreamWithError>;
pub type SvgGoldenBoxedContent = Box<dyn core::any::Any>;

impl SvgGoldenTest {
   #[allow(clippy::missing_panics_doc)]
   #[must_use]
   pub fn new(mint_dir: &str, golden_filestem: &str) -> SvgGoldenTest {
      let mut mint = Mint::new(mint_dir);
      let full_golden_filename = format!("{golden_filestem}.svg");
      let out_stream = mint.new_goldenfile(full_golden_filename.clone()).unwrap();

      SvgGoldenTest {
         _mint_dir: mint_dir.to_string(),
         _output_filename: full_golden_filename.to_string(),
         _mint: mint,
         out_stream,
      }
   }

   // input_filestem can also create
   pub fn read_to_string(&mut self, input_filestem: &str) -> String {
      let full_input_path_string = input_filestem.to_string().clone() + ".svg";
      let full_input_path = Path::new(&full_input_path_string);

      check_panic_with_path(
         std::fs::read_to_string(full_input_path),
         "opening input-data file",
         full_input_path,
      )
   }

   #[allow(clippy::missing_panics_doc)]
   pub fn writeln_as_bytes(&mut self, result: &str) {
      // let bytes_amount = self.out_stream.write(result.as_bytes()).unwrap();
      // assert!(bytes_amount == result.as_bytes().len());
      SvgGoldenTest::filter_result(Box::new(result.as_bytes()), &self.out_stream);
      let bytes_amount_nl = self.out_stream.write(b"\n").unwrap();
      assert!(bytes_amount_nl == 1);
   }

   #[must_use]
   pub fn get_raw_writeable(&self) -> SvgGoldenWriteable {
      Vec::<u8>::new()
   }

   #[allow(clippy::missing_panics_doc)]
   pub fn handover_result(&self, golden_writeable: SvgGoldenBoxedContent) {
      let unboxed_result = &**golden_writeable.downcast::<Vec<u8>>().unwrap();
      Self::filter_result(unboxed_result, &self.out_stream);
   }

   // Replace surface ID with generic ID, since this is changeable in tests.
   #[allow(clippy::missing_panics_doc)]
   pub fn filter_result<R: io::Read, W: io::Write>(boxed_sample_svg: R, mut out_stream: W) {
      let line_reader = std::io::BufReader::new(boxed_sample_svg);

      for l in line_reader.lines() {
         let line = l.unwrap();
         if line.starts_with(r#"<g id="surface"#) {
            writeln!(out_stream, r#"<g id="surfaceXXXX">"#).unwrap();
         } else {
            out_stream.write_all(line.as_bytes()).unwrap();
            out_stream.write_all(b"\n").unwrap();
         }
      }
   }
}

// Helper that handles a golden (Mint) test for Json output.
//
// This is mainly a convenience class, and it can be used to check
// idempotency (perhaps with additional default values in output) of
// deserialization and serialization. It assumes that output is
// pretty-printed, and adds a newline.
//
// The input is passed through a formatter, discarding comments and
// with one "thing" per line.  This enables the user to format their
// input as they see fit, without clever diffing of whitespace.

pub struct JsonGoldenTest {
   pub mint_dir: String,
   _output_filename: String,
   _mint: Mint,
   pub out_stream: fs::File,
}

#[allow(clippy::missing_panics_doc)]
impl JsonGoldenTest {
   pub fn new(mint_dir: &str, golden_filestem: &str) -> JsonGoldenTest {
      let mut mint = Mint::new(mint_dir);
      let full_golden_filename = format!("{golden_filestem}.json");
      // let out_stream = mint.new_goldenfile(output_filename).unwrap();
      let out_stream = mint
         .new_goldenfile_with_differ(full_golden_filename.clone(), Box::new(Self::custom_diff))
         .unwrap();

      JsonGoldenTest {
         mint_dir: mint_dir.to_string(),
         _output_filename: full_golden_filename.to_string(),
         _mint: mint,
         out_stream,
      }
   }

   // input_filestem can also create
   pub fn read_to_string(&mut self, input_filestem: &str) -> String {
      let full_input_path_string = input_filestem.to_string().clone() + ".json";
      let full_input_path = Path::new(&full_input_path_string);

      check_panic_with_path(
         std::fs::read_to_string(full_input_path),
         "opening input-data file",
         full_input_path,
      )
   }

   pub fn writeln_as_bytes(&mut self, result: &str) {
      let bytes_amount = self.out_stream.write(result.as_bytes()).unwrap();
      assert!(bytes_amount == result.as_bytes().len());
      let bytes_amount_nl = self.out_stream.write(b"\n").unwrap();
      assert!(bytes_amount_nl == 1);
   }

   fn custom_diff(old: &Path, new: &Path) {
      let dir: PathBuf = testdir!();
      let mut reformatted_old = dir.join(old.file_name().unwrap());
      reformatted_old.set_file_name(
         reformatted_old.file_stem().unwrap().to_str().unwrap().to_owned()
            + "-old."
            + reformatted_old.extension().unwrap().to_str().unwrap(),
      );
      let mut reformatted_new = dir.join(new.file_name().unwrap());
      reformatted_new.set_file_name(
         reformatted_new.file_stem().unwrap().to_str().unwrap().to_owned()
            + "-new."
            + reformatted_new.extension().unwrap().to_str().unwrap(),
      );

      let data_old_result = fs::read(old);
      let data_old = match data_old_result {
         Ok(string_result) => string_result,
         Err(error) => panic!("Problem opening golden-data file {old:?}: {error:?}"),
      };
      // Parse the old-path file into serde_json::Value.

      let value_old: Value = serde_json::from_slice(&data_old).unwrap();
      // Write old-path JSON into pretty-print.
      let serialized_old = to_string_pretty::<Value>(&value_old).unwrap();
      fs::write(&reformatted_old, serialized_old).unwrap();
      assert!(reformatted_old.exists());

      // We replace the test-generated pretty print with that passed
      // through "Value" so that the ordering is lexicographic.
      let data_new = fs::read(new).unwrap();
      // Parse the new-path file into serde_json::Value.
      let value_new: Value = serde_json::from_slice(&data_new).unwrap();
      // Write new-path JSON into pretty-print.
      let serialized_new = to_string_pretty::<Value>(&value_new).unwrap();
      fs::write(&reformatted_new, serialized_new).unwrap();
      assert!(reformatted_new.exists());

      goldenfile::differs::text_diff(&reformatted_old, &reformatted_new);
   }
}

// The string messaging is a task in the form "opening file".
fn check_panic_with_path<T>(result: Result<T, io::Error>, messaging: &str, path: &Path) -> T {
   match result {
      Ok(string_result) => string_result,
      Err(error) => panic!("Error while {messaging} for file path {path:?}: {error:?}"),
   }
}

#[derive(Debug, Default)]
enum SpartanTypestate {
   #[default]
   Unready,
   Ready,
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone)]
pub enum SizingScheme {
   #[default]
   SquareShrink,
   SquareCenter,
   Fill,
}

#[derive(Debug, Default)]
pub struct SpartanPreparation {
   pub scale: [f64; 2],
   pub offset: [f64; 2],
   pub canvas_size: [f64; 2],
   pub padding: Vec<f64>,
   pub font_size: f64,
   pub point_size: f64,
   pub line_width: f64,
}

#[derive(Debug, Serialize, DefaultFromSerde)]
pub struct SpartanDiagram {
   // pub scale: f64,
   // // #[serde(skip_serializing_if = "is_default", default)]
   #[serde(skip)]
   typestate: SpartanTypestate,

   #[serde(skip)]
   pub prep: SpartanPreparation,
   pub sizing_scheme: SizingScheme,
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_base_width",
      default = "SpartanDiagram::default_base_width"
   )]
   pub base_width: f64,
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_base_height",
      default = "SpartanDiagram::default_base_height"
   )]
   pub base_height: f64,
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_base_font_size",
      default = "SpartanDiagram::default_base_font_size"
   )]
   pub base_font_size: f64,
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_base_point_size",
      default = "SpartanDiagram::default_base_point_size"
   )]
   pub base_point_size: f64,
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_base_line_width",
      default = "SpartanDiagram::default_base_line_width"
   )]
   pub base_line_width: f64,
   // Scaling of 1-D annotations, such as grid line width vs normal.
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_annotation_linear_scale",
      default = "SpartanDiagram::default_annotation_linear_scale"
   )]
   pub annotation_linear_scale: f64,
   // Scaling of 2-D annotations, such as font size vs titling.
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_annotation_area_scale",
      default = "SpartanDiagram::default_annotation_area_scale"
   )]
   pub annotation_area_scale: f64,

   // Optionally (if non-zero) specify scaling of diagram size from base values.
   #[serde(skip_serializing_if = "is_default")]
   pub scale_width: f64,
   #[serde(skip_serializing_if = "is_default")]
   pub scale_height: f64,
   // Main line-width scaling as diagram scales. Default is to use something like the square
   // root of the geometric mean of the width and height scaling, so that content grows
   // gradually.
   #[serde(skip_serializing_if = "is_default")]
   pub scale_content: f64,

   #[serde(skip_serializing_if = "is_default")]
   pub axes_range: Vec<f64>,
   #[serde(skip_serializing_if = "is_default")]
   pub padding: Vec<f64>,
}

impl SpartanDiagram {
   #[must_use]
   pub fn new() -> SpartanDiagram {
      SpartanDiagram {
         // scale: 1.0,
         ..Default::default()
      }
   }

   #[must_use]
   fn is_near_float(v: f64, w: f64) -> bool {
      (v - w).abs() < 0.0001
   }
   #[must_use]
   pub fn is_ready(&self) -> bool {
      matches!(self.typestate, SpartanTypestate::Ready)
   }

   #[must_use]
   fn default_base_width() -> f64 {
      400.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_width(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_width())
   }

   #[must_use]
   fn default_base_height() -> f64 {
      300.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_height(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_height())
   }

   #[must_use]
   fn default_base_font_size() -> f64 {
      12.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_font_size(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_font_size())
   }

   #[must_use]
   fn default_base_point_size() -> f64 {
      15.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_point_size(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_point_size())
   }

   #[must_use]
   fn default_base_line_width() -> f64 {
      1.1
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_line_width(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_line_width())
   }

   #[must_use]
   fn default_annotation_linear_scale() -> f64 {
      0.5
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_annotation_linear_scale(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_annotation_linear_scale())
   }

   #[must_use]
   fn default_annotation_area_scale() -> f64 {
      0.7
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_annotation_area_scale(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_annotation_area_scale())
   }

   fn multiply_default_one(a: f64, b: f64) -> f64 {
      if b == 0.0 {
         a
      } else {
         a * b
      }
   }

   #[allow(clippy::missing_panics_doc)]
   pub fn prepare(&mut self) {
      assert!(matches!(self.typestate, SpartanTypestate::Unready));

      self.prep.canvas_size = [
         Self::multiply_default_one(self.base_width, self.scale_width),
         Self::multiply_default_one(self.base_height, self.scale_height),
      ];

      let mut axes_range = self.axes_range.clone();
      match axes_range.len() {
         1 => {
            axes_range = [-axes_range[0], -axes_range[0], axes_range[0], axes_range[0]].to_vec();
         }
         2 => {
            axes_range = [-axes_range[0], -axes_range[1], axes_range[0], axes_range[1]].to_vec();
         }
         4 => {}
         _ => {
            panic!(
               "axes_range must be vector of size 1, 2 or 4, but found size {}",
               axes_range.len()
            );
         }
      }
      let mut padding = self.padding.clone();
      match padding.len() {
         0 => {
            padding = [0.0, 0.0, 0.0, 0.0].to_vec();
         }
         1 => {
            padding = [-padding[0], -padding[0], padding[0], padding[0]].to_vec();
         }
         2 => {
            padding = [-padding[0], -padding[1], padding[0], padding[1]].to_vec();
         }
         4 => {}
         _ => {
            panic!("padding must be vector of size 0, 1, 2 or 4, but found size {}", padding.len());
         }
      }
      self.prep.padding.clone_from(&padding);

      let x_min = axes_range[0];
      let y_min = axes_range[1];
      let x_max = axes_range[2];
      let y_max = axes_range[3];
      let left_padding = padding[0];
      let bottom_padding = padding[1];
      let right_padding = padding[2];
      let top_padding = padding[3];

      let total_width_range = (x_max - x_min) * (1.0 + left_padding + right_padding);
      let total_height_range = (y_max - y_min) * (1.0 + bottom_padding + top_padding);
      let mut width_adjustment = 0.0;
      let mut height_adjustment = 0.0;

      let is_width_limited: bool = (total_width_range * self.prep.canvas_size[1])
         > (total_height_range * self.prep.canvas_size[0]);

      match self.sizing_scheme {
         SizingScheme::SquareShrink => {
            if is_width_limited {
               self.prep.canvas_size[1] =
                  total_height_range * self.prep.canvas_size[0] / total_width_range;
            } else {
               self.prep.canvas_size[0] =
                  total_width_range * self.prep.canvas_size[1] / total_height_range;
            }
         }
         SizingScheme::SquareCenter => {
            if is_width_limited {
               height_adjustment = 0.5
                  * (total_width_range * self.prep.canvas_size[1] / self.prep.canvas_size[0]
                     - total_height_range);
            } else {
               width_adjustment = 0.5
                  * (total_height_range * self.prep.canvas_size[0] / self.prep.canvas_size[1]
                     - total_width_range);
            }
         }
         SizingScheme::Fill => {}
      }

      self.prep.scale = [
         self.prep.canvas_size[0] / (total_width_range + 2.0 * width_adjustment),
         self.prep.canvas_size[1] / (total_height_range + 2.0 * height_adjustment),
      ];

      self.prep.offset = [
         self.prep.scale[0] * (-x_min + (x_max - x_min) * left_padding + width_adjustment),
         self.prep.scale[1] * (-y_min + (y_max - y_min) * bottom_padding + height_adjustment),
      ];
      // self.prep.font_size = multiply_default_one(self.base_font_size, self.
      // self.prep.point_size
      // self.prep.line_width

      self.typestate = SpartanTypestate::Ready;
   }
}

#[derive(Debug, Default)]
pub struct CairoSpartanRender {
   pub spartan: SpartanDiagram,

   pub saved_matrix: Matrix,
}

impl CairoSpartanRender {
   #[must_use]
   pub fn new() -> CairoSpartanRender {
      CairoSpartanRender { ..Default::default() }
   }

   // This is necessary because line thicknesses and similar are distorted if the x and y
   // scales differ.  Consequently we only use the scaling transform for the Cairo CTM when
   // creating paths.
   pub fn save_set_path_transform(&mut self, context: &Context) {
      self.saved_matrix = context.matrix();

      context.translate(
         self.spartan.prep.offset[0],
         self.spartan.prep.canvas_size[1] - self.spartan.prep.offset[1],
      );
      context.scale(self.spartan.prep.scale[0], -self.spartan.prep.scale[1]);
   }

   // Be sure to restore the original transform before stroking out a path with a pen.  This is
   // so that the original Cairo CTM, which should be isotropic, is used for the stroke pen.
   pub fn restore_transform(&mut self, context: &Context) {
      context.set_matrix(self.saved_matrix);
   }
}
