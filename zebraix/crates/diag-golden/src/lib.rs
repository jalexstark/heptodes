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

#[must_use]
fn is_near_float(v: f64, w: f64) -> bool {
   (v - w).abs() < 0.0001
}

#[must_use]
pub const fn default_unit_f64() -> f64 {
   1.0
}
#[allow(clippy::trivially_copy_pass_by_ref)]
#[must_use]
pub fn is_default_unit_f64(v: &f64) -> bool {
   is_near_float(*v, default_unit_f64())
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
   pub axes_range: [f64; 4],
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

   #[serde(skip, default = "SpartanDiagram::default_num_segments_hyperbolic")]
   pub num_segments_hyperbolic: i32,
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

   #[must_use]
   const fn default_num_segments_hyperbolic() -> i32 {
      50
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
      self.prep.axes_range = [axes_range[0], axes_range[1], axes_range[2], axes_range[3]];

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
         ColorChoice::BlueRed => (0.35, 0.0, 0.5),
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
         if let Some(offset_vector) = &drawable.offsets {
            for offset in offset_vector {
               context.move_to(drawable.start[i][0] + offset[0], drawable.start[i][1] + offset[1]);
               context.line_to(drawable.end[i][0] + offset[0], drawable.end[i][1] + offset[1]);
            }
         } else {
            context.move_to(drawable.start[i][0], drawable.start[i][1]);
            context.line_to(drawable.end[i][0], drawable.end[i][1]);
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

   fn draw_arc(&mut self, context: &Context, drawable: &ArcDrawable, prep: &SpartanPreparation) {
      //    // Elliptical transform matrix.  Zero angle is in direction of x axis.
      //    #[serde(skip_serializing_if = "is_default")]
      //    pub transform: Vec<[f64; 4]>,
      // }
      Self::set_line_choice(context, drawable.line_choice, prep);
      self.set_color(context, prep, drawable.color_choice);

      self.save_set_path_transform(prep, context);

      let arc_transformation_matrix = cairo::Matrix::new(
         drawable.transform[0],
         drawable.transform[1],
         drawable.transform[2],
         drawable.transform[3],
         drawable.center[0],
         drawable.center[1],
      );
      context.transform(arc_transformation_matrix);

      // Logically circle is center (0.0, 0.0) radius 1.0.
      context.move_to(drawable.angle_range[0].cos(), drawable.angle_range[0].sin());
      context.arc(0.0, 0.0, 1.0, drawable.angle_range[0], drawable.angle_range[1]);
      self.restore_transform(context);
      context.stroke().unwrap();
   }

   fn draw_cubic(
      &mut self,
      context: &Context,
      drawable: &CubicDrawable,
      prep: &SpartanPreparation,
   ) {
      Self::set_line_choice(context, drawable.line_choice, prep);
      self.set_color(context, prep, drawable.color_choice);

      self.save_set_path_transform(prep, context);

      context.move_to(drawable.x[0], drawable.y[0]);
      context.curve_to(
         drawable.x[1],
         drawable.y[1],
         drawable.x[2],
         drawable.y[2],
         drawable.x[3],
         drawable.y[3],
      );

      self.restore_transform(context);
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
            OneOfDrawable::Arc(drawable) => {
               self.draw_arc(context, drawable, &spartan.prep);
            }
            OneOfDrawable::Cubic(drawable) => {
               self.draw_cubic(context, drawable, &spartan.prep);
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

         let mut offsets = vertical_light.offsets.clone().unwrap_or_default();

         #[allow(clippy::while_float)]
         while left_scan > one_range[0] - x_tolerance {
            if !is_boxy || (left_scan > one_range[0] + x_tolerance) {
               offsets.push([left_scan * offset_pattern[0], left_scan * offset_pattern[1]]);
            }
            assert!(offsets.len() < 100);
            final_left_location = left_scan;
            left_scan -= horiz_interval;
         }

         #[allow(clippy::while_float)]
         while right_scan < one_range[1] + x_tolerance {
            if !is_boxy || (right_scan < one_range[1] - x_tolerance) {
               offsets.push([right_scan * offset_pattern[0], right_scan * offset_pattern[1]]);
            }
            assert!(offsets.len() < 100);
            final_right_location = right_scan;
            right_scan += horiz_interval;
         }
         vertical_light.offsets = Some(offsets);

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
         offsets: Some(vec![[0.0, 0.0]]),
         color_choice: diagram.base_color_choice,
         ..Default::default()
      };
      let mut horizontal_light = LinesDrawable {
         start: vec![[range[0], 0.0]],
         end: vec![[range[2], 0.0]],
         line_choice: LineChoice::Light,
         color_choice: diagram.light_color_choice,
         offsets: Some(Vec::<[f64; 2]>::new()),
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

      if horizontal_light.offsets.as_ref().is_some_and(|x| !x.is_empty()) {
         let qualified_drawable = QualifiedDrawable {
            layer: axes_layer,
            // color_choice: diagram.light_color_choice,
            drawable: OneOfDrawable::Lines(horizontal_light),
         };
         diagram.drawables.push(qualified_drawable);
      }
      if vertical_light.offsets.as_ref().is_some_and(|x| !x.is_empty()) {
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
   BlueRed,
   RedRedGreen,
   GreenGreenRed,
   BlueBlueGreen,
   GreenGreenBlue,
   RedRedBlue,
   BlueBlueRed,
}

// Length of start and end must match.
//
// Probably refactor to make vector of pairs of coords.
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
   pub offsets: Option<Vec<[f64; 2]>>,
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct ArcDrawable {
   #[serde(skip_serializing_if = "is_default")]
   pub line_choice: LineChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub color_choice: ColorChoice,
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
   pub line_choice: LineChoice,
   #[serde(skip_serializing_if = "is_default")]
   pub color_choice: ColorChoice,
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
   Arc(ArcDrawable),
   Cubic(CubicDrawable),
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum ZebraixAngle {
   Quadrant(f64),
   Radians(f64),
   TanHalf(f64),
}

impl ZebraixAngle {
   #[inline]
   #[must_use]
   pub fn in_radians(&self) -> f64 {
      match self {
         Self::Quadrant(q) => 0.5 * q * std::f64::consts::PI,
         Self::Radians(r) => *r,
         Self::TanHalf(t) => 2.0 * t.atan(),
      }
   }

   // This is really not good. We should deal with half the opening angle, or otherwise we get
   // strangeness as regards interpretation of angles (such as subtracting 2 pi from angle.
   #[inline]
   #[must_use]
   pub fn cos(&self) -> f64 {
      match self {
         Self::Quadrant(_) => self.in_radians().cos(),
         Self::Radians(r) => r.cos(),
         Self::TanHalf(t) => (1.0 - t * t) / (1.0 + t * t),
      }
   }
}

impl Default for ZebraixAngle {
   fn default() -> Self {
      Self::Quadrant(1.0)
   }
}
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum RatQuadState {
   #[default]
   RationalPoly,
   SymmetricRange,       // RationalPoly[nomial] with symmetric range.
   RegularizedSymmetric, // SymmetricRange with zero middle denominator coefficient.
   OffsetOddEven,        // O-O-E weightings of RegularizedSymmetric.

   FourPoint,        // Like cubic.
   ThreePointAngle,  // Form a,b,angle, sigma.
   RationalWeighted, // Polynomial-like, by  difference from end points.
}

#[derive(Debug, Serialize, PartialEq, Copy, Clone)]
pub struct FourPointRatQuad {
   pub state: RatQuadState,
   pub r: [f64; 2], // Range.
   pub x: [f64; 4],
   pub y: [f64; 4],
}

impl Default for FourPointRatQuad {
   fn default() -> Self {
      Self {
         state: RatQuadState::FourPoint,
         r: [0.0, 0.0],
         x: [0.0, 0.0, 0.0, 0.0],
         y: [0.0, 0.0, 0.0, 0.0],
      }
   }
}

// #[derive(Debug, Serialize, PartialEq, Copy, Clone)]
// pub struct ThreePointAngleRatQuad {
//    pub state: RatQuadState,
//    pub r: [f64; 2], // Range.
//    pub x: [f64; 3],
//    pub y: [f64; 3],
//    pub angle: ZebraixAngle,
//    pub sigma: f64,
// }

// impl Default for ThreePointAngleRatQuad {
//    fn default() -> Self {
//       Self {
//          state: RatQuadState::ThreePointAngle,
//          r: [0.0, 0.0],
//          x: [0.0, 0.0, 0.0],
//          y: [0.0, 0.0, 0.0],
//          angle: ZebraixAngle::Quadrant(1.0),
//          sigma: 1.0,
//       }
//    }
// }

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum RatQuadOoeSubtype {
   #[default]
   Elliptical,
   Parabolic,
   Hyperbolic,
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct BaseRatQuad {
   pub state: RatQuadState,
   pub r: [f64; 2], // Range.
   pub a: [f64; 3], // Denominator, as a[2] * t^2 + a[1] * t... .
   pub b: [f64; 3], // Numerator or O-O-E coefficients for x component.
   pub c: [f64; 3], // Numerator or O-O-E coefficients for y component.
   #[serde(skip_serializing_if = "is_default")]
   pub angle: ZebraixAngle,
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
   pub ooe_subtype: RatQuadOoeSubtype,
}

#[derive(Serialize, Debug, Default, Copy, Clone, PartialEq)]
pub enum SpecifiedRatQuad {
   #[default]
   None, // For, say, polynomial directly specified.
   // Base(BaseRatQuad), // Three-points and angle, for example.
   FourPoint(FourPointRatQuad),
   ThreePointAngle(BaseRatQuad),
   // ThreePointAngle(ThreePointAngleRatQuad),
}

impl BaseRatQuad {
   // pub fn new(r: [f64; 2], a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Self {
   //    Self { r, a, b, c, form: RatQuadState::RationalPoly }
   // }

   #[must_use]
   pub fn eval_quad(&self, t: &[f64]) -> Vec<[f64; 2]> {
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
      for item in t {
         let denom_reciprocal = 1.0 / self.a[2].mul_add(*item, self.a[1]).mul_add(*item, self.a[0]);
         ret_val.push([
            self.b[2].mul_add(*item, self.b[1]).mul_add(*item, self.b[0]) * denom_reciprocal,
            self.c[2].mul_add(*item, self.c[1]).mul_add(*item, self.c[0]) * denom_reciprocal,
         ]);
      }
      ret_val
   }

   #[inline]
   // Applies bilinear substitution of the form (wt + x) / (yt + z) with normalization.
   //
   // This function should be applied by a knowledgeable caller, that is one that handles the
   // state of the RatQuad.
   #[allow(clippy::suboptimal_flops)]
   fn apply_bilinear_unranged(&mut self, mut w: f64, mut x: f64, mut y: f64, mut z: f64) {
      #[allow(clippy::suboptimal_flops)]
      let norm = 1.0 / ((w * w + x * x) * (y * y + z * z)).sqrt().sqrt();
      w *= norm;
      x *= norm;
      y *= norm;
      z *= norm;
      self.a = [
         self.a[0] * z * z + self.a[1] * x * z + self.a[2] * x * x,
         2.0 * self.a[0] * y * z + self.a[1] * (x * y + w * z) + 2.0 * self.a[2] * w * x,
         self.a[0] * y * y + self.a[1] * w * y + self.a[2] * w * w,
      ];
      self.b = [
         self.b[0] * z * z + self.b[1] * x * z + self.b[2] * x * x,
         2.0 * self.b[0] * y * z + self.b[1] * (x * y + w * z) + 2.0 * self.b[2] * w * x,
         self.b[0] * y * y + self.b[1] * w * y + self.b[2] * w * w,
      ];
      self.c = [
         self.c[0] * z * z + self.c[1] * x * z + self.c[2] * x * x,
         2.0 * self.c[0] * y * z + self.c[1] * (x * y + w * z) + 2.0 * self.c[2] * w * x,
         self.c[0] * y * y + self.c[1] * w * y + self.c[2] * w * w,
      ];
   }
   #[inline]
   // Applies bilinear transformation with factor sigma, preserving the range.
   #[allow(clippy::suboptimal_flops)]
   fn apply_bilinear(&mut self, sigma: f64) -> Result<(), &'static str> {
      match self.state {
         RatQuadState::OffsetOddEven => {
            Err("Unable to convert offset-even-odd form to symmetric-range form.")
         }
         RatQuadState::RegularizedSymmetric => {
            Err("Applying bilinear to regularized will downgrade it.")
         }
         RatQuadState::SymmetricRange => {
            let r = self.r[1];
            self.apply_bilinear_unranged(
               (sigma + 1.0) * r,
               (sigma - 1.0) * r * r,
               sigma - 1.0,
               (sigma + 1.0) * r,
            );
            Ok(())
         }
         RatQuadState::RationalPoly => {
            let p = self.r[0];
            let q = self.r[1];
            self.apply_bilinear_unranged(
               sigma * q - p,
               -(sigma - 1.0) * p * q,
               sigma - 1.0,
               q - sigma * p,
            );
            Ok(())
         }
         RatQuadState::FourPoint => {
            Err("Bilinear is applicable to four-point form, but not implemented.")
         }
         RatQuadState::ThreePointAngle => {
            Err("Bilinear is applicable to three-point-angle form, but not implemented.")
         }
         RatQuadState::RationalWeighted => {
            Err("Bilinear is applicable to rational-weighted form, but not implemented.")
         }
      }
   }

   #[allow(clippy::suboptimal_flops)]
   fn raise_to_symmetric_range(&mut self) -> Result<(), &'static str> {
      if self.state == RatQuadState::OffsetOddEven {
         return Err("Unable to convert offset-even-odd form to symmetric-range form.");
      }
      // Replace t with t - d.
      let d = 0.5 * (self.r[0] + self.r[1]);
      let r = 0.5 * (self.r[1] - self.r[0]);

      self.a =
         [d * (d * self.a[2] + self.a[1]) + self.a[0], 2.0 * d * self.a[2] + self.a[1], self.a[2]];
      self.b =
         [d * (d * self.b[2] + self.b[1]) + self.b[0], 2.0 * d * self.b[2] + self.b[1], self.b[2]];
      self.c =
         [d * (d * self.c[2] + self.c[1]) + self.c[0], 2.0 * d * self.c[2] + self.c[1], self.c[2]];

      self.r = [-r, r];
      self.state = RatQuadState::SymmetricRange;
      Ok(())
   }

   #[allow(clippy::suboptimal_flops)]
   fn raise_to_regularized_symmetric(&mut self) -> Result<(), &'static str> {
      if self.state != RatQuadState::SymmetricRange {
         return Err("Can only raise from symmetric-range to regularized-symmetric form.");
      }
      let r = self.r[1];
      let a_s = self.a[2] * r * r + self.a[0];
      // let a_d = self.a[2] * r * r - self.a[0];
      let combo_s = a_s + self.a[1] * r;
      let combo_d = a_s - self.a[1] * r;

      let sigma = combo_d.abs().sqrt() / combo_s.abs().sqrt();

      self.apply_bilinear(sigma)?;

      assert!(self.a[1].abs() < 0.001);
      self.state = RatQuadState::RegularizedSymmetric;

      Ok(())
   }

   #[inline]
   fn characterize_endpoints(&self) -> ([f64; 4], [f64; 4]) {
      let mut x = [0.0; 4];
      let mut y = [0.0; 4];
      let speed_scale = self.r[1] - self.r[0];
      for (outer, inner, t) in [(0, 1, self.r[0]), (3, 2, self.r[1])] {
         let recip_a = 1.0 / self.a[2].mul_add(t, self.a[1]).mul_add(t, self.a[0]);
         let b = self.b[2].mul_add(t, self.b[1]).mul_add(t, self.b[0]);
         let c = self.c[2].mul_add(t, self.c[1]).mul_add(t, self.c[0]);
         let da = self.a[2].mul_add(2.0 * t, self.a[1]) * speed_scale;
         let db = self.b[2].mul_add(2.0 * t, self.b[1]) * speed_scale;
         let dc = self.c[2].mul_add(2.0 * t, self.c[1]) * speed_scale;
         x[outer] = b * recip_a;
         y[outer] = c * recip_a;
         x[inner] = (-b * da).mul_add(recip_a, db) * recip_a;
         y[inner] = (-c * da).mul_add(recip_a, dc) * recip_a;
      }
      (x, y)
   }

   #[allow(clippy::suboptimal_flops)]
   fn raise_to_offset_odd_even(&mut self, poly: &Self, tolerance: f64) -> Result<(), &'static str> {
      if poly.state != RatQuadState::RegularizedSymmetric {
         return Err("Can only raise from regularized-symmetric form to offset-odd-even form.");
      }
      *self = *poly;

      let r = self.r[1];
      // assert_eq!(r, 10000.0);
      if (self.a[2].abs() * r * r) < (self.a[0].abs() * tolerance) {
         self.ooe_subtype = RatQuadOoeSubtype::Parabolic;
      } else if self.a[2].signum() == self.a[0].signum() {
         self.ooe_subtype = RatQuadOoeSubtype::Elliptical;

         let s = 1.0 / self.a[0];
         let f = 1.0 / self.a[2];
         self.a[0] = 1.0;
         self.a[2] *= s;

         {
            let offset = 0.5 * (s * self.b[0] + f * self.b[2]);
            let even = 0.5 * (s * self.b[0] - f * self.b[2]);
            let odd = self.b[1] * s;
            self.b = [offset, odd, even];
         }
         {
            let offset = 0.5 * (s * self.c[0] + f * self.c[2]);
            let even = 0.5 * (s * self.c[0] - f * self.c[2]);
            let odd = self.c[1] * s;
            self.c = [offset, odd, even];
         }

         let sss = 1.0 / self.a[2].sqrt();
         let (sx, sy) = (0.5 * sss * self.b[1], 0.5 * sss * self.c[1]);
         let (cx, cy) = (self.b[2], self.c[2]);
         let determinant = sx * cy - cx * sy;
         let frobenius_squared = sx * sx + sy * sy + cx * cx + cy * cy;
         if determinant.abs() < (frobenius_squared * tolerance) {
            // From the plotting point of view this is not a degenerate case, but renderers may
            // want the transformation to be invertible.
            //
            // If one singular value is much larger than the other, the frobenius norm
            // (squared) will be approximately the square of larger.  The determinant is their
            // product, and so the condition effectively compares their magnitude (for small
            // tolerances).
            *self = *poly;
            self.ooe_subtype = RatQuadOoeSubtype::Parabolic;
         }
      } else {
         self.ooe_subtype = RatQuadOoeSubtype::Hyperbolic;
      }

      self.state = RatQuadState::OffsetOddEven;

      Ok(())
   }

   #[allow(clippy::suboptimal_flops)]
   fn weighted_to_polynomial(&mut self) -> Result<(), &'static str> {
      if self.state != RatQuadState::RationalWeighted {
         return Err("Attempted conversion from rational-weighted when not in that state.");
      }
      // Get from self.sigma once confirmed working.
      let sigma = 1.0;
      let v = self.r[0];
      let w = self.r[1];
      // assert_eq!(w, 100.0);
      {
         let h0 = self.a[0];
         let h1 = sigma * self.a[1];
         let h2 = sigma * sigma * self.a[2];
         self.a = [
            w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
            2.0 * (-w * h0 + (w + v) * h1 - v * h2),
            h0 - 2.0 * h1 + h2,
         ];
      }
      {
         let h0 = self.b[0];
         let h1 = sigma * self.b[1];
         let h2 = sigma * sigma * self.b[2];
         self.b = [
            w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
            2.0 * (-w * h0 + (w + v) * h1 - v * h2),
            h0 - 2.0 * h1 + h2,
         ];
      }
      {
         let h0 = self.c[0];
         let h1 = sigma * self.c[1];
         let h2 = sigma * sigma * self.c[2];
         self.c = [
            w * w * h0 - 2.0 * v * w * h1 + v * v * h2,
            2.0 * (-w * h0 + (w + v) * h1 - v * h2),
            h0 - 2.0 * h1 + h2,
         ];
      }

      self.state = RatQuadState::RationalPoly;
      Ok(())
   }
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq)]
pub struct ManagedRatQuad {
   ooe: BaseRatQuad,
   poly: BaseRatQuad,
   specified: SpecifiedRatQuad, // FourPoint or ThreePointAngle.
   canvas_range: [f64; 4],
}

#[allow(clippy::missing_panics_doc)]
impl ManagedRatQuad {
   #[must_use]
   pub fn create_from_polynomial(poly: &BaseRatQuad, canvas_range: [f64; 4]) -> Self {
      assert!(poly.state == RatQuadState::RationalPoly);
      Self { poly: *poly, ooe: BaseRatQuad::default(), canvas_range, ..Default::default() }
   }

   #[must_use]
   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::similar_names)]
   #[allow(clippy::suboptimal_flops)]
   pub fn create_from_four_points(four_points: &FourPointRatQuad, canvas_range: [f64; 4]) -> Self {
      assert!(four_points.state == RatQuadState::FourPoint);
      let x = &four_points.x;
      let y = &four_points.y;
      let delta_x = (x[2] - x[3]) * (y[1] - y[0]);
      let delta_y = (y[2] - y[3]) * (x[1] - x[0]);
      let w_b = delta_x - delta_y;
      let w_b_x_m = (y[3] - y[0]) * (x[2] - x[3]) * (x[1] - x[0]) - x[3] * delta_y + x[0] * delta_x;
      // If we exchange all x and y then we also negate, by implication, w_b.
      let w_b_y_m =
         -1.0 * ((x[3] - x[0]) * (y[2] - y[3]) * (y[1] - y[0]) - y[3] * delta_x + y[0] * delta_y);
      let w_a = 2.0 / 3.0 * (x[0] * (y[2] - y[3]) + x[2] * (y[3] - y[0]) + x[3] * (y[0] - y[2]));
      let w_c = -2.0 / 3.0 * (y[0] * (x[2] - x[3]) + y[2] * (x[3] - x[0]) + y[3] * (x[0] - x[2]));

      let b = [w_a * x[0], w_b_x_m, w_c * x[3]];
      let c = [w_a * y[0], w_b_y_m, w_c * y[3]];
      let a = [w_a, w_b, w_c];
      let mut rat_quad = BaseRatQuad {
         state: RatQuadState::RationalWeighted,
         r: four_points.r,
         a,
         b,
         c,
         ..Default::default()
      };
      rat_quad.weighted_to_polynomial().unwrap();
      Self {
         poly: rat_quad,
         ooe: BaseRatQuad::default(),
         specified: SpecifiedRatQuad::FourPoint(*four_points),
         canvas_range,
         // ..Default::default()
      }
   }
   #[must_use]
   pub fn create_from_three_points(
      three_point_rat_quad: &BaseRatQuad,
      canvas_range: [f64; 4],
   ) -> Self {
      assert!(three_point_rat_quad.state == RatQuadState::ThreePointAngle);
      let xs = &three_point_rat_quad.b;
      let ys = &three_point_rat_quad.c;
      let f_mult_1p5 = three_point_rat_quad.angle.cos();
      // Can construct as four-point rat quad with these values.
      // let x = [xs[0], f * xs[1] + (1.0 - f) * xs[0], f * xs[1] + (1.0 - f) * xs[2], xs[2]];
      // let y = [ys[0], f * ys[1] + (1.0 - f) * ys[0], f * ys[1] + (1.0 - f) * ys[2], ys[2]];

      let b = [xs[0], f_mult_1p5 * xs[1], xs[2]];
      let c = [ys[0], f_mult_1p5 * ys[1], ys[2]];
      let a = [1.0, f_mult_1p5, 1.0];
      let mut rat_quad = BaseRatQuad {
         state: RatQuadState::RationalWeighted,
         r: three_point_rat_quad.r,
         a,
         b,
         c,
         ..Default::default()
      };
      rat_quad.weighted_to_polynomial().unwrap();
      Self {
         poly: rat_quad,
         ooe: BaseRatQuad::default(),
         specified: SpecifiedRatQuad::ThreePointAngle(*three_point_rat_quad),
         canvas_range,
         // ..Default::default()
      }
   }

   #[must_use]
   pub const fn get_ooe_rat_quad(&self) -> &BaseRatQuad {
      &self.ooe
   }
   #[must_use]
   pub const fn get_poly_rat_quad(&self) -> &BaseRatQuad {
      &self.poly
   }

   #[allow(clippy::missing_errors_doc)]
   // Velocity at beginning multiplied by sigma, and velocity at end divided by sigma.
   pub fn apply_bilinear(&mut self, sigma: f64) -> Result<(), &'static str> {
      self.poly.apply_bilinear(sigma)
   }

   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_symmetric_range(&mut self) -> Result<(), &'static str> {
      self.poly.raise_to_symmetric_range()
   }
   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_regularized_symmetric(&mut self) -> Result<(), &'static str> {
      self.poly.raise_to_regularized_symmetric()
   }
   #[allow(clippy::missing_errors_doc)]
   pub fn raise_to_offset_odd_even(&mut self) -> Result<(), &'static str> {
      self.ooe.raise_to_offset_odd_even(&self.poly, 0.01)
   }
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::suboptimal_flops)]
pub fn draw_sample_rat_quad(
   managed_rat_quad: &ManagedRatQuad,
   spartan: &mut SpartanDiagram,
   curve_config: &SampleCurveConfig,
) {
   let rat_quad: &BaseRatQuad = managed_rat_quad.get_poly_rat_quad();

   if let Some(color_choice) = curve_config.control_color {
      // assert!(false);

      let end_points_vec;
      let control_points_vec;
      match managed_rat_quad.specified {
         SpecifiedRatQuad::None => {
            panic!("Unable to draw control points when RQC not specified via control points.");
         }
         // SpecifiedRatQuad:: Base(BaseRatQuad), // Three-points and angle, for example.
         SpecifiedRatQuad::FourPoint(specified) => {
            end_points_vec =
               vec![[specified.x[0], specified.y[0]], [specified.x[3], specified.y[3]]];
            control_points_vec =
               vec![[specified.x[1], specified.y[1]], [specified.x[2], specified.y[2]]];
         }
         SpecifiedRatQuad::ThreePointAngle(specified) => {
            end_points_vec =
               vec![[specified.b[0], specified.c[0]], [specified.b[2], specified.c[2]]];
            control_points_vec = vec![[specified.b[1], specified.c[1]]];
         }
      }

      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.control_point_choices[0],
            color_choice,
            centers: end_points_vec.clone(),
         }),
      });
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.control_point_choices[1],
            color_choice,
            centers: control_points_vec.clone(),
         }),
      });

      let expanded_control_points_vec = if control_points_vec.len() == 2 {
         control_points_vec
      } else {
         vec![control_points_vec[0], control_points_vec[0]]
      };

      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Lines(LinesDrawable {
            line_choice: curve_config.control_line_choice,
            color_choice,
            start: end_points_vec,
            end: expanded_control_points_vec,
            ..Default::default()
         }),
      });
   }

   if let Some(color_choice) = curve_config.points_color {
      // Do not include end points if control points are doing that already.
      let t_int: Vec<i32> = if curve_config.control_color.is_some() {
         (1..curve_config.points_num_segments).collect()
      } else {
         (0..=curve_config.points_num_segments).collect()
      };
      let mut t = Vec::<f64>::with_capacity(t_int.len());
      let scale = (rat_quad.r[1] - rat_quad.r[0]) / f64::from(curve_config.points_num_segments);
      let offset = rat_quad.r[0];
      for item in &t_int {
         t.push(f64::from(*item).mul_add(scale, offset));
      }

      let mut pattern_vec = rat_quad.eval_quad(&t);

      if curve_config.sample_options == SampleOption::XVsT {
         for i in 0..t_int.len() {
            pattern_vec[i] = [t[i], pattern_vec[i][0]];
         }
      }

      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.points_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.points_choice,
            color_choice,
            centers: pattern_vec,
         }),
      });
   }

   if let Some(color_choice) = curve_config.main_color {
      if curve_config.approx_num_segments != 0 {
         let t_int: Vec<i32> = (0..=curve_config.approx_num_segments).collect();
         let mut t = Vec::<f64>::with_capacity(t_int.len());
         let scale = (rat_quad.r[1] - rat_quad.r[0]) / f64::from(curve_config.approx_num_segments);
         let offset = rat_quad.r[0];
         for item in &t_int {
            t.push(f64::from(*item).mul_add(scale, offset));
         }

         let mut pattern_vec = rat_quad.eval_quad(&t);

         if curve_config.sample_options == SampleOption::XVsT {
            for i in 0..t_int.len() {
               pattern_vec[i] = [t[i], pattern_vec[i][0]];
            }
         }

         spartan.drawables.push(QualifiedDrawable {
            layer: curve_config.main_line_layer,
            drawable: OneOfDrawable::Polyline(PolylineDrawable {
               color_choice,
               line_choice: curve_config.main_line_choice,
               locations: pattern_vec,
               ..Default::default()
            }),
         });
      } else {
         let ooe_rat_quad: &BaseRatQuad = managed_rat_quad.get_ooe_rat_quad();
         assert_eq!(ooe_rat_quad.state, RatQuadState::OffsetOddEven);

         match ooe_rat_quad.ooe_subtype {
            RatQuadOoeSubtype::Elliptical => {
               let r = ooe_rat_quad.r[1];
               let s = 1.0 / ooe_rat_quad.a[2].sqrt();
               let mx = ooe_rat_quad.b[0];
               let my = ooe_rat_quad.c[0];
               let (sx, sy) = (0.5 * s * ooe_rat_quad.b[1], 0.5 * s * ooe_rat_quad.c[1]);
               let (cx, cy) = (ooe_rat_quad.b[2], ooe_rat_quad.c[2]);

               // The arc range is [-angle_range, angle_range].
               let angle_range = 2.0 * (r * (ooe_rat_quad.a[2] / ooe_rat_quad.a[0]).sqrt()).atan();

               spartan.drawables.push(QualifiedDrawable {
                  layer: curve_config.main_line_layer,
                  drawable: OneOfDrawable::Arc(ArcDrawable {
                     color_choice,
                     line_choice: curve_config.main_line_choice,
                     angle_range: [-angle_range, angle_range],
                     center: [mx, my],
                     transform: [cx, cy, sx, sy],
                     ..Default::default()
                  }),
               });
            }

            RatQuadOoeSubtype::Parabolic => {
               let (x, y) = rat_quad.characterize_endpoints();
               let f = 1.0 / 3.0;
               let four_x = [x[0], x[0] + f * x[1], x[3] - f * x[2], x[3]];
               let four_y = [y[0], y[0] + f * y[1], y[3] - f * y[2], y[3]];

               if let Some(color_choice) = curve_config.main_color {
                  spartan.drawables.push(QualifiedDrawable {
                     layer: curve_config.main_line_layer,
                     drawable: OneOfDrawable::Cubic(CubicDrawable {
                        color_choice,
                        line_choice: curve_config.main_line_choice,
                        x: four_x,
                        y: four_y,
                        ..Default::default()
                     }),
                  });
               }
            }
            RatQuadOoeSubtype::Hyperbolic => {
               let t_int: Vec<i32> = (0..spartan.num_segments_hyperbolic).collect();
               let mut t = Vec::<f64>::with_capacity(t_int.len());
               let scale =
                  (rat_quad.r[1] - rat_quad.r[0]) / f64::from(spartan.num_segments_hyperbolic);
               let offset = rat_quad.r[0];
               for item in &t_int {
                  t.push(f64::from(*item).mul_add(scale, offset));
               }

               let mut pattern_vec = rat_quad.eval_quad(&t);

               if curve_config.sample_options == SampleOption::XVsT {
                  for i in 0..t_int.len() {
                     pattern_vec[i] = [t[i], pattern_vec[i][0]];
                  }
               }

               spartan.drawables.push(QualifiedDrawable {
                  layer: curve_config.main_line_layer,
                  drawable: OneOfDrawable::Polyline(PolylineDrawable {
                     color_choice,
                     line_choice: curve_config.main_line_choice,
                     locations: pattern_vec,
                     ..Default::default()
                  }),
               });
            }
         }
      }
   }
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum CubicForm {
   #[default]
   FourPoint,
   MidDiff,
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq, Copy, Clone)]
pub struct CubiLinear {
   pub form: CubicForm,
   pub r: [f64; 2], // Range.
   pub x: [f64; 4],
   pub y: [f64; 4],
   #[serde(skip_serializing_if = "is_default_unit_f64", default = "default_unit_f64")]
   pub sigma: f64,
}

