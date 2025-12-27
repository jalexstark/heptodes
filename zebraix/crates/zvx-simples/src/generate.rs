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

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use zvx_base::{
   ArcPath, CubicPath, HyperbolicPath, OneOfSegment, PolylinePath, RatQuadHomogWeighted,
};
use zvx_curves::{
   Curve, CurveEval, ManagedCubic, ManagedRatQuad, RatQuadOoeSubclassed, SpecifiedRatQuad,
};
use zvx_docagram::diagram::DrawableDiagram;
use zvx_drawable::{
   ColorChoice, LineChoice, LinesSetSet, MarkupChoice, OneOfDrawable, PathChoices, PathCompletion,
   PointChoice, PointsDrawable, QualifiedDrawable, SegmentSequence, Strokeable, TextAnchorChoice,
   TextAnchorHorizontal, TextAnchorVertical, TextDrawable, TextOffsetChoice, TextSingle,
   TextSizeChoice,
};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub enum SampleOption {
   #[default]
   Normal,
   XVsT,
}

#[allow(clippy::suboptimal_flops)]
#[allow(clippy::missing_panics_doc)]
#[must_use]
fn create_rat_quad_path(curve: &Curve<RatQuadHomogWeighted>) -> OneOfSegment {
   RatQuadOoeSubclassed::segment_from_ordinary(curve, 0.01).unwrap()
}

// Take a one-of-path (OneOfSegment) and push drawable piece onto drawables vector.
#[allow(clippy::suboptimal_flops)]
#[allow(clippy::missing_panics_doc)]
fn push_path_segment_drawable(
   drawables: &mut Vec<QualifiedDrawable>,
   one_of_path: &OneOfSegment,
   path_choices: PathChoices,
   layer: i32,
) {
   match one_of_path {
      OneOfSegment::Arc(path) => {
         drawables.push(QualifiedDrawable {
            layer,
            drawable: OneOfDrawable::Arc(Strokeable::<ArcPath> {
               path: path.clone(),
               path_choices,
            }),
         });
      }
      OneOfSegment::Cubic(path) => {
         drawables.push(QualifiedDrawable {
            layer,
            drawable: OneOfDrawable::Cubic(Strokeable::<CubicPath> {
               path: path.clone(),
               path_choices,
            }),
         });
      }
      OneOfSegment::Hyperbolic(path) => {
         drawables.push(QualifiedDrawable {
            layer,
            drawable: OneOfDrawable::Hyperbolic(Strokeable::<HyperbolicPath> {
               path: path.clone(),
               path_choices,
            }),
         });
      }
      OneOfSegment::Polyline(path) => {
         drawables.push(QualifiedDrawable {
            layer,
            drawable: OneOfDrawable::Polyline(Strokeable::<PolylinePath> {
               path: path.clone(),
               path_choices,
            }),
         });
      }
      OneOfSegment::Neither => {
         panic!("Unreachable code.");
      }
   }
}

// In each drawn feature (the main line, points, control) the some-ness of the first field
// toggles drawing of the feature.
pub struct SampleCurveConfig {
   pub main_color: Option<ColorChoice>,
   pub main_line_choice: LineChoice,
   pub approx_num_segments: i32,

   pub points_color: Option<ColorChoice>,
   pub points_choice: PointChoice,
   pub points_num_segments: i32,

   pub sample_options: SampleOption,

   pub control_color: Option<ColorChoice>,
   pub control_point_choices: [PointChoice; 2],
   pub control_line_choice: LineChoice,

   pub control_layer: i32,
   pub points_layer: i32,
   pub main_line_layer: i32,
}

impl Default for SampleCurveConfig {
   fn default() -> Self {
      Self {
         main_color: Some(ColorChoice::Blue),
         main_line_choice: LineChoice::Ordinary,
         approx_num_segments: 0,
         points_color: Some(ColorChoice::Green),
         points_choice: PointChoice::Dot,
         points_num_segments: 12,
         sample_options: SampleOption::Normal,
         control_color: None,
         control_point_choices: [PointChoice::Circle, PointChoice::Times],
         control_line_choice: LineChoice::Light,

         control_layer: 10,
         points_layer: 20,
         main_line_layer: 30,
      }
   }
}

