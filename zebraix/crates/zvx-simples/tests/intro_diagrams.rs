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

#[cfg(test)]
mod tests {
   use zvx_base::CubicPath;
   use zvx_curves::{Curve, ManagedCubic};
   use zvx_docagram::{AxesSpec, AxesStyle, AxisNumbering, SizingScheme};
   use zvx_drawable::ColorChoice;
   use zvx_simples::exemplary::tests::{
      add_p_4, build_from_sizing, mul_add_p, p_from_x_y_4, render_and_check, scale_p_4,
      scale_p_4_2, BackgroundBox, TestSizing,
   };
   use zvx_simples::generate::{add_centered_text, draw_sample_cubilinear, SampleCurveConfig};

   const CUBIC_BASIC: [[f64; 2]; 4] =
      p_from_x_y_4(&[-2.5, -1.75, 2.5, 2.5], &[-2.0, -0.75, 0.5, -2.0]);
   // p_from_x_y_4(&[-2.0, -1.0, 2.0, 2.0], &[-2.0, -0.5, 0.5, -2.0]);
   // p_from_x_y_4(&[-3.0, -2.0, 3.0, 3.0], &[-2.0, -0.5, 0.5, -2.0]);
   // p_from_x_y_4(&[-3.0, -1.5, 3.0, 3.0], &[-2.0, 0.25, 0.5, -2.0]);
   // &p_from_x_y_4(&[-3.0, -1.5, 3.0, 3.0], &[-2.0, 0.5, 0.75, -2.0])
   // p_from_x_y_4(&[-3.0, -1.5, 3.0, 3.0], &[-2.0, -0.0, 1.0, -2.0]),
   // p: p_from_x_y_4(&[-3.0, -2.25, 3.0, 3.0], &[-2.0, -0.5, 2.5, -2.0]),

   fn make_cubic_sizing() -> TestSizing {
      TestSizing {
         sizing_scheme: SizingScheme::SquareCenter,
         canvas_size: [350.0, 350.0],
         axes_range: vec![-4.0, -4.0, 4.0, 4.0],
         padding: vec![0.05],
         axes_spec: AxesSpec {
            axis_numbering: AxisNumbering::None,
            axes_style: AxesStyle::Boxed,
            grid_interval: [1.0, 1.0],
            // axes_style: AxesStyle::None,
            // grid_interval: [0.0, 0.0],
            grid_precision: vec![1],
            ..Default::default()
         },
         background_box: BackgroundBox::Shrink,
         ..Default::default()
      }
   }

   #[test]
   fn cubic_controlled_test() {
      let t_range = [-6.0, 14.0];
      let sizing = make_cubic_sizing();
      let mut runner = build_from_sizing("curves/figs-intro/intro_cubic_controlled", &sizing);
      let drawable_diagram = &mut runner.combo.drawable_diagram;

      let main_curve = add_p_4(&scale_p_4(&CUBIC_BASIC, 1.2), [0.0, 1.25]);
      let managed_curve_a = ManagedCubic::create_from_control_points(
         &Curve::<CubicPath> { path: CubicPath { r: t_range, p: main_curve }, sigma: 1.0 },
         drawable_diagram.prep.axes_range,
      );
      draw_sample_cubilinear(
         &managed_curve_a,
         drawable_diagram,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            control_color: Some(ColorChoice::RedRedGreen),
            points_color: None,
            ..Default::default()
         },
      );
      add_centered_text(
         drawable_diagram,
         &SampleCurveConfig { main_color: Some(ColorChoice::BlueBlueRed), ..Default::default() },
         "Intermediate control points",
         [0.5, 1.5],
      );
      add_centered_text(
         drawable_diagram,
         &SampleCurveConfig { main_color: Some(ColorChoice::BlueBlueRed), ..Default::default() },
         "p<sub>1</sub>",
         mul_add_p(&main_curve[1], [1.0, 1.0], [0.0, 0.5]),
      );
      add_centered_text(
         drawable_diagram,
         &SampleCurveConfig { main_color: Some(ColorChoice::BlueBlueRed), ..Default::default() },
         "p<sub>2</sub>",
         mul_add_p(&main_curve[2], [1.0, 1.0], [0.0, 0.5]),
      );
      add_centered_text(
         drawable_diagram,
         &SampleCurveConfig { main_color: Some(ColorChoice::RedRedBlue), ..Default::default() },
         "End control points",
         [0.0, -2.0],
      );
      add_centered_text(
         drawable_diagram,
         &SampleCurveConfig { main_color: Some(ColorChoice::RedRedBlue), ..Default::default() },
         "p<sub>0</sub>",
         mul_add_p(&main_curve[0], [1.0, 1.0], [0.0, -0.5]),
      );
      add_centered_text(
         drawable_diagram,
         &SampleCurveConfig { main_color: Some(ColorChoice::RedRedBlue), ..Default::default() },
         "p<sub>3</sub>",
         mul_add_p(&main_curve[3], [1.0, 1.0], [0.0, -0.5]),
      );