#[allow(clippy::missing_errors_doc)]
impl CubiLinear {
   #[inline]
   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::suboptimal_flops)]
   fn eval_part(b: f64, a: f64, coeffs: &[f64; 4], multiplier: f64) -> f64 {
      multiplier
         * (b * b * b * coeffs[0]
            + 3.0 * b * b * a * coeffs[1]
            + 3.0 * b * a * a * coeffs[2]
            + a * a * a * coeffs[3])
   }

   #[allow(clippy::many_single_char_names)]
   #[allow(clippy::suboptimal_flops)]
   pub fn eval(&self, t: &[f64]) -> Result<Vec<[f64; 2]>, &'static str> {
      if self.form != CubicForm::FourPoint {
         return Err("Can only evaluate cubilinear curves in four-point form.");
      }
      let mut ret_val = Vec::<[f64; 2]>::with_capacity(t.len());
      for item in t {
         let a = self.sigma * (*item - self.r[0]);
         let b = self.r[1] - *item;
         let f0 = 1.0 / (b + a);
         let recip_denom = f0 * f0 * f0;
         let x = Self::eval_part(b, a, &self.x, recip_denom);
         let y = Self::eval_part(b, a, &self.y, recip_denom);
         ret_val.push([x, y]);
      }
      Ok(ret_val)
   }

   #[allow(clippy::similar_names)]
   #[allow(clippy::suboptimal_flops)]
   fn select_range(&mut self, new_range: [f64; 2]) {
      let mut new_x = [0.0; 4];
      let mut new_y = [0.0; 4];

      let a_k = self.sigma * (new_range[0] - self.r[0]);
      let b_k = self.r[1] - new_range[0];
      let a_l = self.sigma * (new_range[1] - self.r[0]);
      let b_l = self.r[1] - new_range[1];
      let f0_k = 1.0 / (b_k + a_k);
      let recip_denom_k = f0_k * f0_k * f0_k;
      let f0_l = 1.0 / (b_l + a_l);
      let recip_denom_l = f0_l * f0_l * f0_l;
      new_x[0] = Self::eval_part(b_k, a_k, &self.x, recip_denom_k);
      new_y[0] = Self::eval_part(b_k, a_k, &self.y, recip_denom_k);
      new_x[3] = Self::eval_part(b_l, a_l, &self.x, recip_denom_l);
      new_y[3] = Self::eval_part(b_l, a_l, &self.y, recip_denom_l);
      let kl_numerator_k = self.sigma * self.r[1] * (new_range[0] - self.r[0])
         + self.r[0] * (self.r[1] - new_range[0]);
      let kl_numerator_l = self.sigma * self.r[1] * (new_range[1] - self.r[0])
         + self.r[0] * (self.r[1] - new_range[1]);
      // This is [k, l] bilinearly transformed.
      let selected_range_bilineared = kl_numerator_l / (a_l + b_l) - kl_numerator_k / (a_k + b_k);
      let fudge_k = selected_range_bilineared / (self.r[1] - self.r[0]);
      let fudge_l = selected_range_bilineared / (self.r[1] - self.r[0]);
      // assert_eq!(1.0 / f0_k, 0.0);
      let dx_1 = fudge_k
         * f0_k
         * f0_k
         * (b_k * b_k * (self.x[1] - self.x[0])
            + 2.0 * b_k * a_k * (self.x[2] - self.x[1])
            + a_k * a_k * (self.x[3] - self.x[2]));
      new_x[1] = new_x[0] + dx_1;
      let dy_1 = fudge_k
         * f0_k
         * f0_k
         * (b_k * b_k * (self.y[1] - self.y[0])
            + 2.0 * b_k * a_k * (self.y[2] - self.y[1])
            + a_k * a_k * (self.y[3] - self.y[2]));
      new_y[1] = new_y[0] + dy_1;
      let dx_1 = fudge_l
         * f0_l
         * f0_l
         * (b_l * b_l * (self.x[1] - self.x[0])
            + 2.0 * b_l * a_l * (self.x[2] - self.x[1])
            + a_l * a_l * (self.x[3] - self.x[2]));
      new_x[2] = new_x[3] - dx_1;
      let dy_1 = fudge_l
         * f0_l
         * f0_l
         * (b_l * b_l * (self.y[1] - self.y[0])
            + 2.0 * b_l * a_l * (self.y[2] - self.y[1])
            + a_l * a_l * (self.y[3] - self.y[2]));
      new_y[2] = new_y[3] - dy_1;

      self.sigma = (a_l + b_l) / (a_k + b_k);
      self.x = new_x;
      self.y = new_y;
      self.r = new_range;
   }

   fn displace(&mut self, d: [f64; 2]) {
      self.x[0] += d[0];
      self.x[1] += d[0];
      self.x[2] += d[0];
      self.x[3] += d[0];
      self.y[0] += d[1];
      self.y[1] += d[1];
      self.y[2] += d[1];
      self.y[3] += d[1];
   }

   fn bilinear_transform(&mut self, sigma: f64) {
      self.sigma *= sigma;
   }

   fn adjust_range(&mut self, new_range: [f64; 2]) {
      self.r = new_range;
   }
}

