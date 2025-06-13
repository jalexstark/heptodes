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
use std::io::Write;
use zvx_cairo::render::CairoSpartanCombo;
use zvx_curves::base::BaseRatQuad;
use zvx_curves::base::CubiLinear;
use zvx_curves::base::FourPointRatQuad;
use zvx_curves::base::RatQuadState;
use zvx_curves::base::ZebraixAngle;
use zvx_curves::managed::ManagedCubic;
use zvx_curves::managed::ManagedRatQuad;
use zvx_docagram::axes::AxesSpec;
use zvx_docagram::axes::AxesStyle;
use zvx_docagram::axes::AxisNumbering;
use zvx_docagram::diagram::SizingScheme;
use zvx_drawable::choices::ColorChoice;
use zvx_drawable::choices::LineChoice;
use zvx_drawable::choices::LineClosureChoice;
use zvx_drawable::choices::PointChoice;
use zvx_drawable::choices::TextAnchorChoice;
use zvx_drawable::choices::TextAnchorHorizontal;
use zvx_drawable::choices::TextAnchorVertical;
use zvx_drawable::choices::TextOffsetChoice;
use zvx_drawable::choices::TextSizeChoice;
use zvx_drawable::kinds::CirclesDrawable;
use zvx_drawable::kinds::LinesDrawable;
use zvx_drawable::kinds::OneOfDrawable;
use zvx_drawable::kinds::PointsDrawable;
use zvx_drawable::kinds::PolylineDrawable;
use zvx_drawable::kinds::QualifiedDrawable;
use zvx_drawable::kinds::SegmentChoices;
use zvx_drawable::kinds::TextDrawable;
use zvx_drawable::kinds::TextSingle;
use zvx_golden::filtered::JsonGoldenTest;
use zvx_golden::filtered::SvgGoldenTest;
use zvx_simples::generate::draw_sample_cubilinear;
use zvx_simples::generate::draw_sample_rat_quad;
use zvx_simples::generate::SampleCurveConfig;
use zvx_simples::generate::SampleOption;

