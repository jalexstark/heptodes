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
use diag_golden::CairoSpartanRender;
use diag_golden::JsonGoldenTest;
use diag_golden::SizingScheme;
use diag_golden::SpartanDiagram;
use diag_golden::SvgGoldenTest;
use serde_json::to_string_pretty;
use std::f64::consts::PI;
use std::io::Write;

// After dependency to test_utils is added, use type-def for the result box.
fn write_full_sample_to_write<W: Write + 'static>(
   out_stream: W,
   cairo_spartan: &mut CairoSpartanRender,
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

   cairo_spartan.save_set_path_transform(&context);
   context.move_to(-0.1 + 0.12, -0.25);
   context.arc(-0.1, -0.25, 0.12, 0.0 * PI, 2.0 * PI);
   cairo_spartan.restore_transform(&context);
   context.stroke().unwrap();

   cairo_spartan.save_set_path_transform(&context);
   context.move_to(0.0, 0.0);
   context.line_to(0.5, 0.3);
   context.move_to(0.0, 0.5);
   context.line_to(0.45, 0.0);
   cairo_spartan.restore_transform(&context);
   context.stroke().unwrap();
   cairo_spartan.save_set_path_transform(&context);
   context.move_to(0.45, 0.0);
   context.set_line_width(0.45);
   context.set_dash(&[4.5, 3.5], 0.0);
   context.line_to(-0.4, -0.35);
   cairo_spartan.restore_transform(&context);
   context.stroke().unwrap();
   context.set_line_width(1.0);
   context.set_dash(&[], 0.0);

   // Draw a point-like circle.
   cairo_spartan.save_set_path_transform(&context);
   let (cx, cy) = context.user_to_device(0.2, -0.7);
   cairo_spartan.restore_transform(&context);
   context.move_to(cx + 2.4, cy);
   context.arc(cx, cy, 2.4, 0.0 * PI, 2.0 * PI);
   context.close_path();
   context.stroke().unwrap();

   // Draw a plus-like symbol.
   cairo_spartan.save_set_path_transform(&context);
   let (cx, cy) = context.user_to_device(0.25, -0.7);
   cairo_spartan.restore_transform(&context);
   let plus_delta = 2.4 * 1.32;
   context.move_to(cx, cy - plus_delta);
   context.line_to(cx, cy + plus_delta);
   context.move_to(cx + plus_delta, cy);
   context.line_to(cx - plus_delta, cy);
   context.close_path();
   context.stroke().unwrap();

   // Draw a times-like symbol.
   cairo_spartan.save_set_path_transform(&context);
   let (cx, cy) = context.user_to_device(0.15, -0.7);
   cairo_spartan.restore_transform(&context);
   let times_delta = 2.4 * 1.32 * (0.5 as f64).sqrt();
   context.move_to(cx - times_delta, cy - times_delta);
   context.line_to(cx + times_delta, cy + times_delta);
   context.move_to(cx + times_delta, cy - times_delta);
   context.line_to(cx - times_delta, cy + times_delta);
   context.close_path();
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

   cairo_spartan.save_set_path_transform(&context);
   context.move_to(0.3, -0.2);
   cairo_spartan.restore_transform(&context);
   pangocairo::show_layout(&context, &text_layout);

   surface.flush();
   surface.finish_output_stream()
}

#[test]
fn simple_full_spartan_test() {
   let mut cairo_spartan = CairoSpartanRender::new();
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

struct TestSizing {
   sizing_scheme: SizingScheme,
   canvas_size: [f64; 2],
   axes_range: Vec<f64>,
   padding: Vec<f64>,
   debug_box: [f64; 4],
}

// After dependency to test_utils is added, use type-def for the result box.
fn write_sample_to_write<W: Write + 'static>(
   out_stream: W,
   cairo_spartan: &mut CairoSpartanRender,
   sizing: &TestSizing,
) -> Result<Box<dyn core::any::Any>, cairo::StreamWithError> {
   assert!(cairo_spartan.spartan.is_ready());
   cairo_spartan.spartan.sizing_scheme = sizing.sizing_scheme;
   cairo_spartan.spartan.base_width = sizing.canvas_size[0];
   cairo_spartan.spartan.base_height = sizing.canvas_size[1];
   cairo_spartan.spartan.axes_range = sizing.axes_range.clone();
   cairo_spartan.spartan.padding = sizing.padding.clone();

   let mut surface = SvgSurface::for_stream(
      cairo_spartan.spartan.prep.canvas_size[0],
      cairo_spartan.spartan.prep.canvas_size[1],
      out_stream,
   )
   .unwrap();
   surface.set_document_unit(Pt);

   let context = cairo::Context::new(&surface).unwrap();
   context.set_line_width(1.0);
   // 1.0 is a regular thickness, definitely not thick, 2.0 definitely thick, 0.6 thin but
   // firm.

   cairo_spartan.save_set_path_transform(&context);
   context.move_to(sizing.debug_box[0], sizing.debug_box[1]);
   context.line_to(sizing.debug_box[0], sizing.debug_box[3]);
   context.move_to(sizing.debug_box[2], sizing.debug_box[1]);
   context.line_to(sizing.debug_box[2], sizing.debug_box[3]);
   cairo_spartan.restore_transform(&context);
   context.stroke().unwrap();

   cairo_spartan.save_set_path_transform(&context);
   context.set_line_width(0.45);
   context.set_dash(&[4.5, 3.5], 0.0);
   context.move_to(sizing.debug_box[0], sizing.debug_box[3]);
   context.line_to(sizing.debug_box[2], sizing.debug_box[1]);
   context.move_to(sizing.debug_box[2], sizing.debug_box[3]);
   context.line_to(sizing.debug_box[0], sizing.debug_box[1]);
   cairo_spartan.restore_transform(&context);
   context.stroke().unwrap();
   context.set_line_width(1.0);
   context.set_dash(&[], 0.0);

   surface.flush();
   surface.finish_output_stream()
}

