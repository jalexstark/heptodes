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
use std::error::Error;
use std::f64::consts::PI;
use std::io::Write;
use zvx_docagram::diagram::SpartanDiagram;
use zvx_drawable::choices::{
   CanvasLayout, ColorChoice, ContinuationChoice, DiagramChoices, LineChoice, LineClosureChoice,
   PathCompletion, PointChoice, TextAnchorChoice, TextAnchorHorizontal, TextAnchorVertical,
   TextOffsetChoice, TextSizeChoice,
};
use zvx_drawable::kinds::{
   ArcPath, CirclesDrawable, CubicPath, LinesDrawable, OneOfDrawable, OneOfSegment, PathChoices,
   PointsDrawable, PolylinePath, QualifiedDrawable, SegmentChoices, SegmentSequence, TextDrawable,
   TextSingle,
};

#[derive(Debug, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct CairoSpartanRender {
   pub saved_matrix: Matrix,
}

pub struct TextMetrics {
   pub strikethrough_center: f64,
   pub even_half_height: f64,
   pub font_ascent: f64,
   pub font_descent: f64,
   pub font_height: f64,
   // Fields above are generally independent of text content.
   pub text_width: f64,
   pub text_height: f64,
}

// Note on special functions.
//
// Rust is (as of rustc 1.85.1) unable to convert a boxed heap object to (a reference to) its
// concrete implementation type when any kind of non-static lifetime is involved. As a result an
// implementation such as Cairo+Pango has no means to call functions with its own data. In order
// to work around this, the `render_layout` method was created for the text layout trait, even
// though this really is the business of the implementation. In order to future-proof the
// interface, extra placeholder special functions were added.
//
// Refs: https://users.rust-lang.org/t/borrowing-as-any-non-static/131565,
// https://crates.io/crates/better_any

pub trait ZvxTextLayout {
   fn set_layout(&mut self, font_family: &str, font_size: f64, text_content: &str);
   fn get_metrics(&mut self) -> &Option<TextMetrics>;
   #[allow(clippy::missing_errors_doc)]
   fn render_layout(&mut self) -> Result<(), Box<dyn Error>>;

   // See note above about special functions.
   #[allow(clippy::missing_errors_doc)]
   fn special_function_0(&mut self) -> Result<(), Box<dyn Error>>;
   #[allow(clippy::missing_errors_doc)]
   fn special_function_1(&mut self) -> Result<(), Box<dyn Error>>;
   #[allow(clippy::missing_errors_doc)]
   fn special_function_2(&mut self) -> Result<(), Box<dyn Error>>;
   #[allow(clippy::missing_errors_doc)]
   fn special_function_3(&mut self) -> Result<(), Box<dyn Error>>;
}

pub struct ZvxPangoTextLayout<'parent> {
   pub pango_text_layout: PangoLayout,
   pub metrics: Option<TextMetrics>,
   parent_cairo: &'parent CairoContext,
}

impl<'parent> ZvxPangoTextLayout<'parent> {
   #[must_use]
   pub fn create_pango_layout<'a>(
      parent_cairo: &'parent CairoContext,
      text_context: &'a PangoContext,
   ) -> Box<(dyn ZvxTextLayout + 'a)>
   where
      'parent: 'a,
   {
      let new_layout: ZvxPangoTextLayout = ZvxPangoTextLayout {
         pango_text_layout: PangoLayout::new(text_context),
         metrics: None,
         parent_cairo,
      };

      Box::new(new_layout)
   }
}