#[allow(clippy::suboptimal_flops)]
#[allow(clippy::similar_names)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::missing_panics_doc)]
pub fn draw_sample_rat_quad(
   managed_rat_quad: &ManagedRatQuad,
   spartan: &mut DrawableDiagram,
   curve_config: &SampleCurveConfig,
) {
   let deprecated_rat_quad: &Curve<RatQuadHomogWeighted> = &managed_rat_quad.rq_curve;

   if let Some(color_choice) = &curve_config.control_color {
      let end_points_vec;
      let control_points_vec;
      match &managed_rat_quad.specified {
         SpecifiedRatQuad::None | SpecifiedRatQuad::FourPoint => {
            let ([[x_0, y_0], [x_3, y_3]], [[dx_1, dy_1], [dx_2, dy_2]]) =
               deprecated_rat_quad.characterize_endpoints();
            let scale = 1.0 / 3.0;
            end_points_vec = vec![[x_0, y_0], [x_3, y_3]];
            control_points_vec = vec![
               [x_0 + dx_1 * scale, y_0 + dy_1 * scale],
               [x_3 - dx_2 * scale, y_3 - dy_2 * scale],
            ];
         }
         SpecifiedRatQuad::ThreePointAngle => {
            let ([[x_0, y_0], [x_3, y_3]], [[dx_1, dy_1], [dx_2, dy_2]]) =
               deprecated_rat_quad.characterize_endpoints();
            let det = dx_1 * dy_2 - dx_2 * dy_1;
            let scale;
            if det.abs() > 0.001 {
               // Calculate intersection.
               scale = ((x_3 - x_0) * dy_2 - dx_2 * (y_3 - y_0)) / det;
            } else if (x_3 - x_0).abs() > (y_3 - y_0).abs() {
               // Otherwise assume sigma is unit.
               scale = (x_3 - x_0) / (dx_1 + dx_2);
            } else {
               scale = (y_3 - y_0) / (dy_1 + dy_2);
            }
            end_points_vec = vec![[x_0, y_0], [x_3, y_3]];
            control_points_vec = vec![[x_0 + dx_1 * scale, y_0 + dy_1 * scale]];
         }
      }

      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.control_point_choices[0],
            color_choice: color_choice.clone(),
            centers: end_points_vec.clone(),
         }),
      });
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.control_point_choices[1],
            color_choice: color_choice.clone(),
            centers: control_points_vec.clone(),
         }),
      });

      let expanded_control_points_vec = if control_points_vec.len() == 2 {
         control_points_vec
      } else {
         vec![control_points_vec[0], control_points_vec[0]]
      };

      assert_eq!(end_points_vec.len(), 2);
      assert_eq!(expanded_control_points_vec.len(), 2);
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Lines(Strokeable::<LinesSetSet> {
            path_choices: PathChoices {
               color: color_choice.clone(),
               line_choice: curve_config.control_line_choice,
               ..Default::default()
            },
            path: LinesSetSet {
               coords: vec![
                  (end_points_vec[0], expanded_control_points_vec[0]),
                  (end_points_vec[1], expanded_control_points_vec[1]),
               ],
               ..Default::default()
            },
         }),
      });
   }

   if let Some(color_choice) = &curve_config.points_color {
      // Do not include end points if control points are doing that already.
      let t_int: Vec<i32> = if curve_config.control_color.is_some() {
         (1..curve_config.points_num_segments).collect()
      } else {
         (0..=curve_config.points_num_segments).collect()
      };
      let mut t = Vec::<f64>::with_capacity(t_int.len());
      let scale = (deprecated_rat_quad.path.r[1] - deprecated_rat_quad.path.r[0])
         / f64::from(curve_config.points_num_segments);
      let offset = deprecated_rat_quad.path.r[0];
      for item in &t_int {
         t.push(f64::from(*item).mul_add(scale, offset));
      }

      let mut pattern_vec = deprecated_rat_quad.eval_with_bilinear(&t);

      if curve_config.sample_options == SampleOption::XVsT {
         for i in 0..t_int.len() {
            pattern_vec[i] = [t[i], pattern_vec[i][0]];
         }
      }

      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.points_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.points_choice,
            color_choice: color_choice.clone(),
            centers: pattern_vec,
         }),
      });
   }

   if let Some(color_choice) = &curve_config.main_color {
      if curve_config.approx_num_segments != 0 {
         let t_int: Vec<i32> = (0..=curve_config.approx_num_segments).collect();
         let mut t = Vec::<f64>::with_capacity(t_int.len());
         let scale = (deprecated_rat_quad.path.r[1] - deprecated_rat_quad.path.r[0])
            / f64::from(curve_config.approx_num_segments);
         let offset = deprecated_rat_quad.path.r[0];
         for item in &t_int {
            t.push(f64::from(*item).mul_add(scale, offset));
         }

         let mut pattern_vec = deprecated_rat_quad.eval_with_bilinear(&t);

         // // XXXXXXXXXXXXXXXXXXXXXXXX

         if curve_config.sample_options == SampleOption::XVsT {
            for i in 0..t_int.len() {
               pattern_vec[i] = [t[i], pattern_vec[i][0]];
            }
         }

         spartan.drawables.push(QualifiedDrawable {
            layer: curve_config.main_line_layer,
            drawable: OneOfDrawable::Polyline(Strokeable::<PolylinePath> {
               path_choices: PathChoices {
                  color: color_choice.clone(),
                  line_choice: curve_config.main_line_choice,
                  ..Default::default()
               },
               path: pattern_vec,
            }),
         });
      } else {
         let ordinary_rat_quad: &Curve<RatQuadHomogWeighted> = &managed_rat_quad.rq_curve;
         let one_of_path = create_rat_quad_path(ordinary_rat_quad);
         push_path_segment_drawable(
            &mut spartan.drawables,
            &one_of_path,
            PathChoices {
               color: color_choice.clone(),
               line_choice: curve_config.main_line_choice,
               ..Default::default()
            },
            curve_config.main_line_layer,
         );
      }
   }
}

