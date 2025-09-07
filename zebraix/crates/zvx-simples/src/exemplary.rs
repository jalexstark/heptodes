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

// Rust does not apply cfg(test) during cargo test.
// #[cfg(test)]
pub mod tests {
   use serde_json::to_writer_pretty;
   use std::io::Write;
   use zvx_cairo::CairoSpartanCombo;
   use zvx_docagram::diagram::{SpartanDiagram, SpartanPreparation};
   use zvx_docagram::{AxesSpec, SizingScheme};
   use zvx_drawable::{ColorChoice, FillChoices, LineChoice, PathChoices};
   use zvx_golden::filtered::JsonGoldenTest;
   use zvx_golden::filtered::SvgGoldenTest;

   #[must_use]
   pub fn scale_coord_vec(v: &[[f64; 2]], s: f64) -> Vec<[f64; 2]> {
      let mut result = v.to_owned();
      for i in 0..v.len() {
         result[i] = [v[i][0] * s, v[i][1] * s];
      }
      result
   }

   #[must_use]
   pub const fn p_from_x_y_4(x: &[f64; 4], y: &[f64; 4]) -> [[f64; 2]; 4] {
      [[x[0], y[0]], [x[1], y[1]], [x[2], y[2]], [x[3], y[3]]]
   }

   #[must_use]
   pub const fn p_from_x_y_3(x: &[f64; 3], y: &[f64; 3]) -> [[f64; 2]; 3] {
      [[x[0], y[0]], [x[1], y[1]], [x[2], y[2]]]
   }

   #[derive(Default)]
   pub enum BackgroundBox {
      #[default]
      Nothing,
      Shrink,
   }

   #[derive(Default)]
   pub struct TestSizing {
      pub sizing_scheme: SizingScheme,
      pub canvas_size: [f64; 2],
      pub axes_range: Vec<f64>,
      pub padding: Vec<f64>,
      pub debug_box: [f64; 4],
      pub axes_spec: AxesSpec,
      pub background_box: BackgroundBox,
   }

   #[must_use]
   pub fn create_sized_diagram(sizing: &TestSizing) -> SpartanDiagram {
      let mut spartan = SpartanDiagram {
         sizing_scheme: sizing.sizing_scheme,
         canvas_size: (sizing.canvas_size[0], sizing.canvas_size[1]),
         axes_range: sizing.axes_range.clone(),
         padding: sizing.padding.clone(),
         ..Default::default()
      };

      // Only shrink (currently) supported in axes generation.
      spartan.background_box = match sizing.background_box {
         BackgroundBox::Nothing => None,
         BackgroundBox::Shrink => Some(PathChoices {
            line_choice: LineChoice::Ordinary,
            color: spartan.base_color_choice.clone(),
            fill_choices: FillChoices { color: ColorChoice::ZvxBackground, opacity: 1.0 },
         }),
      };

      spartan
   }

   pub struct JsonSvgRunner {
      filestem: String,
      svg_golden: SvgGoldenTest,
      pub combo: CairoSpartanCombo,
      raw_result: Option<Box<dyn core::any::Any>>,
   }

   impl JsonSvgRunner {
      #[must_use]
      pub fn new(filestem: &str, preparation: &SpartanPreparation) -> Self {
         let svg_golden = SvgGoldenTest::new("tests/goldenfiles/", filestem);
         let combo =
            CairoSpartanCombo::create_for_stream(svg_golden.get_raw_writeable(), preparation);

         Self { filestem: filestem.to_string(), svg_golden, combo, raw_result: None }
      }

      fn render(&mut self) {
         self.raw_result = Some(self.combo.render_diagram().unwrap());
      }

      fn check_svg_and_json(&mut self) {
         let raw_result = self.raw_result.take();
         self
            .svg_golden
            .handover_result(raw_result.expect("No output generated in SVG test runner."));

         //

         let mut json_golden = JsonGoldenTest::new("tests/goldenfiles/", &self.filestem);
         // This only really fails if keys cannot be rendered.
         //
         // Consider moving into golden test crate. This is only trigger for serde_json dependency.
         to_writer_pretty(&json_golden.out_stream, &self.combo.drawable_diagram).unwrap();
         let bytes_amount_nl = json_golden.out_stream.write(b"\n").unwrap();
         assert!(bytes_amount_nl == 1);
      }
   }

   impl Drop for JsonSvgRunner {
      fn drop(&mut self) {
         // Improve error reporting.  A check method should have been run to clear the drawing
         // result.
         assert!(self.raw_result.is_none());
      }
   }

   #[must_use]
   pub fn build_from_sizing(filestem: &str, sizing: &TestSizing) -> JsonSvgRunner {
      let spartan = create_sized_diagram(sizing);
      let preparation = spartan.prepare();

      let mut runner = JsonSvgRunner::new(filestem, &preparation);
      sizing.axes_spec.generate_axes(&mut runner.combo.drawable_diagram);

      runner
   }

   pub fn render_and_check(runner: &mut JsonSvgRunner) {
      runner.render();
      runner.check_svg_and_json();
   }
}
