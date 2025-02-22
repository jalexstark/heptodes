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

// Try use crate::... instead of use diag_golden::... ?

// extern crate goldenfile;

use cairo::SvgSurface;
use cairo::SvgUnit::Pt;
use diag_golden::draw_sample_cubilinear;
use diag_golden::draw_sample_rat_quad;
use diag_golden::AxesSpec;
use diag_golden::AxesStyle;
use diag_golden::AxisNumbering;
use diag_golden::BaseRatQuad;
use diag_golden::CairoSpartanCombo;
use diag_golden::CirclesDrawable;
use diag_golden::ColorChoice;
use diag_golden::CubiLinear;
use diag_golden::JsonGoldenTest;
use diag_golden::LineChoice;
use diag_golden::LineClosureChoice;
use diag_golden::LinesDrawable;
use diag_golden::ManagedCubic;
use diag_golden::ManagedRatQuad;
use diag_golden::OneOfDrawable;
use diag_golden::PointChoice;
use diag_golden::PointsDrawable;
use diag_golden::PolylineDrawable;
use diag_golden::QualifiedDrawable;
use diag_golden::SampleCurveConfig;
use diag_golden::SampleOption;
use diag_golden::SizingScheme;
use diag_golden::SpartanDiagram;
use diag_golden::SvgGoldenTest;
use diag_golden::TextAnchorHorizontal;
use diag_golden::TextAnchorVertical;
use diag_golden::TextDrawable;
use diag_golden::TextOffsetChoice;
use diag_golden::TextSingle;
use diag_golden::TextSizeChoice;
use serde_json::to_string_pretty;
use std::f64::consts::PI;
use std::io::Write;

fn scale_coord_vec(v: &Vec<[f64; 2]>, s: f64) -> Vec<[f64; 2]> {
   let mut result = v.clone();
   for i in 0..v.len() {
      result[i] = [v[i][0] * s, v[i][1] * s];
   }
   result
}

// After dependency to test_utils is added, use type-def for the result box.
fn write_full_sample_to_write<W: Write + 'static>(
   out_stream: W,
   cairo_spartan: &mut CairoSpartanCombo,
) -> Result<Box<dyn core::any::Any>, cairo::StreamWithError> {
   assert!(cairo_spartan.spartan.is_ready());

   assert!((cairo_spartan.spartan.base_width - 400.0).abs() < 0.0001);
   let mut surface = SvgSurface::for_stream(
      cairo_spartan.spartan.base_width,
      cairo_spartan.spartan.base_height,
      out_stream,
   )
   .unwrap();
   surface.set_document_unit(Pt);

   let context = cairo::Context::new(&surface).unwrap();
   context.set_line_width(1.0);
   // 1.0 is a regular thickness, definitely not thick, 2.0 definitely thick, 0.6 thin but
   // firm.

   cairo_spartan.render_controller.save_set_path_transform(&cairo_spartan.spartan.prep, &context);
   context.move_to(-0.1 + 0.12, -0.25);
   context.arc(-0.1, -0.25, 0.12, 0.0 * PI, 2.0 * PI);
   cairo_spartan.render_controller.restore_transform(&context);
   context.stroke().unwrap();

   cairo_spartan.render_controller.save_set_path_transform(&cairo_spartan.spartan.prep, &context);
   context.move_to(0.0, 0.0);
   context.line_to(0.5, 0.3);
   context.move_to(0.0, 0.5);
   context.line_to(0.45, 0.0);
   cairo_spartan.render_controller.restore_transform(&context);
   context.stroke().unwrap();
   cairo_spartan.render_controller.save_set_path_transform(&cairo_spartan.spartan.prep, &context);
   context.move_to(0.45, 0.0);
   context.set_line_width(0.45);
   context.set_dash(&[4.5, 3.5], 0.0);
   context.line_to(-0.4, -0.35);
   cairo_spartan.render_controller.restore_transform(&context);
   context.stroke().unwrap();
   context.set_line_width(1.0);
   context.set_dash(&[], 0.0);

   // Draw a point-like circle.
   cairo_spartan.render_controller.save_set_path_transform(&cairo_spartan.spartan.prep, &context);
   let (cx, cy) = context.user_to_device(0.2, -0.7);
   cairo_spartan.render_controller.restore_transform(&context);
   context.move_to(cx + 2.4, cy);
   context.arc(cx, cy, 2.4, 0.0 * PI, 2.0 * PI);
   context.close_path();
   context.stroke().unwrap();

   // Draw a plus-like symbol.
   cairo_spartan.render_controller.save_set_path_transform(&cairo_spartan.spartan.prep, &context);
   let (cx, cy) = context.user_to_device(0.25, -0.7);
   cairo_spartan.render_controller.restore_transform(&context);
   let plus_delta = 2.4 * 1.32;
   context.move_to(cx, cy - plus_delta);
   context.line_to(cx, cy + plus_delta);
   context.move_to(cx + plus_delta, cy);
   context.line_to(cx - plus_delta, cy);
   context.close_path();
   context.stroke().unwrap();

   // Draw a times-like symbol.
   cairo_spartan.render_controller.save_set_path_transform(&cairo_spartan.spartan.prep, &context);
   let (cx, cy) = context.user_to_device(0.15, -0.7);
   cairo_spartan.render_controller.restore_transform(&context);
   let times_delta = 2.4 * 1.32 * 0.5_f64.sqrt();
   context.move_to(cx - times_delta, cy - times_delta);
   context.line_to(cx + times_delta, cy + times_delta);
   context.move_to(cx + times_delta, cy - times_delta);
   context.line_to(cx - times_delta, cy + times_delta);
   context.close_path();
   context.stroke().unwrap();

   // Create a single context, instead of using create_layout.  This
   // demonstrates avoiding lots of Pango contexts.
   let text_context = pangocairo::functions::create_context(&context);
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
      assert_eq!(strikethrough_center, 3041.0);

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

   cairo_spartan.render_controller.save_set_path_transform(&cairo_spartan.spartan.prep, &context);
   context.move_to(0.3, -0.2);
   cairo_spartan.render_controller.restore_transform(&context);
   pangocairo::functions::show_layout(&context, &text_layout);

   surface.flush();
   surface.finish_output_stream()
}

