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

use cairo::Context as CairoContext;
use cairo::Matrix;
use cairo::SvgSurface;
use cairo::SvgUnit::Pt;
use pango::Context as PangoContext;
use pango::FontDescription;
use pango::Layout as PangoLayout;
use pangocairo::functions::create_context as pangocairo_create_context;
use pangocairo::functions::show_layout as pangocairo_show_layout;
use std::f64::consts::PI;
use std::io::Write;
use zvx_docagram::diagram::SpartanDiagram;
use zvx_drawable::choices::CanvasLayout;
use zvx_drawable::choices::ColorChoice;
use zvx_drawable::choices::DiagramChoices;
use zvx_drawable::choices::LineChoice;
use zvx_drawable::choices::LineClosureChoice;
use zvx_drawable::choices::PointChoice;
use zvx_drawable::choices::TextAnchorChoice;
use zvx_drawable::choices::TextAnchorHorizontal;
use zvx_drawable::choices::TextAnchorVertical;
use zvx_drawable::choices::TextOffsetChoice;
use zvx_drawable::choices::TextSizeChoice;
use zvx_drawable::kinds::ArcDrawable;
use zvx_drawable::kinds::CirclesDrawable;
use zvx_drawable::kinds::CubicDrawable;
use zvx_drawable::kinds::LinesDrawable;
use zvx_drawable::kinds::OneOfDrawable;
use zvx_drawable::kinds::PointsDrawable;
use zvx_drawable::kinds::PolylineDrawable;
use zvx_drawable::kinds::QualifiedDrawable;
use zvx_drawable::kinds::TextDrawable;
use zvx_drawable::kinds::TextSingle;

#[derive(Debug, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct CairoSpartanRender {
   pub saved_matrix: Matrix,
}

impl CairoSpartanRender {
   #[must_use]
   pub fn new() -> Self {
      Self::default()
   }
   // This is necessary because line thicknesses and similar are distorted if the x and y
   // scales differ.  Consequently we only use the scaling transform for the Cairo CTM when
   // creating paths.
   pub fn save_set_path_transform(&mut self, canvas_layout: &CanvasLayout, context: &CairoContext) {
      self.saved_matrix = context.matrix();

      context.translate(
         canvas_layout.offset[0],
         canvas_layout.canvas_size[1] - canvas_layout.offset[1],
      );
      context.scale(canvas_layout.scale[0], -canvas_layout.scale[1]);
   }

   // Be sure to restore the original transform before stroking out a path with a pen.  This is
   // so that the original Cairo CTM, which should be isotropic, is used for the stroke pen.
   pub fn restore_transform(&mut self, context: &CairoContext) {
      context.set_matrix(self.saved_matrix);
   }

   #[allow(clippy::unused_self)]
   fn set_color(
      &self,
      context: &CairoContext,
      _diagram_choices: &DiagramChoices,
      color: ColorChoice,
   ) {
      let (r, g, b) = color.to_rgb();
      context.set_source_rgb(r, g, b);
   }

   fn set_line_choice(
      context: &CairoContext,
      line_choice: LineChoice,
      diagram_choices: &DiagramChoices,
   ) {
      let line_parameters = line_choice.to_line_parameters(diagram_choices);
      context.set_line_width(line_parameters.line_width);
      context.set_dash(&line_parameters.dashes, line_parameters.dash_offset);
   }

   fn draw_lines_set(
      &mut self,
      context: &CairoContext,
      drawable: &LinesDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(context, drawable.line_choice, diagram_choices);
      self.set_color(context, diagram_choices, drawable.color_choice);

      self.save_set_path_transform(canvas_layout, context);
      for i in 0..drawable.coords.len() {
         if let Some(offset_vector) = &drawable.offsets {
            for offset in offset_vector {
               context.move_to(
                  drawable.coords[i].0[0] + offset[0],
                  drawable.coords[i].0[1] + offset[1],
               );
               context.line_to(
                  drawable.coords[i].1[0] + offset[0],
                  drawable.coords[i].1[1] + offset[1],
               );
            }
         } else {
            context.move_to(drawable.coords[i].0[0], drawable.coords[i].0[1]);
            context.line_to(drawable.coords[i].1[0], drawable.coords[i].1[1]);
         }
      }
      self.restore_transform(context);
      context.stroke().unwrap();
   }