#[derive(Debug, Serialize, DefaultFromSerde, PartialEq, Clone)]
pub struct ManagedCubic {
   four_point: CubiLinear,
   canvas_range: [f64; 4],
}

#[allow(clippy::missing_panics_doc)]
impl ManagedCubic {
   #[must_use]
   pub const fn create_from_control_points(
      control_points: &CubiLinear,
      canvas_range: [f64; 4],
   ) -> Self {
      let mut ret_val = Self { four_point: *control_points, canvas_range };
      ret_val.four_point.form = CubicForm::FourPoint;
      ret_val
   }

   #[must_use]
   pub const fn get_form(&self) -> CubicForm {
      self.four_point.form
   }

   #[must_use]
   pub const fn get_four_point(&self) -> &CubiLinear {
      &self.four_point
   }

   pub fn displace(&mut self, d: [f64; 2]) {
      self.four_point.displace(d);
   }

   pub fn bilinear_transform(&mut self, sigma: f64) {
      self.four_point.bilinear_transform(sigma);
   }

   pub fn adjust_range(&mut self, new_range: [f64; 2]) {
      self.four_point.adjust_range(new_range);
   }

   pub fn select_range(&mut self, new_range: [f64; 2]) {
      self.four_point.select_range(new_range);
   }
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum SampleOption {
   #[default]
   Normal,
   XVsT,
}

// In each drawn feature (the main line, points, control) the some-ness of the first field
// toggles drawing of the feature.
pub struct SampleCurveConfig {
   pub main_color: Option<ColorChoice>,
   pub main_line_choice: LineChoice,
   pub approx_num_segments: i32,