fn scale_coord_vec(v: &Vec<[f64; 2]>, s: f64) -> Vec<[f64; 2]> {
   let mut result = v.clone();
   for i in 0..v.len() {
      result[i] = [v[i][0] * s, v[i][1] * s];
   }
   result
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

// // After dependency to test_utils is added, use type-def for the result box.
// fn write_sample_to_write<W: Write + 'static>(
//    out_stream: W,
//    cairo_spartan: &mut CairoSpartanCombo,
// ) -> Result<Box<dyn core::any::Any>, cairo::StreamWithError> {
//    assert!(cairo_spartan.spartan.is_ready());
//    cairo_spartan.render_diagram_to_stream(out_stream)
// }

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
            coords: vec![(
               [sizing.debug_box[0], sizing.debug_box[1]],
               [sizing.debug_box[0], sizing.debug_box[3]],
            )],
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
            coords: vec![
               (
                  [sizing.debug_box[0], sizing.debug_box[3]],
                  [sizing.debug_box[2], sizing.debug_box[1]],
               ),
               (
                  [sizing.debug_box[2], sizing.debug_box[3]],
                  [sizing.debug_box[0], sizing.debug_box[1]],
               ),
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
      // let mut json_golden = JsonGoldenTest::new("tests/goldenfiles/", filestem);
      // let serialized = to_string_pretty::<SpartanDiagram>(&cairo_spartan.spartan).unwrap();
      // json_golden.writeln_as_bytes(&serialized);
      let mut json_golden = JsonGoldenTest::new("tests/goldenfiles/", filestem);
      // This only really fails if keys cannot be rendered.
      //
      // Consider moving into golden test crate. This is only trigger for serde_json dependency.
      to_writer_pretty(&json_golden.out_stream, &cairo_spartan.spartan).unwrap();
      let bytes_amount_nl = json_golden.out_stream.write(b"\n").unwrap();
      assert!(bytes_amount_nl == 1);
   }

   {
      let svg_golden = SvgGoldenTest::new("tests/goldenfiles/", filestem);
      let raw_result = cairo_spartan.render_diagram_to_write(svg_golden.get_raw_writeable());
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
         axes_style: AxesStyle::Boxed,
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
         // offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Center,
            TextAnchorVertical::Bottom,
         ),
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
         // offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Center,
            TextAnchorVertical::Top,
         ),
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
            coords: vec![
               ([0.0, 0.0], [5.0, 5.0]),
               ([0.0, 0.0], [0.0, 5.0]),
               ([0.0, 0.0], [-2.5, 5.0]),
            ],
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
            coords: vec![
               ([0.0, 0.0], [5.0, -5.0]),
               ([0.0, 0.0], [0.0, -5.0]),
               ([0.0, 0.0], [-2.5, -5.0]),
            ],
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
         anchor_choice: TextAnchorChoice::Centered,
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
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Left,
            TextAnchorVertical::Middle,
         ),
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
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Left,
            TextAnchorVertical::Middle,
         ),
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
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Left,
            TextAnchorVertical::Middle,
         ),
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
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Left,
            TextAnchorVertical::Top,
         ),
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
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Left,
            TextAnchorVertical::Middle,
         ),
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
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Left,
            TextAnchorVertical::Middle,
         ),
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [4.0, 0.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Small,
         color_choice: ColorChoice::BlueRed,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Left,
            TextAnchorVertical::Middle,
         ),
         texts: vec![TextSingle { content: "xodflox".to_string(), location: [4.0, -1.5] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Normal,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Left,
            TextAnchorVertical::Bottom,
         ),
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
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Center,
            TextAnchorVertical::Bottom,
         ),
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
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Center,
            TextAnchorVertical::Bottom,
         ),
         texts: vec![TextSingle { content: "Elpo x lpoE".to_string(), location: [0.0, 3.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Small,
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Center,
            TextAnchorVertical::Bottom,
         ),
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
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Right,
            TextAnchorVertical::Middle,
         ),
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
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Right,
            TextAnchorVertical::Middle,
         ),
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
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Right,
            TextAnchorVertical::Middle,
         ),
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [-2.0, -1.5] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Large,
         color_choice: ColorChoice::BlueRed,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Right,
            TextAnchorVertical::Middle,
         ),
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
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Right,
            TextAnchorVertical::Middle,
         ),
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
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Right,
            TextAnchorVertical::Middle,
         ),
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
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Right,
            TextAnchorVertical::Bottom,
         ),
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
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Right,
            TextAnchorVertical::Top,
         ),
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
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Center,
            TextAnchorVertical::Top,
         ),
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
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Center,
            TextAnchorVertical::Top,
         ),
         texts: vec![TextSingle { content: spanning_string.to_string(), location: [0.0, -3.0] }],
         ..Default::default()
      }),
      ..Default::default()
   });
   cairo_spartan.spartan.drawables.push(QualifiedDrawable {
      layer: pattern_layer,
      drawable: OneOfDrawable::Text(TextDrawable {
         size_choice: TextSizeChoice::Small,
         offset_choice: TextOffsetChoice::Diagram,
         anchor_choice: TextAnchorChoice::ThreeByThree(
            TextAnchorHorizontal::Center,
            TextAnchorVertical::Top,
         ),
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
         axes_style: AxesStyle::Boxed,
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
         axes_style: AxesStyle::Boxed,
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
         segment_choices: SegmentChoices {
            color: ColorChoice::Red,
            // line_closure_choice: LineClosureChoice::Open,
            ..Default::default()
         },
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
         segment_choices: SegmentChoices {
            color: ColorChoice::Green,
            closure: LineClosureChoice::Closes,
            ..Default::default()
         },
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
         axes_style: AxesStyle::Boxed,
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

   let managed_curve =
      ManagedRatQuad::create_from_polynomial(&rat_quad, cairo_spartan.spartan.prep.axes_range);
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
         axes_style: AxesStyle::Boxed,
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

   let managed_curve =
      ManagedRatQuad::create_from_polynomial(&rat_quad, cairo_spartan.spartan.prep.axes_range);
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
         axes_style: AxesStyle::Boxed,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let mut managed_curve = ManagedRatQuad::create_from_polynomial(
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
         axes_style: AxesStyle::Boxed,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let mut managed_curve = ManagedRatQuad::create_from_polynomial(
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
         axes_style: AxesStyle::Boxed,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let mut managed_curve = ManagedRatQuad::create_from_polynomial(
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
         axes_style: AxesStyle::Boxed,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let mut managed_curve = ManagedRatQuad::create_from_polynomial(
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

#[test]
fn spartan_sizing_p_test() {
   let t_range = [-6.0, 14.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::Fill,
      canvas_size: [400.0, 300.0],
      axes_range: vec![-4.5, -2.5, 1.5, 2.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Boxed,
         grid_interval: [1.0, 1.0],
         grid_precision: vec![1],
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   let mut managed_curve = ManagedRatQuad::create_from_polynomial(
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
         axes_style: AxesStyle::Boxed,
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

#[test]
fn spartan_sizing_r_test() {
   // let t_range = [-1.0, 1.0];
   let t_range = [-1.0, 11.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [600.0, 350.0],
      axes_range: vec![-4.5, -3.5, 7.5, 4.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Boxed,
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

   {
      let shift_x = -3.0;
      let shift_y = -2.0;
      let h = 0.005;
      let mut managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [1.0 + shift_x, 1.0 + shift_x, -1.0 + h + shift_x, -1.0 + shift_x],
            y: [-1.0 + shift_y, -1.0 + h + shift_y, 1.0 + shift_y, 1.0 + shift_y],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            control_color: Some(ColorChoice::YellowBrown),
            points_num_segments: 12,
            ..Default::default()
         },
      );
   }

   {
      let shift_x = -2.0;
      let shift_y = -1.0;
      let h = 0.5;
      let mut managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [1.0 + shift_x, 1.0 + shift_x, -1.0 + h + shift_x, -1.0 + shift_x],
            y: [-1.0 + shift_y, -1.0 + h + shift_y, 1.0 + shift_y, 1.0 + shift_y],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            control_color: Some(ColorChoice::YellowBrown),
            points_num_segments: 12,
            ..Default::default()
         },
      );
   }

   {
      let s = 2.0;
      let h = s * 2.0_f64.sqrt() / 3.0;

      let shift_x = -1.0;
      let shift_y = 0.0;
      let mut managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [1.0 + shift_x, 1.0 + shift_x, -1.0 + h + shift_x, -1.0 + shift_x],
            y: [-1.0 + shift_y, -1.0 + h + shift_y, 1.0 + shift_y, 1.0 + shift_y],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            control_color: Some(ColorChoice::YellowBrown),
            ..Default::default()
         },
      );
   }

   {
      let shift_x = 0.0;
      let shift_y = 1.0;
      let h = 4.0 / 3.0;
      let mut managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [1.0 + shift_x, 1.0 + shift_x, -1.0 + h + shift_x, -1.0 + shift_x],
            y: [-1.0 + shift_y, -1.0 + h + shift_y, 1.0 + shift_y, 1.0 + shift_y],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            control_color: Some(ColorChoice::YellowBrown),
            ..Default::default()
         },
      );
   }

   {
      let shift_x = 1.0;
      let shift_y = 2.0;
      let h = 3.0;
      let mut managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [1.0 + shift_x, 1.0 + shift_x, -1.0 + h + shift_x, -1.0 + shift_x],
            y: [-1.0 + shift_y, -1.0 + h + shift_y, 1.0 + shift_y, 1.0 + shift_y],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            control_color: Some(ColorChoice::YellowBrown),
            ..Default::default()
         },
      );
   }

   //

   {
      let shift_x = 2.0;
      let shift_y = -2.0;
      let h = 0.005;
      let mut managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [1.0 + shift_x, 1.0 + h + shift_x, -1.0 + h + shift_x, -1.0 + shift_x],
            y: [-1.0 + shift_y, -1.0 + h + shift_y, 1.0 + h + shift_y, 1.0 + shift_y],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            control_color: Some(ColorChoice::YellowBrown),
            points_num_segments: 12,
            ..Default::default()
         },
      );
   }

   {
      let shift_x = 3.0;
      let shift_y = -1.0;
      let h = 1.0 / 3.0;
      let mut managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [1.0 + shift_x, 1.0 + h + shift_x, -1.0 + h + shift_x, -1.0 + shift_x],
            y: [-1.0 + shift_y, -1.0 + h + shift_y, 1.0 + h + shift_y, 1.0 + shift_y],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            control_color: Some(ColorChoice::YellowBrown),
            points_num_segments: 12,
            ..Default::default()
         },
      );
   }

   {
      let h = 2.0 / 3.0;

      let shift_x = 4.0;
      let shift_y = 0.0;
      let mut managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [1.0 + shift_x, 1.0 + h + shift_x, -1.0 + h + shift_x, -1.0 + shift_x],
            y: [-1.0 + shift_y, -1.0 + h + shift_y, 1.0 + h + shift_y, 1.0 + shift_y],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            control_color: Some(ColorChoice::YellowBrown),
            ..Default::default()
         },
      );
   }

   {
      let shift_x = 5.0;
      let shift_y = 1.0;
      let h = 1.0;
      let mut managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [1.0 + shift_x, 1.0 + h + shift_x, -1.0 + h + shift_x, -1.0 + shift_x],
            y: [-1.0 + shift_y, -1.0 + h + shift_y, 1.0 + h + shift_y, 1.0 + shift_y],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            control_color: Some(ColorChoice::YellowBrown),
            ..Default::default()
         },
      );
   }

   run_json_svg("spartan_sizing_r", &mut cairo_spartan);
}