#[allow(clippy::missing_panics_doc)]
pub fn draw_derivatives_rat_quad(
   managed_rat_quad: &ManagedRatQuad,
   spartan: &mut DrawableDiagram,
   curve_config: &SampleCurveConfig,
) {
   let deprecated_rat_quad: &Curve<RatQuadHomogWeighted> = &managed_rat_quad.rq_curve;

   let t_int: Vec<i32> = (0..=curve_config.points_num_segments).collect();
   let mut t = Vec::<f64>::with_capacity(t_int.len());
   let scale = (deprecated_rat_quad.path.r[1] - deprecated_rat_quad.path.r[0])
      / f64::from(curve_config.points_num_segments);
   let offset = deprecated_rat_quad.path.r[0];
   for item in &t_int {
      t.push(f64::from(*item).mul_add(scale, offset));
   }

   let start_vec = managed_rat_quad.rq_curve.eval_homog(&t);
   let mut end_vec = managed_rat_quad.rq_curve.eval_derivative_scaled(&t, scale);
   let mut delta_lines = Vec::<([f64; 2], [f64; 2])>::with_capacity(t_int.len());
   assert_eq!(start_vec.len(), t_int.len());
   assert_eq!(end_vec.len(), t_int.len());
   for i in 0..start_vec.len() {
      end_vec[i][0] += start_vec[i][0];
      end_vec[i][1] += start_vec[i][1];
      delta_lines.push((start_vec[i], end_vec[i]));
   }

   if let Some(color_choice) = &curve_config.main_color {
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.main_line_layer + 1,
         drawable: OneOfDrawable::Lines(Strokeable::<LinesSetSet> {
            path_choices: PathChoices {
               color: color_choice.clone(),
               line_choice: curve_config.main_line_choice,
               ..Default::default()
            },
            path: LinesSetSet { coords: delta_lines, ..Default::default() },
         }),
      });
   }

   if let Some(color_choice) = &curve_config.points_color {
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.main_line_layer + 2,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.points_choice,
            color_choice: color_choice.clone(),
            centers: end_vec,
         }),
      });
   }
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::suboptimal_flops)]
pub fn draw_sample_cubilinear(
   managed_cubic: &ManagedCubic,
   spartan: &mut DrawableDiagram,
   curve_config: &SampleCurveConfig,
) {
   let four_point = &managed_cubic.four_point;
   if let Some(color_choice) = &curve_config.control_color {
      let end_points_vec = vec![
         [four_point.path.h.0[0][0], four_point.path.h.0[1][0]],
         [four_point.path.h.0[0][3], four_point.path.h.0[1][3]],
      ];
      let control_points_vec = vec![
         [four_point.path.h.0[0][1] / 3.0, four_point.path.h.0[1][1] / 3.0],
         [four_point.path.h.0[0][2] / 3.0, four_point.path.h.0[1][2] / 3.0],
      ];

      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.control_point_choices[0],
            color_choice: color_choice.clone(),
            centers: end_points_vec.clone(),
         }),
      });
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.control_point_choices[1],
            color_choice: color_choice.clone(),
            centers: control_points_vec.clone(),
         }),
      });

      assert_eq!(end_points_vec.len(), 2);
      assert_eq!(control_points_vec.len(), 2);
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Lines(Strokeable::<LinesSetSet> {
            path_choices: PathChoices {
               color: color_choice.clone(),
               line_choice: curve_config.control_line_choice,
               ..Default::default()
            },
            path: LinesSetSet {
               coords: vec![
                  (end_points_vec[0], control_points_vec[0]),
                  (end_points_vec[1], control_points_vec[1]),
               ],
               ..Default::default()
            },
         }),
      });
   }

   if let Some(color_choice) = &curve_config.points_color {
      // Do not include end points if control points are doing that already.
      let t_int: Vec<i32> = if curve_config.control_color.is_some() {
         (1..curve_config.points_num_segments).collect()
      } else {
         (0..=curve_config.points_num_segments).collect()
      };
      let mut t = Vec::<f64>::with_capacity(t_int.len());
      let scale = (four_point.path.r[1] - four_point.path.r[0])
         / f64::from(curve_config.points_num_segments);
      let offset = four_point.path.r[0];
      for item in &t_int {
         t.push(f64::from(*item).mul_add(scale, offset));
      }

      let pattern_vec = four_point.eval_with_bilinear(&t);

      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.points_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.points_choice,
            color_choice: color_choice.clone(),
            centers: pattern_vec,
         }),
      });
   }

   if let Some(color_choice) = &curve_config.main_color {
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.main_line_layer,
         drawable: OneOfDrawable::Cubic(Strokeable::<CubicPath> {
            path_choices: PathChoices {
               color: color_choice.clone(),
               line_choice: curve_config.main_line_choice,
               ..Default::default()
            },
            path: four_point.path.clone(),
         }),
      });
   }
}