fn spartan_sizing(filestem: &str, sizing: &TestSizing) {
   let mut cairo_spartan = CairoSpartanRender::new();
   cairo_spartan.spartan.sizing_scheme = sizing.sizing_scheme;
   cairo_spartan.spartan.base_width = sizing.canvas_size[0];
   cairo_spartan.spartan.base_height = sizing.canvas_size[1];
   cairo_spartan.spartan.axes_range = sizing.axes_range.clone();
   cairo_spartan.spartan.padding = sizing.padding.clone();
   cairo_spartan.spartan.prepare();

   {
      let mut json_golden = JsonGoldenTest::new("tests/goldenfiles/", filestem);
      let serialized = to_string_pretty::<SpartanDiagram>(&cairo_spartan.spartan).unwrap();
      json_golden.writeln_as_bytes(&serialized);
   }

   {
      let svg_golden = SvgGoldenTest::new("tests/goldenfiles/", filestem);
      let raw_result =
         write_sample_to_write(svg_golden.get_raw_writeable(), &mut cairo_spartan, sizing);
      svg_golden.handover_result(raw_result.unwrap());
   }
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
   };
   spartan_sizing("spartan_sizing_b", &sizing);
}

#[test]
fn spartan_sizing_c_test() {
   // range (-2.0, 2.0), mixed padding.
   let r = 2.0;
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareShrink,
      canvas_size: [500.0, 200.0],
      axes_range: vec![r],
      padding: vec![],
      debug_box: [-r, -r, r, r],
   };
   spartan_sizing("spartan_sizing_c", &sizing);
}

#[test]
fn spartan_sizing_d_test() {
   // range (-2.0, 2.0), mixed padding.
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareShrink,
      canvas_size: [300.0, 450.0],
      axes_range: vec![-2.0, -1.5, 2.0, 1.5],
      padding: vec![],
      debug_box: [-2.0, -1.5, 2.0, 1.5],
   };
   spartan_sizing("spartan_sizing_d", &sizing);
}

#[test]
fn spartan_sizing_e_test() {
   // range (-2.0, 2.0), mixed padding.
   let r = 2.0;
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [500.0, 200.0],
      axes_range: vec![r],
      padding: vec![],
      debug_box: [-r, -r, r, r],
   };
   spartan_sizing("spartan_sizing_e", &sizing);
}

#[test]
fn spartan_sizing_f_test() {
   // range (-2.0, 2.0), mixed padding.
   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [300.0, 450.0],
      axes_range: vec![-2.0, -1.5, 2.0, 1.5],
      padding: vec![],
      debug_box: [-2.0, -1.5, 2.0, 1.5],
   };
   spartan_sizing("spartan_sizing_f", &sizing);
}

struct RatQuad {
   a: [f64; 3],
   b: [f64; 3],
}

impl RatQuad {
   fn eval_quad(&self, t: &Vec<f64>) -> Vec<f64> {
      let mut ret_val = Vec::<f64>::with_capacity(t.len());
      for i in 0..t.len() {
         ret_val.push(
            ((self.b[2] * t[i] + self.b[1]) * t[i] + self.b[0])
               / ((self.a[2] * t[i] + self.a[1]) * t[i] + self.a[0]),
         );
      }
      return ret_val;
   }
}

#[test]
fn rat_quad_test() {
   let r: f64 = 1.5;
   let orig_quad = RatQuad { a: [-21.0, 1.0, -2.0], b: [-3.1414, 4.7811, 6.5534] };

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

   let inter_quad = RatQuad {
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
   };

   let t_gold = orig_quad.eval_quad(&unwarped_t);
   let t_inter = inter_quad.eval_quad(&t);

   for _i in 0..t_gold.len() {
      assert!((t_gold[1] - t_inter[1]).abs() < 0.0001);
   }

   assert!((0.5 * (sigma * sigma + 1.0) * (a_s + a_1 * r) - a_s).abs() < 0.0001);
   assert!((0.5 * (sigma * sigma - 1.0) * (a_s + a_1 * r) + a_1 * r).abs() < 0.0001);
   assert!((a_s * a_s - a_1 * a_1 * r * r) >= 0.0);
   let lambda = (a_s * a_s - a_1 * a_1 * r * r).sqrt() * (a_s + a_1 * r).signum();
   assert!((lambda - sigma * (a_s + a_1 * r)).abs() < 0.0001);

   let final_quad = RatQuad {
      a: [r * r * lambda * (lambda - a_d), 0.0, lambda * (lambda + a_d)],
      b: [
         r * r * (a_s * b_s - a_1 * b_1 * r * r - lambda * b_d),
         2.0 * r * r * (a_s * b_1 - a_1 * b_s),
         (a_s * b_s - a_1 * b_1 * r * r + lambda * b_d),
      ],
   };

   let t_gold = orig_quad.eval_quad(&unwarped_t);
   let t_final = final_quad.eval_quad(&t);

   for _i in 0..t_gold.len() {
      assert!((t_gold[1] - t_final[1]).abs() < 0.0001);
   }
}