#[test]
fn spartan_sizing_s_test() {
   // let t_range = [-1.0, 1.0];
   let t_range = [-6.0, 14.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [400.0, 300.0],
      axes_range: vec![-4.5, -3.5, 4.5, 3.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Boxed,
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

   let s = 2.5;

   {
      let h = s * 2.0_f64.sqrt() / 3.0;
      let shift_x = -3.5;
      let shift_y = 0.0;
      let managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [s + shift_x, s + shift_x, h + shift_x, 0.0 + shift_x],
            y: [0.0 + shift_y, h + shift_y, s + shift_y, s + shift_y],
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
            points_color: None,
            points_num_segments: 12,
            approx_num_segments: 30,
            ..Default::default()
         },
      );
   }

   {
      let h = s * 2.0_f64.sqrt() / 3.0;
      let shift_x = -3.5;
      let shift_y = 0.0;
      let managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [
               0.8 * s + shift_x,
               0.8 * s + 0.6 * h + shift_x,
               0.6 * s + 0.8 * h + shift_x,
               0.6 * s + shift_x,
            ],
            y: [
               -0.6 * s + shift_y,
               -0.6 * s + 0.8 * h + shift_y,
               0.8 * s - 0.6 * h + shift_y,
               0.8 * s + shift_y,
            ],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );
      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: None,
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            points_num_segments: 12,
            approx_num_segments: 30,
            ..Default::default()
         },
      );

      {
         let h = s * 2.0 / 3.0;
         let shift_x = 0.5;
         let shift_y = 0.0;
         let managed_curve = ManagedRatQuad::create_from_four_points(
            &FourPointRatQuad {
               x: [0.0 + shift_x, 0.0 + h + shift_x, 0.0 + h + shift_x, 0.0 + shift_x],
               y: [-s + shift_y, -s + shift_y, s + shift_y, s + shift_y],
               // x: [0.0, 1.0, 1.0, 0.0],
               // y: [-1.0, -1.0, 1.0, 1.0],
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
               points_choice: PointChoice::Dot,
               points_num_segments: 12,
               approx_num_segments: 30,
               ..Default::default()
            },
         );
      }

      // managed_curve.raise_to_symmetric_range().unwrap();
      // managed_curve.raise_to_regularized_symmetric().unwrap();
      // managed_curve.raise_to_offset_odd_even().unwrap();

      // draw_sample_rat_quad(
      //    &managed_curve,
      //    &mut cairo_spartan.spartan,
      //    &SampleCurveConfig {
      //       main_color: Some(ColorChoice::Green),
      //       points_color: None,
      //       ..Default::default()
      //    },
      // );
   }

   run_json_svg("spartan_sizing_s", &mut cairo_spartan);
}

