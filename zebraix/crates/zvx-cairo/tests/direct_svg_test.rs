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

use serde_json::to_writer_pretty;
use std::f64::consts::PI;
use std::io::Write;
use zvx_cairo::CairoSpartanRender;
use zvx_docagram::diagram::{DrawableDiagram, SizingScheme, SpartanDiagram, SpartanPreparation};
use zvx_golden::filtered::JsonGoldenTest;
use zvx_golden::filtered::SvgGoldenTest;

// This may seem odd, but is Rust-inspired. The diagram and the renderer can be separately
// borrowed with different mutability.
// #[derive(Debug)]
pub struct NotBoxedCairoSpartanCombo {
   pub drawable_diagram: DrawableDiagram,
   pub render_controller: Box<CairoSpartanRender>,
}

// Simplified test version of the standard combo structure that does not box-trait the renderer.
// This provides direct access to rendering members.
impl NotBoxedCairoSpartanCombo {
   #[allow(clippy::missing_panics_doc)]
   pub fn create_for_stream<W: Write + 'static>(
      out_stream: W,
      preparation: &SpartanPreparation,
   ) -> Self {
      Self {
         drawable_diagram: DrawableDiagram { prep: preparation.clone(), drawables: vec![] },
         render_controller: Box::new(CairoSpartanRender::create_not_boxed_for_stream(
            out_stream,
            &preparation.canvas_layout,
            &preparation.diagram_choices,
         )),
      }
   }
}

