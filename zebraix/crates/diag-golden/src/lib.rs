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
use cairo::SvgSurface;
use cairo::SvgUnit::Pt;
use goldenfile::Mint;
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use serde_json::to_string_pretty;
use serde_json::Value;
use std::f64::consts::PI;
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
   pub fn new(mint_dir: &str, golden_filestem: &str) -> Self {
      let mut mint = Mint::new(mint_dir);
      let full_golden_filename = format!("{golden_filestem}.svg");
      let out_stream = mint.new_goldenfile(full_golden_filename.clone()).unwrap();

      Self {
         _mint_dir: mint_dir.to_string(),
         _output_filename: full_golden_filename,
         _mint: mint,
         out_stream,
      }
   }

   // input_filestem can also create
   pub fn read_to_string(&mut self, input_filestem: &str) -> String {
      let full_input_path_string = input_filestem.to_string() + ".svg";
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
      Self::filter_result(Box::new(result.as_bytes()), &self.out_stream);
      let bytes_amount_nl = self.out_stream.write(b"\n").unwrap();
      assert!(bytes_amount_nl == 1);
   }

   #[must_use]
   pub const fn get_raw_writeable(&self) -> SvgGoldenWriteable {
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
   pub fn new(mint_dir: &str, golden_filestem: &str) -> Self {
      let mut mint = Mint::new(mint_dir);
      let full_golden_filename = format!("{golden_filestem}.json");
      // let out_stream = mint.new_goldenfile(output_filename).unwrap();
      let out_stream = mint
         .new_goldenfile_with_differ(full_golden_filename.clone(), Box::new(Self::custom_diff))
         .unwrap();

      Self {
         mint_dir: mint_dir.to_string(),
         _output_filename: full_golden_filename,
         _mint: mint,
         out_stream,
      }
   }

   // input_filestem can also create
   pub fn read_to_string(&mut self, input_filestem: &str) -> String {
      let full_input_path_string = input_filestem.to_string() + ".json";
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
      Ok(result) => result,
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
   pub annotation_offset_absolute: [f64; 2], // Horiz and vert text offsets, relative to font size.
   pub scale_content: f64,
   pub annotation_linear_scale: f64,
   pub annotation_area_scale: f64,
   pub axes_range: Vec<f64>,
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

   #[serde(skip_serializing_if = "is_default")]
   pub base_color_choice: ColorChoice,

   #[serde(skip_serializing_if = "is_default")]
   pub light_color_choice: ColorChoice,

   #[serde(skip_serializing_if = "is_default")]
   pub text_color_choice: ColorChoice,

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

   // Applied as horiz and vert scalings of the font size.
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_annotation_offset",
      default = "SpartanDiagram::default_annotation_offset"
   )]
   pub annotation_offset: [f64; 2],

   // Optionally (if non-zero) specify scaling of diagram size from base values.
   #[serde(skip_serializing_if = "is_default")]
   pub scale_width: f64,
   #[serde(skip_serializing_if = "is_default")]
   pub scale_height: f64,
   // Main line-width scaling as diagram scales. If zero, use something like the square
   // root of the geometric mean of the width and height scaling, so that content grows
   // gradually.
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_scale_content",
      default = "SpartanDiagram::default_scale_content"
   )]
   pub scale_content: f64,

   #[serde(skip_serializing_if = "is_default")]
   pub axes_range: Vec<f64>,
   #[serde(skip_serializing_if = "is_default")]
   pub padding: Vec<f64>,

   #[serde(skip_serializing_if = "is_default")]
   pub drawables: Vec<QualifiedDrawable>,
}

impl SpartanDiagram {
   #[must_use]
   pub fn new() -> Self {
      Self::default()
   }

   #[must_use]
   fn is_near_float(v: f64, w: f64) -> bool {
      (v - w).abs() < 0.0001
   }
   #[must_use]
   pub const fn is_ready(&self) -> bool {
      matches!(self.typestate, SpartanTypestate::Ready)
   }

   #[must_use]
   const fn default_base_width() -> f64 {
      400.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_width(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_width())
   }

   #[must_use]
   const fn default_base_height() -> f64 {
      300.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_height(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_height())
   }

   #[must_use]
   const fn default_base_font_size() -> f64 {
      11.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_font_size(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_font_size())
   }