// Parabola test, with most plotted via RatQuad, but confirmatory circles via cubic.
#[test]
fn spartan_sizing_t_test() {
   // let t_range = [-1.0, 1.0];
   let t_range = [-2.0, 10.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [400.0, 300.0],
      axes_range: vec![-3.5, -2.5, 3.5, 2.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Boxed,
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

   let x_a = [-3.0, -1.0, -1.0, -3.0];
   let y_a = [-2.0, 0.0, 0.0, 2.0];

   let x_b = [0.0, 1.0, 1.0, -1.0];
   let y_b = [-2.0, 1.0, 1.0, 2.0];

   let x_c = [3.0, 3.0, 3.0, 1.0];
   let y_c = [-2.0, 2.0, 2.0, 2.0];

   {
      let managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            // Quarter-circle temporary test.
            // x: [-3.0, -2.5, -2.5, -3.0],
            // y: [-2.0, -1.5, 1.5, 2.0],
            x: [x_a[0], (x_a[0] + 2.0 * x_a[1]) / 3.0, (2.0 * x_a[2] + x_a[3]) / 3.0, x_a[3]],
            y: [y_a[0], (y_a[0] + 2.0 * y_a[1]) / 3.0, (2.0 * y_a[2] + y_a[3]) / 3.0, y_a[3]],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );
      // Switch from approx when implemented!!!!!!!
      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::BrightGreen),
            points_choice: PointChoice::Dot,
            points_num_segments: 12,
            approx_num_segments: 30,
            ..Default::default()
         },
      );
   }
   {
      let managed_curve = ManagedCubic::create_from_control_points(
         &CubiLinear {
            r: t_range,
            x: [x_a[0], (x_a[0] + 2.0 * x_a[1]) / 3.0, (2.0 * x_a[2] + x_a[3]) / 3.0, x_a[3]],
            y: [y_a[0], (y_a[0] + 2.0 * y_a[1]) / 3.0, (2.0 * y_a[2] + y_a[3]) / 3.0, y_a[3]],
            sigma: 1.0,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );
      draw_sample_cubilinear(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: None,
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            ..Default::default()
         },
      );
   }

   {
      let managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [x_b[0], (x_b[0] + 2.0 * x_b[1]) / 3.0, (2.0 * x_b[2] + x_b[3]) / 3.0, x_b[3]],
            y: [y_b[0], (y_b[0] + 2.0 * y_b[1]) / 3.0, (2.0 * y_b[2] + y_b[3]) / 3.0, y_b[3]],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );
      // Switch from approx when implemented!!!!!!!
      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::BrightGreen),
            points_choice: PointChoice::Dot,
            points_num_segments: 12,
            approx_num_segments: 30,
            ..Default::default()
         },
      );
   }
   {
      let managed_curve = ManagedCubic::create_from_control_points(
         &CubiLinear {
            r: t_range,
            x: [x_b[0], (x_b[0] + 2.0 * x_b[1]) / 3.0, (2.0 * x_b[2] + x_b[3]) / 3.0, x_b[3]],
            y: [y_b[0], (y_b[0] + 2.0 * y_b[1]) / 3.0, (2.0 * y_b[2] + y_b[3]) / 3.0, y_b[3]],
            sigma: 1.0,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );
      draw_sample_cubilinear(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: None,
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            ..Default::default()
         },
      );
   }

   {
      let managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad {
            x: [x_c[0], (x_c[0] + 2.0 * x_c[1]) / 3.0, (2.0 * x_c[2] + x_c[3]) / 3.0, x_c[3]],
            y: [y_c[0], (y_c[0] + 2.0 * y_c[1]) / 3.0, (2.0 * y_c[2] + y_c[3]) / 3.0, y_c[3]],
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );
      // Switch from approx when implemented!!!!!!!
      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::BrightGreen),
            points_choice: PointChoice::Dot,
            points_num_segments: 12,
            approx_num_segments: 30,
            ..Default::default()
         },
      );
   }
   {
      let managed_curve = ManagedCubic::create_from_control_points(
         &CubiLinear {
            r: t_range,
            x: [x_c[0], (x_c[0] + 2.0 * x_c[1]) / 3.0, (2.0 * x_c[2] + x_c[3]) / 3.0, x_c[3]],
            y: [y_c[0], (y_c[0] + 2.0 * y_c[1]) / 3.0, (2.0 * y_c[2] + y_c[3]) / 3.0, y_c[3]],
            sigma: 1.0,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );
      draw_sample_cubilinear(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: None,
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Circle,
            ..Default::default()
         },
      );
   }

   run_json_svg("spartan_sizing_t", &mut cairo_spartan);
}

fn rotate_3_simply(p: ([f64; 3], [f64; 3]), transformation: [f64; 4]) -> ([f64; 3], [f64; 3]) {
   let t = &transformation;
   let x = &p.0;
   let y = &p.1;
   let result_x = [t[0] * x[0] + t[1] * y[0], t[0] * x[1] + t[1] * y[1], t[0] * x[2] + t[1] * y[2]];
   let result_y = [t[2] * x[0] + t[3] * y[0], t[2] * x[1] + t[3] * y[1], t[2] * x[2] + t[3] * y[2]];
   (result_x, result_y)
}

fn translate_3_simply(p: ([f64; 3], [f64; 3]), offset: [f64; 2]) -> ([f64; 3], [f64; 3]) {
   let t = &offset;
   let x = &p.0;
   let y = &p.1;
   let result_x = [x[0] + t[0], x[1] + t[0], x[2] + t[0]];
   let result_y = [y[0] + t[1], y[1] + t[1], y[2] + t[1]];
   (result_x, result_y)
}

fn rotate_4_simply(p: ([f64; 4], [f64; 4]), transformation: [f64; 4]) -> ([f64; 4], [f64; 4]) {
   let t = &transformation;
   let x = &p.0;
   let y = &p.1;
   let result_x = [
      t[0] * x[0] + t[1] * y[0],
      t[0] * x[1] + t[1] * y[1],
      t[0] * x[2] + t[1] * y[2],
      t[0] * x[3] + t[1] * y[3],
   ];
   let result_y = [
      t[2] * x[0] + t[3] * y[0],
      t[2] * x[1] + t[3] * y[1],
      t[2] * x[2] + t[3] * y[2],
      t[2] * x[3] + t[3] * y[3],
   ];
   (result_x, result_y)
}

fn translate_4_simply(p: ([f64; 4], [f64; 4]), offset: [f64; 2]) -> ([f64; 4], [f64; 4]) {
   let t = &offset;
   let x = &p.0;
   let y = &p.1;
   let result_x = [x[0] + t[0], x[1] + t[0], x[2] + t[0], x[3] + t[0]];
   let result_y = [y[0] + t[1], y[1] + t[1], y[2] + t[1], y[3] + t[1]];
   (result_x, result_y)
}

