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

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use zvx_curves::base::{
   BaseRatQuad, CubiLinear, RatQuadOoeSubclassed, RatQuadRepr, SpecifiedRatQuad,
};
use zvx_curves::managed::ManagedCubic;
use zvx_curves::managed::ManagedRatQuad;
use zvx_docagram::diagram::SpartanDiagram;
use zvx_drawable::choices::{ColorChoice, LineChoice, PathCompletion, PointChoice};
use zvx_drawable::kinds::{
   ArcDrawable, ArcPath, CubicDrawable, LinesDrawable, OneOfDrawable, OneOfSegment, PathChoices,
   PointsDrawable, PolylineDrawable, QualifiedDrawable, SegmentSequence,
};

const fn extract_x_from_4(p: &[[f64; 2]; 4]) -> [f64; 4] {
   [p[0][0], p[1][0], p[2][0], p[3][0]]
}

const fn extract_y_from_4(p: &[[f64; 2]; 4]) -> [f64; 4] {
   [p[0][1], p[1][1], p[2][1], p[3][1]]
}

const fn extract_x_from_3(p: &[[f64; 2]; 3]) -> [f64; 3] {
   [p[0][0], p[1][0], p[2][0]]
}

const fn extract_y_from_3(p: &[[f64; 2]; 3]) -> [f64; 3] {
   [p[0][1], p[1][1], p[2][1]]
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum SampleOption {
   #[default]
   Normal,
   XVsT,
}

#[allow(clippy::suboptimal_flops)]
#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn create_rat_quad_path(
   num_segments_hyperbolic: i32,
   // TODO: Remove this path-generation version.
   _deprecate_rat_quad: &RatQuadRepr,
   reg_symm_rat_quad: &BaseRatQuad,
) -> OneOfSegment {
   assert!(matches!(reg_symm_rat_quad, BaseRatQuad::RegularizedSymmetric { .. }));
   if let BaseRatQuad::RegularizedSymmetric(reg_symmetric) = reg_symm_rat_quad {
      let ooe_rat_quad_extracted: RatQuadOoeSubclassed =
         reg_symmetric.classify_offset_odd_even(0.01);

      match ooe_rat_quad_extracted {
         RatQuadOoeSubclassed::Nothing => unimplemented!("Never should reach"),
         RatQuadOoeSubclassed::Elliptical(ooe_rat_quad) => {
            let r = ooe_rat_quad.r_bound;
            let s = 1.0 / ooe_rat_quad.a_2.sqrt();
            let mx = ooe_rat_quad.b[0];
            let my = ooe_rat_quad.c[0];
            let (sx, sy) = (0.5 * s * ooe_rat_quad.b[1], 0.5 * s * ooe_rat_quad.c[1]);
            let (cx, cy) = (ooe_rat_quad.b[2], ooe_rat_quad.c[2]);

            // The arc range is [-angle_range, angle_range].
            let angle_range = 2.0 * (r * (ooe_rat_quad.a_2 / ooe_rat_quad.a_0).sqrt()).atan();

            OneOfSegment::Arc(ArcPath {
               angle_range: [-angle_range, angle_range],
               center: [mx, my],
               transform: [cx, cy, sx, sy],
            })
         }

         RatQuadOoeSubclassed::Parabolic(four_point) => OneOfSegment::Cubic(four_point.p),

         // Since hyperbolic is not supported in SVG, we do a simple polyline approximation.
         RatQuadOoeSubclassed::Hyperbolic(hyper_rat_quad) => {
            let t_int: Vec<i32> = (0..num_segments_hyperbolic).collect();
            let mut t = Vec::<f64>::with_capacity(t_int.len());
            let scale = 2.0 * hyper_rat_quad.r_bound / f64::from(num_segments_hyperbolic);
            let offset = -hyper_rat_quad.r_bound;
            for item in &t_int {
               t.push(f64::from(*item).mul_add(scale, offset));
            }

            // TODO: Better preserve original intent, and do not retain rat_quad
            let pattern_vec = hyper_rat_quad.eval(&t);
            // let pattern_vec = deprecate_rat_quad.rq_eval_quad(&t);

            // This appears unused. Also, we should split the logic a bit if we actually want to
            // draw hyperbolics. In reality, drawing a hyperbolic should really be a matter for
            // extracting path points and then creating a drawable if desired.
            //
            // if curve_config.sample_options == SampleOption::XVsT {
            //    for i in 0..t_int.len() {
            //       pattern_vec[i] = [t[i], pattern_vec[i][0]];
            //    }
            // }

            OneOfSegment::Polyline(pattern_vec)
         }
      }
   } else {
      OneOfSegment::Polyline(vec![])
   }
}