   #[must_use]
   const fn default_base_point_size() -> f64 {
      15.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_point_size(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_point_size())
   }

   // 1.0 is a regular thickness, definitely not thick, 2.0 definitely thick, 0.6 thin but
   // firm.
   #[must_use]
   const fn default_base_line_width() -> f64 {
      1.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_line_width(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_line_width())
   }

   #[must_use]
   const fn default_scale_content() -> f64 {
      1.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_scale_content(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_scale_content())
   }

   #[must_use]
   const fn default_annotation_linear_scale() -> f64 {
      0.45
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_annotation_linear_scale(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_annotation_linear_scale())
   }

   #[must_use]
   const fn default_annotation_area_scale() -> f64 {
      0.85
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_annotation_area_scale(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_annotation_area_scale())
   }

   #[must_use]
   const fn default_annotation_offset() -> [f64; 2] {
      [0.5, 0.2]
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_annotation_offset(v: &[f64; 2]) -> bool {
      let default_value = Self::default_annotation_offset();
      Self::is_near_float((*v)[0], default_value[0])
         && Self::is_near_float((*v)[1], default_value[1])
   }

   fn multiply_default_one(a: f64, b: f64) -> f64 {
      if b == 0.0 {
         a
      } else {
         a * b
      }
   }

   #[allow(clippy::too_many_lines)]
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
      self.prep.axes_range.clone_from(&axes_range);

      let mut padding = self.padding.clone();
      match padding.len() {
         0 => {
            padding = [0.0, 0.0, 0.0, 0.0].to_vec();
         }
         1 => {
            padding = [padding[0], padding[0], padding[0], padding[0]].to_vec();
         }
         2 => {
            padding = [padding[0], padding[1], padding[0], padding[1]].to_vec();
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
         self.prep.canvas_size[0] / 2.0f64.mul_add(width_adjustment, total_width_range),
         self.prep.canvas_size[1] / 2.0f64.mul_add(height_adjustment, total_height_range),
      ];

      self.prep.offset = [
         self.prep.scale[0] * (x_max - x_min).mul_add(left_padding, -x_min + width_adjustment),
         self.prep.scale[1] * (y_max - y_min).mul_add(bottom_padding, -y_min + height_adjustment),
      ];

      let mut scale_content = self.scale_content;

      // If content scaling not specified, use a heuristic based on overall diagram scaling.
      if scale_content == 0.0 {
         scale_content =
            (self.prep.scale[0] * (x_max - x_min) * self.prep.scale[1] * (y_max - y_min)
               / Self::default_base_width()
               / Self::default_base_height())
            .sqrt();
      }
      self.prep.scale_content = scale_content;

      self.prep.font_size = self.base_font_size * self.prep.scale_content;
      self.prep.point_size = self.base_point_size * self.prep.scale_content;
      self.prep.line_width = self.base_line_width * self.prep.scale_content;
      self.prep.annotation_offset_absolute[0] = self.base_font_size * self.annotation_offset[0];
      self.prep.annotation_offset_absolute[1] = self.base_font_size * self.annotation_offset[1];
      self.prep.annotation_linear_scale = self.annotation_linear_scale;
      self.prep.annotation_area_scale = self.annotation_area_scale;

      if self.light_color_choice == ColorChoice::default() {
         self.light_color_choice = self.base_color_choice;
      }

      if self.text_color_choice == ColorChoice::default() {
         self.text_color_choice = self.base_color_choice;
      }

      self.typestate = SpartanTypestate::Ready;
   }
}

#[derive(Debug, Default)]
pub struct CairoSpartanRender {
   pub saved_matrix: Matrix,
}

impl CairoSpartanRender {
   #[must_use]
   pub fn new() -> Self {
      Self::default()
   }
   // This is necessary because line thicknesses and similar are distorted if the x and y
   // scales differ.  Consequently we only use the scaling transform for the Cairo CTM when
   // creating paths.
   pub fn save_set_path_transform(&mut self, prep: &SpartanPreparation, context: &Context) {
      self.saved_matrix = context.matrix();

      context.translate(prep.offset[0], prep.canvas_size[1] - prep.offset[1]);
      context.scale(prep.scale[0], -prep.scale[1]);
   }

   // Be sure to restore the original transform before stroking out a path with a pen.  This is
   // so that the original Cairo CTM, which should be isotropic, is used for the stroke pen.
   pub fn restore_transform(&mut self, context: &Context) {
      context.set_matrix(self.saved_matrix);
   }