#[test]
fn simple_full_spartan_test() {
   let mut cairo_spartan = CairoSpartanCombo::new();
   cairo_spartan.spartan.sizing_scheme = SizingScheme::Fill;
   cairo_spartan.spartan.axes_range = vec![-0.5, -0.5, 1.5, 3.5];
   cairo_spartan.spartan.prepare();

   {
      let mut json_golden = JsonGoldenTest::new("tests/goldenfiles/", "simple_spartan");
      let serialized = to_string_pretty::<SpartanDiagram>(&cairo_spartan.spartan).unwrap();
      json_golden.writeln_as_bytes(&serialized);
   }

   {
      let svg_golden = SvgGoldenTest::new("tests/goldenfiles/", "simple_spartan");
      let raw_result =
         write_full_sample_to_write(svg_golden.get_raw_writeable(), &mut cairo_spartan);
      svg_golden.handover_result(raw_result.unwrap());
   }
}

#[derive(Default)]
struct TestSizing {
   sizing_scheme: SizingScheme,
   canvas_size: [f64; 2],
   axes_range: Vec<f64>,
   padding: Vec<f64>,
   debug_box: [f64; 4],
   axes_spec: AxesSpec,
}

// After dependency to test_utils is added, use type-def for the result box.
fn write_sample_to_write<W: Write + 'static>(
   out_stream: W,
   cairo_spartan: &mut CairoSpartanCombo,
) -> Result<Box<dyn core::any::Any>, cairo::StreamWithError> {
   assert!(cairo_spartan.spartan.is_ready());
   cairo_spartan.render_controller.render_to_stream(out_stream, &cairo_spartan.spartan)
}

fn create_sized_diagram(sizing: &TestSizing) -> CairoSpartanCombo {
   let mut cairo_spartan = CairoSpartanCombo::new();
   cairo_spartan.spartan.sizing_scheme = sizing.sizing_scheme;
   cairo_spartan.spartan.base_width = sizing.canvas_size[0];
   cairo_spartan.spartan.base_height = sizing.canvas_size[1];
   cairo_spartan.spartan.axes_range = sizing.axes_range.clone();
   cairo_spartan.spartan.padding = sizing.padding.clone();

   cairo_spartan
}

fn build_diagram(sizing: &TestSizing) -> CairoSpartanCombo {
   let mut cairo_spartan = create_sized_diagram(sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   {
      let pattern_layer = 0;
      let qualified_drawable = QualifiedDrawable {
         layer: pattern_layer,
         drawable: OneOfDrawable::Lines(LinesDrawable {
            start: vec![[sizing.debug_box[0], sizing.debug_box[1]]],
            end: vec![[sizing.debug_box[0], sizing.debug_box[3]]],
            offsets: Some(vec![[0.0, 0.0], [sizing.debug_box[2] - sizing.debug_box[0], 0.0]]),
            ..Default::default()
         }),
         ..Default::default()
      };
      cairo_spartan.spartan.drawables.push(qualified_drawable);
   }
   {
      let pattern_layer = 0;
      let qualified_drawable = QualifiedDrawable {
         layer: pattern_layer,
         drawable: OneOfDrawable::Lines(LinesDrawable {
            line_choice: LineChoice::Light,
            start: vec![
               [sizing.debug_box[0], sizing.debug_box[3]],
               [sizing.debug_box[2], sizing.debug_box[3]],
            ],
            end: vec![
               [sizing.debug_box[2], sizing.debug_box[1]],
               [sizing.debug_box[0], sizing.debug_box[1]],
            ],
            offsets: Some(vec![[0.0, 0.0]]),
            ..Default::default()
         }),
         ..Default::default()
      };
      cairo_spartan.spartan.drawables.push(qualified_drawable);
   }

   cairo_spartan
}

fn run_json_svg(filestem: &str, cairo_spartan: &mut CairoSpartanCombo) {
   {
      let mut json_golden = JsonGoldenTest::new("tests/goldenfiles/", filestem);
      let serialized = to_string_pretty::<SpartanDiagram>(&cairo_spartan.spartan).unwrap();
      json_golden.writeln_as_bytes(&serialized);
   }

   {
      let svg_golden = SvgGoldenTest::new("tests/goldenfiles/", filestem);
      let raw_result = write_sample_to_write(svg_golden.get_raw_writeable(), cairo_spartan);
      svg_golden.handover_result(raw_result.unwrap());
   }
}

fn spartan_sizing(filestem: &str, sizing: &TestSizing) {
   let mut cairo_spartan = build_diagram(sizing);
   run_json_svg(filestem, &mut cairo_spartan);
}

#[test]
fn spartan_sizing_a_test() {
   // range (-2.0, 2.0), no padding.
   let r = 2.0;
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::Fill,
      canvas_size: [400.0, 300.0],
      axes_range: vec![r],
      padding: vec![0.0],
      debug_box: [-r * 0.5, -r * 0.5, r * 0.5, r * 0.5],
      ..Default::default()
   };
   spartan_sizing("spartan_sizing_a", &sizing);
}

