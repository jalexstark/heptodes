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

extern crate goldenfile;

use cairo::SvgSurface;
use cairo::SvgUnit::Pt;
use std::f64::consts::PI;
use std::io::Write;

use diag_golden::SvgGoldenTest;

// After dependency to test_utils is added, use type-def for the result box.
pub fn write_sample_to_write<W: Write + 'static>(
   out_stream: W,
) -> Result<Box<dyn core::any::Any>, cairo::StreamWithError> {
   let canvas_width = 400.0; //  overall.canvas_width,
   let canvas_height = 300.0; // overall.canvas_height
   let x_offset = 40.0; //       overall.canvas_x_offset
   let y_offset = 50.0; //       overall.canvas_y_offset

   let mut surface = SvgSurface::for_stream(canvas_width, canvas_height, out_stream).unwrap();
   surface.set_document_unit(Pt);
   let context = cairo::Context::new(&surface).unwrap();
   context.translate(x_offset, y_offset);
   // 1.0 is a regular thickness, definitely not thick, 2.0 definitely thick, 0.6 thin but
   // firm.
   context.set_line_width(1.0);

   context.move_to(160.0 + 30.0, 120.0);
   context.arc(160.0, 120.0, 30.0, 0.0 * PI, 2.0 * PI);
   context.line_to(canvas_width - 40.0, 0.0 - 50.0);
   context.stroke().unwrap();
   context.move_to(canvas_width - 40.0, 0.0 - 50.0);
   context.set_line_width(0.45);
   context.set_dash(&[4.5, 3.5], 0.0);
   context.line_to(0.0 - 40.0, canvas_height - 50.0);
   context.line_to(0.0, 0.0);
   context.stroke().unwrap();

   // Create a single context, instead of using create_layout.  This
   // demonstrates avoiding lots of Pango contexts.
   let text_context = pangocairo::create_context(&context);
   let text_layout = pango::Layout::new(&text_context);

   let k_label_font_size = 12.0;

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

#[test]
fn simple_cairo_test() {
   let svg_golden = SvgGoldenTest::new("tests/goldenfiles/", "simple_cairo_svg");
   let raw_result = write_sample_to_write(svg_golden.get_raw_writeable());
   svg_golden.handover_result(raw_result.unwrap());
}