   #[allow(clippy::unused_self)]
   fn set_color(&self, context: &Context, _prep: &SpartanPreparation, color_choice: ColorChoice) {
      let (r, g, b) = match color_choice {
         ColorChoice::DefaultBlack | ColorChoice::Black => (0.0, 0.0, 0.0),
         ColorChoice::Gray => (0.55, 0.55, 0.55),
         ColorChoice::DarkGray => (0.35, 0.35, 0.35),
         ColorChoice::LightGray => (0.7, 0.7, 0.7),
         ColorChoice::BrightRed => (1.0, 0.0, 0.0),
         ColorChoice::BrightGreen => (0.0, 1.0, 0.0),
         ColorChoice::BrightBlue => (0.0, 0.0, 1.0),
         ColorChoice::BrightYellow => (1.0, 1.0, 0.0),
         ColorChoice::BrightCyan => (0.0, 1.0, 1.0),
         ColorChoice::BrightMagenta => (1.0, 0.0, 1.0),
         ColorChoice::Red => (0.6, 0.0, 0.0),
         ColorChoice::Green => (0.0, 0.4, 0.0),
         ColorChoice::Blue => (0.0, 0.0, 0.65),
         ColorChoice::YellowBrown => (0.37, 0.28, 0.0),
         ColorChoice::BlueGreen => (0.0, 0.3, 0.3),
         ColorChoice::Magenta => (0.35, 0.0, 0.5),
         ColorChoice::RedRedGreen => (0.45, 0.18, 0.0),
         ColorChoice::GreenGreenRed => (0.24, 0.32, 0.0),
         ColorChoice::BlueBlueGreen => (0.0, 0.18, 0.45),
         ColorChoice::GreenGreenBlue => (0.0, 0.36, 0.18),
         ColorChoice::RedRedBlue => (0.47, 0.0, 0.34),
         ColorChoice::BlueBlueRed => (0.23, 0.0, 0.55),
      };
      context.set_source_rgb(r, g, b);
   }

   fn set_line_choice(context: &Context, line_choice: LineChoice, prep: &SpartanPreparation) {
      match line_choice {
         LineChoice::Ordinary => {
            context.set_line_width(prep.line_width);
            context.set_dash(&[], 0.0);
         }
         LineChoice::Light => {
            context.set_line_width(prep.line_width * prep.annotation_linear_scale);
            // assert_eq!(prep.annotation_linear_scale, 0.45);
            context.set_dash(
               &[10.0 * prep.annotation_linear_scale, 7.0 * prep.annotation_linear_scale],
               0.0,
            );
         }
      }
   }

   fn draw_lines_set(
      &mut self,
      context: &Context,
      drawable: &LinesDrawable,
      prep: &SpartanPreparation,
   ) {
      Self::set_line_choice(context, drawable.line_choice, prep);
      self.set_color(context, prep, drawable.color_choice);

      self.save_set_path_transform(prep, context);
      assert_eq!(drawable.start.len(), drawable.end.len());
      for i in 0..drawable.start.len() {
         for offset in &drawable.offsets {
            context.move_to(drawable.start[i][0] + offset[0], drawable.start[i][1] + offset[1]);
            context.line_to(drawable.end[i][0] + offset[0], drawable.end[i][1] + offset[1]);
         }
      }
      self.restore_transform(context);
      context.stroke().unwrap();
   }