#[allow(clippy::needless_lifetimes)]
impl<'parent> ZvxTextLayout for ZvxPangoTextLayout<'parent> {
   fn set_layout(&mut self, font_family: &str, font_size: f64, text_content: &str) {
      let mut font_description = FontDescription::new();

      font_description.set_family(font_family);
      font_description.set_absolute_size(font_size * f64::from(pango::SCALE));
      self.pango_text_layout.set_font_description(Some(&font_description));

      let pango_metrics = self.pango_text_layout.context().metrics(Some(&font_description), None);
      let font_ascent = f64::from(pango_metrics.ascent());
      let font_descent = f64::from(pango_metrics.descent());
      let font_height = f64::from(pango_metrics.height());
      // Strikethrough is top of line above baseline.
      let strikethrough_center = 0.5
         * f64::from(
            2 * pango_metrics.strikethrough_position() - pango_metrics.strikethrough_thickness(),
         );
      let even_half_height =
         f64::max(font_ascent - strikethrough_center, font_descent + strikethrough_center);

      // Text content dependence below.

      self.pango_text_layout.set_text(text_content);

      let (layout_text_width, layout_text_height) = self.pango_text_layout.size();
      let text_width = f64::from(layout_text_width);
      let text_height = f64::from(layout_text_height);

      self.metrics = Some(TextMetrics {
         strikethrough_center,
         even_half_height,
         font_ascent,
         font_descent,
         font_height,
         text_width,
         text_height,
      });
   }

   fn get_metrics(&mut self) -> &Option<TextMetrics> {
      &self.metrics
   }

   #[allow(clippy::missing_errors_doc)]
   fn render_layout(&mut self) -> Result<(), Box<dyn Error>> {
      pangocairo_show_layout(self.parent_cairo, &self.pango_text_layout);
      Ok(())
   }

   #[allow(clippy::missing_errors_doc)]
   fn special_function_0(&mut self) -> Result<(), Box<dyn Error>> {
      Err("Cairo-pango text layout does not implement `special_function_0`.".into())
   }
   #[allow(clippy::missing_errors_doc)]
   fn special_function_1(&mut self) -> Result<(), Box<dyn Error>> {
      Err("Cairo-pango text layout does not implement `special_function_1`.".into())
   }
   #[allow(clippy::missing_errors_doc)]
   fn special_function_2(&mut self) -> Result<(), Box<dyn Error>> {
      Err("Cairo-pango text layout does not implement `special_function_2`.".into())
   }
   #[allow(clippy::missing_errors_doc)]
   fn special_function_3(&mut self) -> Result<(), Box<dyn Error>> {
      Err("Cairo-pango text layout does not implement `special_function_3`.".into())
   }
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
      Self::set_line_choice(context, drawable.path_choices.line_choice, diagram_choices);
      self.set_color(context, diagram_choices, drawable.path_choices.color);

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
      path: &ArcPath,
      path_choices: PathChoices,
      segment_choices: SegmentChoices,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(context, path_choices.line_choice, diagram_choices);
      self.set_color(context, diagram_choices, path_choices.color);

      self.save_set_path_transform(canvas_layout, context);

      let arc_transformation_matrix = cairo::Matrix::new(
         path.transform[0],
         path.transform[1],
         path.transform[2],
         path.transform[3],
         path.center[0],
         path.center[1],
      );
      context.transform(arc_transformation_matrix);