#[allow(clippy::missing_panics_doc)]
pub fn draw_derivatives_cubilinear(
   managed_cubic: &ManagedCubic,
   spartan: &mut DrawableDiagram,
   curve_config: &SampleCurveConfig,
) {
   let four_point = &managed_cubic.four_point;

   let t_int: Vec<i32> = (0..=curve_config.points_num_segments).collect();
   let mut t = Vec::<f64>::with_capacity(t_int.len());
   let scale =
      (four_point.path.r[1] - four_point.path.r[0]) / f64::from(curve_config.points_num_segments);

   let offset = four_point.path.r[0];
   for item in &t_int {
      t.push(f64::from(*item).mul_add(scale, offset));
   }

   let start_vec = four_point.eval_with_bilinear(&t);
   let mut end_vec = four_point.eval_derivative_scaled(&t, scale);
   let mut delta_lines = Vec::<([f64; 2], [f64; 2])>::with_capacity(t_int.len());
   assert_eq!(start_vec.len(), t_int.len());
   assert_eq!(end_vec.len(), t_int.len());
   for i in 0..start_vec.len() {
      end_vec[i][0] += start_vec[i][0];
      end_vec[i][1] += start_vec[i][1];
      delta_lines.push((start_vec[i], end_vec[i]));
   }

   if let Some(color_choice) = &curve_config.main_color {
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.main_line_layer + 1,
         drawable: OneOfDrawable::Lines(Strokeable::<LinesSetSet> {
            path_choices: PathChoices {
               color: color_choice.clone(),
               line_choice: curve_config.main_line_choice,
               ..Default::default()
            },
            path: LinesSetSet { coords: delta_lines, ..Default::default() },
         }),
      });
   }

   if let Some(color_choice) = &curve_config.points_color {
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.main_line_layer + 2,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.points_choice,
            color_choice: color_choice.clone(),
            centers: end_vec,
         }),
      });
   }
}

#[allow(clippy::large_enum_variant)]
pub enum OneOfManagedSegment {
   ManagedCubic(ManagedCubic),
   ManagedRatQuad(ManagedRatQuad),
   Polyline(Vec<[f64; 2]>),
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::suboptimal_flops)]
pub fn draw_sample_segment_sequence(
   segments: &VecDeque<OneOfManagedSegment>,
   path_choices: PathChoices,
   completion: PathCompletion,
   layer: i32,
   spartan: &mut DrawableDiagram,
) {
   let mut segments_paths: Vec<OneOfSegment> = Vec::new();

   for segment in segments {
      match &segment {
         OneOfManagedSegment::ManagedCubic(managed_cubic) => {
            let four_point = &managed_cubic.four_point;
            segments_paths.push(OneOfSegment::Cubic(four_point.path.clone()));
         }

         OneOfManagedSegment::ManagedRatQuad(managed_rat_quad) => {
            let ordinary_rat_quad: &Curve<RatQuadHomogWeighted> = &managed_rat_quad.rq_curve;
            segments_paths.push(create_rat_quad_path(ordinary_rat_quad));
         }
         OneOfManagedSegment::Polyline(locations) => {
            segments_paths.push(OneOfSegment::Polyline(locations.clone()));
         }
      }
   }

   spartan.drawables.push(QualifiedDrawable {
      layer,
      drawable: OneOfDrawable::SegmentSequence(SegmentSequence {
         path_choices,
         completion,
         segments: segments_paths,
      }),
   });
}

pub fn add_centered_text(
   drawable_diagram: &mut DrawableDiagram,
   curve_config: &SampleCurveConfig,
   text: &'static str,
   location: [f64; 2],
) {
   drawable_diagram.drawables.push(QualifiedDrawable {
      layer: curve_config.main_line_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         color_choice: curve_config.main_color.clone().unwrap_or_default(),
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Center,
            TextAnchorVertical::Middle,
         ),
         texts: vec![TextSingle {
            content: text.to_string(),
            location,
            markup: MarkupChoice::Pango,
         }],
      }),
   });
}
