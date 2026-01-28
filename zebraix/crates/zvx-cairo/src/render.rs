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
use zvx_base::{ArcPath, CubicPath, HyperbolicPath, OneOfSegment, PolylinePath};
use zvx_curves::base::TEval;
use zvx_drawable::choices::{
   CanvasLayout, ColorChoice, ContinuationChoice, DiagramChoices, LineChoice, LineClosureChoice,
   PathCompletion, PointChoice, TextAnchorChoice, TextAnchorHorizontal, TextAnchorVertical,
   TextOffsetChoice, TextSizeChoice,
};
use zvx_drawable::interface::{TextMetrics, ZvxRenderEngine, ZvxTextLayout};
use zvx_drawable::kinds::{
   CirclesSet, LinesSetSet, MarkupChoice, OneOfDrawable, PathChoices, PointsDrawable,
   QualifiedDrawable, SegmentChoices, SegmentSequence, Strokeable, TextDrawable, TextSingle,
};

#[derive(Debug)]
pub struct UnfixedCairoSpartanRender {
   pub context: CairoContext,
   pub surface: SvgSurface,
   pub transform_saver: TransformSaver,
   pub pango_context: PangoContext,
   pub num_segments_hyperbolic: i32, // Actually fixed, but more convenient here.
}

#[derive(Debug)]
pub struct TransformSaver {
   pub saved_matrix: Matrix,
}

#[derive(Debug)]
pub struct CairoSpartanRender {
   pub unfixed: UnfixedCairoSpartanRender,
   pub canvas_layout: CanvasLayout,
   pub diagram_choices: DiagramChoices,
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
   ) -> Box<dyn ZvxTextLayout + 'a>
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

impl CairoSpartanRender {
   // Since SvgSurface::for_stream will be monomorphized, this function cannot be made
   // non-templated.
   #[allow(clippy::missing_panics_doc)]
   pub fn create_for_stream<W: Write + 'static>(
      out_stream: W,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) -> Box<dyn ZvxRenderEngine> {
      Box::new(Self::create_not_boxed_for_stream(out_stream, canvas_layout, diagram_choices))
   }

   // Since SvgSurface::for_stream will be monomorphized, this function cannot be made
   // non-templated.
   #[allow(clippy::missing_panics_doc)]
   pub fn create_not_boxed_for_stream<W: Write + 'static>(
      out_stream: W,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) -> Self {
      let mut surface = SvgSurface::for_stream(
         canvas_layout.canvas_size[0],
         canvas_layout.canvas_size[1],
         out_stream,
      )
      .unwrap();
      surface.set_document_unit(Pt);

      let context = cairo::Context::new(&surface).unwrap();
      let pango_context: PangoContext = pangocairo_create_context(&context);

      Self {
         unfixed: UnfixedCairoSpartanRender {
            context,
            surface,
            pango_context,
            transform_saver: TransformSaver { saved_matrix: Matrix::default() },
            num_segments_hyperbolic: Self::default_num_segments_hyperbolic(),
         },
         canvas_layout: canvas_layout.clone(),
         diagram_choices: diagram_choices.clone(),
      }
   }
}

