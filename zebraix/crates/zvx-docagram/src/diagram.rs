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
use serde_default::DefaultFromSerde;
use zvx_base::is_default;
use zvx_drawable::choices::{CanvasLayout, ColorChoice, DiagramChoices};
use zvx_drawable::kinds::QualifiedDrawable;

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone)]
pub enum SizingScheme {
   #[default]
   SquareShrink,
   SquareCenter,
   Fill,
}

#[derive(Debug, Default, Clone)]
pub struct SpartanPreparation {
   pub canvas_layout: CanvasLayout,
   pub diagram_choices: DiagramChoices,
   pub padding: Vec<f64>,
   pub scale_content: f64,
   pub axes_range: [f64; 4],
   pub main_color_choice: ColorChoice,
   pub light_color_choice: ColorChoice,
   pub text_color_choice: ColorChoice,
}

#[derive(Debug, Serialize, DefaultFromSerde, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct SpartanDiagram {
   pub sizing_scheme: SizingScheme,

   // At the creation of the renderer the canvas size is required.  If in future other
   // renderers require more information, this will be expanded into, say, a
   // CanvasSpecification structure.
   //
   // The units of the canvas are, for now, fixed to pt.
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_canvas_size",
      default = "SpartanDiagram::default_canvas_size"
   )]
   pub canvas_size: (f64, f64),

   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_base_font_size",
      default = "SpartanDiagram::default_base_font_size"
   )]
   pub base_font_size: f64,
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_base_point_size",
      default = "SpartanDiagram::default_base_point_size"
   )]
   pub base_point_size: f64,
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_base_line_width",
      default = "SpartanDiagram::default_base_line_width"
   )]
   pub base_line_width: f64,

   #[serde(skip_serializing_if = "is_default")]
   pub base_color_choice: ColorChoice,

   #[serde(skip_serializing_if = "is_default")]
   pub light_color_choice: ColorChoice,

   #[serde(skip_serializing_if = "is_default")]
   pub text_color_choice: ColorChoice,

   // Scaling of 1-D annotations, such as grid line width vs normal.
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_annotation_linear_scale",
      default = "SpartanDiagram::default_annotation_linear_scale"
   )]
   pub annotation_linear_scale: f64,
   // Scaling of 2-D annotations, such as font size vs titling.
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_annotation_area_scale",
      default = "SpartanDiagram::default_annotation_area_scale"
   )]
   pub annotation_area_scale: f64,

   // Applied as horiz and vert scalings of the font size.
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_annotation_offset",
      default = "SpartanDiagram::default_annotation_offset"
   )]
   pub annotation_offset: [f64; 2],

   // Removed because apparently just complicating things.
   // // // Optionally (if non-zero) specify scaling of diagram size from base values.
   // // #[serde(skip_serializing_if = "is_default")]
   // // pub scale_width: f64,
   // // #[serde(skip_serializing_if = "is_default")]
   // // pub scale_height: f64,
   // Main line-width scaling as diagram scales. If zero, use something like the square
   // root of the geometric mean of the width and height scaling, so that content grows
   // gradually.
   #[serde(
      skip_serializing_if = "SpartanDiagram::is_default_scale_content",
      default = "SpartanDiagram::default_scale_content"
   )]
   pub scale_content: f64,

   #[serde(skip_serializing_if = "is_default")]
   pub axes_range: Vec<f64>,
   #[serde(skip_serializing_if = "is_default")]
   pub padding: Vec<f64>,
}

#[derive(Debug, Serialize, DefaultFromSerde)]
#[allow(clippy::module_name_repetitions)]
pub struct DrawableDiagram {
   #[serde(skip)]
   pub prep: SpartanPreparation,

   #[serde(skip_serializing_if = "is_default")]
   pub drawables: Vec<QualifiedDrawable>,
}

impl SpartanDiagram {
   #[must_use]
   pub fn new() -> Self {
      Self::default()
   }

   #[must_use]
   fn is_near_float(v: f64, w: f64) -> bool {
      (v - w).abs() < 0.0001
   }