#[test]
fn spartan_sizing_u_test() {
   // let t_range = [-1.0, 1.0];
   let t_range = [-1.0, 11.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [400.0, 350.0],
      axes_range: vec![-4.5, -4.5, 5.5, 4.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Boxed,
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

   // Need to enable: fix ratquad rendering that is parabolic.

   {
      let shift_x = -2.0;
      let shift_y = -4.0;
      let (x, y) = translate_3_simply(
         rotate_3_simply(([1.0, 0.0, -1.0], [-1.0, 0.0, 1.0]), [1.0, -1.0, 1.0, 1.0]),
         [shift_x, shift_y],
      );
      let mut managed_curve = ManagedRatQuad::create_from_three_points(
         &BaseRatQuad {
            state: RatQuadState::ThreePointAngle,
            b: x,
            c: y,
            angle: ZebraixAngle::Quadrant(0.5),
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            control_color: Some(ColorChoice::YellowBrown),
            ..Default::default()
         },
      );
   }

   {
      let shift_x = -2.0;
      let shift_y = -3.0;
      let (x, y) = translate_3_simply(
         rotate_3_simply(([1.0, 0.5, -1.0], [-1.0, 0.5, 1.0]), [1.0, -1.0, 1.0, 1.0]),
         [shift_x, shift_y],
      );
      let mut managed_curve = ManagedRatQuad::create_from_three_points(
         &BaseRatQuad {
            state: RatQuadState::ThreePointAngle,
            b: x,
            c: y,
            angle: ZebraixAngle::Quadrant(0.5),
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            control_color: Some(ColorChoice::YellowBrown),
            ..Default::default()
         },
      );
   }

   {
      let shift_x = -2.0;
      let shift_y = -1.0;
      let (x, y) = translate_3_simply(
         rotate_3_simply(([1.0, 1.0, -1.0], [-1.0, 1.0, 1.0]), [1.0, -1.0, 1.0, 1.0]),
         [shift_x, shift_y],
      );
      let mut managed_curve = ManagedRatQuad::create_from_three_points(
         &BaseRatQuad {
            state: RatQuadState::ThreePointAngle,
            b: x,
            c: y,
            angle: ZebraixAngle::Quadrant(0.5),
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            control_color: Some(ColorChoice::YellowBrown),
            ..Default::default()
         },
      );
   }

   {
      let shift_x = -2.0;
      let shift_y = 1.0;
      let (x, y) = translate_3_simply(
         rotate_3_simply(([1.0, 1.5, -1.0], [-1.0, 1.5, 1.0]), [1.0, -1.0, 1.0, 1.0]),
         [shift_x, shift_y],
      );
      let mut managed_curve = ManagedRatQuad::create_from_three_points(
         &BaseRatQuad {
            state: RatQuadState::ThreePointAngle,
            b: x,
            c: y,
            angle: ZebraixAngle::Quadrant(0.5),
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            control_color: Some(ColorChoice::YellowBrown),
            ..Default::default()
         },
      );
   }

   {
      let shift_x = 3.0;
      let shift_y = 3.0;
      let (x, y) = translate_3_simply(
         rotate_3_simply(([1.0, 1.0, -1.0], [-1.0, 1.0, 1.0]), [0.5, -0.5, 0.5, 0.5]),
         [shift_x, shift_y],
      );
      let mut managed_curve = ManagedRatQuad::create_from_three_points(
         &BaseRatQuad {
            state: RatQuadState::ThreePointAngle,
            b: x,
            c: y,
            angle: ZebraixAngle::Quadrant(0.05),
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: None,
            ..Default::default()
         },
      );
   }

   {
      let shift_x = 3.0;
      let shift_y = 2.0;
      let (x, y) = translate_3_simply(
         rotate_3_simply(([1.0, 1.0, -1.0], [-1.0, 1.0, 1.0]), [0.5, -0.5, 0.5, 0.5]),
         [shift_x, shift_y],
      );
      let mut managed_curve = ManagedRatQuad::create_from_three_points(
         &BaseRatQuad {
            state: RatQuadState::ThreePointAngle,
            b: x,
            c: y,
            angle: ZebraixAngle::Quadrant(0.5),
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: None,
            ..Default::default()
         },
      );
   }

   {
      let shift_x = 3.0;
      let shift_y = 1.0;
      let (x, y) = translate_3_simply(
         rotate_3_simply(([1.0, 1.0, -1.0], [-1.0, 1.0, 1.0]), [0.5, -0.5, 0.5, 0.5]),
         [shift_x, shift_y],
      );
      let mut managed_curve = ManagedRatQuad::create_from_three_points(
         &BaseRatQuad {
            state: RatQuadState::ThreePointAngle,
            b: x,
            c: y,
            angle: ZebraixAngle::Quadrant(0.75),
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: None,
            ..Default::default()
         },
      );
   }

   {
      let shift_x = 3.0;
      let shift_y = 0.0;
      let (x, y) = translate_3_simply(
         rotate_3_simply(([1.0, 1.0, -1.0], [-1.0, 1.0, 1.0]), [0.5, -0.5, 0.5, 0.5]),
         [shift_x, shift_y],
      );
      let mut managed_curve = ManagedRatQuad::create_from_three_points(
         &BaseRatQuad {
            state: RatQuadState::ThreePointAngle,
            b: x,
            c: y,
            angle: ZebraixAngle::Quadrant(1.25),
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: None,
            ..Default::default()
         },
      );
   }

   {
      let shift_x = 3.0;
      let shift_y = -1.0;
      let (x, y) = translate_3_simply(
         rotate_3_simply(([1.0, 1.0, -1.0], [-1.0, 1.0, 1.0]), [0.5, -0.5, 0.5, 0.5]),
         [shift_x, shift_y],
      );
      let mut managed_curve = ManagedRatQuad::create_from_three_points(
         &BaseRatQuad {
            state: RatQuadState::ThreePointAngle,
            b: x,
            c: y,
            angle: ZebraixAngle::Quadrant(1.5),
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: None,
            ..Default::default()
         },
      );
   }

   run_json_svg("spartan_sizing_u", &mut cairo_spartan);
}

#[test]
fn spartan_sizing_v_test() {
   // let t_range = [-1.0, 1.0];
   let t_range = [-1.0, 11.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [400.0, 350.0],
      axes_range: vec![-4.5, -3.5, 2.5, 3.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Boxed,
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

   let p_d = [1.5, 0.5];
   let p_a = [0.0, 3.0];
   let p_z = [0.0, -3.0];

   let t = 1.0 / 2.0;
   let alpha = t;
   let beta = 1.0 / t;

   let f = 1.0 / (alpha + beta);
   let p_m = [
      f * (2.0 * alpha * beta * p_d[0] + beta * p_a[0] + alpha * p_z[0]),
      f * (2.0 * alpha * beta * p_d[1] + beta * p_a[1] + alpha * p_z[1]),
   ];

   let rotation = [-0.25, 1.0, 1.0, 0.25];
   let shift = [-1.0, 0.0];
   {
      let (x, y) = translate_3_simply(
         rotate_3_simply(
            ([p_a[0], p_a[0] + alpha * p_d[0], p_m[0]], [p_a[1], p_a[1] + alpha * p_d[1], p_m[1]]),
            rotation,
         ),
         shift,
      );

      let mut managed_curve = ManagedRatQuad::create_from_three_points(
         &BaseRatQuad {
            state: RatQuadState::ThreePointAngle,
            b: x,
            c: y,
            angle: ZebraixAngle::Radians(t.atan()),
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            // main_color: None,
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            // points_choice: PointChoice::Circle,
            control_color: Some(ColorChoice::YellowBrown),
            ..Default::default()
         },
      );
   }

   {
      let (x, y) = translate_3_simply(
         rotate_3_simply(
            ([p_m[0], p_z[0] + beta * p_d[0], p_z[0]], [p_m[1], p_z[1] + beta * p_d[1], p_z[1]]),
            rotation,
         ),
         shift,
      );

      let mut managed_curve = ManagedRatQuad::create_from_three_points(
         &BaseRatQuad {
            state: RatQuadState::ThreePointAngle,
            b: x,
            c: y,
            angle: ZebraixAngle::Radians((1.0 / t).atan()),
            r: t_range,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );

      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            control_color: Some(ColorChoice::YellowBrown),
            ..Default::default()
         },
      );
   }

   {
      let g = -2.0 / 3.0;
      let (x, y) = translate_4_simply(
         rotate_4_simply(
            (
               [p_a[0], p_a[0] + g * p_d[0], p_z[0] + g * p_d[0], p_z[0]],
               [p_a[1], p_a[1] + g * p_d[1], p_z[1] + g * p_d[1], p_z[1]],
            ),
            rotation,
         ),
         shift,
      );

      let mut managed_curve = ManagedRatQuad::create_from_four_points(
         &FourPointRatQuad { x, y, r: t_range, ..Default::default() },
         cairo_spartan.spartan.prep.axes_range,
      );
      managed_curve.raise_to_symmetric_range().unwrap();
      managed_curve.raise_to_regularized_symmetric().unwrap();
      managed_curve.raise_to_offset_odd_even().unwrap();

      draw_sample_rat_quad(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::RedRedBlue),
            // main_line_choice: LineChoice::Light,
            points_color: None,
            ..Default::default()
         },
      );
   }

   run_json_svg("spartan_sizing_v", &mut cairo_spartan);
}

// Closed polyline test.
#[test]
fn spartan_sizing_w_test() {
   // let t_range = [-1.0, 1.0];
   let t_range = [-1.0, 11.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [400.0, 350.0],
      axes_range: vec![-4.5, -3.5, 2.5, 3.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Boxed,
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

   {
      let managed_curve = ManagedCubic::create_from_control_points(
         &CubiLinear {
            r: t_range,
            x: [0.0, 1.0, -1.0, 0.0],
            y: [2.0, 0.0, 0.0, -2.0],
            sigma: 1.0,
            ..Default::default()
         },
         cairo_spartan.spartan.prep.axes_range,
      );
      draw_sample_cubilinear(
         &managed_curve,
         &mut cairo_spartan.spartan,
         &SampleCurveConfig {
            main_color: Some(ColorChoice::Green),
            points_color: Some(ColorChoice::Blue),
            points_choice: PointChoice::Dot,
            ..Default::default()
         },
      );
   }

   run_json_svg("spartan_sizing_w", &mut cairo_spartan);
}

// Mid-range tangent.
#[test]
fn spartan_sizing_x_test() {
   // let t_range = [-1.0, 1.0];
   let t_range = [-3.0, 3.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [600.0, 350.0],
      axes_range: vec![-3.2, -3.5, 9.8, 3.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Boxed,
         grid_interval: [6.0, 100.0],
         grid_precision: vec![1],
         axis_numbering: AxisNumbering::None,
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   {
      let c_x = 2.0;
      let a_x = -0.2;
      let a_y = 1.0;
      let b_x = 1.2;
      let b_y = 2.4;
      let shift = [0.0, 0.0];
      {
         let (x, y) = translate_4_simply(
            ([-c_x + a_x, -c_x - a_x, c_x - b_x, c_x + b_x], [a_y, -a_y, -b_y, b_y]),
            shift,
         );

         let managed_curve = ManagedCubic::create_from_control_points(
            &CubiLinear { r: t_range, x, y, sigma: 1.0, ..Default::default() },
            cairo_spartan.spartan.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve,
            &mut cairo_spartan.spartan,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               points_color: Some(ColorChoice::Blue),
               points_choice: PointChoice::Circle,
               control_color: Some(ColorChoice::YellowBrown),
               control_point_choices: [PointChoice::Circle, PointChoice::Plus],
               points_num_segments: 2,
               ..Default::default()
            },
         );
      }
      {
         let qualified_drawable = QualifiedDrawable {
            drawable: OneOfDrawable::Lines(LinesDrawable {
               line_choice: LineChoice::Ordinary,
               color_choice: ColorChoice::Red,
               coords: vec![(
                  [0.5 * (a_x + b_x), 0.5 * (a_y + b_y)],
                  [-0.5 * (a_x + b_x), -0.5 * (a_y + b_y)],
               )],
               offsets: Some(vec![[0.0 + shift[0], 0.0 + shift[1]]]),
               ..Default::default()
            }),
            ..Default::default()
         };
         cairo_spartan.spartan.drawables.push(qualified_drawable);
      }
      {
         let qualified_drawable = QualifiedDrawable {
            drawable: OneOfDrawable::Lines(LinesDrawable {
               line_choice: LineChoice::Light,
               coords: vec![
                  ([-c_x + a_x, a_y], [c_x + b_x, b_y]),
                  ([-c_x - a_x, -a_y], [c_x - b_x, -b_y]),
               ],
               offsets: Some(vec![[0.0 + shift[0], 0.0 + shift[1]]]),
               ..Default::default()
            }),
            ..Default::default()
         };
         cairo_spartan.spartan.drawables.push(qualified_drawable);
      }
      {
         let qualified_drawable = QualifiedDrawable {
            drawable: OneOfDrawable::Lines(LinesDrawable {
               line_choice: LineChoice::Ordinary,
               color_choice: ColorChoice::Blue,
               coords: vec![([-c_x, 0.0], [c_x, 0.0])],
               offsets: Some(vec![
                  [0.0 + shift[0], 0.0 + shift[1]],
                  [-0.25 * (a_x + b_x) + shift[0], -0.25 * (a_y + b_y) + shift[1]],
               ]),
               ..Default::default()
            }),
            ..Default::default()
         };
         cairo_spartan.spartan.drawables.push(qualified_drawable);
      }
   }

   {
      let c_x = 2.0;
      let a_x = -0.2;
      let a_y = 1.0;
      let b_x = 1.2;
      let b_y = 2.4;
      let shift = [6.0, 0.0];
      {
         let (x, y) = translate_4_simply(
            ([-c_x + a_x, -c_x - a_x, c_x - b_x, c_x + b_x], [a_y, -a_y, -b_y, b_y]),
            shift,
         );

         let managed_curve = ManagedCubic::create_from_control_points(
            &CubiLinear { r: t_range, x, y, sigma: 1.0, ..Default::default() },
            cairo_spartan.spartan.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve,
            &mut cairo_spartan.spartan,
            &SampleCurveConfig {
               main_color: None,
               points_color: Some(ColorChoice::Red),
               points_num_segments: 2,
               control_color: Some(ColorChoice::YellowBrown),
               control_point_choices: [PointChoice::Circle, PointChoice::Plus],
               ..Default::default()
            },
         );
      }
      {
         let (x, y) = translate_4_simply(
            (
               [-c_x + a_x, -c_x - a_x, c_x - 0.0 * b_x, c_x + 0.0 * b_x],
               [a_y, -a_y, -0.0 * b_y, 0.0 * b_y],
            ),
            shift,
         );

         let managed_curve = ManagedCubic::create_from_control_points(
            &CubiLinear { r: t_range, x, y, sigma: 1.0, ..Default::default() },
            cairo_spartan.spartan.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve,
            &mut cairo_spartan.spartan,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               points_color: Some(ColorChoice::Green),
               points_num_segments: 2,
               ..Default::default()
            },
         );
      }
      {
         let (x, y) = translate_4_simply(
            (
               [-c_x + 0.0 * a_x, -c_x - 0.0 * a_x, c_x - b_x, c_x + b_x],
               [0.0 * a_y, -0.0 * a_y, -b_y, b_y],
            ),
            shift,
         );

         let managed_curve = ManagedCubic::create_from_control_points(
            &CubiLinear { r: t_range, x, y, sigma: 1.0, ..Default::default() },
            cairo_spartan.spartan.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve,
            &mut cairo_spartan.spartan,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Blue),
               points_color: Some(ColorChoice::Blue),
               points_num_segments: 2,
               ..Default::default()
            },
         );
      }
      {
         let qualified_drawable = QualifiedDrawable {
            drawable: OneOfDrawable::Lines(LinesDrawable {
               line_choice: LineChoice::Ordinary,
               color_choice: ColorChoice::Red,
               coords: vec![([-c_x, 0.0], [c_x, 0.0])],
               offsets: Some(vec![
                  [0.0 + shift[0], 0.0 + shift[1]],
                  // [-0.25 * (a_x + b_x) + shift[0], -0.25 * (a_y + b_y) + shift[1]],
               ]),
               ..Default::default()
            }),
            ..Default::default()
         };
         cairo_spartan.spartan.drawables.push(qualified_drawable);
      }
   }

   run_json_svg("spartan_sizing_x", &mut cairo_spartan);
}

// Mid-range tangent.
#[test]
fn spartan_sizing_y_test() {
   // let t_range = [-1.0, 1.0];
   let t_range = [-3.0, 3.0];

   let sizing = TestSizing {
      sizing_scheme: SizingScheme::SquareCenter,
      canvas_size: [600.0, 350.0],
      axes_range: vec![-3.2, -3.5, 9.8, 3.5],
      padding: vec![0.05],
      axes_spec: AxesSpec {
         axes_style: AxesStyle::Boxed,
         grid_interval: [6.0, 100.0],
         grid_precision: vec![1],
         axis_numbering: AxisNumbering::None,
         ..Default::default()
      },
      ..Default::default()
   };

   let mut cairo_spartan = create_sized_diagram(&sizing);
   cairo_spartan.spartan.prepare();
   sizing.axes_spec.generate_axes(&mut cairo_spartan.spartan);

   {
      let c_x = 2.0;
      let a_x = -0.2;
      let a_y = 1.0;
      let b_x = -1.2;
      let b_y = -2.4;
      let shift = [0.0, 0.0];
      {
         let (x, y) = translate_4_simply(
            ([-c_x + a_x, -c_x - a_x, c_x - b_x, c_x + b_x], [a_y, -a_y, -b_y, b_y]),
            shift,
         );

         let managed_curve = ManagedCubic::create_from_control_points(
            &CubiLinear { r: t_range, x, y, sigma: 1.0, ..Default::default() },
            cairo_spartan.spartan.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve,
            &mut cairo_spartan.spartan,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               points_color: Some(ColorChoice::Blue),
               points_choice: PointChoice::Circle,
               control_color: Some(ColorChoice::YellowBrown),
               control_point_choices: [PointChoice::Circle, PointChoice::Plus],
               points_num_segments: 2,
               ..Default::default()
            },
         );
      }
      {
         let qualified_drawable = QualifiedDrawable {
            drawable: OneOfDrawable::Lines(LinesDrawable {
               line_choice: LineChoice::Ordinary,
               color_choice: ColorChoice::Red,
               coords: vec![(
                  [0.5 * (a_x + b_x), 0.5 * (a_y + b_y)],
                  [-0.5 * (a_x + b_x), -0.5 * (a_y + b_y)],
               )],
               offsets: Some(vec![[0.0 + shift[0], 0.0 + shift[1]]]),
               ..Default::default()
            }),
            ..Default::default()
         };
         cairo_spartan.spartan.drawables.push(qualified_drawable);
      }
      {
         let qualified_drawable = QualifiedDrawable {
            drawable: OneOfDrawable::Lines(LinesDrawable {
               line_choice: LineChoice::Light,
               coords: vec![
                  ([-c_x + a_x, a_y], [c_x + b_x, b_y]),
                  ([-c_x - a_x, -a_y], [c_x - b_x, -b_y]),
               ],
               offsets: Some(vec![[0.0 + shift[0], 0.0 + shift[1]]]),
               ..Default::default()
            }),
            ..Default::default()
         };
         cairo_spartan.spartan.drawables.push(qualified_drawable);
      }
      {
         let qualified_drawable = QualifiedDrawable {
            drawable: OneOfDrawable::Lines(LinesDrawable {
               line_choice: LineChoice::Ordinary,
               color_choice: ColorChoice::Blue,
               coords: vec![([-c_x, 0.0], [c_x, 0.0])],
               offsets: Some(vec![
                  [0.0 + shift[0], 0.0 + shift[1]],
                  [-0.25 * (a_x + b_x) + shift[0], -0.25 * (a_y + b_y) + shift[1]],
               ]),
               ..Default::default()
            }),
            ..Default::default()
         };
         cairo_spartan.spartan.drawables.push(qualified_drawable);
      }
   }

   {
      let c_x = 1.0;
      let c_y = 0.75;
      let a_x = -1.0;
      let a_y = c_y;
      let b_x = 1.0;
      let b_y = -c_y;
      let shift = [6.0, 0.0];
      {
         let (x, y) = translate_4_simply(
            (
               [-c_x + a_x, -c_x - a_x, c_x - b_x, c_x + b_x],
               [-c_y + a_y, -c_y - a_y, c_y - b_y, c_y + b_y],
            ),
            shift,
         );

         let managed_curve = ManagedCubic::create_from_control_points(
            &CubiLinear { r: t_range, x, y, sigma: 1.0, ..Default::default() },
            cairo_spartan.spartan.prep.axes_range,
         );
         draw_sample_cubilinear(
            &managed_curve,
            &mut cairo_spartan.spartan,
            &SampleCurveConfig {
               main_color: Some(ColorChoice::Green),
               points_color: Some(ColorChoice::Blue),
               points_choice: PointChoice::Circle,
               control_color: Some(ColorChoice::YellowBrown),
               control_point_choices: [PointChoice::Circle, PointChoice::Plus],
               points_num_segments: 2,
               ..Default::default()
            },
         );
      }
      {
         let qualified_drawable = QualifiedDrawable {
            drawable: OneOfDrawable::Lines(LinesDrawable {
               line_choice: LineChoice::Ordinary,
               color_choice: ColorChoice::Red,
               coords: vec![(
                  [0.5 * (a_x + b_x), 0.5 * (a_y + b_y)],
                  [-0.5 * (a_x + b_x), -0.5 * (a_y + b_y)],
               )],
               offsets: Some(vec![[0.0 + shift[0], 0.0 + shift[1]]]),
               ..Default::default()
            }),
            ..Default::default()
         };
         cairo_spartan.spartan.drawables.push(qualified_drawable);
      }
      // {
      //    let qualified_drawable = QualifiedDrawable {
      //       drawable: OneOfDrawable::Lines(LinesDrawable {
      //          line_choice: LineChoice::Light,
      //          coords: vec![([-c_x + a_x, a_y], [-c_x - a_x, -a_y], [c_x + b_x, b_y], [c_x - b_x, -b_y])],
      //          offsets: Some(vec![[0.0 + shift[0], 0.0 + shift[1]]]),
      //          ..Default::default()
      //       }),
      //       ..Default::default()
      //    };
      //    cairo_spartan.spartan.drawables.push(qualified_drawable);
      // }
      // {
      //    let qualified_drawable = QualifiedDrawable {
      //       drawable: OneOfDrawable::Lines(LinesDrawable {
      //          line_choice: LineChoice::Ordinary,
      //          color_choice: ColorChoice::Blue,
      //          coords: vec![([-c_x, 0.0],[c_x, 0.0])],
      //          offsets: Some(vec![
      //             [0.0 + shift[0], 0.0 + shift[1]],
      //             [-0.25 * (a_x + b_x) + shift[0], -0.25 * (a_y + b_y) + shift[1]],
      //          ]),
      //          ..Default::default()
      //       }),
      //       ..Default::default()
      //    };
      //    cairo_spartan.spartan.drawables.push(qualified_drawable);
      // }
   }

   run_json_svg("spartan_sizing_y", &mut cairo_spartan);
}