   fn draw_points_set(
      &mut self,
      context: &CairoContext,
      drawable: &PointsDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(context, LineChoice::Ordinary, diagram_choices);
      self.set_color(context, diagram_choices, drawable.color_choice);

      match drawable.point_choice {
         PointChoice::Circle => {
            for center in &drawable.centers {
               self.save_set_path_transform(canvas_layout, context);
               let (cx, cy) = context.user_to_device(center[0], center[1]);
               self.restore_transform(context);
               context.move_to(cx + 2.8, cy);
               context.arc(cx, cy, 2.8, 0.0 * PI, 2.0 * PI);
               context.close_path();
            }
         }
         PointChoice::Dot => {
            for center in &drawable.centers {
               self.save_set_path_transform(canvas_layout, context);
               let (cx, cy) = context.user_to_device(center[0], center[1]);
               self.restore_transform(context);
               #[allow(clippy::suboptimal_flops)]
               context.move_to(cx + 2.8 * 0.92, cy);
               #[allow(clippy::suboptimal_flops)]
               context.arc(cx, cy, 2.8 * 0.92, 0.0 * PI, 2.0 * PI);
               context.fill().unwrap();
               context.close_path();
            }
         }
         PointChoice::Plus => {
            for center in &drawable.centers {
               self.save_set_path_transform(canvas_layout, context);
               let (cx, cy) = context.user_to_device(center[0], center[1]);
               self.restore_transform(context);
               let plus_delta = 2.8 * 1.48;
               context.move_to(cx, cy - plus_delta);
               context.line_to(cx, cy + plus_delta);
               context.move_to(cx + plus_delta, cy);
               context.line_to(cx - plus_delta, cy);
               context.close_path();
            }
         }
         PointChoice::Times => {
            for center in &drawable.centers {
               self.save_set_path_transform(canvas_layout, context);
               let (cx, cy) = context.user_to_device(center[0], center[1]);
               self.restore_transform(context);
               let times_delta = 2.8 * 1.48 * (0.5_f64).sqrt();
               context.move_to(cx - times_delta, cy - times_delta);
               context.line_to(cx + times_delta, cy + times_delta);
               context.move_to(cx + times_delta, cy - times_delta);
               context.line_to(cx - times_delta, cy + times_delta);
               context.close_path();
            }
         }
      }
      context.stroke().unwrap();
   }

   fn draw_arc(
      &mut self,
      context: &CairoContext,
      drawable: &ArcDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(context, drawable.segment_choices.line_choice, diagram_choices);
      self.set_color(context, diagram_choices, drawable.segment_choices.color);

      self.save_set_path_transform(canvas_layout, context);

      let arc_transformation_matrix = cairo::Matrix::new(
         drawable.transform[0],
         drawable.transform[1],
         drawable.transform[2],
         drawable.transform[3],
         drawable.center[0],
         drawable.center[1],
      );
      context.transform(arc_transformation_matrix);

      // Logically circle is center (0.0, 0.0) radius 1.0.
      context.move_to(drawable.angle_range[0].cos(), drawable.angle_range[0].sin());
      context.arc(0.0, 0.0, 1.0, drawable.angle_range[0], drawable.angle_range[1]);
      self.restore_transform(context);
      context.stroke().unwrap();
   }

   fn draw_cubic(
      &mut self,
      context: &CairoContext,
      drawable: &CubicDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(context, drawable.segment_choices.line_choice, diagram_choices);
      self.set_color(context, diagram_choices, drawable.segment_choices.color);

      self.save_set_path_transform(canvas_layout, context);

      context.move_to(drawable.x[0], drawable.y[0]);
      context.curve_to(
         drawable.x[1],
         drawable.y[1],
         drawable.x[2],
         drawable.y[2],
         drawable.x[3],
         drawable.y[3],
      );

      self.restore_transform(context);
      context.stroke().unwrap();
   }