      render_and_check(&mut runner);
   }

   #[test]
   fn cubic_slider_test() {
      let t_range = [-6.0, 14.0];
      let sizing = make_cubic_sizing();
      let mut runner = build_from_sizing("curves/figs-intro/intro_cubic_slider", &sizing);
      let drawable_diagram = &mut runner.combo.drawable_diagram;

      let managed_curve_a = ManagedCubic::create_from_control_points(
         &Curve::<CubicPath> {
            path: CubicPath { r: t_range, p: add_p_4(&scale_p_4(&CUBIC_BASIC, 0.9), [0.0, -1.2]) },
            sigma: 1.0,
         },
         drawable_diagram.prep.axes_range,
      );
      draw_sample_cubilinear(
         &managed_curve_a,
         drawable_diagram,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            control_color: Some(ColorChoice::RedRedGreen),
            points_color: Some(ColorChoice::Blue),
            ..Default::default()
         },
      );

      let managed_curve_b = ManagedCubic::create_from_control_points(
         &Curve::<CubicPath> {
            path: CubicPath {
               r: t_range,
               p: add_p_4(
                  &scale_p_4_2(
                     &p_from_x_y_4(&[0.0, 1.0, 2.0, 3.0], &[0.0, 1.0, 2.0, 3.0]),
                     [0.0, 0.9 * 2.5],
                  ),
                  [CUBIC_BASIC[3][0] * 0.9 + 0.9, -3.0], // -1.2 - 1.8 = -3.0.
               ),
            },
            sigma: 1.0,
         },
         drawable_diagram.prep.axes_range,
      );
      draw_sample_cubilinear(
         &managed_curve_b,
         drawable_diagram,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            control_color: Some(ColorChoice::RedRedGreen),
            points_color: Some(ColorChoice::Blue),
            ..Default::default()
         },
      );

      let norm = 1.0 / (0.75 * 0.75 + 1.25 * 1.25f64).sqrt();
      let managed_curve_c = ManagedCubic::create_from_control_points(
         &Curve::<CubicPath> {
            path: CubicPath {
               r: t_range,
               p: add_p_4(
                  &scale_p_4_2(
                     &p_from_x_y_4(&[0.0, 1.0, 2.0, 3.0], &[0.0, 1.0, 2.0, 3.0]),
                     [0.9 * 0.75, 0.9 * 1.25],
                  ),
                  [CUBIC_BASIC[0][0] * 0.9 - 1.25 * norm * 0.9, -3.0 + 0.75 * norm * 0.9],
               ),
            },
            sigma: 1.0,
         },
         drawable_diagram.prep.axes_range,
      );
      draw_sample_cubilinear(
         &managed_curve_c,
         drawable_diagram,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            control_color: Some(ColorChoice::RedRedGreen),
            points_color: Some(ColorChoice::Blue),
            ..Default::default()
         },
      );

      let mid_point = [
         0.125 * CUBIC_BASIC[0][0]
            + 0.375 * CUBIC_BASIC[1][0]
            + 0.375 * CUBIC_BASIC[2][0]
            + 0.125 * CUBIC_BASIC[3][0],
         0.125 * CUBIC_BASIC[0][1]
            + 0.375 * CUBIC_BASIC[1][1]
            + 0.375 * CUBIC_BASIC[2][1]
            + 0.125 * CUBIC_BASIC[3][1],
      ];
      let mid_diff = [
         -0.5 * CUBIC_BASIC[0][0] - 0.5 * CUBIC_BASIC[1][0]
            + 0.5 * CUBIC_BASIC[2][0]
            + 0.5 * CUBIC_BASIC[3][0],
         -0.5 * CUBIC_BASIC[0][1] - 0.5 * CUBIC_BASIC[1][1]
            + 0.5 * CUBIC_BASIC[2][1]
            + 0.5 * CUBIC_BASIC[3][1],
      ];
      let norm = 1.0 / (mid_diff[0] * mid_diff[0] + mid_diff[1] * mid_diff[1]).sqrt();

      let managed_curve_d = ManagedCubic::create_from_control_points(
         &Curve::<CubicPath> {
            path: CubicPath {
               r: t_range,
               p: add_p_4(
                  &scale_p_4_2(
                     &p_from_x_y_4(&[-1.5, -0.5, 0.5, 1.5], &[-1.5, -0.5, 0.5, 1.5]),
                     [0.5 * 0.9 * mid_diff[0], 0.5 * 0.9 * mid_diff[1]],
                  ),
                  [
                     0.9 * mid_point[0] - norm * mid_diff[1],
                     0.9 * mid_point[1] + norm * mid_diff[0] - 1.2,
                  ],
               ),
            },
            sigma: 1.0,
         },
         drawable_diagram.prep.axes_range,
      );
      draw_sample_cubilinear(
         &managed_curve_d,
         drawable_diagram,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            control_color: Some(ColorChoice::RedRedGreen),
            points_color: Some(ColorChoice::Blue),
            ..Default::default()
         },
      );

      render_and_check(&mut runner);
   }
}
