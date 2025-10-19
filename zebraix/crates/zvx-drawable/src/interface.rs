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

use crate::{QualifiedDrawable, TextSingle};
use std::error::Error;

pub struct TextMetrics {
   pub strikethrough_center: f64,
   pub even_half_height: f64,
   pub font_ascent: f64,
   pub font_descent: f64,
   pub font_height: f64,
   // Fields above are generally independent of text content.
   pub text_width: f64,
   pub text_height: f64,
}

// Note on special functions.
//
// Rust is (as of rustc 1.85.1) unable to convert a boxed heap object to (a reference to) its
// concrete implementation type when any kind of non-static lifetime is involved.  As a result
// an implementation such as Cairo+Pango has no means to call functions with its own data.  In
// order to work around this, the `render_layout` method was created for the text layout trait,
// even though this really is the business of the implementation.  In order to future-proof the
// interface, extra placeholder special functions were added.
//
// Refs: https://users.rust-lang.org/t/borrowing-as-any-non-static/131565,
// https://crates.io/crates/better_any

pub trait ZvxTextLayout {
   fn set_layout(&mut self, font_family: &str, font_size: f64, single_text: &TextSingle);
   // fn set_markup_with_accel(&self, markup: &str, accel_marker: char) -> char;
   fn get_metrics(&mut self) -> &Option<TextMetrics>;
   #[allow(clippy::missing_errors_doc)]
   fn render_layout(&mut self) -> Result<(), Box<dyn Error>>;

   // See note above about special functions.
   #[allow(clippy::missing_errors_doc)]
   fn special_function_0(&mut self) -> Result<(), Box<dyn Error>>;
   #[allow(clippy::missing_errors_doc)]
   fn special_function_1(&mut self) -> Result<(), Box<dyn Error>>;
   #[allow(clippy::missing_errors_doc)]
   fn special_function_2(&mut self) -> Result<(), Box<dyn Error>>;
   #[allow(clippy::missing_errors_doc)]
   fn special_function_3(&mut self) -> Result<(), Box<dyn Error>>;
}

// Move to an interface location, but note dependence on QualifiedDrawable.
pub trait ZvxRenderEngine {
   #[must_use]
   fn create_text_layout<'parent, 'a>(&'parent self) -> Box<dyn ZvxTextLayout + 'a>
   where
      'parent: 'a;

   #[allow(clippy::missing_errors_doc)]
   fn render_drawables(
      &mut self,
      drawables: &[QualifiedDrawable],
   ) -> Result<Box<dyn core::any::Any>, Box<dyn Error>>;

   // See note above about special functions.
   #[allow(clippy::missing_errors_doc)]
   fn special_function_0(&mut self) -> Result<(), Box<dyn Error>>;
   #[allow(clippy::missing_errors_doc)]
   fn special_function_1(&mut self) -> Result<(), Box<dyn Error>>;
   #[allow(clippy::missing_errors_doc)]
   fn special_function_2(&mut self) -> Result<(), Box<dyn Error>>;
   #[allow(clippy::missing_errors_doc)]
   fn special_function_3(&mut self) -> Result<(), Box<dyn Error>>;
}