   // This function is (somewhat) disassociated from the renderer and from Cairo, and is specific to Pango.
   //
   // text_context: The Pango context that gives canvas-like rendering information. This
   // inherits content from the Cairo context.
   //
   // single_text: The content and text specific to this "string".
   // drawable: The parent of the text, that provides choices such as alignment.
   // prep: Wider choices, such as how fonts are generally scaled in this diagram.
   #[inline]
   fn layout_text(
      text_context: &PangoContext,
      single_text: &TextSingle,
      drawable: &TextDrawable,
      diagram_choices: &DiagramChoices,
   ) -> (PangoLayout, f64, f64) {
      let area_based_scale = match drawable.size_choice {
         TextSizeChoice::Normal => 1.0,
         TextSizeChoice::Large => 1.0 / diagram_choices.annotation_area_scale,
         TextSizeChoice::Small => diagram_choices.annotation_area_scale,
      };
      let font_size = diagram_choices.font_size * area_based_scale;

      let text_layout = PangoLayout::new(text_context);

      let mut font_description = FontDescription::new();
      font_description.set_family("sans");
      font_description.set_absolute_size(font_size * f64::from(pango::SCALE));
      text_layout.set_font_description(Some(&font_description));

      let metrics = text_layout.context().metrics(Some(&font_description), None);
      // Strikethrough is top of line above baseline.
      let strikethrough_center =
         0.5 * f64::from(2 * metrics.strikethrough_position() - metrics.strikethrough_thickness());
      let even_half_height = f64::max(
         f64::from(metrics.ascent()) - strikethrough_center,
         f64::from(metrics.descent()) + strikethrough_center,
      );

      text_layout.set_text(&single_text.content);
      let (text_width, text_height) = text_layout.size();

      let (offset_x, offset_y) = match drawable.offset_choice {
         TextOffsetChoice::None => (0.0, 0.0),
         TextOffsetChoice::Diagram => (
            diagram_choices.annotation_offset_absolute[0]
               * area_based_scale
               * f64::from(pango::SCALE),
            diagram_choices.annotation_offset_absolute[1]
               * area_based_scale
               * f64::from(pango::SCALE),
         ),
      };

      let mut height_adjust = f64::from(metrics.ascent()) - strikethrough_center;
      let multiline_adjust = f64::from(text_height - metrics.height());
      let mut width_adjust = 0.0;

      match drawable.anchor_choice {
         TextAnchorChoice::Centered => {
            height_adjust += 0.5 * multiline_adjust;
            width_adjust += 0.5 * f64::from(text_width);
         }

         TextAnchorChoice::ThreeByThree(horizontal, vertical) => {
            height_adjust += match vertical {
               TextAnchorVertical::Bottom => even_half_height + multiline_adjust + offset_y,
               TextAnchorVertical::Middle => 0.5 * multiline_adjust,
               TextAnchorVertical::Top => -even_half_height - offset_y,
            };
            width_adjust += match horizontal {
               TextAnchorHorizontal::Left => -offset_x,
               TextAnchorHorizontal::Center => 0.5 * f64::from(text_width),
               TextAnchorHorizontal::Right => f64::from(text_width) + offset_x,
            };
         }
      }

      (text_layout, width_adjust, height_adjust)
   }

   fn draw_text_set(
      &mut self,
      cairo_context: &CairoContext,
      drawable: &TextDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      for single_text in &drawable.texts {
         // Create a single context, instead of using create_layout.  This
         // demonstrates avoiding lots of Pango contexts.
         let text_context: PangoContext = pangocairo_create_context(cairo_context);

         let (text_layout, width_adjust, height_adjust) =
            Self::layout_text(&text_context, single_text, drawable, diagram_choices);

         self.set_color(cairo_context, diagram_choices, drawable.color_choice);

         self.save_set_path_transform(canvas_layout, cairo_context);
         let (tx, ty) =
            cairo_context.user_to_device(single_text.location[0], single_text.location[1]);
         self.restore_transform(cairo_context);

         cairo_context.move_to(
            tx - width_adjust / f64::from(pango::SCALE),
            ty - height_adjust / f64::from(pango::SCALE),
         );
         pangocairo_show_layout(cairo_context, &text_layout);
         cairo_context.stroke().unwrap();
      }
   }