   fn draw_points_set(
      &mut self,
      context: &Context,
      drawable: &PointsDrawable,
      prep: &SpartanPreparation,
   ) {
      Self::set_line_choice(context, LineChoice::Ordinary, prep);
      self.set_color(context, prep, drawable.color_choice);

      match drawable.point_choice {
         PointChoice::Circle => {
            for center in &drawable.centers {
               self.save_set_path_transform(prep, context);
               let (cx, cy) = context.user_to_device(center[0], center[1]);
               self.restore_transform(context);
               context.move_to(cx + 2.8, cy);
               context.arc(cx, cy, 2.8, 0.0 * PI, 2.0 * PI);
               context.close_path();
            }
         }
         PointChoice::Dot => {
            for center in &drawable.centers {
               self.save_set_path_transform(prep, context);
               let (cx, cy) = context.user_to_device(center[0], center[1]);
               self.restore_transform(context);
               #[allow(clippy::suboptimal_flops)]
               context.move_to(cx + 2.8 * 0.92, cy);
               #[allow(clippy::suboptimal_flops)]
               context.arc(cx, cy, 2.8 * 0.92, 0.0 * PI, 2.0 * PI);
               context.fill().unwrap();
               context.close_path();
            }
         }
         PointChoice::Plus => {
            for center in &drawable.centers {
               self.save_set_path_transform(prep, context);
               let (cx, cy) = context.user_to_device(center[0], center[1]);
               self.restore_transform(context);
               let plus_delta = 2.8 * 1.48;
               context.move_to(cx, cy - plus_delta);
               context.line_to(cx, cy + plus_delta);
               context.move_to(cx + plus_delta, cy);
               context.line_to(cx - plus_delta, cy);
               context.close_path();
            }
         }
         PointChoice::Times => {
            for center in &drawable.centers {
               self.save_set_path_transform(prep, context);
               let (cx, cy) = context.user_to_device(center[0], center[1]);
               self.restore_transform(context);
               let times_delta = 2.8 * 1.48 * (0.5_f64).sqrt();
               context.move_to(cx - times_delta, cy - times_delta);
               context.line_to(cx + times_delta, cy + times_delta);
               context.move_to(cx + times_delta, cy - times_delta);
               context.line_to(cx - times_delta, cy + times_delta);
               context.close_path();
            }
         }
      }
      context.stroke().unwrap();
   }

   fn draw_text_set(
      &mut self,
      context: &Context,
      drawable: &TextDrawable,
      prep: &SpartanPreparation,
   ) {
      let area_based_scale = match drawable.size_choice {
         TextSizeChoice::Normal => 1.0,
         TextSizeChoice::Large => 1.0 / prep.annotation_area_scale,
         TextSizeChoice::Small => prep.annotation_area_scale,
      };
      let font_size = prep.font_size * area_based_scale;

      for single_text in &drawable.texts {
         // Create a single context, instead of using create_layout.  This
         // demonstrates avoiding lots of Pango contexts.
         let text_context = pangocairo::functions::create_context(context);
         let text_layout = pango::Layout::new(&text_context);

         let mut font_description = pango::FontDescription::new();
         font_description.set_family("sans");
         font_description.set_absolute_size(font_size * f64::from(pango::SCALE));
         text_layout.set_font_description(Some(&font_description));

         let metrics = text_layout.context().metrics(Some(&font_description), None);
         // Strikethrough is top of line above baseline.
         let strikethrough_center = 0.5
            * f64::from(2 * metrics.strikethrough_position() - metrics.strikethrough_thickness());
         let even_half_height = f64::max(
            f64::from(metrics.ascent()) - strikethrough_center,
            f64::from(metrics.descent()) + strikethrough_center,
         );

         text_layout.set_text(&single_text.content);
         let (text_width, text_height) = text_layout.size();

         self.set_color(context, prep, drawable.color_choice);

         self.save_set_path_transform(prep, context);
         let (tx, ty) = context.user_to_device(single_text.location[0], single_text.location[1]);
         let (offset_x, offset_y) = match drawable.offset_choice {
            TextOffsetChoice::None => (0.0, 0.0),
            TextOffsetChoice::Both => (
               prep.annotation_offset_absolute[0] * area_based_scale * f64::from(pango::SCALE),
               prep.annotation_offset_absolute[1] * area_based_scale * f64::from(pango::SCALE),
            ),
         };
         self.restore_transform(context);

         let mut height_adjust = f64::from(metrics.ascent()) - strikethrough_center;
         let multiline_adjust = f64::from(text_height - metrics.height());

         height_adjust += match drawable.anchor_vertical {
            TextAnchorVertical::Bottom => even_half_height + multiline_adjust + offset_y,
            TextAnchorVertical::Middle => 0.5 * multiline_adjust,
            TextAnchorVertical::Top => -even_half_height - offset_y,
         };
         let width_adjust = match drawable.anchor_horizontal {
            TextAnchorHorizontal::Left => -offset_x,
            TextAnchorHorizontal::Center => 0.5 * f64::from(text_width),
            TextAnchorHorizontal::Right => f64::from(text_width) + offset_x,
         };

         context.move_to(
            tx - width_adjust / f64::from(pango::SCALE),
            ty - height_adjust / f64::from(pango::SCALE),
         );
         pangocairo::functions::show_layout(context, &text_layout);
         context.stroke().unwrap();
      }
   }

