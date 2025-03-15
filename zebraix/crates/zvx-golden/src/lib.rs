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
//! use zvx_golden::filtered::JsonGoldenTest;
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
//! use zvx_golden::filtered::SvgGoldenTest;
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

pub mod axes;
pub mod diagram;
pub mod filtered;
pub mod render;

use std::io;
use std::path::Path;

// The string messaging is a task in the form "opening file".
fn check_panic_with_path<T>(result: Result<T, io::Error>, messaging: &str, path: &Path) -> T {
   match result {
      Ok(result) => result,
      Err(error) => panic!("Error while {messaging} for file path {path:?}: {error:?}"),
   }
}

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
