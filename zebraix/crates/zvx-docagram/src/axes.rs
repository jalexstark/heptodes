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

// use crate::diagram::SpartanDiagram;
use crate::diagram::DrawableDiagram;
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use zvx_base::is_default;
use zvx_base::OneOfSegment;
use zvx_drawable::{
   LineChoice, LinesSetSet, OneOfDrawable, PathChoices, PathCompletion, QualifiedDrawable,
   SegmentSequence, Strokeable, TextAnchorChoice, TextAnchorHorizontal, TextAnchorVertical,
   TextDrawable, TextOffsetChoice, TextSingle, TextSizeChoice,
};

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum AxesStyle {
   #[default]
   None,
   Boxed,
   Cross,
   BoxCross,
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum AxisNumbering {
   #[default]
   At,
   Before,
   After,
   None,
}

#[derive(Debug, Serialize, DefaultFromSerde)]
#[allow(clippy::module_name_repetitions)]
pub struct AxesSpec {
   #[serde(skip_serializing_if = "is_default")]
   pub axes_style: AxesStyle,
   #[serde(skip_serializing_if = "is_default")]
   pub axis_numbering: AxisNumbering,
   #[serde(skip_serializing_if = "is_default")]
   pub grid_interval: [f64; 2],
   #[serde(skip_serializing_if = "is_default")]
   pub grid_precision: Vec<usize>,
}

impl AxesSpec {
   #[must_use]
   pub fn new(style: AxesStyle) -> Self {
      Self { axes_style: style, ..Default::default() }
   }

   #[must_use]
   fn add_grid_lines(
      &self,
      vertical_light: &mut Strokeable<LinesSetSet>,
      one_range: [f64; 2],
      horiz_interval: f64,
      x_tolerance: f64,
      has_vert_zero: bool,
      offset_pattern: [f64; 2],
   ) -> (Option<f64>, Option<f64>) {
      let left_numbering_location: Option<f64>;
      let right_numbering_location: Option<f64>;

      let is_boxy: bool = match self.axes_style {
         AxesStyle::BoxCross | AxesStyle::Boxed => true,
         AxesStyle::Cross | AxesStyle::None => false,
      };

      if horiz_interval == 0.0 {
         if self.axes_style == AxesStyle::None {
            left_numbering_location = None;
            right_numbering_location = None;
         } else {
            left_numbering_location = Some(one_range[0]);
            right_numbering_location = Some(one_range[1]);
         }
      } else {
         let (mut left_scan, mut right_scan) = if has_vert_zero {
            match self.axes_style {
               AxesStyle::Boxed | AxesStyle::None => (0.0, horiz_interval),
               AxesStyle::Cross | AxesStyle::BoxCross => (-horiz_interval, horiz_interval),
            }
         } else {
            let snapped_mid_range =
               (0.5 * (one_range[0] + one_range[1]) / horiz_interval).floor() * horiz_interval;
            (snapped_mid_range, snapped_mid_range + horiz_interval)
         };

         let mid_range = 0.5 * (left_scan + right_scan);
         let mut final_left_location = right_scan;
         let mut final_right_location = left_scan;
         // If the following assertions remain true after grid line scan, we have not found a
         // numbering location.
         assert!(final_left_location > mid_range);
         assert!(final_right_location < mid_range);

         let mut offsets = vertical_light.path.offsets.clone().unwrap_or_default();

         #[allow(clippy::while_float)]
         while left_scan > one_range[0] - x_tolerance {
            if !is_boxy || (left_scan > one_range[0] + x_tolerance) {
               offsets.push([left_scan * offset_pattern[0], left_scan * offset_pattern[1]]);
            }
            assert!(offsets.len() < 100);
            final_left_location = left_scan;
            left_scan -= horiz_interval;
         }

         #[allow(clippy::while_float)]
         while right_scan < one_range[1] + x_tolerance {
            if !is_boxy || (right_scan < one_range[1] - x_tolerance) {
               offsets.push([right_scan * offset_pattern[0], right_scan * offset_pattern[1]]);
            }
            assert!(offsets.len() < 100);
            final_right_location = right_scan;
            right_scan += horiz_interval;
         }
         vertical_light.path.offsets = Some(offsets);

         if final_left_location > mid_range {
            left_numbering_location = None;
         } else {
            left_numbering_location = Some(final_left_location);
         }
         if final_right_location < mid_range {
            right_numbering_location = None;
         } else {
            right_numbering_location = Some(final_right_location);
         }
      }
      (left_numbering_location, right_numbering_location)
   }

   #[allow(clippy::too_many_lines)]
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::cognitive_complexity)]
   #[allow(clippy::suboptimal_flops)]
   pub fn generate_axes(&self, diagram: &mut DrawableDiagram) {
      // Future possible enhancement would be to provide options for form of box.  Currently
      // this shrink-wraps to the axes-plus-padding.
      //
      // The line around the edge is brought in by a line width, leaving a very small gap
      // around the edge.  This is intentional, so that the line is never clipped.
      if let Some(box_choices) = &diagram.prep.background_box {
         // Create background box.
         let background_layer = -10;
         let left = diagram.prep.axes_range[0]
            - (diagram.prep.axes_range[2] - diagram.prep.axes_range[0]) * diagram.prep.padding[0]
            + diagram.prep.diagram_choices.line_width / diagram.prep.canvas_layout.scale[0];
         let right = diagram.prep.axes_range[2]
            + (diagram.prep.axes_range[2] - diagram.prep.axes_range[0]) * diagram.prep.padding[2]
            - diagram.prep.diagram_choices.line_width / diagram.prep.canvas_layout.scale[0];
         let bottom = diagram.prep.axes_range[1]
            - (diagram.prep.axes_range[3] - diagram.prep.axes_range[1]) * diagram.prep.padding[1]
            + diagram.prep.diagram_choices.line_width / diagram.prep.canvas_layout.scale[1];
         let top = diagram.prep.axes_range[3]
            + (diagram.prep.axes_range[3] - diagram.prep.axes_range[1]) * diagram.prep.padding[3]
            - diagram.prep.diagram_choices.line_width / diagram.prep.canvas_layout.scale[1];

         let qualified_drawable = QualifiedDrawable {
            layer: background_layer,
            drawable: OneOfDrawable::SegmentSequence(SegmentSequence {
               // This should be miter-join even if we switch default later.
               completion: PathCompletion::Closed,
               segments: vec![OneOfSegment::Polyline(vec![
                  [left, bottom],
                  [left, top],
                  [right, top],
                  [right, bottom],
               ])],
               path_choices: box_choices.clone(),
            }),
         };
         diagram.drawables.push(qualified_drawable);
      }

      // Future improvement ideas:
      //
      // * Generate box as closed polygon.
      if (self.axes_style == AxesStyle::None)
         && (self.grid_interval[0] == 0.0)
         && (self.grid_interval[1] == 0.0)
      {
         return;
      }
      let range = &diagram.prep.axes_range;
      assert!(range[2] > range[0]);
      assert!(range[3] > range[1]);
      let relative_tolerance = 1000.0;
      let x_tolerance = (range[2] - range[0]).abs() / relative_tolerance;
      let y_tolerance = (range[3] - range[1]).abs() / relative_tolerance;

      let has_vert_zero = (-range[0] > x_tolerance) && (range[2] > x_tolerance);
      let has_horiz_zero = (-range[1] > y_tolerance) && (range[3] > y_tolerance);

      let axes_layer = 0;
      let mut lines_ordinary = Strokeable::<LinesSetSet> {
         path: LinesSetSet { offsets: Some(vec![[0.0, 0.0]]), ..Default::default() },
         path_choices: PathChoices {
            color: diagram.prep.main_color_choice.clone(),
            ..Default::default()
         },
      };
      let mut horizontal_light = Strokeable::<LinesSetSet> {
         path: LinesSetSet {
            coords: vec![([range[0], 0.0], [range[2], 0.0])],
            offsets: Some(Vec::<[f64; 2]>::new()),
         },
         path_choices: PathChoices {
            line_choice: LineChoice::Light,
            color: diagram.prep.light_color_choice.clone(),
            ..Default::default()
         },
      };
      let mut vertical_light = Strokeable::<LinesSetSet> {
         path: LinesSetSet {
            coords: vec![([0.0, range[1]], [0.0, range[3]])],
            ..Default::default()
         },
         path_choices: PathChoices {
            line_choice: LineChoice::Light,
            color: diagram.prep.light_color_choice.clone(),
            ..Default::default()
         },
      };

      match self.axes_style {
         AxesStyle::BoxCross | AxesStyle::Boxed => {
            diagram.drawables.push(QualifiedDrawable {
               drawable: OneOfDrawable::SegmentSequence(SegmentSequence {
                  // This should be miter-join even if we switch default later.
                  completion: PathCompletion::Closed,
                  segments: vec![OneOfSegment::Polyline(vec![
                     [range[0], range[1]],
                     [range[0], range[3]],
                     [range[2], range[3]],
                     [range[2], range[1]],
                  ])],
                  path_choices: PathChoices::default(),
               }),
               ..Default::default()
            });
         }
         AxesStyle::Cross | AxesStyle::None => {}
      }

      match self.axes_style {
         AxesStyle::BoxCross | AxesStyle::Cross => {
            if has_vert_zero {
               lines_ordinary.path.coords.push(([0.0, range[1]], [0.0, range[3]]));
            }
            if has_horiz_zero {
               lines_ordinary.path.coords.push(([range[0], 0.0], [range[2], 0.0]));
            }
         }
         AxesStyle::Boxed | AxesStyle::None => {}
      }

      // Grid lines, horizontal interval, vertical lines.
      let (left_numbering_location, right_numbering_location) = self.add_grid_lines(
         &mut vertical_light,
         [range[0], range[2]],
         self.grid_interval[0],
         x_tolerance,
         has_vert_zero,
         [1.0, 0.0],
      );

      // Grid lines, vertical interval, horizontal lines.
      let (bottom_numbering_location, top_numbering_location) = self.add_grid_lines(
         &mut horizontal_light,
         [range[1], range[3]],
         self.grid_interval[1],
         y_tolerance,
         has_horiz_zero,
         [0.0, 1.0],
      );

      if !lines_ordinary.path.coords.is_empty() {
         let qualified_drawable = QualifiedDrawable {
            layer: axes_layer,
            // color_choice: diagram.prep.base_color_choice.clone(),
            drawable: OneOfDrawable::Lines(lines_ordinary),
         };
         diagram.drawables.push(qualified_drawable);
      }

      if horizontal_light.path.offsets.as_ref().is_some_and(|x| !x.is_empty()) {
         let qualified_drawable = QualifiedDrawable {
            layer: axes_layer,
            // color_choice: diagram.prep.light_color_choice.clone(),
            drawable: OneOfDrawable::Lines(horizontal_light),
         };
         diagram.drawables.push(qualified_drawable);
      }
      if vertical_light.path.offsets.as_ref().is_some_and(|x| !x.is_empty()) {
         let qualified_drawable = QualifiedDrawable {
            layer: axes_layer,
            // color_choice: diagram.prep.light_color_choice.clone(),
            drawable: OneOfDrawable::Lines(vertical_light),
         };
         diagram.drawables.push(qualified_drawable);
      }

      if self.axis_numbering != AxisNumbering::None {
         let horizontal_precision =
            if self.grid_precision.is_empty() { 20_usize } else { self.grid_precision[0] };
         let vertical_precision = if self.grid_precision.len() > 1 {
            self.grid_precision[1]
         } else {
            horizontal_precision
         };
         let (anchor_horizontal, anchor_vertical) = match self.axis_numbering {
            AxisNumbering::Before => (TextAnchorHorizontal::Right, TextAnchorVertical::Top),
            AxisNumbering::After => (TextAnchorHorizontal::Left, TextAnchorVertical::Bottom),
            AxisNumbering::At | AxisNumbering::None => {
               (TextAnchorHorizontal::Center, TextAnchorVertical::Middle)
            }
         };
         let mut horizontal_numbering = TextDrawable {
            size_choice: TextSizeChoice::Small,
            color_choice: diagram.prep.text_color_choice.clone(),
            offset_choice: TextOffsetChoice::Diagram,
            anchor_choice: TextAnchorChoice::ThreeByThree(
               anchor_horizontal,
               TextAnchorVertical::Top,
            ),
            ..Default::default()
         };
         let mut vertical_numbering = TextDrawable {
            size_choice: TextSizeChoice::Small,
            color_choice: diagram.prep.text_color_choice.clone(),
            offset_choice: TextOffsetChoice::Diagram,
            anchor_choice: TextAnchorChoice::ThreeByThree(
               TextAnchorHorizontal::Right,
               anchor_vertical,
            ),

            ..Default::default()
         };

         let number_at_zero = self.axes_style == AxesStyle::Cross;

         let vertical_for_horizontal = if has_vert_zero && number_at_zero { 0.0 } else { range[1] };
         let horizontal_for_vertical =
            if has_horiz_zero && number_at_zero { 0.0 } else { range[0] };

         if let Some(location) = left_numbering_location {
            horizontal_numbering.texts.push(TextSingle {
               content: format!("{location:.horizontal_precision$}"),
               location: [location, vertical_for_horizontal],
               ..Default::default()
            });
         }
         if has_vert_zero && !number_at_zero {
            horizontal_numbering.texts.push(TextSingle {
               content: format!("{:.horizontal_precision$}", 0.0),
               location: [0.0, vertical_for_horizontal],
               ..Default::default()
            });
         }
         if let Some(location) = right_numbering_location {
            horizontal_numbering.texts.push(TextSingle {
               content: format!("{location:.horizontal_precision$}"),
               location: [location, vertical_for_horizontal],
               ..Default::default()
            });
         }
         if let Some(location) = bottom_numbering_location {
            vertical_numbering.texts.push(TextSingle {
               content: format!("{location:.vertical_precision$}"),
               location: [horizontal_for_vertical, location],
               ..Default::default()
            });
         }
         if has_horiz_zero && !number_at_zero {
            vertical_numbering.texts.push(TextSingle {
               content: format!("{:.vertical_precision$}", 0.0),
               location: [horizontal_for_vertical, 0.0],
               ..Default::default()
            });
         }
         if let Some(location) = top_numbering_location {
            vertical_numbering.texts.push(TextSingle {
               content: format!("{location:.vertical_precision$}"),
               location: [horizontal_for_vertical, location],
               ..Default::default()
            });
         }

         // Change layer to depth.
         let axes_layer = 0;
         if !horizontal_numbering.texts.is_empty() {
            let qualified_drawable = QualifiedDrawable {
               layer: axes_layer,
               // color_choice: diagram.prep.text_color_choice.clone(),
               drawable: OneOfDrawable::Text(horizontal_numbering),
            };
            diagram.drawables.push(qualified_drawable);
         }
         let axes_layer = 0;
         if !vertical_numbering.texts.is_empty() {
            let qualified_drawable = QualifiedDrawable {
               layer: axes_layer,
               // color_choice: diagram.prep.text_color_choice.clone(),
               drawable: OneOfDrawable::Text(vertical_numbering),
            };
            diagram.drawables.push(qualified_drawable);
         }
      }
   }
}