   fn draw_polyine(
      &mut self,
      context: &Context,
      drawable: &PolylineDrawable,
      prep: &SpartanPreparation,
   ) {
      Self::set_line_choice(context, drawable.line_choice, prep);
      self.set_color(context, prep, drawable.color_choice);

      self.save_set_path_transform(prep, context);
      assert!(!drawable.locations.is_empty());
      context.move_to(drawable.locations[0][0], drawable.locations[0][1]);
      for i in 1..drawable.locations.len() {
         context.line_to(drawable.locations[i][0], drawable.locations[i][1]);
      }
      if drawable.line_closure_choice == LineClosureChoice::Closed {
         context.close_path();
      }
      self.restore_transform(context);
      context.stroke().unwrap();
   }

   fn draw_circles_set(
      &mut self,
      context: &Context,
      drawable: &CirclesDrawable,
      prep: &SpartanPreparation,
   ) {
      Self::set_line_choice(context, drawable.line_choice, prep);
      self.set_color(context, prep, drawable.color_choice);

      for center in &drawable.centers {
         self.save_set_path_transform(prep, context);
         let (cx, cy) = (center[0], center[1]);
         let r = drawable.radius;
         // let (cx, cy) = context.user_to_device(center[0], center[1]);
         // let r = context.user_to_device_distance(drawable.radius, 0.0);
         // self.restore_transform(context);
         context.move_to(cx + r, cy);
         context.arc(cx, cy, r, 0.0 * PI, 2.0 * PI);
         context.close_path();
         self.restore_transform(context);
      }
      context.stroke().unwrap();
   }

   #[allow(clippy::missing_panics_doc)]
   pub fn render_drawables(&mut self, spartan: &SpartanDiagram, context: &Context) {
      let drawables = &spartan.drawables;
      let mut indices = (0..drawables.len()).collect::<Vec<_>>();
      indices.sort_by_key(|&i| &drawables[i].layer);
      // for qualified_drawable in &spartan.drawables {
      //   match &qualified_drawable.drawable {
      for i in indices {
         match &drawables[i].drawable {
            OneOfDrawable::Lines(drawable) => {
               self.draw_lines_set(context, drawable, &spartan.prep);
            }
            OneOfDrawable::Points(drawable) => {
               self.draw_points_set(context, drawable, &spartan.prep);
            }
            OneOfDrawable::Text(drawable) => {
               self.draw_text_set(context, drawable, &spartan.prep);
            }
            OneOfDrawable::Circles(drawable) => {
               self.draw_circles_set(context, drawable, &spartan.prep);
            }
            OneOfDrawable::Polyline(drawable) => {
               self.draw_polyine(context, drawable, &spartan.prep);
            }
            OneOfDrawable::Nothing => {}
         }
      }
   }

   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   pub fn render_to_stream<W: Write + 'static>(
      &mut self,
      out_stream: W,
      spartan: &SpartanDiagram,
   ) -> Result<Box<dyn core::any::Any>, cairo::StreamWithError> {
      let canvas_size = &spartan.prep.canvas_size;
      let mut surface = SvgSurface::for_stream(canvas_size[0], canvas_size[1], out_stream).unwrap();
      surface.set_document_unit(Pt);

      let context = cairo::Context::new(&surface).unwrap();

      self.render_drawables(spartan, &context);

      surface.flush();
      surface.finish_output_stream()
   }
}

#[derive(Debug, Default)]
pub struct CairoSpartanCombo {
   pub spartan: SpartanDiagram,

   pub render_controller: CairoSpartanRender,
}