   pub points_color: Option<ColorChoice>,
   pub points_choice: PointChoice,
   pub points_num_segments: i32,

   pub sample_options: SampleOption,

   pub control_color: Option<ColorChoice>,
   pub control_point_choices: [PointChoice; 2],
   pub control_line_choice: LineChoice,

   pub control_layer: i32,
   pub points_layer: i32,
   pub main_line_layer: i32,
}

impl Default for SampleCurveConfig {
   fn default() -> Self {
      Self {
         main_color: Some(ColorChoice::Blue),
         main_line_choice: LineChoice::Ordinary,
         approx_num_segments: 0,
         points_color: Some(ColorChoice::Green),
         points_choice: PointChoice::Dot,
         points_num_segments: 12,
         sample_options: SampleOption::Normal,
         control_color: None,
         control_point_choices: [PointChoice::Circle, PointChoice::Times],
         control_line_choice: LineChoice::Light,

         control_layer: 10,
         points_layer: 20,
         main_line_layer: 30,
      }
   }
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::suboptimal_flops)]
pub fn draw_sample_cubilinear(
   managed_cubic: &ManagedCubic,
   spartan: &mut SpartanDiagram,
   curve_config: &SampleCurveConfig,
) {
   let four_point = &managed_cubic.four_point;

   if let Some(color_choice) = curve_config.control_color {
      let end_points_vec =
         vec![[four_point.x[0], four_point.y[0]], [four_point.x[3], four_point.y[3]]];
      let control_points_vec =
         vec![[four_point.x[1], four_point.y[1]], [four_point.x[2], four_point.y[2]]];

      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.control_point_choices[0],
            color_choice,
            centers: end_points_vec.clone(),
         }),
      });
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.control_point_choices[1],
            color_choice,
            centers: control_points_vec.clone(),
         }),
      });

      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Lines(LinesDrawable {
            line_choice: curve_config.control_line_choice,
            color_choice,
            start: end_points_vec,
            end: control_points_vec,
            ..Default::default()
         }),
      });
   }

   if let Some(color_choice) = curve_config.points_color {
      // Do not include end points if control points are doing that already.
      let t_int: Vec<i32> = if curve_config.control_color.is_some() {
         (1..curve_config.points_num_segments).collect()
      } else {
         (0..=curve_config.points_num_segments).collect()
      };
      let mut t = Vec::<f64>::with_capacity(t_int.len());
      let scale = (four_point.r[1] - four_point.r[0]) / f64::from(curve_config.points_num_segments);
      let offset = four_point.r[0];
      for item in &t_int {
         t.push(f64::from(*item).mul_add(scale, offset));
      }

      let pattern_vec = four_point.eval(&t).unwrap();

      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.points_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.points_choice,
            color_choice,
            centers: pattern_vec,
         }),
      });
   }

   if let Some(color_choice) = curve_config.main_color {
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.main_line_layer,
         drawable: OneOfDrawable::Cubic(CubicDrawable {
            color_choice,
            line_choice: curve_config.main_line_choice,
            x: four_point.x,
            y: four_point.y,
            ..Default::default()
         }),
      });
   }
}