      // Logically circle is center (0.0, 0.0) radius 1.0.
      if segment_choices.continuation == ContinuationChoice::Starts {
         context.move_to(path.angle_range[0].cos(), path.angle_range[0].sin());
      }
      context.arc(0.0, 0.0, 1.0, path.angle_range[0], path.angle_range[1]);
      match segment_choices.closure {
         LineClosureChoice::Closes => {
            context.close_path();
            self.restore_transform(context);
            context.stroke().unwrap();
         }
         LineClosureChoice::OpenEnd => {
            self.restore_transform(context);
            context.stroke().unwrap();
         }
         LineClosureChoice::Unfinished => {
            self.restore_transform(context);
         }
      }
   }

   fn draw_cubic(
      &mut self,
      context: &CairoContext,
      path: &CubicPath,
      path_choices: PathChoices,
      segment_choices: SegmentChoices,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(context, path_choices.line_choice, diagram_choices);
      self.set_color(context, diagram_choices, path_choices.color);

      self.save_set_path_transform(canvas_layout, context);

      if segment_choices.continuation == ContinuationChoice::Starts {
         context.move_to(path[0][0], path[0][1]);
      }
      context.curve_to(path[1][0], path[1][1], path[2][0], path[2][1], path[3][0], path[3][1]);
      match segment_choices.closure {
         LineClosureChoice::Closes => {
            context.close_path();
            self.restore_transform(context);
            context.stroke().unwrap();
         }
         LineClosureChoice::OpenEnd => {
            self.restore_transform(context);
            context.stroke().unwrap();
         }
         LineClosureChoice::Unfinished => {
            self.restore_transform(context);
         }
      }
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
   fn figure_text_adjust<'a>(
      boxed_text_layout: &mut Box<dyn ZvxTextLayout + 'a>,
      single_text: &TextSingle,
      drawable: &TextDrawable,
      diagram_choices: &DiagramChoices,
   ) -> (f64, f64) {
      // ) -> (PangoLayout, f64, f64) {
      let area_based_scale = match drawable.size_choice {
         TextSizeChoice::Normal => 1.0,
         TextSizeChoice::Large => 1.0 / diagram_choices.annotation_area_scale,
         TextSizeChoice::Small => diagram_choices.annotation_area_scale,
      };
      let font_size = diagram_choices.font_size * area_based_scale;

      let text_layout: &mut (dyn ZvxTextLayout + 'a) = boxed_text_layout.as_mut();

      text_layout.set_layout("sans", font_size, &single_text.content);
      let metrics = text_layout.get_metrics().as_ref().unwrap();

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

      let mut height_adjust = metrics.font_ascent - metrics.strikethrough_center;
      let multiline_adjust = metrics.text_height - metrics.font_height;
      let mut width_adjust = 0.0;

      match drawable.anchor_choice {
         TextAnchorChoice::Centered => {
            height_adjust += 0.5 * multiline_adjust;
            width_adjust += 0.5 * metrics.text_width;
         }

         TextAnchorChoice::ThreeByThree(horizontal, vertical) => {
            height_adjust += match vertical {
               TextAnchorVertical::Bottom => metrics.even_half_height + multiline_adjust + offset_y,
               TextAnchorVertical::Middle => 0.5 * multiline_adjust,
               TextAnchorVertical::Top => -metrics.even_half_height - offset_y,
            };
            width_adjust += match horizontal {
               TextAnchorHorizontal::Left => -offset_x,
               TextAnchorHorizontal::Center => 0.5 * metrics.text_width,
               TextAnchorHorizontal::Right => metrics.text_width + offset_x,
            };
         }
      }

      (width_adjust, height_adjust)
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
   fn layout_text<'a, 'parent>(
      cairo_context: &'parent CairoContext,
      text_context: &'a PangoContext,
      single_text: &TextSingle,
      drawable: &TextDrawable,
      diagram_choices: &DiagramChoices,
   ) -> (Box<dyn ZvxTextLayout + 'a>, f64, f64)
   where
      'parent: 'a,
   {
      let mut pango_text_layout: Box<(dyn ZvxTextLayout + 'a)> =
         ZvxPangoTextLayout::create_pango_layout(cairo_context, text_context);

      let (width_adjust, height_adjust) =
         Self::figure_text_adjust(&mut pango_text_layout, single_text, drawable, diagram_choices);

      (pango_text_layout, width_adjust, height_adjust)
   }

   #[inline]
   fn draw_text_set_with_lifetimes<'semi_global, 'child, 'parent>(
      &'semi_global mut self,
      cairo_context: &'parent CairoContext,
      text_context: &'child PangoContext,
      single_text: &TextSingle,
      drawable: &TextDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) where
      'parent: 'child,
   {
      let (mut generic_text_layout, width_adjust, height_adjust): (
         Box<dyn ZvxTextLayout + 'child>,
         f64,
         f64,
      ) = Self::layout_text(cairo_context, text_context, single_text, drawable, diagram_choices);

      self.set_color(cairo_context, diagram_choices, drawable.color_choice);

      self.save_set_path_transform(canvas_layout, cairo_context);
      let (tx, ty) = cairo_context.user_to_device(single_text.location[0], single_text.location[1]);
      self.restore_transform(cairo_context);

      cairo_context.move_to(
         tx - width_adjust / f64::from(pango::SCALE),
         ty - height_adjust / f64::from(pango::SCALE),
      );

      let _ = generic_text_layout.render_layout();
   }

   #[allow(clippy::needless_lifetimes)]
   fn draw_text_set<'parent>(
      &mut self,
      cairo_context: &'parent CairoContext,
      drawable: &TextDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      for single_text in &drawable.texts {
         // Create a single context, instead of using create_layout.  This
         // demonstrates avoiding lots of Pango contexts.
         let text_context: PangoContext = pangocairo_create_context(cairo_context);

         self.draw_text_set_with_lifetimes(
            cairo_context,
            &text_context,
            single_text,
            drawable,
            canvas_layout,
            diagram_choices,
         );
         cairo_context.stroke().unwrap();
      }
   }

   fn draw_polyline(
      &mut self,
      context: &CairoContext,
      locations: &PolylinePath,
      path_choices: PathChoices,
      segment_choices: SegmentChoices,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(context, path_choices.line_choice, diagram_choices);
      self.set_color(context, diagram_choices, path_choices.color);

      self.save_set_path_transform(canvas_layout, context);
      assert!(!locations.is_empty());
      if segment_choices.continuation == ContinuationChoice::Starts {
         context.move_to(locations[0][0], locations[0][1]);
      }
      for location in locations.iter().skip(1) {
         context.line_to(location[0], location[1]);
      }
      match segment_choices.closure {
         LineClosureChoice::Closes => {
            context.close_path();
            self.restore_transform(context);
            context.stroke().unwrap();
         }
         LineClosureChoice::OpenEnd => {
            self.restore_transform(context);
            context.stroke().unwrap();
         }
         LineClosureChoice::Unfinished => {
            self.restore_transform(context);
         }
      }
   }

   fn draw_circles_set(
      &mut self,
      context: &CairoContext,
      drawable: &CirclesDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(context, drawable.path_choices.line_choice, diagram_choices);
      self.set_color(context, diagram_choices, drawable.path_choices.color);

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

   fn draw_segment_sequence(
      &mut self,
      context: &CairoContext,
      segment_sequence: &SegmentSequence,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      let mut segment_choices: SegmentChoices =
         SegmentChoices { closure: LineClosureChoice::Unfinished, ..Default::default() };

      for i in 0..segment_sequence.segments.len() {
         let segment = &segment_sequence.segments[i];
         if i == (segment_sequence.segments.len() - 1) {
            if segment_sequence.completion == PathCompletion::Closed {
               segment_choices.closure = LineClosureChoice::Closes;
            } else {
               segment_choices.closure = LineClosureChoice::OpenEnd;
            }
         }
         match &segment {
            OneOfSegment::Arc(path) => {
               self.draw_arc(
                  context,
                  path,
                  segment_sequence.path_choices,
                  segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfSegment::Cubic(path) => {
               self.draw_cubic(
                  context,
                  path,
                  segment_sequence.path_choices,
                  segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfSegment::Polyline(path) => {
               self.draw_polyline(
                  context,
                  path,
                  segment_sequence.path_choices,
                  segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfSegment::Nothing => {}
         }

         segment_choices.continuation = ContinuationChoice::Continues;
      }
   }

   #[allow(clippy::missing_panics_doc)]
   pub fn render_drawables(
      &mut self,
      drawables: &[QualifiedDrawable],
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
      context: &CairoContext,
   ) {
      let segment_choices: SegmentChoices = SegmentChoices::default();

      let mut indices = (0..drawables.len()).collect::<Vec<_>>();
      indices.sort_by_key(|&i| &drawables[i].layer);

      for i in indices {
         match &drawables[i].drawable {
            OneOfDrawable::Lines(drawable) => {
               self.draw_lines_set(context, drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Arc(drawable) => {
               self.draw_arc(
                  context,
                  &drawable.path,
                  drawable.path_choices,
                  segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfDrawable::Cubic(drawable) => {
               self.draw_cubic(
                  context,
                  &drawable.path,
                  drawable.path_choices,
                  segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
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
               self.draw_polyline(
                  context,
                  &drawable.path,
                  drawable.path_choices,
                  segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfDrawable::SegmentSequence(drawable) => {
               self.draw_segment_sequence(context, drawable, canvas_layout, diagram_choices);
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