impl CairoSpartanCombo {
   #[must_use]
   pub fn new() -> Self {
      Self::default()
   }
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum AxesStyle {
   #[default]
   None,
   Box,
   Cross,
   BoxCross,
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum AxisNumbering {
   #[default]
   At,
   Before,
   After,
   None,
}

#[derive(Debug, Serialize, DefaultFromSerde)]
pub struct AxesSpec {
   #[serde(skip_serializing_if = "is_default")]
   pub axes_style: AxesStyle,
   #[serde(skip_serializing_if = "is_default")]
   pub axis_numbering: AxisNumbering,
   #[serde(skip_serializing_if = "is_default")]
   pub grid_interval: [f64; 2],
   #[serde(skip_serializing_if = "is_default")]
   pub grid_precision: Vec<usize>,
}

impl AxesSpec {
   #[must_use]
   pub fn new(style: AxesStyle) -> Self {
      Self { axes_style: style, ..Default::default() }
   }

   #[must_use]
   fn add_grid_lines(
      &self,
      vertical_light: &mut LinesDrawable,
      one_range: [f64; 2],
      horiz_interval: f64,
      x_tolerance: f64,
      has_vert_zero: bool,
      offset_pattern: [f64; 2],
   ) -> (Option<f64>, Option<f64>) {
      let left_numbering_location: Option<f64>;
      let right_numbering_location: Option<f64>;

      let is_boxy: bool = match self.axes_style {
         AxesStyle::BoxCross | AxesStyle::Box => true,
         AxesStyle::Cross | AxesStyle::None => false,
      };

      if horiz_interval == 0.0 {
         if self.axes_style == AxesStyle::None {
            left_numbering_location = None;
            right_numbering_location = None;
         } else {
            left_numbering_location = Some(one_range[0]);
            right_numbering_location = Some(one_range[1]);
         }
      } else {
         let (mut left_scan, mut right_scan) = if has_vert_zero {
            match self.axes_style {
               AxesStyle::Box | AxesStyle::None => (0.0, horiz_interval),
               AxesStyle::Cross | AxesStyle::BoxCross => (-horiz_interval, horiz_interval),
            }
         } else {
            let snapped_mid_range =
               (0.5 * (one_range[0] + one_range[1]) / horiz_interval).floor() * horiz_interval;
            (snapped_mid_range, snapped_mid_range + horiz_interval)
         };

         let mid_range = 0.5 * (left_scan + right_scan);
         let mut final_left_location = right_scan;
         let mut final_right_location = left_scan;
         // If the following assertions remain true after grid line scan, we have not found a
         // numbering location.
         assert!(final_left_location > mid_range);
         assert!(final_right_location < mid_range);
         #[allow(clippy::while_float)]
         while left_scan > one_range[0] - x_tolerance {
            if !is_boxy || (left_scan > one_range[0] + x_tolerance) {
               vertical_light
                  .offsets
                  .push([left_scan * offset_pattern[0], left_scan * offset_pattern[1]]);
            }
            assert!(vertical_light.offsets.len() < 100);
            final_left_location = left_scan;
            left_scan -= horiz_interval;
         }
         #[allow(clippy::while_float)]
         while right_scan < one_range[1] + x_tolerance {
            if !is_boxy || (right_scan < one_range[1] - x_tolerance) {
               vertical_light
                  .offsets
                  .push([right_scan * offset_pattern[0], right_scan * offset_pattern[1]]);
            }
            assert!(vertical_light.offsets.len() < 100);
            final_right_location = right_scan;
            right_scan += horiz_interval;
         }

         if final_left_location > mid_range {
            left_numbering_location = None;
         } else {
            left_numbering_location = Some(final_left_location);
         }
         if final_right_location < mid_range {
            right_numbering_location = None;
         } else {
            right_numbering_location = Some(final_right_location);
         }
      }
      (left_numbering_location, right_numbering_location)
   }

   #[allow(clippy::too_many_lines)]
   #[allow(clippy::missing_panics_doc)]
   pub fn generate_axes(&self, diagram: &mut SpartanDiagram) {
      // Future improvement ideas:
      //
      // * Generate box as closed polygon.
      if (self.axes_style == AxesStyle::None)
         && (self.grid_interval[0] == 0.0)
         && (self.grid_interval[1] == 0.0)
      {
         return;
      }
      let range = &diagram.prep.axes_range;
      assert!(range[2] > range[0]);
      assert!(range[3] > range[1]);
      let relative_tolerance = 1000.0;
      let x_tolerance = (range[2] - range[0]).abs() / relative_tolerance;
      let y_tolerance = (range[3] - range[1]).abs() / relative_tolerance;

      let has_vert_zero = (-range[0] > x_tolerance) && (range[2] > x_tolerance);
      let has_horiz_zero = (-range[1] > y_tolerance) && (range[3] > y_tolerance);

      let axes_layer = 0;
      let mut lines_ordinary = LinesDrawable {
         offsets: vec![[0.0, 0.0]],
         color_choice: diagram.base_color_choice,
         ..Default::default()
      };
      let mut horizontal_light = LinesDrawable {
         start: vec![[range[0], 0.0]],
         end: vec![[range[2], 0.0]],
         line_choice: LineChoice::Light,
         color_choice: diagram.light_color_choice,
         ..Default::default()
      };
      let mut vertical_light = LinesDrawable {
         start: vec![[0.0, range[1]]],
         end: vec![[0.0, range[3]]],
         line_choice: LineChoice::Light,
         color_choice: diagram.light_color_choice,
         ..Default::default()
      };

      match self.axes_style {
         AxesStyle::BoxCross | AxesStyle::Box => {
            diagram.drawables.push(QualifiedDrawable {
               drawable: OneOfDrawable::Polyline(PolylineDrawable {
                  // This should be miter-join even if we switch default later.
                  line_closure_choice: LineClosureChoice::Closed,
                  locations: vec![
                     [range[0], range[1]],
                     [range[0], range[3]],
                     [range[2], range[3]],
                     [range[2], range[1]],
                  ],
                  ..Default::default()
               }),
               ..Default::default()
            });
         }
         AxesStyle::Cross | AxesStyle::None => {}
      }

      match self.axes_style {
         AxesStyle::BoxCross | AxesStyle::Cross => {
            if has_vert_zero {
               lines_ordinary.start.push([0.0, range[1]]);
               lines_ordinary.end.push([0.0, range[3]]);
            }
            if has_horiz_zero {
               lines_ordinary.start.push([range[0], 0.0]);
               lines_ordinary.end.push([range[2], 0.0]);
            }
         }
         AxesStyle::Box | AxesStyle::None => {}
      }

      // Grid lines, horizontal interval, vertical lines.
      let (left_numbering_location, right_numbering_location) = self.add_grid_lines(
         &mut vertical_light,
         [range[0], range[2]],
         self.grid_interval[0],
         x_tolerance,
         has_vert_zero,
         [1.0, 0.0],
      );

      // Grid lines, vertical interval, horizontal lines.
      let (bottom_numbering_location, top_numbering_location) = self.add_grid_lines(
         &mut horizontal_light,
         [range[1], range[3]],
         self.grid_interval[1],
         y_tolerance,
         has_horiz_zero,
         [0.0, 1.0],
      );

      if !lines_ordinary.start.is_empty() {
         // assert!(false);
         let qualified_drawable = QualifiedDrawable {
            layer: axes_layer,
            // color_choice: diagram.base_color_choice,
            drawable: OneOfDrawable::Lines(lines_ordinary),
         };
         diagram.drawables.push(qualified_drawable);
      }
      if !horizontal_light.offsets.is_empty() {
         let qualified_drawable = QualifiedDrawable {
            layer: axes_layer,
            // color_choice: diagram.light_color_choice,
            drawable: OneOfDrawable::Lines(horizontal_light),
         };
         diagram.drawables.push(qualified_drawable);
      }
      if !vertical_light.offsets.is_empty() {
         let qualified_drawable = QualifiedDrawable {
            layer: axes_layer,
            // color_choice: diagram.light_color_choice,
            drawable: OneOfDrawable::Lines(vertical_light),
         };
         diagram.drawables.push(qualified_drawable);
      }

      if self.axis_numbering != AxisNumbering::None {
         let horizontal_precision =
            if self.grid_precision.is_empty() { 20_usize } else { self.grid_precision[0] };
         let vertical_precision = if self.grid_precision.len() > 1 {
            self.grid_precision[1]
         } else {
            horizontal_precision
         };
         let (anchor_horizontal, anchor_vertical) = match self.axis_numbering {
            AxisNumbering::Before => (TextAnchorHorizontal::Right, TextAnchorVertical::Top),
            AxisNumbering::After => (TextAnchorHorizontal::Left, TextAnchorVertical::Bottom),
            AxisNumbering::At | AxisNumbering::None => {
               (TextAnchorHorizontal::Center, TextAnchorVertical::Middle)
            }
         };
         let mut horizontal_numbering = TextDrawable {
            size_choice: TextSizeChoice::Small,
            color_choice: diagram.text_color_choice,
            offset_choice: TextOffsetChoice::Both,
            anchor_horizontal,
            anchor_vertical: TextAnchorVertical::Top,
            ..Default::default()
         };
         let mut vertical_numbering = TextDrawable {
            size_choice: TextSizeChoice::Small,
            color_choice: diagram.text_color_choice,
            offset_choice: TextOffsetChoice::Both,
            anchor_horizontal: TextAnchorHorizontal::Right,
            anchor_vertical,
            ..Default::default()
         };

         let number_at_zero = self.axes_style == AxesStyle::Cross;

         let vertical_for_horizontal = if has_vert_zero && number_at_zero { 0.0 } else { range[1] };
         let horizontal_for_vertical =
            if has_horiz_zero && number_at_zero { 0.0 } else { range[0] };

         if let Some(location) = left_numbering_location {
            horizontal_numbering.texts.push(TextSingle {
               content: format!("{location:.horizontal_precision$}"),
               location: [location, vertical_for_horizontal],
            });
         }
         if has_vert_zero && !number_at_zero {
            horizontal_numbering.texts.push(TextSingle {
               content: format!("{:.horizontal_precision$}", 0.0),
               location: [0.0, vertical_for_horizontal],
            });
         }
         if let Some(location) = right_numbering_location {
            horizontal_numbering.texts.push(TextSingle {
               content: format!("{location:.horizontal_precision$}"),
               location: [location, vertical_for_horizontal],
            });
         }
         if let Some(location) = bottom_numbering_location {
            vertical_numbering.texts.push(TextSingle {
               content: format!("{location:.vertical_precision$}"),
               location: [horizontal_for_vertical, location],
            });
         }
         if has_horiz_zero && !number_at_zero {
            vertical_numbering.texts.push(TextSingle {
               content: format!("{:.vertical_precision$}", 0.0),
               location: [horizontal_for_vertical, 0.0],
            });
         }
         if let Some(location) = top_numbering_location {
            vertical_numbering.texts.push(TextSingle {
               content: format!("{location:.vertical_precision$}"),
               location: [horizontal_for_vertical, location],
            });
         }

         // Change layer to depth.
         let axes_layer = 0;
         if !horizontal_numbering.texts.is_empty() {
            let qualified_drawable = QualifiedDrawable {
               layer: axes_layer,
               // color_choice: diagram.text_color_choice,
               drawable: OneOfDrawable::Text(horizontal_numbering),
            };
            diagram.drawables.push(qualified_drawable);
         }
         let axes_layer = 0;
         if !vertical_numbering.texts.is_empty() {
            let qualified_drawable = QualifiedDrawable {
               layer: axes_layer,
               // color_choice: diagram.text_color_choice,
               drawable: OneOfDrawable::Text(vertical_numbering),
            };
            diagram.drawables.push(qualified_drawable);
         }
      }
   }
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
   Both,
}

// Directions (horizontal, vertical) over which to offset anchoring.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum LineClosureChoice {
   #[default]
   Open,
   Closed,
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
   Magenta,
   RedRedGreen,
   GreenGreenRed,
   BlueBlueGreen,
   GreenGreenBlue,
   RedRedBlue,
   BlueBlueRed,
}

// Length of start and end must match.
#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct LinesDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub line_choice: LineChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub color_choice: ColorChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub start: Vec<[f64; 2]>,
   #[serde(skip_serializing_if = "is_default")]
   pub end: Vec<[f64; 2]>,
   // If offsets is empty, draw single line with no offset.
   #[serde(skip_serializing_if = "is_default")]
   pub offsets: Vec<[f64; 2]>,
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
   pub anchor_horizontal: TextAnchorHorizontal,
   #[serde(skip_serializing_if = "is_default")]
   pub anchor_vertical: TextAnchorVertical,
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
   pub line_choice: LineChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub line_closure_choice: LineClosureChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub color_choice: ColorChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub locations: Vec<[f64; 2]>,
}

#[derive(Serialize, Debug, Default, PartialEq)]
pub enum OneOfDrawable {
   #[default]
   Nothing,
   Lines(LinesDrawable),
   Points(PointsDrawable),
   Text(TextDrawable),
   Circles(CirclesDrawable),
   Polyline(PolylineDrawable),
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct QualifiedDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub layer: i32,
   #[serde(skip_serializing_if = "is_default")]
   pub drawable: OneOfDrawable,
}