// After dependency to test_utils is added, use type-def for the result box.
fn write_full_sample_to_write<W: Write + 'static>(
   out_stream: W,
   preparation: &SpartanPreparation,
) -> Result<Box<dyn core::any::Any>, cairo::StreamWithError> {
   let mut cairo_spartan: NotBoxedCairoSpartanCombo =
      NotBoxedCairoSpartanCombo::create_for_stream(out_stream, preparation);

   cairo_spartan.render_controller.unfixed.context.set_line_width(1.0);
   // 1.0 is a regular thickness, definitely not thick, 2.0 definitely thick, 0.6 thin but
   // firm.

   let transform_saver = &mut cairo_spartan.render_controller.unfixed.transform_saver;
   let canvas_layout = &cairo_spartan.drawable_diagram.prep.canvas_layout;
   let context = &cairo_spartan.render_controller.unfixed.context;

   transform_saver.save_set_path_transform(context, canvas_layout);
   context.move_to(-0.1 + 0.12, -0.25);
   context.arc(-0.1, -0.25, 0.12, 0.0 * PI, 2.0 * PI);
   transform_saver.restore_transform(context);
   context.stroke().unwrap();

   transform_saver.save_set_path_transform(context, canvas_layout);
   context.move_to(0.0, 0.0);
   context.line_to(0.5, 0.3);
   context.move_to(0.0, 0.5);
   context.line_to(0.45, 0.0);
   transform_saver.restore_transform(context);
   context.stroke().unwrap();
   transform_saver.save_set_path_transform(context, canvas_layout);
   context.move_to(0.45, 0.0);
   context.set_line_width(0.45);
   context.set_dash(&[4.5, 3.5], 0.0);
   context.line_to(-0.4, -0.35);
   transform_saver.restore_transform(context);
   context.stroke().unwrap();
   context.set_line_width(1.0);
   context.set_dash(&[], 0.0);

   // Draw a point-like circle.
   transform_saver.save_set_path_transform(context, canvas_layout);
   let (cx, cy) = context.user_to_device(0.2, -0.7);
   transform_saver.restore_transform(context);
   context.move_to(cx + 2.4, cy);
   context.arc(cx, cy, 2.4, 0.0 * PI, 2.0 * PI);
   context.close_path();
   context.stroke().unwrap();

   // Draw a plus-like symbol.
   transform_saver.save_set_path_transform(context, canvas_layout);
   let (cx, cy) = context.user_to_device(0.25, -0.7);
   transform_saver.restore_transform(context);
   let plus_delta = 2.4 * 1.32;
   context.move_to(cx, cy - plus_delta);
   context.line_to(cx, cy + plus_delta);
   context.move_to(cx + plus_delta, cy);
   context.line_to(cx - plus_delta, cy);
   context.close_path();
   context.stroke().unwrap();

   // Draw a times-like symbol.
   transform_saver.save_set_path_transform(context, canvas_layout);
   let (cx, cy) = context.user_to_device(0.15, -0.7);
   transform_saver.restore_transform(context);
   let times_delta = 2.4 * 1.32 * 0.5_f64.sqrt();
   context.move_to(cx - times_delta, cy - times_delta);
   context.line_to(cx + times_delta, cy + times_delta);
   context.move_to(cx + times_delta, cy - times_delta);
   context.line_to(cx - times_delta, cy + times_delta);
   context.close_path();
   context.stroke().unwrap();

   // Create a single context, instead of using create_layout.  This
   // demonstrates avoiding lots of Pango contexts.
   let text_context = pangocairo::functions::create_context(context);
   let text_layout = pango::Layout::new(&text_context);

   let k_label_font_size = 10.0;

   let mut font_description = pango::FontDescription::new();
   font_description.set_family("sans");
   font_description.set_absolute_size(k_label_font_size * pango::SCALE as f64);

   text_layout.set_font_description(Some(&font_description));
   text_layout.set_text("Hello world!");
   {
      let metrics = text_layout.context().metrics(Some(&font_description), None);

      assert_eq!(metrics.height(), metrics.descent() + metrics.ascent());
      assert_eq!(pango::SCALE, 1024);
      assert_eq!(metrics.height(), 13947); // 13947 = 13.62 * 1024.
      assert_eq!(metrics.descent(), 3000);
      assert_eq!(metrics.ascent(), 10947);
      // Strikethrough is top of line above baseline.
      let strikethrough_center =
         metrics.strikethrough_position() as f64 - metrics.strikethrough_thickness() as f64 * 0.5;
      assert!((strikethrough_center - 3041.0).abs() < 1e-10);

      let (_text_width, text_height) = text_layout.size();
      assert_eq!(text_height, 13947);
      text_layout.set_text("xopqgox");
      let (_text_width, text_height) = text_layout.size();
      assert_eq!(text_height, 13947);
      text_layout.set_text("Hello world!");

      // renderer_data
      //    .debug_lines
      //    .push(DebugLine::SimpleString(format!("Text height = {h}", h = metrics.height())));
      // renderer_data
      //    .debug_lines
      //    .push(DebugLine::SimpleString(format!("Text descent = {h}", h = metrics.descent())));
      // renderer_data
      //    .debug_lines
      //    .push(DebugLine::SimpleString(format!("Text ascent = {h}", h = metrics.ascent())));

      // let strikethrough_centre =
      //    metrics.strikethrough_position() + metrics.strikethrough_thickness() / 2;
      // renderer_data
      //    .debug_lines
      //    .push(DebugLine::SimpleString(format!("Text anchor = {h}", h = strikethrough_centre)));

      // Extents depend on set_absolute_size.  Assume pango::SCALE = 1024.
      //
      // Note that 10 * 1024 * 1.362 = 13946.88.
      //
      // 14*1024: Logical extents (x, y, w, h) = 0, 0, 81920, 19525 for "Hello world!"
      // 10*1024: Logical extents (x, y, w, h) = 0, 0, 59392, 13947 for "Hello world!"
      // 10*1024: Logical extents (x, y, w, h) = 0, 0, 64512, 13947 for "Hello worldy!"
      // 10*1024: Logical extents (x, y, w, h) = 0, 0, 45056, 13947 for "ace noun"
      //
      // 14*1024: Ink extents (x, y, w, h) = 1391, 4430, 79243, 11096 for "Hello world!"
      // 10*1024: Ink extents (x, y, w, h) = 993, 3165, 57334, 7926 for "Hello world!"
      // 10*1024: Ink extents (x, y, w, h) = 993, 3165, 62454, 10239 for "Hello worldy!"
      // 10*1024: Ink extents (x, y, w, h) = 471, 5356, 43939, 5693 for "ace noun"
      //
      // 10239 ~= 10*1024.
      // 3165 + 10239 = 13404.
      // Ascent = 7926; descent = 10239 - 7926 = 2313.  So 7926 : 2313 =  0.774 : 0.226.
      //
      // Layout.spacing() = 0.
      //
      // Layout context is independent of set_absolute_size.
      // Text height = 22315
      // Text descent = 4801
      // Text ascent = 17514
      // Text anchor = 5685  (This is strikethrough position + half thickness.)
      // 17514 : 4801 = : 0.785 : 0.215, which is not correct, so ascent is padded.
      // 4801 * 7926 / 2313 = 16452.  4801 * 10239 / 2313 = 21253.
      // (22315 - 21253) / 21253 = 0.05
      //
      // This mess is unresolvable.  For now, if using anchor, descent, etc, scale by
      // 1024 / 21253 = 0.0482.
      // In other words, the metrics seem to be for 20.755pt font.
      //
      // Corrected: Context obtained with font description.
      // Text height = 13947
      // Text descent = 3000
      // Text ascent = 10947
      // Text anchor = 3553  (This is strikethrough position + half thickness.)
   }
   context.set_source_rgb(0.0, 0.0, 1.0);

   transform_saver.save_set_path_transform(context, canvas_layout);
   context.move_to(0.3, -0.2);
   transform_saver.restore_transform(context);
   pangocairo::functions::show_layout(context, &text_layout);

   cairo_spartan.render_controller.unfixed.surface.flush();
   cairo_spartan.render_controller.unfixed.surface.finish_output_stream()
}

#[test]
fn simple_full_spartan_test() {
   let spartan = SpartanDiagram {
      sizing_scheme: SizingScheme::Fill,
      axes_range: vec![-0.5, -0.5, 1.5, 3.5],
      ..Default::default()
   };
   let preparation = spartan.prepare();

   {
      let mut json_golden = JsonGoldenTest::new("tests/goldenfiles/", "simple_spartan");
      // This only really fails if keys cannot be rendered.
      //
      // Consider moving into golden test crate. This is only trigger for serde_json dependency.
      to_writer_pretty(&json_golden.out_stream, &spartan).unwrap();
      // let serialized = to_string_pretty::<SpartanDiagram>(&cairo_spartan.spartan).unwrap();
      // json_golden.writeln_as_bytes(&serialized);
      let bytes_amount_nl = json_golden.out_stream.write(b"\n").unwrap();
      assert!(bytes_amount_nl == 1);
   }

   {
      let svg_golden = SvgGoldenTest::new("tests/goldenfiles/", "simple_spartan");
      let raw_result = write_full_sample_to_write(svg_golden.get_raw_writeable(), &preparation);
      svg_golden.handover_result(raw_result.unwrap());
   }
}
