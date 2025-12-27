// Copyright 2026 Google LLC
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
   use zvx_base::{CubicFourPoint, CubicHomog, OneOfSegment};
   use zvx_curves::{FourPointRatQuad, ManagedCubic, ManagedRatQuad};
   use zvx_docagram::{AxesSpec, AxesStyle, AxisNumbering, SizingScheme};
   use zvx_drawable::{
      ColorChoice, FillChoices, LineChoice, LinesSetSet, OneOfDrawable, PathChoices,
      PathCompletion, QualifiedDrawable, SegmentSequence, Strokeable,
   };
   use zvx_simples::exemplary::tests::{
      add_p_4, build_from_sizing, mul_add_p, mul_add_p_4, p_from_x_y_4, render_and_check,
      scale_p_4, scale_p_4_2, x_y_from_p_4, BackgroundBox, TestSizing,
   };
   use zvx_simples::generate::{
      add_centered_text, draw_sample_cubilinear, draw_sample_rat_quad, SampleCurveConfig,
   };

   const CUBIC_BASIC: [[f64; 2]; 4] =
      p_from_x_y_4(&[-2.5, -1.75, 2.5, 2.5], &[-2.0, -0.75, 0.5, -2.0]);

   fn make_cubic_sizing(base_size: [f64; 2]) -> TestSizing {
      TestSizing {
         sizing_scheme: SizingScheme::SquareCenter,
         canvas_size: mul_add_p(&base_size, [87.5, 87.5], [0.0, 0.0]),
         axes_range: vec![-base_size[0], -base_size[1], base_size[0], base_size[1]],
         padding: vec![0.05],
         axes_spec: AxesSpec {
            axis_numbering: AxisNumbering::None,
            // axes_style: AxesStyle::Boxed,
            // grid_interval: [1.0, 1.0],
            axes_style: AxesStyle::None,
            grid_interval: [0.0, 0.0],
            grid_precision: vec![1],
            // ..Default::default()
         },
         background_box: BackgroundBox::Shrink,
         ..Default::default()
      }
   }

   #[test]
   fn cubic_controlled_test() {
      let t_range = [-6.0, 14.0];
      let sizing = make_cubic_sizing([4.0, 3.0]);
      let mut runner = build_from_sizing("curves/figs-intro/intro_cubic_controlled", &sizing);
      let drawable_diagram = &mut runner.combo.drawable_diagram;

      let main_curve = add_p_4(&scale_p_4(&CUBIC_BASIC, 1.2), [0.0, 1.25]);
      let managed_curve_a = ManagedCubic::create_from_control_points(
         &CubicFourPoint {
            r: t_range,
            h: CubicHomog(x_y_from_p_4(&main_curve)),
            sigma: (1.0, 1.0),
         },
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
         [0.0, 1.75],
      );
      add_centered_text(
         drawable_diagram,
         &SampleCurveConfig { main_color: Some(ColorChoice::BlueBlueRed), ..Default::default() },
         "P<sub>1</sub>",
         mul_add_p(&main_curve[1], [1.0, 1.0], [0.0, 0.5]),
      );
      add_centered_text(
         drawable_diagram,
         &SampleCurveConfig { main_color: Some(ColorChoice::BlueBlueRed), ..Default::default() },
         "P<sub>2</sub>",
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
         "P<sub>0</sub>",
         mul_add_p(&main_curve[0], [1.0, 1.0], [0.0, -0.5]),
      );
      add_centered_text(
         drawable_diagram,
         &SampleCurveConfig { main_color: Some(ColorChoice::RedRedBlue), ..Default::default() },
         "P<sub>3</sub>",
         mul_add_p(&main_curve[3], [1.0, 1.0], [0.0, -0.5]),
      );

      render_and_check(&mut runner);
   }

   #[test]
   fn cubic_velocity_test() {
      let t_range = [-6.0, 14.0];
      let sizing = make_cubic_sizing([3.5, 3.5]);
      let mut runner = build_from_sizing("curves/figs-intro/intro_cubic_velocity", &sizing);
      let drawable_diagram = &mut runner.combo.drawable_diagram;

      let s = 0.85;

      let managed_curve_a = ManagedCubic::create_from_control_points(
         &CubicFourPoint {
            r: t_range,
            h: CubicHomog(x_y_from_p_4(&add_p_4(&scale_p_4(&CUBIC_BASIC, s), [0.0, -1.2]))),
            sigma: (1.0, 1.0),
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
         &CubicFourPoint {
            r: t_range,
            h: CubicHomog(x_y_from_p_4(&add_p_4(
               &scale_p_4_2(
                  &p_from_x_y_4(&[0.0, 1.0, 2.0, 3.0], &[0.0, 1.0, 2.0, 3.0]),
                  [0.0, s * 2.5],
               ),
               [CUBIC_BASIC[3][0] * s + s, -3.0], // -1.2 - 1.8 = -3.0.
            ))),
            sigma: (1.0, 1.0),
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

      let norm = 1.0 / 0.75_f64.hypot(1.25);
      let managed_curve_c = ManagedCubic::create_from_control_points(
         &CubicFourPoint {
            r: t_range,
            h: CubicHomog(x_y_from_p_4(&add_p_4(
               &scale_p_4_2(
                  &p_from_x_y_4(&[0.0, 1.0, 2.0, 3.0], &[0.0, 1.0, 2.0, 3.0]),
                  [s * 0.75, s * 1.25],
               ),
               [CUBIC_BASIC[0][0] * s - 1.25 * norm * s, -3.0 + 0.75 * norm * s],
            ))),
            sigma: (1.0, 1.0),
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
      let norm = 1.0 / mid_diff[0].hypot(mid_diff[1]);

      let managed_curve_d = ManagedCubic::create_from_control_points(
         &CubicFourPoint {
            r: t_range,
            h: CubicHomog(x_y_from_p_4(&add_p_4(
               &scale_p_4_2(
                  &p_from_x_y_4(&[-1.5, -0.5, 0.5, 1.5], &[-1.5, -0.5, 0.5, 1.5]),
                  [0.5 * s * mid_diff[0], 0.5 * s * mid_diff[1]],
               ),
               [s * mid_point[0] - norm * mid_diff[1], s * mid_point[1] + norm * mid_diff[0] - 1.2],
            ))),
            sigma: (1.0, 1.0),
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

   #[test]
   #[allow(clippy::needless_update)]
   fn cubic_slider_test() {
      let t_range = [-6.0, 14.0];
      let sizing = make_cubic_sizing([3.5, 3.5]);
      let mut runner = build_from_sizing("curves/figs-intro/intro_cubic_slider", &sizing);
      let drawable_diagram = &mut runner.combo.drawable_diagram;

      let s = 1.0;
      let d = -0.6;
      {
         let layer_slot = -20;

         let half_width = 0.05;
         let slider_pos = 3.9 / 12.0;
         let hw = half_width * 6.5; // Half-width of slider.
         let hh = half_width * 7.5; // Half-height of slider.
         let tick_offset = half_width * 4.0;
         let tick_hw = half_width * 1.6;
         let slider_mid = 0.28 * s + d;

         let slot_rect = mul_add_p_4(
            &[[-half_width, -3.0], [half_width, -3.0], [half_width, 3.0], [-half_width, 3.0]],
            [s, s * (3.0 + 2.0 * half_width) / 3.0],
            [slider_mid, 0.0],
         );

         drawable_diagram.drawables.push(QualifiedDrawable {
            layer: layer_slot,
            drawable: OneOfDrawable::SegmentSequence(SegmentSequence {
               path_choices: PathChoices {
                  line_choice: LineChoice::Ordinary,
                  color: ColorChoice::Black,
                  ..Default::default()
               },
               completion: PathCompletion::Closed,
               segments: vec![OneOfSegment::Polyline(slot_rect.to_vec())],
            }),
         });

         let c = [0.0, 6.0 * slider_pos - 3.0];
         let slider_rect = mul_add_p_4(
            &[
               [c[0] - hw, c[1] - hh],
               [c[0] + hw, c[1] - hh],
               [c[0] + hw, c[1] + hh],
               [c[0] - hw, c[1] + hh],
            ],
            [s, s],
            [slider_mid, 0.0],
         );
         drawable_diagram.drawables.push(QualifiedDrawable {
            layer: layer_slot + 1,
            drawable: OneOfDrawable::SegmentSequence(SegmentSequence {
               path_choices: PathChoices {
                  color: ColorChoice::Green,
                  fill_choices: FillChoices { color: ColorChoice::ZvxBackground, opacity: 1.0 },
                  ..Default::default()
               },
               completion: PathCompletion::Closed,
               segments: vec![OneOfSegment::Polyline(slider_rect.to_vec())],
            }),
         });

         let thumb_vec = vec![
            ([-s * hw * 0.65 + slider_mid, s * c[1]], [s * hw * 0.65 + slider_mid, s * c[1]]),
            (
               [-s * hw * 0.35 + slider_mid, s * c[1] - s * hh * 0.21],
               [s * hw * 0.35 + slider_mid, s * c[1] - s * hh * 0.21],
            ),
            (
               [-s * hw * 0.35 + slider_mid, s * c[1] + s * hh * 0.21],
               [s * hw * 0.35 + slider_mid, s * c[1] + s * hh * 0.21],
            ),
         ];
         drawable_diagram.drawables.push(QualifiedDrawable {
            layer: layer_slot + 1,
            drawable: OneOfDrawable::Lines(Strokeable::<LinesSetSet> {
               path_choices: PathChoices {
                  color: ColorChoice::Blue,
                  line_choice: LineChoice::Ordinary,
                  ..Default::default()
               },
               path: LinesSetSet { coords: thumb_vec, ..Default::default() },
            }),
         });

         let mut tick_vec = vec![];
         let mut l = -7.0;
         tick_vec.resize_with(13, || {
            l += 1.0;
            (
               [-s * tick_hw + slider_mid, s * l * 6.0 / 12.0],
               [s * tick_hw + slider_mid, s * l * 6.0 / 12.0],
            )
         });

         drawable_diagram.drawables.push(QualifiedDrawable {
            layer: layer_slot,
            drawable: OneOfDrawable::Lines(Strokeable::<LinesSetSet> {
               path_choices: PathChoices {
                  color: ColorChoice::Red,
                  line_choice: LineChoice::Ordinary,
                  ..Default::default()
               },
               path: LinesSetSet {
                  coords: tick_vec,
                  offsets: Some(vec![[-tick_offset, 0.0], [tick_offset, 0.0]]),
                  ..Default::default()
               },
            }),
         });
      }

      let straight_line = mul_add_p_4(
         &[[-3.0, -3.0], [-1.0, -1.0], [1.0, 1.0], [3.0, 3.0]],
         [0.0, s],
         [-1.2 * s + d, 0.0],
      );
      let managed_curve_b = ManagedCubic::create_from_control_points(
         &CubicFourPoint {
            r: t_range,
            h: CubicHomog(x_y_from_p_4(&straight_line)),
            sigma: (1.0, 1.0),
         },
         drawable_diagram.prep.axes_range,
      );
      draw_sample_cubilinear(
         &managed_curve_b,
         drawable_diagram,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            control_color: None,
            points_color: Some(ColorChoice::Blue),
            ..Default::default()
         },
      );

      let managed_curve_c = ManagedCubic::create_from_control_points(
         &CubicFourPoint {
            r: t_range,
            h: CubicHomog(x_y_from_p_4(&add_p_4(&straight_line, [-1.05 * s, 0.0]))),
            sigma: (1.0, 1.0),
         },
         drawable_diagram.prep.axes_range,
      );
      draw_sample_cubilinear(
         &managed_curve_c,
         drawable_diagram,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            control_color: Some(ColorChoice::RedRedGreen),
            points_color: None,
            ..Default::default()
         },
      );

      let wiggly_line = mul_add_p_4(
         &[[0.0, -3.0], [4.0, -1.0], [-1.0, 1.0], [0.0, 3.0]],
         [0.5 * s, s],
         [3.1 * s + d, 0.0],
      );

      let managed_curve_d = ManagedCubic::create_from_control_points(
         &CubicFourPoint {
            r: t_range,
            h: CubicHomog(x_y_from_p_4(&add_p_4(&wiggly_line, [-1.2 * s, 0.0]))),
            sigma: (1.0, 1.0),
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

   #[test]
   #[allow(clippy::unreadable_literal)]
   fn cubic_multi_test() {
      let t_range = [-6.0, 14.0];
      let sizing = make_cubic_sizing([4.0, 3.0]);
      let mut runner = build_from_sizing("curves/figs-intro/intro_cubic_multi", &sizing);
      let drawable_diagram = &mut runner.combo.drawable_diagram;

      {
         let s = 0.6666666666667;
         let d = -3.9;

         let wiggly_line = mul_add_p_4(
            &[[0.0, 12.0], [1.0, 11.0], [0.0, -11.0], [1.0, -12.0]],
            [0.6 * s, 0.35 * s],
            [d, 0.0],
         );

         let managed_curve_d = ManagedCubic::create_from_control_points(
            &CubicFourPoint {
               r: t_range,
               h: CubicHomog(x_y_from_p_4(&wiggly_line)),
               sigma: (1.0, 1.0),
            },
            drawable_diagram.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve_d,
            drawable_diagram,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               control_color: None,
               points_color: None,
               ..Default::default()
            },
         );
      }

      {
         let s = 1.7;
         let dx = -1.0;
         let dy = 2.0;

         let wiggly_line = mul_add_p_4(
            &[[-1.0, 0.0], [0.0, 1.0], [0.0, -1.0], [1.0, -0.0]],
            [s, 1.4 * s],
            [dx, dy],
         );

         let managed_curve_d = ManagedCubic::create_from_control_points(
            &CubicFourPoint {
               r: t_range,
               h: CubicHomog(x_y_from_p_4(&wiggly_line)),
               sigma: (1.0, 1.0),
            },
            drawable_diagram.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve_d,
            drawable_diagram,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               control_color: None,
               points_color: None,
               ..Default::default()
            },
         );
      }

      {
         let s = 1.1;
         let dx = -1.0;
         let dy = -1.7;
         let a = 1.0;
         let b = 1.0;

         let wiggly_line = mul_add_p_4(&[[-a, -a], [a, a], [-b, b], [b, -b]], [s, s], [dx, dy]);

         let managed_curve_d = ManagedCubic::create_from_control_points(
            &CubicFourPoint {
               r: t_range,
               h: CubicHomog(x_y_from_p_4(&wiggly_line)),
               sigma: (1.0, 1.0),
            },
            drawable_diagram.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve_d,
            drawable_diagram,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               control_color: None,
               points_color: None,
               ..Default::default()
            },
         );
      }

      {
         let s = 1.4;
         let dx = -1.0;
         let dy = 0.1;
         let a = 0.6;

         let wiggly_line = mul_add_p_4(
            &[[-1.0, -a], [-1.0 - 0.90 * a, a], [1.0 + 0.90 * a, a], [1.0, -a]],
            [s, s],
            [dx, dy],
         );

         let managed_curve_d = ManagedCubic::create_from_control_points(
            &CubicFourPoint {
               r: t_range,
               h: CubicHomog(x_y_from_p_4(&wiggly_line)),
               sigma: (1.0, 1.0),
            },
            drawable_diagram.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve_d,
            drawable_diagram,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               control_color: None,
               points_color: None,
               ..Default::default()
            },
         );
      }

      {
         let s = 1.8;
         let dx = 3.85;
         let dy = 2.8;
         let a = 1.2;
         let b = 0.65;

         let wiggly_line_c = mul_add_p_4(
            &[[-1.5, 0.0], [-1.4 + a, 0.0], [0.0, 1.0 - a], [0.0, 1.0]],
            [s, s],
            [dx, -dy],
         );

         let managed_curve_c = ManagedCubic::create_from_control_points(
            &CubicFourPoint {
               r: t_range,
               h: CubicHomog(x_y_from_p_4(&wiggly_line_c)),
               sigma: (1.0, 1.0),
            },
            drawable_diagram.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve_c,
            drawable_diagram,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               control_color: None,
               points_color: None,
               ..Default::default()
            },
         );

         let wiggly_line_d = mul_add_p_4(
            &[[-1.5, 0.0], [-1.4 + a, 0.0], [0.0, -1.0 + b], [0.0, -1.0]],
            [s, s],
            [dx, dy],
         );

         let managed_curve_d = ManagedCubic::create_from_control_points(
            &CubicFourPoint {
               r: t_range,
               h: CubicHomog(x_y_from_p_4(&wiggly_line_d)),
               sigma: (1.0, 1.0),
            },
            drawable_diagram.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve_d,
            drawable_diagram,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               control_color: None,
               points_color: None,
               ..Default::default()
            },
         );
      }

      {
         let s = 1.2;
         let dx = 5.3;
         let dy = 0.6;
         let a = 0.25;
         let b = -1.25;
         let c = -0.65;

         let wiggly_line = mul_add_p_4(
            &[[-3.0, 3.0 * a], [-1.0, a], [b, c], [3.0 * b, 3.0 * c]],
            [s, s],
            [dx, dy],
         );

         let managed_curve_d = ManagedCubic::create_from_control_points(
            &CubicFourPoint {
               r: t_range,
               h: CubicHomog(x_y_from_p_4(&wiggly_line)),
               sigma: (1.0, 1.0),
            },
            drawable_diagram.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve_d,
            drawable_diagram,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               control_color: None,
               points_color: None,
               ..Default::default()
            },
         );
      }

      render_and_check(&mut runner);
   }

   #[test]
   fn overview_twosie_test() {
      let t_range = [-1.0, 1.0];
      let sizing = make_cubic_sizing([3.5, 3.5]);
      let mut runner = build_from_sizing("curves/figs-intro/intro_overview_twosie", &sizing);
      let drawable_diagram = &mut runner.combo.drawable_diagram;

      {
         let s = 1.1;
         let dx = 0.2;
         let dy = 3.7;

         let main_curve = add_p_4(&scale_p_4(&CUBIC_BASIC, s), [dx, dy]);
         let mut managed_curve_a = ManagedCubic::create_from_control_points(
            &CubicFourPoint {
               r: t_range,
               h: CubicHomog(x_y_from_p_4(&main_curve)),
               sigma: (1.0, 1.0),
            },
            drawable_diagram.prep.axes_range,
         );

         draw_sample_cubilinear(
            &managed_curve_a,
            drawable_diagram,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               control_color: None,
               points_color: None,
               ..Default::default()
            },
         );

         managed_curve_a.select_range([-1.55, -1.0]);
         draw_sample_cubilinear(
            &managed_curve_a,
            drawable_diagram,
            &SampleCurveConfig {
               main_line_choice: LineChoice::Light,
               main_color: Some(ColorChoice::Blue),
               control_color: None,
               points_color: None,
               ..Default::default()
            },
         );

         managed_curve_a.select_range([1.0, 1.25]);
         draw_sample_cubilinear(
            &managed_curve_a,
            drawable_diagram,
            &SampleCurveConfig {
               main_line_choice: LineChoice::Light,
               main_color: Some(ColorChoice::Blue),
               control_color: None,
               points_color: None,
               ..Default::default()
            },
         );
      }

      {
         {
            let s = 1.1;
            let dx = 0.2;
            let dy = 0.7;

            let mut managed_curve = ManagedRatQuad::create_from_four_points(
               &FourPointRatQuad {
                  p: add_p_4(&scale_p_4(&CUBIC_BASIC, s), [dx, dy]),
                  r: t_range,
                  ..Default::default()
               },
               drawable_diagram.prep.axes_range,
            );

            draw_sample_rat_quad(
               &managed_curve,
               drawable_diagram,
               &SampleCurveConfig {
                  main_color: Some(ColorChoice::Green),
                  points_color: None,
                  control_color: None,
                  ..Default::default()
               },
            );

            managed_curve.select_range([-1.0, 1.0]);

            draw_sample_rat_quad(
               &managed_curve,
               drawable_diagram,
               &SampleCurveConfig {
                  main_color: Some(ColorChoice::Green),
                  points_color: None,
                  control_color: None,
                  ..Default::default()
               },
            );
         }
      }

      render_and_check(&mut runner);
   }

   #[test]
   fn distribution_irrational_test() {
      let t_range = [0.0, 1.0];
      let sizing = make_cubic_sizing([3.5, 3.5]);
      let mut runner =
         build_from_sizing("curves/figs-intro/intro_distribution_irrational", &sizing);
      let drawable_diagram = &mut runner.combo.drawable_diagram;

      let s = 5.0;

      {
         let h = s * 2.0_f64.sqrt() / 3.0;
         let shift_x = -3.0;
         let shift_y = -3.0;
         let managed_curve = ManagedRatQuad::create_from_four_points(
            &FourPointRatQuad {
               p: p_from_x_y_4(
                  &[s + shift_x, s + shift_x, h + shift_x, 0.0 + shift_x],
                  &[0.0 + shift_y, h + shift_y, s + shift_y, s + shift_y],
               ),
               r: t_range,
               sigma: (3.0_f64.sqrt(), 1.0),
               ..Default::default()
            },
            drawable_diagram.prep.axes_range,
         );
         draw_sample_rat_quad(
            &managed_curve,
            drawable_diagram,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               points_color: Some(ColorChoice::Blue),
               points_num_segments: 18,
               ..Default::default()
            },
         );
      }

      let radial_division: u32 = 6;
      let f = 0.18;
      let mut radials = Vec::<([f64; 2], [f64; 2])>::with_capacity(radial_division as usize + 1);
      for i in 0..=radial_division {
         let angle = 0.5 * std::f64::consts::PI * (i as f64) / (radial_division as f64);
         radials.push((
            [angle.cos() * s * (1.0 - f), angle.sin() * s * (1.0 - f)],
            [angle.cos() * s * (1.0 + f), angle.sin() * s * (1.0 + f)],
         ));
      }
      {
         let pattern_layer = 0;
         let shift_x = -3.0;
         let shift_y = -3.0;
         let qualified_drawable = QualifiedDrawable {
            layer: pattern_layer,
            drawable: OneOfDrawable::Lines(Strokeable::<LinesSetSet> {
               path: LinesSetSet { coords: radials, offsets: Some(vec![[shift_x, shift_y]]) },
               ..Default::default()
            }),
         };
         drawable_diagram.drawables.push(qualified_drawable);
      }

      render_and_check(&mut runner);
   }
}