#[allow(clippy::elidable_lifetime_names)]
impl<'parent> ZvxTextLayout for ZvxPangoTextLayout<'parent> {
   // Not a great method name.
   fn set_layout(&mut self, font_family: &str, font_size: f64, single_text: &TextSingle) {
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

      match single_text.markup {
         MarkupChoice::Auto | MarkupChoice::Plain => {
            self.pango_text_layout.set_text(&single_text.content);
            // text_layout.set_layout("sans", font_size);
         }
         MarkupChoice::Pango => {
            let accel_marker = '_';
            // let (attr_list, plain_text, accel_char) =
            //    pangocairo_parse_markup(&single_text.content, accel_marker).unwrap();
            self.pango_text_layout.set_markup_with_accel(&single_text.content, accel_marker);
         }
      }

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

   // Call set_layout first.
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
   pub const fn default_num_segments_hyperbolic() -> i32 {
      50
   }
}

impl TransformSaver {
   // This is necessary because line thicknesses and similar are distorted if the x and y
   // scales differ.  Consequently we only use the scaling transform for the Cairo CTM when
   // creating paths.
   pub fn save_set_path_transform(&mut self, context: &CairoContext, canvas_layout: &CanvasLayout) {
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
}

impl UnfixedCairoSpartanRender {
   #[allow(clippy::unused_self)]
   fn set_color(context: &CairoContext, _diagram_choices: &DiagramChoices, color: &ColorChoice) {
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

   fn stroke_and_fill(
      &self,
      context: &CairoContext,
      diagram_choices: &DiagramChoices,
      path_choices: &PathChoices,
   ) {
      if path_choices.fill_choices.opacity != 0.0 {
         Self::set_color(context, diagram_choices, &path_choices.fill_choices.color);
         self.context.fill_preserve().unwrap();
      }
      Self::set_color(context, diagram_choices, &path_choices.color);
      self.context.stroke().unwrap();
   }

   fn draw_lines_set(
      &mut self,
      drawable: &Strokeable<LinesSetSet>,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(&self.context, drawable.path_choices.line_choice, diagram_choices);
      Self::set_color(&self.context, diagram_choices, &drawable.path_choices.color);

      self.transform_saver.save_set_path_transform(&self.context, canvas_layout);
      for i in 0..drawable.path.coords.len() {
         if let Some(offset_vector) = &drawable.path.offsets {
            for offset in offset_vector {
               self.context.move_to(
                  drawable.path.coords[i].0[0] + offset[0],
                  drawable.path.coords[i].0[1] + offset[1],
               );
               self.context.line_to(
                  drawable.path.coords[i].1[0] + offset[0],
                  drawable.path.coords[i].1[1] + offset[1],
               );
            }
         } else {
            self.context.move_to(drawable.path.coords[i].0[0], drawable.path.coords[i].0[1]);
            self.context.line_to(drawable.path.coords[i].1[0], drawable.path.coords[i].1[1]);
         }
      }
      self.transform_saver.restore_transform(&self.context);
      self.context.stroke().unwrap();
   }

   fn draw_points_set(
      &mut self,
      drawable: &PointsDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(&self.context, LineChoice::Ordinary, diagram_choices);
      Self::set_color(&self.context, diagram_choices, &drawable.color_choice);

      match drawable.point_choice {
         PointChoice::Circle => {
            for center in &drawable.centers {
               self.transform_saver.save_set_path_transform(&self.context, canvas_layout);
               let (cx, cy) = self.context.user_to_device(center[0], center[1]);
               self.transform_saver.restore_transform(&self.context);
               self.context.move_to(cx + 2.8, cy);
               self.context.arc(cx, cy, 2.8, 0.0 * PI, 2.0 * PI);
               self.context.close_path();
            }
         }
         PointChoice::Dot => {
            for center in &drawable.centers {
               self.transform_saver.save_set_path_transform(&self.context, canvas_layout);
               let (cx, cy) = self.context.user_to_device(center[0], center[1]);
               self.transform_saver.restore_transform(&self.context);
               #[allow(clippy::suboptimal_flops)]
               self.context.move_to(cx + 2.8 * 0.92, cy);
               #[allow(clippy::suboptimal_flops)]
               self.context.arc(cx, cy, 2.8 * 0.92, 0.0 * PI, 2.0 * PI);
               self.context.fill().unwrap();
               self.context.close_path();
            }
         }
         PointChoice::Plus => {
            for center in &drawable.centers {
               self.transform_saver.save_set_path_transform(&self.context, canvas_layout);
               let (cx, cy) = self.context.user_to_device(center[0], center[1]);
               self.transform_saver.restore_transform(&self.context);
               let plus_delta = 2.8 * 1.48;
               self.context.move_to(cx, cy - plus_delta);
               self.context.line_to(cx, cy + plus_delta);
               self.context.move_to(cx + plus_delta, cy);
               self.context.line_to(cx - plus_delta, cy);
               self.context.close_path();
            }
         }
         PointChoice::Times => {
            for center in &drawable.centers {
               self.transform_saver.save_set_path_transform(&self.context, canvas_layout);
               let (cx, cy) = self.context.user_to_device(center[0], center[1]);
               self.transform_saver.restore_transform(&self.context);
               let times_delta = 2.8 * 1.48 * (0.5_f64).sqrt();
               self.context.move_to(cx - times_delta, cy - times_delta);
               self.context.line_to(cx + times_delta, cy + times_delta);
               self.context.move_to(cx + times_delta, cy - times_delta);
               self.context.line_to(cx - times_delta, cy + times_delta);
               self.context.close_path();
            }
         }
         PointChoice::Square => {
            for center in &drawable.centers {
               self.transform_saver.save_set_path_transform(&self.context, canvas_layout);
               let (cx, cy) = self.context.user_to_device(center[0], center[1]);
               self.transform_saver.restore_transform(&self.context);
               let square_delta = 2.8 * 1.1 * (0.5_f64).sqrt();
               self.context.move_to(cx - square_delta, cy - square_delta);
               self.context.line_to(cx + square_delta, cy - square_delta);
               self.context.line_to(cx + square_delta, cy + square_delta);
               self.context.line_to(cx - square_delta, cy + square_delta);
               self.context.close_path();
            }
         }
      }
      self.context.stroke().unwrap();
   }

   fn draw_arc(
      &mut self,
      path: &ArcPath,
      path_choices: &PathChoices,
      segment_choices: &SegmentChoices,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(&self.context, path_choices.line_choice, diagram_choices);

      self.transform_saver.save_set_path_transform(&self.context, canvas_layout);

      let arc_transformation_matrix = cairo::Matrix::new(
         path.transform[0],
         path.transform[1],
         path.transform[2],
         path.transform[3],
         path.center[0],
         path.center[1],
      );
      self.context.transform(arc_transformation_matrix);

      // Logically circle is center (0.0, 0.0) radius 1.0.
      if segment_choices.continuation == ContinuationChoice::Starts {
         self.context.move_to(path.angle_range[0].cos(), path.angle_range[0].sin());
      }
      self.context.arc(0.0, 0.0, 1.0, path.angle_range[0], path.angle_range[1]);
      match segment_choices.closure {
         LineClosureChoice::Closes => {
            self.context.close_path();
            self.transform_saver.restore_transform(&self.context);
            self.stroke_and_fill(&self.context, diagram_choices, path_choices);
         }
         LineClosureChoice::OpenEnd => {
            self.transform_saver.restore_transform(&self.context);
            self.stroke_and_fill(&self.context, diagram_choices, path_choices);
         }
         LineClosureChoice::Unfinished => {
            self.transform_saver.restore_transform(&self.context);
         }
      }
   }

   fn draw_cubic(
      &mut self,
      path: &CubicPath,
      path_choices: &PathChoices,
      segment_choices: &SegmentChoices,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(&self.context, path_choices.line_choice, diagram_choices);

      self.transform_saver.save_set_path_transform(&self.context, canvas_layout);

      if segment_choices.continuation == ContinuationChoice::Starts {
         self.context.move_to(path.h.0[0][0], path.h.0[1][0]);
      }
      let third = 1.0 / 3.0;
      self.context.curve_to(
         third * path.h.0[0][1],
         third * path.h.0[1][1],
         third * path.h.0[0][2],
         third * path.h.0[1][2],
         path.h.0[0][3],
         path.h.0[1][3],
      );
      match segment_choices.closure {
         LineClosureChoice::Closes => {
            self.context.close_path();
            self.transform_saver.restore_transform(&self.context);
            self.stroke_and_fill(&self.context, diagram_choices, path_choices);
         }
         LineClosureChoice::OpenEnd => {
            self.transform_saver.restore_transform(&self.context);
            self.stroke_and_fill(&self.context, diagram_choices, path_choices);
         }
         LineClosureChoice::Unfinished => {
            self.transform_saver.restore_transform(&self.context);
         }
      }
   }

   fn draw_hyperbolic(
      &mut self,
      path: &HyperbolicPath,
      path_choices: &PathChoices,
      segment_choices: &SegmentChoices,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      // OK to delete when actual numbers are set (other than default).
      assert_eq!(self.num_segments_hyperbolic, 50);
      // Since hyperbolic is not supported in SVG, we do a simple polyline approximation.
      let t_int: Vec<i32> = (0..self.num_segments_hyperbolic).collect();
      let mut t = Vec::<f64>::with_capacity(t_int.len());
      let scale = (path.range.1 - path.range.0) / f64::from(self.num_segments_hyperbolic);
      let offset = path.range.0;
      for item in &t_int {
         t.push(f64::from(*item).mul_add(scale, offset));
      }

      let pattern_vec = path.eval_maybe_bilinear(&t);

      self.draw_polyline(
         &pattern_vec,
         path_choices,
         segment_choices,
         canvas_layout,
         diagram_choices,
      );
   }

   fn draw_polyline(
      &mut self,
      locations: &PolylinePath,
      path_choices: &PathChoices,
      segment_choices: &SegmentChoices,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(&self.context, path_choices.line_choice, diagram_choices);

      self.transform_saver.save_set_path_transform(&self.context, canvas_layout);
      assert!(!locations.is_empty());
      if segment_choices.continuation == ContinuationChoice::Starts {
         self.context.move_to(locations[0][0], locations[0][1]);
      }
      for location in locations.iter().skip(1) {
         self.context.line_to(location[0], location[1]);
      }
      match segment_choices.closure {
         LineClosureChoice::Closes => {
            self.context.close_path();
            self.transform_saver.restore_transform(&self.context);
            self.stroke_and_fill(&self.context, diagram_choices, path_choices);
         }
         LineClosureChoice::OpenEnd => {
            self.transform_saver.restore_transform(&self.context);
            self.stroke_and_fill(&self.context, diagram_choices, path_choices);
         }
         LineClosureChoice::Unfinished => {
            self.transform_saver.restore_transform(&self.context);
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
   //
   // This is an awkward function, and perhaps be a method in `ZvxTextLayout` trait.
   //
   // Also the return values should be in a struct, with perhaps options as to how deeply to
   // analyse.  For example, it could be that the center of "x" should be a centerline
   // estimate, or the center of "+", depending on user choice.
   #[inline]
   fn layout_text_adjust_impl<'a>(
      boxed_text_layout: &mut Box<dyn ZvxTextLayout + 'a>,
      single_text: &TextSingle,
      drawable: &TextDrawable,
      diagram_choices: &DiagramChoices,
   ) -> (f64, f64) {
      let area_based_scale = match drawable.size_choice {
         TextSizeChoice::Normal => 1.0,
         TextSizeChoice::Large => 1.0 / diagram_choices.annotation_area_scale,
         TextSizeChoice::Small => diagram_choices.annotation_area_scale,
      };
      let font_size = diagram_choices.font_size * area_based_scale;

      let text_layout: &mut (dyn ZvxTextLayout + 'a) = boxed_text_layout.as_mut();

      text_layout.set_layout("sans", font_size, single_text);

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
   //
   // This is an awkward function, and perhaps be a method in `ZvxTextLayout` trait.
   #[inline]
   fn layout_text<'a, 'parent>(
      cairo_context: &'parent CairoContext,
      pango_context: &'a PangoContext,
      single_text: &TextSingle,
      drawable: &TextDrawable,
      diagram_choices: &DiagramChoices,
   ) -> (Box<dyn ZvxTextLayout + 'a>, f64, f64)
   where
      'parent: 'a,
   {
      let mut pango_text_layout: Box<dyn ZvxTextLayout + 'a> =
         ZvxPangoTextLayout::create_pango_layout(cairo_context, pango_context);

      let (width_adjust, height_adjust) = Self::layout_text_adjust_impl(
         &mut pango_text_layout,
         single_text,
         drawable,
         diagram_choices,
      );

      (pango_text_layout, width_adjust, height_adjust)
   }

   // This is deliberately not a class method.  The caller needs to borrow parts of
   // `UnfixedCairoSpartanRender` with different mutability.  While some implementations can
   // resolve the mutability issues by reordring code, this would introduce unacceptable
   // long-term constraints.
   #[inline]
   fn draw_text_set_with_lifetimes<'semi_global, 'child, 'parent>(
      // &'parent mut self,
      transform_saver: &'semi_global mut TransformSaver,
      cairo_context: &'parent CairoContext,
      pango_context: &'child PangoContext,
      single_text: &TextSingle,
      drawable: &TextDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) where
      // 'semi_global: 'child,
      'parent: 'child,
   {
      // This call sits awkwardly here.  It maybe should be in the caller, and be a self-method.
      //
      // Or, call `ZvxPangoTextLayout::create_pango_layout` in the caller, and pass the
      // resulting Box<dyn ZvxTextLayout + 'a> in as an argument here.
      let (mut generic_text_layout, width_adjust, height_adjust): (
         Box<dyn ZvxTextLayout + 'child>,
         f64,
         f64,
      ) = Self::layout_text(cairo_context, pango_context, single_text, drawable, diagram_choices);

      Self::set_color(cairo_context, diagram_choices, &drawable.color_choice);

      transform_saver.save_set_path_transform(cairo_context, canvas_layout);
      let (tx, ty) = cairo_context.user_to_device(single_text.location[0], single_text.location[1]);
      transform_saver.restore_transform(cairo_context);

      cairo_context.move_to(
         tx - width_adjust / f64::from(pango::SCALE),
         ty - height_adjust / f64::from(pango::SCALE),
      );

      let _ = generic_text_layout.render_layout();
   }

   #[allow(clippy::needless_lifetimes)]
   fn draw_text_set<'parent>(
      &'parent mut self,
      drawable: &TextDrawable,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      for single_text in &drawable.texts {
         Self::draw_text_set_with_lifetimes(
            &mut self.transform_saver,
            &self.context,
            &self.pango_context,
            single_text,
            drawable,
            canvas_layout,
            diagram_choices,
         );
         self.context.stroke().unwrap();
      }
   }

   fn draw_circles_set(
      &mut self,
      drawable: &Strokeable<CirclesSet>,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      Self::set_line_choice(&self.context, drawable.path_choices.line_choice, diagram_choices);

      self.transform_saver.save_set_path_transform(&self.context, canvas_layout);
      for center in &drawable.path.centers {
         let (cx, cy) = (center[0], center[1]);
         let r = drawable.path.radius;

         self.context.move_to(cx + r, cy);
         self.context.arc(cx, cy, r, 0.0 * PI, 2.0 * PI);
         self.context.close_path();
      }
      self.transform_saver.restore_transform(&self.context);
      self.stroke_and_fill(&self.context, diagram_choices, &drawable.path_choices);
   }

   fn draw_segment_sequence(
      &mut self,
      segment_sequence: &SegmentSequence,
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      let mut line_closure_choice = LineClosureChoice::Unfinished;
      let mut line_continuation_choice = ContinuationChoice::Starts;
      for i in 0..segment_sequence.segments.len() {
         let segment = &segment_sequence.segments[i];
         if i == (segment_sequence.segments.len() - 1) {
            if segment_sequence.completion == PathCompletion::Closed {
               line_closure_choice = LineClosureChoice::Closes;
            } else {
               line_closure_choice = LineClosureChoice::OpenEnd;
            }
         }
         let segment_choices: SegmentChoices =
            SegmentChoices { closure: line_closure_choice, continuation: line_continuation_choice };

         match &segment {
            OneOfSegment::Arc(path) => {
               self.draw_arc(
                  path,
                  &segment_sequence.path_choices,
                  &segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfSegment::Cubic(path) => {
               self.draw_cubic(
                  path,
                  &segment_sequence.path_choices,
                  &segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfSegment::Hyperbolic(path) => {
               self.draw_hyperbolic(
                  path,
                  &segment_sequence.path_choices,
                  &segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfSegment::Polyline(path) => {
               self.draw_polyline(
                  path,
                  &segment_sequence.path_choices,
                  &segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfSegment::Neither => {}
         }

         line_continuation_choice = ContinuationChoice::Continues;
      }
   }

   #[allow(clippy::missing_panics_doc)]
   pub fn render_drawables_impl(
      &mut self,
      drawables: &[QualifiedDrawable],
      canvas_layout: &CanvasLayout,
      diagram_choices: &DiagramChoices,
   ) {
      let segment_choices: SegmentChoices = SegmentChoices::default();

      let mut indices = (0..drawables.len()).collect::<Vec<_>>();
      indices.sort_by_key(|&i| &drawables[i].layer);

      for i in indices {
         match &drawables[i].drawable {
            OneOfDrawable::Lines(drawable) => {
               self.draw_lines_set(drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Arc(drawable) => {
               self.draw_arc(
                  &drawable.path,
                  &drawable.path_choices,
                  &segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfDrawable::Hyperbolic(drawable) => {
               self.draw_hyperbolic(
                  &drawable.path,
                  &drawable.path_choices,
                  &segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfDrawable::Cubic(drawable) => {
               self.draw_cubic(
                  &drawable.path,
                  &drawable.path_choices,
                  &segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfDrawable::Points(drawable) => {
               self.draw_points_set(drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Text(drawable) => {
               self.draw_text_set(drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Circles(drawable) => {
               self.draw_circles_set(drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Polyline(drawable) => {
               self.draw_polyline(
                  &drawable.path,
                  &drawable.path_choices,
                  &segment_choices,
                  canvas_layout,
                  diagram_choices,
               );
            }
            OneOfDrawable::SegmentSequence(drawable) => {
               self.draw_segment_sequence(drawable, canvas_layout, diagram_choices);
            }
            OneOfDrawable::Neither => {}
         }
      }
   }
}

impl ZvxRenderEngine for CairoSpartanRender {
   fn create_text_layout<'parent, 'a>(&'parent self) -> Box<dyn ZvxTextLayout + 'a>
   where
      'parent: 'a,
   {
      ZvxPangoTextLayout::create_pango_layout(&self.unfixed.context, &self.unfixed.pango_context)
   }

   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   fn render_drawables(
      &mut self,
      drawables: &[QualifiedDrawable],
   ) -> Result<Box<dyn core::any::Any>, Box<dyn Error>> {
      let canvas_layout: &CanvasLayout = &self.canvas_layout;
      let diagram_choices: &DiagramChoices = &self.diagram_choices;

      self.unfixed.render_drawables_impl(drawables, canvas_layout, diagram_choices);

      self.unfixed.surface.flush();

      match self.unfixed.surface.finish_output_stream() {
         Ok(good) => Ok(good),
         // SvgSurface keeps the stream, and returns when there is an error, but we just drop it
         // and pass the error.
         Err(stream_with_error) => Err(Box::new(stream_with_error.error)),
      }
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