#[test]
fn spartan_sizing_b_test() {
   // range (-2.0, 2.0), mixed padding.
   let r = 2.0;
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::Fill,
      canvas_size: [400.0, 300.0],
      axes_range: vec![r],
      padding: vec![0.1, 0.2, 0.15, 0.05],
      debug_box: [-r, -r, r, r],
      ..Default::default()
   };
   spartan_sizing("spartan_sizing_b", &sizing);
}

#[test]
fn spartan_sizing_c_test() {
   // range (-2.0, 2.0), no padding.
   let r = 2.0;
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareShrink,
      canvas_size: [500.0, 200.0],
      axes_range: vec![r],
      padding: vec![],
      debug_box: [-r, -r, r, r],
      ..Default::default()
   };
   spartan_sizing("spartan_sizing_c", &sizing);
}

#[test]
fn spartan_sizing_d_test() {
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareShrink,
      canvas_size: [300.0, 450.0],
      axes_range: vec![-2.0, -1.5, 2.0, 1.5],
      padding: vec![],
      debug_box: [-2.0, -1.5, 2.0, 1.5],
      ..Default::default()
   };
   spartan_sizing("spartan_sizing_d", &sizing);
}

#[test]
fn spartan_sizing_e_test() {
   // range (-2.0, 2.0), no padding.
   let r = 2.0;
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [500.0, 200.0],
      axes_range: vec![r],
      padding: vec![],
      debug_box: [-r, -r, r, r],
      ..Default::default()
   };
   spartan_sizing("spartan_sizing_e", &sizing);
}

#[test]
fn spartan_sizing_f_test() {
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [300.0, 450.0],
      axes_range: vec![-2.0, -1.5, 2.0, 1.5],
      padding: vec![],
      debug_box: [-2.0, -1.5, 2.0, 1.5],
      ..Default::default()
   };
   spartan_sizing("spartan_sizing_f", &sizing);
}

#[test]
fn spartan_sizing_g_test() {
   // range (-2.0, 2.0), mixed padding.
   let r = 2.0;
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [400.0, 300.0],
      axes_range: vec![r],
      padding: vec![0.06],
      debug_box: [-0.5 * r, -0.5 * r, 0.5 * r, 0.5 * r],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Box,
         grid_interval: [0.4, 0.6],
         grid_precision: vec![0, 1],
         ..Default::default()
      },
      ..Default::default()
   };
   spartan_sizing("spartan_sizing_g", &sizing);
}

#[test]
fn spartan_sizing_h_test() {
   // range (-2.0, 2.0), mixed padding.
   let r = 2.0;
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::Fill,
      canvas_size: [400.0, 350.0],
      axes_range: vec![r],
      padding: vec![0.09, 0.23],
      debug_box: [-0.5 * r, -0.5 * r, 0.5 * r, 0.5 * r],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Cross,
         axis_numbering: AxisNumbering::Before,
         grid_interval: [0.4, 0.75],
         grid_precision: vec![1, 2],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = build_diagram(&sizing);

   let title_layer = 10;
   let vertical_title_anchor = -2.48;
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: title_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Large,
         color_choice: ColorChoice::BrightBlue,
         // offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Center,
         anchor_vertical: TextAnchorVertical::Bottom,
         texts: vec![TextSingle {
            content: "This is a title test".to_string(),
            location: [0.0, vertical_title_anchor],
         }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: title_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         color_choice: ColorChoice::BrightBlue,
         // offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Center,
         anchor_vertical: TextAnchorVertical::Top,
         texts: vec![TextSingle {
            content: "This subtitle has the same anchor location".to_string(),
            location: [0.0, vertical_title_anchor],
         }],
         ..Default::default()
      }),
      ..Default::default()
   });
   run_json_svg("spartan_sizing_h", &mut cairo_spartan);
}