   #[must_use]
   const fn default_canvas_size() -> (f64, f64) {
      (400.0, 300.0)
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_canvas_size(v: &(f64, f64)) -> bool {
      Self::is_near_float(v.0, Self::default_canvas_size().0)
         && Self::is_near_float(v.1, Self::default_canvas_size().1)
   }

   #[must_use]
   const fn default_base_font_size() -> f64 {
      11.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_font_size(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_font_size())
   }

   #[must_use]
   const fn default_base_point_size() -> f64 {
      15.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_point_size(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_point_size())
   }

   // 1.0 is a regular thickness, definitely not thick, 2.0 definitely thick, 0.6 thin but
   // firm.
   #[must_use]
   const fn default_base_line_width() -> f64 {
      1.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_base_line_width(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_base_line_width())
   }

   #[must_use]
   const fn default_scale_content() -> f64 {
      1.0
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_scale_content(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_scale_content())
   }

   #[must_use]
   const fn default_annotation_linear_scale() -> f64 {
      0.45
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_annotation_linear_scale(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_annotation_linear_scale())
   }

   #[must_use]
   const fn default_annotation_area_scale() -> f64 {
      0.85
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_annotation_area_scale(v: &f64) -> bool {
      Self::is_near_float(*v, Self::default_annotation_area_scale())
   }

   #[must_use]
   const fn default_annotation_offset() -> [f64; 2] {
      [0.5, 0.2]
   }
   #[allow(clippy::trivially_copy_pass_by_ref)]
   #[must_use]
   fn is_default_annotation_offset(v: &[f64; 2]) -> bool {
      let default_value = Self::default_annotation_offset();
      Self::is_near_float((*v)[0], default_value[0])
         && Self::is_near_float((*v)[1], default_value[1])
   }

   // fn multiply_default_one(a: f64, b: f64) -> f64 {
   //    if b == 0.0 {
   //       a
   //    } else {
   //       a * b
   //    }
   // }

   #[allow(clippy::too_many_lines)]
   #[allow(clippy::missing_panics_doc)]
   #[allow(clippy::suboptimal_flops)]
   #[must_use]
   pub fn prepare(&self) -> SpartanPreparation {
      let mut preparation = SpartanPreparation::default();

      preparation.canvas_layout.canvas_size = [self.canvas_size.0, self.canvas_size.1];

      let mut axes_range = self.axes_range.clone();
      match axes_range.len() {
         1 => {
            axes_range = [-axes_range[0], -axes_range[0], axes_range[0], axes_range[0]].to_vec();
         }
         2 => {
            axes_range = [-axes_range[0], -axes_range[1], axes_range[0], axes_range[1]].to_vec();
         }
         4 => {}
         _ => {
            panic!(
               "axes_range must be vector of size 1, 2 or 4, but found size {}",
               axes_range.len()
            );
         }
      }
      preparation.axes_range = [axes_range[0], axes_range[1], axes_range[2], axes_range[3]];

      let mut padding = self.padding.clone();
      match padding.len() {
         0 => {
            padding = [0.0, 0.0, 0.0, 0.0].to_vec();
         }
         1 => {
            padding = [padding[0], padding[0], padding[0], padding[0]].to_vec();
         }
         2 => {
            padding = [padding[0], padding[1], padding[0], padding[1]].to_vec();
         }
         4 => {}
         _ => {
            panic!("padding must be vector of size 0, 1, 2 or 4, but found size {}", padding.len());
         }
      }
      preparation.padding.clone_from(&padding);

      let x_min = axes_range[0];
      let y_min = axes_range[1];
      let x_max = axes_range[2];
      let y_max = axes_range[3];
      let left_padding = padding[0];
      let bottom_padding = padding[1];
      let right_padding = padding[2];
      let top_padding = padding[3];

      let total_width_range = (x_max - x_min) * (1.0 + left_padding + right_padding);
      let total_height_range = (y_max - y_min) * (1.0 + bottom_padding + top_padding);
      let mut width_adjustment = 0.0;
      let mut height_adjustment = 0.0;

      let is_width_limited: bool = (total_width_range * preparation.canvas_layout.canvas_size[1])
         > (total_height_range * preparation.canvas_layout.canvas_size[0]);

      match self.sizing_scheme {
         SizingScheme::SquareShrink => {
            if is_width_limited {
               preparation.canvas_layout.canvas_size[1] =
                  total_height_range * preparation.canvas_layout.canvas_size[0] / total_width_range;
            } else {
               preparation.canvas_layout.canvas_size[0] =
                  total_width_range * preparation.canvas_layout.canvas_size[1] / total_height_range;
            }
         }
         SizingScheme::SquareCenter => {
            if is_width_limited {
               height_adjustment = 0.5
                  * (total_width_range * preparation.canvas_layout.canvas_size[1]
                     / preparation.canvas_layout.canvas_size[0]
                     - total_height_range);
            } else {
               width_adjustment = 0.5
                  * (total_height_range * preparation.canvas_layout.canvas_size[0]
                     / preparation.canvas_layout.canvas_size[1]
                     - total_width_range);
            }
         }
         SizingScheme::Fill => {}
      }

      preparation.canvas_layout.scale = [
         preparation.canvas_layout.canvas_size[0]
            / width_adjustment.mul_add(2.0f64, total_width_range),
         preparation.canvas_layout.canvas_size[1]
            / height_adjustment.mul_add(2.0f64, total_height_range),
      ];

      preparation.canvas_layout.offset = [
         preparation.canvas_layout.scale[0]
            * ((x_max - x_min) * left_padding - x_min + width_adjustment),
         preparation.canvas_layout.scale[1]
            * ((y_max - y_min) * bottom_padding - y_min + height_adjustment),
      ];

      let mut scale_content = self.scale_content;

      // If content scaling not specified, use a heuristic based on overall diagram scaling.
      if scale_content == 0.0 {
         scale_content = (preparation.canvas_layout.scale[0]
            * (x_max - x_min)
            * preparation.canvas_layout.scale[1]
            * (y_max - y_min)
            / self.canvas_size.0
            / self.canvas_size.1)
            .sqrt();
      }
      preparation.scale_content = scale_content;

      preparation.diagram_choices.font_size = self.base_font_size * preparation.scale_content;
      preparation.diagram_choices.point_size = self.base_point_size * preparation.scale_content;
      preparation.diagram_choices.line_width = self.base_line_width * preparation.scale_content;
      preparation.diagram_choices.annotation_offset_absolute[0] =
         self.base_font_size * self.annotation_offset[0];
      preparation.diagram_choices.annotation_offset_absolute[1] =
         self.base_font_size * self.annotation_offset[1];
      preparation.diagram_choices.annotation_linear_scale = self.annotation_linear_scale;
      preparation.diagram_choices.annotation_area_scale = self.annotation_area_scale;

      preparation.main_color_choice = self.base_color_choice.clone();
      if self.light_color_choice == ColorChoice::default() {
         preparation.light_color_choice = self.base_color_choice.clone();
      } else {
         preparation.light_color_choice = self.light_color_choice.clone();
      }
      if self.text_color_choice == ColorChoice::default() {
         preparation.text_color_choice = self.base_color_choice.clone();
      } else {
         preparation.text_color_choice = self.text_color_choice.clone();
      }

      preparation
   }
}
