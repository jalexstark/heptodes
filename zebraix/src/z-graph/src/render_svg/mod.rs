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

use cairo::SvgUnit::Pt;
// use cairo::enums::SvgUnit;
use cairo::SvgSurface;
use std::f64::consts::PI;
use std::io::Write;

// After dependency to test_utils is added, use type-def for the result box.
pub fn write_sample_to_write<W: Write + 'static>(
   out_stream: W,
) -> Result<Box<dyn core::any::Any>, cairo::StreamWithError> {
   let canvas_width = 320.0; //  overall.canvas_width,
   let canvas_height = 240.0; // overall.canvas_height
   let x_offset = 40.0; //       overall.canvas_x_offset
   let y_offset = 50.0; //       overall.canvas_y_offset

   let mut surface = SvgSurface::for_stream(canvas_width, canvas_height, out_stream).unwrap();
   surface.set_document_unit(Pt);
   let context = cairo::Context::new(&surface).unwrap();
   context.translate(x_offset, y_offset);

   context.move_to(160.0 + 30.0, 120.0);
   context.arc(160.0, 120.0, 30.0, 0.0 * PI, 2.0 * PI);
   context.stroke().unwrap();

   // Create a single context, instead of using create_layout.  This
   // demonstrates avoiding lots of Pango contexts.
   let text_context = pangocairo::create_context(&context);
   let text_layout = pango::Layout::new(&text_context);

   let k_label_font_size = 14.0;

   let mut font_description = pango::FontDescription::new();
   font_description.set_family("sans");
   font_description.set_absolute_size(k_label_font_size * pango::SCALE as f64);

   text_layout.set_font_description(Some(&font_description));
   text_layout.set_text("Hello world!");

   context.set_source_rgb(0.0, 0.0, 1.0);

   context.move_to(120.0, 60.0);
   pangocairo::show_layout(&context, &text_layout);

   surface.flush();
   surface.finish_output_stream()
}

#[cfg(test)]
mod tests {
   #[test]
   fn it_works() {
      assert_eq!(2 + 2, 4);
   }
}

pub struct SplineTest<'a> {
   pub name: &'a str,
}

pub fn write_spline_test_to_file<W: Write + 'static>(
   out_stream: W,
   _spline_def: &SplineTest,
) -> Result<Box<dyn core::any::Any>, cairo::StreamWithError> {
   let canvas_width = 320.0; //  overall.canvas_width,
   let canvas_height = 240.0; // overall.canvas_height
   let x_offset = 40.0; //       overall.canvas_x_offset
   let y_offset = 50.0; //       overall.canvas_y_offset

   let surface = SvgSurface::for_stream(canvas_width, canvas_height, out_stream).unwrap();
   let context = cairo::Context::new(&surface).unwrap();
   context.translate(x_offset, y_offset);

   context.move_to(160.0 + 30.0, 120.0);
   context.arc(160.0, 120.0, 30.0, 0.0 * PI, 2.0 * PI);
   context.stroke().unwrap();

   let text_context = pangocairo::create_context(&context);
   let text_layout = pango::Layout::new(&text_context);

   let k_label_font_size = 14.0;

   let mut font_description = pango::FontDescription::new();
   font_description.set_family("sans");
   font_description.set_absolute_size(k_label_font_size * pango::SCALE as f64);

   text_layout.set_font_description(Some(&font_description));
   text_layout.set_text("Hello world!");

   context.set_source_rgb(0.0, 0.0, 1.0);

   context.move_to(120.0, 60.0);
   pangocairo::show_layout(&context, &text_layout);

   surface.flush();
   surface.finish_output_stream()
}