#[test]
fn spartan_sizing_i_test() {
   // Points illustration.
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [500.0, 500.0],
      axes_range: vec![5.0],
      padding: vec![0.05],
      // axes_spec: AxesSpec {
      //    axes_style: AxesStyle::Cross,
      //    grid_interval: [0.4, 0.75],
      //    ..Default::default()
      // },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   {
      let pattern_layer = 0;
      let qualified_drawable = QualifiedDrawable {
         layer: pattern_layer,
         drawable: OneOfDrawable::Lines(LinesDrawable {
            start: vec![[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]],
            end: vec![[5.0, 5.0], [0.0, 5.0], [-2.5, 5.0]],
            offsets: None,
            ..Default::default()
         }),
         ..Default::default()
      };
      cairo_spartan.spartan.drawables.push(qualified_drawable);
   }
   {
      let pattern_layer = 0;
      let qualified_drawable = QualifiedDrawable {
         layer: pattern_layer,
         drawable: OneOfDrawable::Lines(LinesDrawable {
            line_choice: LineChoice::Light,
            start: vec![[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]],
            end: vec![[5.0, -5.0], [0.0, -5.0], [-2.5, -5.0]],
            ..Default::default()
         }),
         ..Default::default()
      };
      cairo_spartan.spartan.drawables.push(qualified_drawable);
   }

   let pattern_vec =
      vec![[1.0, 1.0], [0.0, 1.0], [-0.5, 1.0], [1.0, -1.0], [0.0, -1.0], [-0.5, -1.0]];

   let pattern_layer = 0;
   {
      let qualified_drawable = QualifiedDrawable {
         layer: pattern_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            centers: pattern_vec.clone(),
            ..Default::default()
         }),
         ..Default::default()
      };
      cairo_spartan.spartan.drawables.push(qualified_drawable);
   }

   {
      let qualified_drawable = QualifiedDrawable {
         layer: pattern_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: PointChoice::Times,
            centers: scale_coord_vec(&pattern_vec, 2.0),
            ..Default::default()
         }),
         ..Default::default()
      };
      cairo_spartan.spartan.drawables.push(qualified_drawable);
   }

   {
      let qualified_drawable = QualifiedDrawable {
         layer: pattern_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: PointChoice::Plus,
            centers: scale_coord_vec(&pattern_vec, 3.0),
            ..Default::default()
         }),
         ..Default::default()
      };
      cairo_spartan.spartan.drawables.push(qualified_drawable);
   }

   {
      let qualified_drawable = QualifiedDrawable {
         layer: pattern_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: PointChoice::Dot,
            centers: scale_coord_vec(&pattern_vec, 4.0),
            ..Default::default()
         }),
         ..Default::default()
      };
      cairo_spartan.spartan.drawables.push(qualified_drawable);
   }

   run_json_svg("spartan_sizing_i", &mut cairo_spartan);
}