   fn draw_polyine(
      &mut self,
      context: &CairoContext,
      drawable: &PolylineDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(context, drawable.segment_choices.line_choice, diagram_choices);
      self.set_color(context, diagram_choices, drawable.segment_choices.color);

      self.save_set_path_transform(canvas_layout, context);
      assert!(!drawable.locations.is_empty());
      context.move_to(drawable.locations[0][0], drawable.locations[0][1]);
      for i in 1..drawable.locations.len() {
         context.line_to(drawable.locations[i][0], drawable.locations[i][1]);
      }
      if drawable.segment_choices.closure == LineClosureChoice::Closes {
         context.close_path();
      }
      self.restore_transform(context);
      context.stroke().unwrap();
   }

   fn draw_circles_set(
      &mut self,
      context: &CairoContext,
      drawable: &CirclesDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(context, drawable.line_choice, diagram_choices);
      self.set_color(context, diagram_choices, drawable.color_choice);

      self.save_set_path_transform(canvas_layout, context);
      for center in &drawable.centers {
         let (cx, cy) = (center[0], center[1]);
         let r = drawable.radius;

         context.move_to(cx + r, cy);
         context.arc(cx, cy, r, 0.0 * PI, 2.0 * PI);
         context.close_path();
      }
      self.restore_transform(context);
      context.stroke().unwrap();
   }

   #[allow(clippy::missing_panics_doc)]
   pub fn render_drawables(
      &mut self,
      drawables: &[QualifiedDrawable],
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
      context: &CairoContext,
   ) {
      let mut indices = (0..drawables.len()).collect::<Vec<_>>();
      indices.sort_by_key(|&i| &drawables[i].layer);

      for i in indices {
         match &drawables[i].drawable {
            OneOfDrawable::Lines(drawable) => {
               self.draw_lines_set(context, drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Arc(drawable) => {
               self.draw_arc(context, drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Cubic(drawable) => {
               self.draw_cubic(context, drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Points(drawable) => {
               self.draw_points_set(context, drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Text(drawable) => {
               self.draw_text_set(context, drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Circles(drawable) => {
               self.draw_circles_set(context, drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Polyline(drawable) => {
               self.draw_polyine(context, drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Nothing => {}
         }
      }
   }

   // Move out of class. Pass 3 parts of diagram to render_drawables, not diagram.
   //     CairoSpartanRender becomes CairoSampleRender.
   //
   // Then move to sample.rs

   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   pub fn render_drawables_to_stream<W: Write + 'static>(
      &mut self,
      out_stream: W,
      drawables: &[QualifiedDrawable],
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) -> Result<Box<dyn core::any::Any>, cairo::StreamWithError> {
      let canvas_size = &canvas_layout.canvas_size;
      let mut surface = SvgSurface::for_stream(canvas_size[0], canvas_size[1], out_stream).unwrap();
      surface.set_document_unit(Pt);

      let context = CairoContext::new(&surface).unwrap();

      self.render_drawables(drawables, canvas_layout, diagram_choices, &context);

      surface.flush();
      surface.finish_output_stream()
   }
}

// This may seem odd, but is Rust-inspired. The diagram and the renderer can be separately
// borrowed with different mutability.
#[derive(Debug, Default)]
pub struct CairoSpartanCombo {
   pub spartan: SpartanDiagram,

   pub render_controller: CairoSpartanRender,
}

impl CairoSpartanCombo {
   #[must_use]
   pub fn new() -> Self {
      Self::default()
   }

   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   pub fn render_diagram_to_write<W: Write + 'static>(
      &mut self,
      out_stream: W,
   ) -> Result<Box<dyn core::any::Any>, cairo::StreamWithError> {
      assert!(self.spartan.is_ready());

      self.render_controller.render_drawables_to_stream(
         out_stream,
         &self.spartan.drawables,
         &self.spartan.prep.canvas_layout,
         &self.spartan.prep.diagram_choices,
      )
   }
}