#[allow(clippy::suboptimal_flops)]
#[allow(clippy::missing_panics_doc)]
pub fn push_rat_quad_drawable(
   spartan: &mut SpartanDiagram,
   rat_quad: &RatQuadRepr,
   reg_symm_base: &BaseRatQuad,
   path_choices: PathChoices,
   layer: i32,
) {
   assert!(matches!(reg_symm_base, BaseRatQuad::RegularizedSymmetric { .. }));
   if let BaseRatQuad::RegularizedSymmetric(_ooe_rat_quad) = reg_symm_base {
      let one_of_path =
         create_rat_quad_path(spartan.num_segments_hyperbolic, rat_quad, reg_symm_base);

      match one_of_path {
         OneOfSegment::Arc(path) => {
            spartan.drawables.push(QualifiedDrawable {
               layer,
               drawable: OneOfDrawable::Arc(ArcDrawable { path, path_choices }),
            });
         }
         OneOfSegment::Cubic(path) => {
            spartan.drawables.push(QualifiedDrawable {
               layer,
               drawable: OneOfDrawable::Cubic(CubicDrawable { path, path_choices }),
            });
         }
         OneOfSegment::Polyline(path) => {
            spartan.drawables.push(QualifiedDrawable {
               layer,
               drawable: OneOfDrawable::Polyline(PolylineDrawable { path, path_choices }),
            });
         }
         OneOfSegment::Nothing => {
            panic!("Unreachable code.");
         }
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

#[allow(clippy::too_many_lines)]
#[allow(clippy::missing_panics_doc)]
pub fn draw_sample_rat_quad(
   managed_rat_quad: &ManagedRatQuad,
   spartan: &mut SpartanDiagram,
   curve_config: &SampleCurveConfig,
) {
   let rat_quad: RatQuadRepr =
      managed_rat_quad.get_poly_rat_quad_repr().expect("Never should be missing");

   if let Some(color_choice) = curve_config.control_color {
      // assert!(false);

      let end_points_vec;
      let control_points_vec;
      match managed_rat_quad.specified {
         SpecifiedRatQuad::None => {
            panic!("Unable to draw control points when RQC not specified via control points.");
         }
         // SpecifiedRatQuad:: Base(BaseRatQuad), // Three-points and angle, for example.
         SpecifiedRatQuad::FourPoint(specified) => {
            let x = extract_x_from_4(&specified.p);
            let y = extract_y_from_4(&specified.p);
            end_points_vec = vec![[x[0], y[0]], [x[3], y[3]]];
            control_points_vec = vec![[x[1], y[1]], [x[2], y[2]]];
         }
         SpecifiedRatQuad::ThreePointAngle(specified) => {
            let x = extract_x_from_3(&specified.p);
            let y = extract_y_from_3(&specified.p);
            end_points_vec = vec![[x[0], y[0]], [x[2], y[2]]];
            control_points_vec = vec![[x[1], y[1]]];
         }
      }

      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.control_point_choices[0],
            color_choice,
            centers: end_points_vec.clone(),
         }),
      });
      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.control_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.control_point_choices[1],
            color_choice,
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
         drawable: OneOfDrawable::Lines(LinesDrawable {
            path_choices: PathChoices {
               color: color_choice,
               line_choice: curve_config.control_line_choice,
            },
            coords: vec![
               (end_points_vec[0], expanded_control_points_vec[0]),
               (end_points_vec[1], expanded_control_points_vec[1]),
            ],
            ..Default::default()
         }),
      });
   }

   if let Some(color_choice) = curve_config.points_color {
      // Do not include end points if control points are doing that already.
      let t_int: Vec<i32> = if curve_config.control_color.is_some() {
         (1..curve_config.points_num_segments).collect()
      } else {
         (0..=curve_config.points_num_segments).collect()
      };
      let mut t = Vec::<f64>::with_capacity(t_int.len());
      let scale = (rat_quad.r[1] - rat_quad.r[0]) / f64::from(curve_config.points_num_segments);
      let offset = rat_quad.r[0];
      for item in &t_int {
         t.push(f64::from(*item).mul_add(scale, offset));
      }

      let mut pattern_vec = rat_quad.rq_eval_quad(&t);

      if curve_config.sample_options == SampleOption::XVsT {
         for i in 0..t_int.len() {
            pattern_vec[i] = [t[i], pattern_vec[i][0]];
         }
      }

      spartan.drawables.push(QualifiedDrawable {
         layer: curve_config.points_layer,
         drawable: OneOfDrawable::Points(PointsDrawable {
            point_choice: curve_config.points_choice,
            color_choice,
            centers: pattern_vec,
         }),
      });
   }

   if let Some(color_choice) = curve_config.main_color {
      if curve_config.approx_num_segments != 0 {
         let t_int: Vec<i32> = (0..=curve_config.approx_num_segments).collect();
         let mut t = Vec::<f64>::with_capacity(t_int.len());
         let scale = (rat_quad.r[1] - rat_quad.r[0]) / f64::from(curve_config.approx_num_segments);
         let offset = rat_quad.r[0];
         for item in &t_int {
            t.push(f64::from(*item).mul_add(scale, offset));
         }

         let mut pattern_vec = rat_quad.rq_eval_quad(&t);

         if curve_config.sample_options == SampleOption::XVsT {
            for i in 0..t_int.len() {
               pattern_vec[i] = [t[i], pattern_vec[i][0]];
            }
         }

         spartan.drawables.push(QualifiedDrawable {
            layer: curve_config.main_line_layer,
            drawable: OneOfDrawable::Polyline(PolylineDrawable {
               path_choices: PathChoices {
                  color: color_choice,
                  line_choice: curve_config.main_line_choice,
               },
               path: pattern_vec,
            }),
         });
      } else {
         let regularized_rat_quad: &BaseRatQuad = managed_rat_quad.get_regularized_rat_quad();
         push_rat_quad_drawable(
            spartan,
            &rat_quad,
            regularized_rat_quad,
            PathChoices { color: color_choice, line_choice: curve_config.main_line_choice },
            curve_config.main_line_layer,
         );
      }
   }
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::suboptimal_flops)]
pub fn draw_sample_cubilinear(
   managed_cubic: &ManagedCubic,
   spartan: &mut SpartanDiagram,
   curve_config: &SampleCurveConfig,
) {
   // let four_point = &managed_cubic.four_point;
   assert!(matches!(managed_cubic.four_point, CubiLinear::FourPoint { .. }));
   if let CubiLinear::FourPoint(four_point) = managed_cubic.four_point {
      if let Some(color_choice) = curve_config.control_color {
         let end_points_vec = vec![four_point.p[0], four_point.p[3]];
         let control_points_vec = vec![four_point.p[1], four_point.p[2]];

         spartan.drawables.push(QualifiedDrawable {
            layer: curve_config.control_layer,
            drawable: OneOfDrawable::Points(PointsDrawable {
               point_choice: curve_config.control_point_choices[0],
               color_choice,
               centers: end_points_vec.clone(),
            }),
         });
         spartan.drawables.push(QualifiedDrawable {
            layer: curve_config.control_layer,
            drawable: OneOfDrawable::Points(PointsDrawable {
               point_choice: curve_config.control_point_choices[1],
               color_choice,
               centers: control_points_vec.clone(),
            }),
         });

         assert_eq!(end_points_vec.len(), 2);
         assert_eq!(control_points_vec.len(), 2);
         spartan.drawables.push(QualifiedDrawable {
            layer: curve_config.control_layer,
            drawable: OneOfDrawable::Lines(LinesDrawable {
               path_choices: PathChoices {
                  color: color_choice,
                  line_choice: curve_config.control_line_choice,
               },
               coords: vec![
                  (end_points_vec[0], control_points_vec[0]),
                  (end_points_vec[1], control_points_vec[1]),
               ],
               ..Default::default()
            }),
         });
      }

      if let Some(color_choice) = curve_config.points_color {
         // Do not include end points if control points are doing that already.
         let t_int: Vec<i32> = if curve_config.control_color.is_some() {
            (1..curve_config.points_num_segments).collect()
         } else {
            (0..=curve_config.points_num_segments).collect()
         };
         let mut t = Vec::<f64>::with_capacity(t_int.len());
         let scale =
            (four_point.r[1] - four_point.r[0]) / f64::from(curve_config.points_num_segments);
         let offset = four_point.r[0];
         for item in &t_int {
            t.push(f64::from(*item).mul_add(scale, offset));
         }

         let pattern_vec = four_point.eval(&t);

         spartan.drawables.push(QualifiedDrawable {
            layer: curve_config.points_layer,
            drawable: OneOfDrawable::Points(PointsDrawable {
               point_choice: curve_config.points_choice,
               color_choice,
               centers: pattern_vec,
            }),
         });
      }

      if let Some(color_choice) = curve_config.main_color {
         spartan.drawables.push(QualifiedDrawable {
            layer: curve_config.main_line_layer,
            drawable: OneOfDrawable::Cubic(CubicDrawable {
               path_choices: PathChoices {
                  color: color_choice,
                  line_choice: curve_config.main_line_choice,
               },
               path: four_point.p,
            }),
         });
      }
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
   spartan: &mut SpartanDiagram,
) {
   let mut segments_paths: Vec<OneOfSegment> = Vec::new();

   for segment in segments {
      match &segment {
         OneOfManagedSegment::ManagedCubic(managed_cubic) => {
            // let four_point = &managed_cubic.four_point;
            assert!(matches!(managed_cubic.four_point, CubiLinear::FourPoint { .. }));
            if let CubiLinear::FourPoint(four_point) = managed_cubic.four_point {
               segments_paths.push(OneOfSegment::Cubic(four_point.p));
            }
         }

         OneOfManagedSegment::ManagedRatQuad(managed_rat_quad) => {
            let rat_quad: &RatQuadRepr =
               &managed_rat_quad.get_poly_rat_quad_repr().expect("Should be there");
            let regularized_rat_quad: &BaseRatQuad = managed_rat_quad.get_regularized_rat_quad();
            segments_paths.push(create_rat_quad_path(
               spartan.num_segments_hyperbolic,
               rat_quad,
               regularized_rat_quad,
            ));
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