#[test]
fn spartan_sizing_j_test() {
   // Points illustration.
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [600.0, 500.0],
      axes_range: vec![6.5, 5.0],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::None,
         grid_interval: [2.0, 1.5],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let pattern_vec = vec![
      [2.0, 0.0],
      [2.0, 1.5],
      [2.0, -1.5], // Left-justified, 3 variations.
      [4.0, 0.0],
      [4.0, 1.5],
      [4.0, -1.5], // Left-justified, 3 variations.
      [2.0, -3.0],
      [2.0, 3.0], // Corner-anchored.
      [0.0, 1.5],
      [0.0, 3.0],
      [0.0, 4.5], // Centered, 3 variations.
      [0.0, 0.0],
   ];

   let pattern_layer = 0;
   {
      cairo_spartan.spartan.drawables.push(QualifiedDrawable {
         layer: pattern_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: PointChoice::Dot,
            color_choice: ColorChoice::Gray,
            centers: scale_coord_vec(&pattern_vec, 1.0),
            ..Default::default()
         }),
         ..Default::default()
      });
   }
   {
      let qualified_drawable = QualifiedDrawable {
         layer: pattern_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: PointChoice::Dot,
            color_choice: ColorChoice::Gray,
            centers: scale_coord_vec(&pattern_vec, -1.0),
            ..Default::default()
         }),
         ..Default::default()
      };
      cairo_spartan.spartan.drawables.push(qualified_drawable);
   }

   let spanning_string = "Elpo xftdg";

   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         anchor_horizontal: TextAnchorHorizontal::Center,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle { content: "o+=-x-=+o".to_string(), location: [0.0, 0.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Large,
         color_choice: ColorChoice::Red,
         offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Left,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [2.0, 1.5] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         color_choice: ColorChoice::Green,
         offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Left,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [2.0, 0.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Small,
         color_choice: ColorChoice::Blue,
         offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Left,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [2.0, -1.5] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         color_choice: ColorChoice::BlueBlueGreen,
         anchor_horizontal: TextAnchorHorizontal::Left,
         anchor_vertical: TextAnchorVertical::Top,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [2.0, -3.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Large,
         color_choice: ColorChoice::YellowBrown,
         anchor_horizontal: TextAnchorHorizontal::Left,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle { content: "xopqgox".to_string(), location: [4.0, 1.5] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         color_choice: ColorChoice::BlueGreen,
         anchor_horizontal: TextAnchorHorizontal::Left,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [4.0, 0.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Small,
         color_choice: ColorChoice::Magenta,
         anchor_horizontal: TextAnchorHorizontal::Left,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle { content: "xodflox".to_string(), location: [4.0, -1.5] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         anchor_horizontal: TextAnchorHorizontal::Left,
         anchor_vertical: TextAnchorVertical::Bottom,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [2.0, 3.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         color_choice: ColorChoice::RedRedBlue,
         offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Center,
         anchor_vertical: TextAnchorVertical::Bottom,
         texts: vec![TextSingle { content: "Elpo x lpoE".to_string(), location: [0.0, 1.5] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Large,
         color_choice: ColorChoice::BlueBlueRed,
         offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Center,
         anchor_vertical: TextAnchorVertical::Bottom,
         texts: vec![TextSingle { content: "Elpo x lpoE".to_string(), location: [0.0, 3.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Small,
         offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Center,
         anchor_vertical: TextAnchorVertical::Bottom,
         texts: vec![TextSingle { content: "Elpo x lpoE".to_string(), location: [0.0, 4.5] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Large,
         color_choice: ColorChoice::Blue,
         offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Right,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [-2.0, 1.5] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         color_choice: ColorChoice::Green,
         offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Right,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [-2.0, 0.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Small,
         color_choice: ColorChoice::Red,
         offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Right,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [-2.0, -1.5] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Large,
         color_choice: ColorChoice::Magenta,
         anchor_horizontal: TextAnchorHorizontal::Right,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle {
            content: "oxacoxocaxo\nox=c-+-c=xo\noxacoxocaxo".to_string(),
            location: [-4.0, 1.5],
         }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         color_choice: ColorChoice::BlueGreen,
         anchor_horizontal: TextAnchorHorizontal::Right,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle {
            content: "oxacoxocaxo\nox=c-+-c=xo\noxacoxocaxo".to_string(),
            location: [-4.0, 0.0],
         }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Small,
         color_choice: ColorChoice::YellowBrown,
         anchor_horizontal: TextAnchorHorizontal::Right,
         anchor_vertical: TextAnchorVertical::Middle,
         texts: vec![TextSingle {
            content: "oxacoxocaxo\nox=c-+-c=xo\noxacoxocaxo".to_string(),
            location: [-4.0, -1.5],
         }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         anchor_horizontal: TextAnchorHorizontal::Right,
         anchor_vertical: TextAnchorVertical::Bottom,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [-2.0, 3.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         color_choice: ColorChoice::RedRedGreen,
         anchor_horizontal: TextAnchorHorizontal::Right,
         anchor_vertical: TextAnchorVertical::Top,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [-2.0, -3.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         color_choice: ColorChoice::GreenGreenBlue,
         offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Center,
         anchor_vertical: TextAnchorVertical::Top,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [0.0, -1.5] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Large,
         color_choice: ColorChoice::GreenGreenRed,
         offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Center,
         anchor_vertical: TextAnchorVertical::Top,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [0.0, -3.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Small,
         offset_choice: TextOffsetChoice::Both,
         anchor_horizontal: TextAnchorHorizontal::Center,
         anchor_vertical: TextAnchorVertical::Top,
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [0.0, -4.5] }],
         ..Default::default()
      }),
      ..Default::default()
   });

   run_json_svg("spartan_sizing_j", &mut cairo_spartan);
}

#[test]
fn spartan_sizing_k_test() {
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [400.0, 300.0],
      axes_range: vec![5.0],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Box,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   // let mut cairo_spartan = build_diagram(&sizing);

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.base_line_width = 4.0;
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let behind_layer = 10;
   let front_layer = 15;

   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: front_layer,
      drawable: OneOfDrawable::Circles(CirclesDrawable {
         color_choice: ColorChoice::BrightRed,
         radius: 1.2,
         centers: vec![[-1.5, 3.0], [1.5, 3.0]],
         ..Default::default()
      }),
      ..Default::default()
   });

   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: behind_layer,
      drawable: OneOfDrawable::Circles(CirclesDrawable {
         color_choice: ColorChoice::Blue,
         radius: 1.2,
         centers: vec![[-3.0, 3.0], [0.0, 3.0], [3.0, 3.0]],
         ..Default::default()
      }),
      ..Default::default()
   });

   run_json_svg("spartan_sizing_k", &mut cairo_spartan);
}

#[test]
fn spartan_sizing_l_test() {
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [400.0, 300.0],
      axes_range: vec![5.0],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Box,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let drawable_layer = 0;

   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: drawable_layer,
      drawable: OneOfDrawable::Polyline(PolylineDrawable {
         color_choice: ColorChoice::Red,
         // line_closure_choice: LineClosureChoice::Open,
         locations: vec![
            [-3.0, 2.0],
            [-2.0, 3.0],
            [-1.0, 1.0],
            [0.0, 3.0],
            [1.0, 1.0],
            [2.0, 3.0],
            [3.0, 2.0],
         ],
         ..Default::default()
      }),
      ..Default::default()
   });

   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: drawable_layer,
      drawable: OneOfDrawable::Polyline(PolylineDrawable {
         color_choice: ColorChoice::Green,
         line_closure_choice: LineClosureChoice::Closed,
         locations: vec![
            [-3.0, -2.0],
            [-2.0, -3.0],
            [-1.0, -1.0],
            [0.0, -3.0],
            [1.0, -1.0],
            [2.0, -3.0],
            [3.0, -2.0],
         ],
         ..Default::default()
      }),
      ..Default::default()
   });

   run_json_svg("spartan_sizing_l", &mut cairo_spartan);
}

#[test]
fn spartan_sizing_m_test() {
   let t_range = [-10.0, 10.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::Fill,
      canvas_size: [400.0, 300.0],
      axes_range: vec![-12.0, -5.0, 12.0, 1.0],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Box,
         grid_interval: [2.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let rat_quad = BaseRatQuad {
      a: [-21.0, 1.0, -2.0],
      b: [-3.1414, 4.7811, 6.5534],
      r: t_range,
      ..Default::default()
   };

   let managed_curve = ManagedRatQuad::new(&rat_quad, cairo_spartan.spartan.prep.axes_range);
   draw_sample_rat_quad(
      &managed_curve,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::Red),
         points_color: Some(ColorChoice::Blue),
         points_choice: PointChoice::Circle,
         points_num_segments: 12,
         approx_num_segments: 50,
         sample_options: SampleOption::XVsT,
         ..Default::default()
      },
   );

   run_json_svg("spartan_sizing_m", &mut cairo_spartan);
}

#[test]
fn spartan_sizing_n_test() {
   let t_range = [-6.0, 14.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::Fill,
      canvas_size: [400.0, 300.0],
      axes_range: vec![-4.5, -2.5, 1.5, 2.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Box,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let rat_quad = BaseRatQuad {
      a: [-21.0, 1.0, -2.0],
      b: [-3.1414, 4.7811, 6.5534],
      c: [0.0, 20.0, 0.0],
      r: t_range,
      ..Default::default()
   };

   let managed_curve = ManagedRatQuad::new(&rat_quad, cairo_spartan.spartan.prep.axes_range);
   draw_sample_rat_quad(
      &managed_curve,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::Red),
         points_color: Some(ColorChoice::Blue),
         points_choice: PointChoice::Circle,
         points_num_segments: 12,
         approx_num_segments: 50,
         ..Default::default()
      },
   );

   run_json_svg("spartan_sizing_n", &mut cairo_spartan);
}

// This does not need to be graphical, but instead should match numerically.  The polyline
// points should not move.
#[test]
fn spartan_sizing_n1_test() {
   let t_range = [-6.0, 14.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::Fill,
      canvas_size: [400.0, 300.0],
      axes_range: vec![-4.5, -2.5, 1.5, 2.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Box,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let mut managed_curve = ManagedRatQuad::new(
      &BaseRatQuad {
         a: [-21.0, 1.0, -2.0],
         b: [-3.1414, 4.7811, 6.5534],
         c: [0.0, 20.0, 0.0],
         r: t_range,
         ..Default::default()
      },
      cairo_spartan.spartan.prep.axes_range,
   );
   managed_curve.raise_to_symmetric_range().unwrap();

   draw_sample_rat_quad(
      &managed_curve,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::Red),
         points_color: Some(ColorChoice::Blue),
         points_choice: PointChoice::Circle,
         points_num_segments: 12,
         approx_num_segments: 50,
         ..Default::default()
      },
   );

   run_json_svg("spartan_sizing_n1", &mut cairo_spartan);
}

#[test]
fn spartan_sizing_o_test() {
   let t_range = [-6.0, 14.0];
   let sigma = 0.5; // Curve is slower at the start, so this balances a bit.

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::Fill,
      canvas_size: [400.0, 300.0],
      axes_range: vec![-4.5, -2.5, 1.5, 2.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Box,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let mut managed_curve = ManagedRatQuad::new(
      &BaseRatQuad {
         a: [-21.0, 1.0, -2.0],
         b: [-3.1414, 4.7811, 6.5534],
         c: [0.0, 20.0, 0.0],
         r: t_range,
         ..Default::default()
      },
      cairo_spartan.spartan.prep.axes_range,
   );
   managed_curve.apply_bilinear(sigma).unwrap();
   draw_sample_rat_quad(
      &managed_curve,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::Red),
         points_color: Some(ColorChoice::Blue),
         points_choice: PointChoice::Circle,
         points_num_segments: 12,
         approx_num_segments: 50,
         ..Default::default()
      },
   );

   run_json_svg("spartan_sizing_o", &mut cairo_spartan);
}

// Symmetric range, warped.
//
// This does not need to be graphical, but instead should match numerically.  The polyline
// points should not move.
#[test]
fn spartan_sizing_o1_test() {
   let t_range = [-6.0, 14.0];
   let sigma = 0.5; // Curve is slower at the start, so this balances a bit.

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::Fill,
      canvas_size: [400.0, 300.0],
      axes_range: vec![-4.5, -2.5, 1.5, 2.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Box,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let mut managed_curve = ManagedRatQuad::new(
      &BaseRatQuad {
         a: [-21.0, 1.0, -2.0],
         b: [-3.1414, 4.7811, 6.5534],
         c: [0.0, 20.0, 0.0],
         r: t_range,
         ..Default::default()
      },
      cairo_spartan.spartan.prep.axes_range,
   );
   managed_curve.raise_to_symmetric_range().unwrap();
   managed_curve.apply_bilinear(sigma).unwrap();

   draw_sample_rat_quad(
      &managed_curve,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::Red),
         points_color: Some(ColorChoice::Blue),
         points_choice: PointChoice::Circle,
         points_num_segments: 12,
         approx_num_segments: 50,
         ..Default::default()
      },
   );

   run_json_svg("spartan_sizing_o1", &mut cairo_spartan);
}

// Symmetric range, warped.
//
// This does not need to be graphical, but instead should match numerically.  The polyline
// points should not move.
#[test]
fn spartan_sizing_o2_test() {
   let t_range = [-6.0, 14.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::Fill,
      canvas_size: [400.0, 300.0],
      axes_range: vec![-4.5, -2.5, 1.5, 2.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Box,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let mut managed_curve = ManagedRatQuad::new(
      &BaseRatQuad {
         a: [-21.0, 1.0, -2.0],
         b: [-3.1414, 4.7811, 6.5534],
         c: [0.0, 20.0, 0.0],
         r: t_range,
         ..Default::default()
      },
      cairo_spartan.spartan.prep.axes_range,
   );
   managed_curve.raise_to_symmetric_range().unwrap();
   managed_curve.raise_to_regularized_symmetric().unwrap();

   draw_sample_rat_quad(
      &managed_curve,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::Red),
         points_color: Some(ColorChoice::Blue),
         points_choice: PointChoice::Circle,
         points_num_segments: 12,
         approx_num_segments: 50,
         ..Default::default()
      },
   );

   run_json_svg("spartan_sizing_o2", &mut cairo_spartan);
}

// Symmetric range, warped.
//
// This does not need to be graphical, but instead should match numerically.  The polyline
// points should not move.
#[test]
fn spartan_sizing_p_test() {
   let t_range = [-6.0, 14.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::Fill,
      canvas_size: [400.0, 300.0],
      axes_range: vec![-4.5, -2.5, 1.5, 2.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Box,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let mut managed_curve = ManagedRatQuad::new(
      &BaseRatQuad {
         a: [-21.0, 1.0, -2.0],
         b: [-3.1414, 4.7811, 6.5534],
         c: [0.0, 20.0, 0.0],
         r: t_range,
         ..Default::default()
      },
      cairo_spartan.spartan.prep.axes_range,
   );

   draw_sample_rat_quad(
      &managed_curve,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::Green),
         points_color: Some(ColorChoice::Blue),
         points_choice: PointChoice::Circle,
         points_num_segments: 12,
         approx_num_segments: 30,
         ..Default::default()
      },
   );

   managed_curve.raise_to_symmetric_range().unwrap();
   managed_curve.raise_to_regularized_symmetric().unwrap();
   managed_curve.raise_to_offset_odd_even().unwrap();

   draw_sample_rat_quad(
      &managed_curve,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::BrightBlue),
         points_color: None,
         ..Default::default()
      },
   );

   run_json_svg("spartan_sizing_p", &mut cairo_spartan);
}

// Test transformations relevant especially to linear point arrangement.
#[test]
fn rat_quad_test() {
   let r: f64 = 1.5;
   let orig_quad = BaseRatQuad {
      a: [-21.0, 1.0, -2.0],
      b: [-3.1414, 4.7811, 6.5534],
      r: [r, r],
      ..Default::default()
   };

   let t_int: Vec<i32> = (0..12).collect();
   let mut t = Vec::<f64>::with_capacity(t_int.len());
   for i in 0..t_int.len() {
      t.push(t_int[i] as f64 / 3.0 - 2.0);
   }

   let a_1 = orig_quad.a[1];
   let a_s = r * r * orig_quad.a[2] + orig_quad.a[0];
   let a_d = r * r * orig_quad.a[2] - orig_quad.a[0];
   let sigma = ((a_s - a_1 * r) / (a_s + a_1 * r)).abs().sqrt();

   let mut unwarped_t = Vec::<f64>::with_capacity(t.len());
   for i in 0..t.len() {
      unwarped_t.push(
         r * ((sigma - 1.0) * r + (sigma + 1.0) * t[i])
            / ((sigma + 1.0) * r + (sigma - 1.0) * t[i]),
      );
   }

   let b_1 = orig_quad.b[1];
   let b_s = r * r * orig_quad.b[2] + orig_quad.b[0];
   let b_d = r * r * orig_quad.b[2] - orig_quad.b[0];

   let inter_quad = BaseRatQuad {
      a: [
         r * r
            * ((sigma * sigma + 1.0) * a_s + (sigma * sigma - 1.0) * a_1 * r - 2.0 * sigma * a_d),
         2.0 * r * ((sigma * sigma - 1.0) * a_s + (sigma * sigma + 1.0) * a_1 * r),
         ((sigma * sigma + 1.0) * a_s + (sigma * sigma - 1.0) * a_1 * r + 2.0 * sigma * a_d),
      ],
      b: [
         r * r
            * ((sigma * sigma + 1.0) * b_s + (sigma * sigma - 1.0) * b_1 * r - 2.0 * sigma * b_d),
         2.0 * r * ((sigma * sigma - 1.0) * b_s + (sigma * sigma + 1.0) * b_1 * r),
         ((sigma * sigma + 1.0) * b_s + (sigma * sigma - 1.0) * b_1 * r + 2.0 * sigma * b_d),
      ],
      r: [r, r],
      ..Default::default()
   };

   let t_gold = orig_quad.eval_quad(&unwarped_t);
   let t_inter = inter_quad.eval_quad(&t);

   for i in 0..t_gold.len() {
      assert!((t_gold[i][0] - t_inter[i][0]).abs() < 0.0001);
   }

   assert!((0.5 * (sigma * sigma + 1.0) * (a_s + a_1 * r) - a_s).abs() < 0.0001);
   assert!((0.5 * (sigma * sigma - 1.0) * (a_s + a_1 * r) + a_1 * r).abs() < 0.0001);
   assert!((a_s * a_s - a_1 * a_1 * r * r) >= 0.0);
   let lambda = (a_s * a_s - a_1 * a_1 * r * r).sqrt() * (a_s + a_1 * r).signum();
   assert!((lambda - sigma * (a_s + a_1 * r)).abs() < 0.0001);

   let final_quad = BaseRatQuad {
      a: [r * r * lambda * (lambda - a_d), 0.0, lambda * (lambda + a_d)],
      b: [
         r * r * (a_s * b_s - a_1 * b_1 * r * r - lambda * b_d),
         2.0 * r * r * (a_s * b_1 - a_1 * b_s),
         (a_s * b_s - a_1 * b_1 * r * r + lambda * b_d),
      ],
      r: [r, r],
      ..Default::default()
   };

   let t_gold = orig_quad.eval_quad(&unwarped_t);
   let t_final = final_quad.eval_quad(&t);

   for i in 0..t_gold.len() {
      assert!((t_gold[i][0] - t_final[i][0]).abs() < 0.0001);
   }
}

// Symmetric range, warped.
//
// This does not need to be graphical, but instead should match numerically.  The polyline
// points should not move.
#[test]
fn spartan_sizing_q_test() {
   let t_range = [-3.0, 9.0];
   let sigma = 3.0;

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::Fill,
      canvas_size: [400.0, 300.0],
      axes_range: vec![-1.5, -2.5, 4.5, 2.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Box,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         axis_numbering: AxisNumbering::None,
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let managed_curve_a = ManagedCubic::create_from_control_points(
      &CubiLinear {
         r: t_range,
         x: [0.0, -0.5, 0.5, -1.0],
         y: [-1.5, -2.0, 1.5, 2.0],
         sigma: 1.0,
         ..Default::default()
      },
      cairo_spartan.spartan.prep.axes_range,
   );
   draw_sample_cubilinear(
      &managed_curve_a,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::Green),
         control_color: Some(ColorChoice::YellowBrown),
         points_color: None,
         ..Default::default()
      },
   );

   let mut managed_curve_b = managed_curve_a.clone();
   managed_curve_b.displace([2.0, 0.0]);
   managed_curve_b.bilinear_transform(sigma);
   draw_sample_cubilinear(
      &managed_curve_b,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::BrightBlue),
         points_color: Some(ColorChoice::Blue),
         points_num_segments: 12,
         ..Default::default()
      },
   );

   let mut managed_curve_d = managed_curve_b.clone();
   managed_curve_d.select_range([t_range[0] + 0.5, t_range[0] + 5.5]);
   draw_sample_cubilinear(
      &managed_curve_d,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::Green),
         points_color: Some(ColorChoice::Green),
         control_color: Some(ColorChoice::YellowBrown),
         points_choice: PointChoice::Circle,
         points_num_segments: 5,
         ..Default::default()
      },
   );

   let mut managed_curve_c = managed_curve_a.clone();
   managed_curve_c.displace([4.0, 0.0]);
   managed_curve_c.bilinear_transform(sigma);
   managed_curve_c.adjust_range([t_range[0] - 1.5, t_range[1] + 4.5]);
   draw_sample_cubilinear(
      &managed_curve_c,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::BrightBlue),
         points_color: Some(ColorChoice::Blue),
         points_num_segments: 12,
         ..Default::default()
      },
   );

   let mut managed_curve_e = managed_curve_c.clone();
   managed_curve_e.select_range([t_range[0] - 1.5 + 1.5 * 4.0, t_range[0] - 1.5 + 1.5 * 10.0]);
   draw_sample_cubilinear(
      &managed_curve_e,
      &mut cairo_spartan.spartan,
      &SampleCurveConfig {
         main_color: Some(ColorChoice::Green),
         points_color: Some(ColorChoice::Green),
         control_color: Some(ColorChoice::YellowBrown),
         points_choice: PointChoice::Circle,
         points_num_segments: 6,
         ..Default::default()
      },
   );

   run_json_svg("spartan_sizing_q", &mut cairo_spartan);
}
