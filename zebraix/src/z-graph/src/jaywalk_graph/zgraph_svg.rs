// Copyright 2023 Google LLC
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

use crate::jaywalk_graph::zgraph_base::CoordReal2D;
use crate::jaywalk_graph::zgraph_base::PortPieceTyped;
use crate::jaywalk_graph::zgraph_base::ZColor;
use crate::jaywalk_graph::zgraph_base::ZFontStyle;
use crate::jaywalk_graph::zgraph_base::ZGraphError;
use crate::jaywalk_graph::zgraph_base::ZNodeStateData;
use crate::jaywalk_graph::zgraph_base::ZPiece;
use crate::jaywalk_graph::zgraph_base::ZPieceType;
use crate::jaywalk_graph::zgraph_base::ZRendererData;
use crate::jaywalk_graph::zgraph_graphdef::ZGraphDef;
use crate::jaywalk_graph::zgraph_machine::DebugLine;
use crate::jaywalk_graph::zgraph_machine::ZMachine;
use crate::jaywalk_graph::zgraph_registry::ZNodeRegistrationBuilder;
use crate::jaywalk_graph::zgraph_registry::ZRegistry;
use cairo::Context;
use cairo::SvgSurface;
use cairo::SvgUnit;
use json5::from_str;
use pango::Language;
use pango::Layout;
use std::any::Any;
use std::f64::consts::PI;
use std::io::Write;

pub struct SvgRendererData {
   pub main_surface: SvgSurface,
   pub main_context: Context,

   pub debug_lines: Vec<DebugLine>,
}

pub struct SvgTextNode {
   pub sample_text_layout: Layout,
}
const DEFAULT_LANGUAGE: &str = "en-US";

fn sample_text_metrics(renderer_data_in: &mut ZRendererData) {
   let renderer_data: &mut SvgRendererData =
      renderer_data_in.as_mut().unwrap().downcast_mut::<SvgRendererData>().unwrap();

   let text: String = "Hello world! pygq".to_string();
   let font_size = 10.0;
   let font_family = "sans".to_string();
   let language = DEFAULT_LANGUAGE.to_string();

   let context: &Context = &renderer_data.main_context;

   // Create a single context, instead of using create_layout.  This
   // demonstrates avoiding lots of Pango contexts.
   let text_context = pangocairo::create_context(context);
   let text_layout = pango::Layout::new(&text_context);

   let mut font_description = pango::FontDescription::new();
   font_description.set_family(&font_family[..]);
   font_description.set_absolute_size(font_size * pango::SCALE as f64);

   text_layout.set_font_description(Some(&font_description));

   text_layout.set_text(&text[..]);
   text_layout.context().set_language(Some(&Language::from_string(&language[..])));

   renderer_data.debug_lines.push(DebugLine::SimpleString("Test debug string".to_string()));

   renderer_data.debug_lines.push(DebugLine::SimpleString(format!(
      "Layout context language = {l}",
      l = text_layout.context().language().unwrap()
   )));

   let metrics = text_layout.context().metrics(Some(&font_description), None);

   renderer_data
      .debug_lines
      .push(DebugLine::SimpleString(format!("Text height = {h}", h = metrics.height())));
   renderer_data
      .debug_lines
      .push(DebugLine::SimpleString(format!("Text descent = {h}", h = metrics.descent())));
   renderer_data
      .debug_lines
      .push(DebugLine::SimpleString(format!("Text ascent = {h}", h = metrics.ascent())));

   let strikethrough_centre =
      metrics.strikethrough_position() + metrics.strikethrough_thickness() / 2;
   renderer_data
      .debug_lines
      .push(DebugLine::SimpleString(format!("Text anchor = {h}", h = strikethrough_centre)));

   // Extents depend on set_absolute_size.  Assume pango::SCALE = 1024.
   //
   // Note that 10 * 1024 * 1.362 = 13946.88.
   //
   // 14*1024: Logical extents (x, y, w, h) = 0, 0, 81920, 19525 for "Hello world!"
   // 10*1024: Logical extents (x, y, w, h) = 0, 0, 59392, 13947 for "Hello world!"
   // 10*1024: Logical extents (x, y, w, h) = 0, 0, 64512, 13947 for "Hello worldy!"
   // 10*1024: Logical extents (x, y, w, h) = 0, 0, 45056, 13947 for "ace noun"
   //
   // 14*1024: Ink extents (x, y, w, h) = 1391, 4430, 79243, 11096 for "Hello world!"
   // 10*1024: Ink extents (x, y, w, h) = 993, 3165, 57334, 7926 for "Hello world!"
   // 10*1024: Ink extents (x, y, w, h) = 993, 3165, 62454, 10239 for "Hello worldy!"
   // 10*1024: Ink extents (x, y, w, h) = 471, 5356, 43939, 5693 for "ace noun"
   //
   // 10239 ~= 10*1024.
   // 3165 + 10239 = 13404.
   // Ascent = 7926; descent = 10239 - 7926 = 2313.  So 7926 : 2313 =  0.774 : 0.226.
   //
   // Layout.spacing() = 0.
   //
   // Layout context is independent of set_absolute_size.
   // Text height = 22315
   // Text descent = 4801
   // Text ascent = 17514
   // Text anchor = 5685  (This is strikethrough position + half thickness.)
   // 17514 : 4801 = : 0.785 : 0.215, which is not correct, so ascent is padded.
   // 4801 * 7926 / 2313 = 16452.  4801 * 10239 / 2313 = 21253.
   // (22315 - 21253) / 21253 = 0.05
   //
   // This mess is unresolvable.  For now, if using anchor, descent, etc, scale by
   // 1024 / 21253 = 0.0482.
   // In other words, the metrics seem to be for 20.755pt font.
   //
   // Corrected: Context obtained with font description.
   // Text height = 13947
   // Text descent = 3000
   // Text ascent = 10947
   // Text anchor = 3553  (This is strikethrough position + half thickness.)

   let (ink_rect, logical_rect) = text_layout.extents();
   renderer_data.debug_lines.push(DebugLine::SimpleString(format!(
      "Ink extents (x, y, w, h) = {x}, {y}, {w}, {h}",
      x = ink_rect.x(),
      y = ink_rect.y(),
      w = ink_rect.width(),
      h = ink_rect.height()
   )));
   renderer_data.debug_lines.push(DebugLine::SimpleString(format!(
      "Logical extents (x, y, w, h) = {x}, {y}, {w}, {h}",
      x = logical_rect.x(),
      y = logical_rect.y(),
      w = logical_rect.width(),
      h = logical_rect.height()
   )));

   renderer_data
      .debug_lines
      .push(DebugLine::SimpleString(format!("Text spacing = {h}", h = text_layout.line_spacing())));
}

pub fn test_text_calculation(
   renderer_data_in: &mut ZRendererData,
   state_data: &mut ZNodeStateData,
   in_data: &[ZPiece],
   _out_data: &mut [ZPiece],
) {
   sample_text_metrics(renderer_data_in);

   let text: &String = in_data[0].get_text().unwrap();
   let font_style: &ZFontStyle = in_data[2].get_font_style().unwrap();

   let font_size = font_style.size;
   let font_family = &font_style.family;

   // Rust cannot do what we actually want, either for the language we
   // are pulling in, or for the default.
   let default_language_rust_copy: String = String::from(DEFAULT_LANGUAGE);
   let language: &String = if font_style.language.is_some() {
      font_style.language.as_ref().unwrap()
   } else {
      &default_language_rust_copy
   };

   let renderer_data: &mut SvgRendererData =
      renderer_data_in.as_mut().unwrap().downcast_mut::<SvgRendererData>().unwrap();

   let context: &Context = &renderer_data.main_context;

   // Create a single context, instead of using create_layout.  This
   // demonstrates avoiding lots of Pango contexts.
   let text_context = pangocairo::create_context(context);
   let text_layout = pango::Layout::new(&text_context);

   let mut font_description = pango::FontDescription::new();
   font_description.set_family(&font_family[..]);
   font_description.set_absolute_size(font_size * pango::SCALE as f64);

   text_layout.set_font_description(Some(&font_description));

   text_layout.set_text(&text[..]);
   text_layout.context().set_language(Some(&Language::from_string(&language[..])));

   *state_data = Some(Box::new(SvgTextNode { sample_text_layout: text_layout }));
}

pub fn set_source_color(context: &Context, color: &ZColor) {
   match color {
      ZColor::Rgb(r, g, b) => context.set_source_rgb(*r, *g, *b),
      _default => {}
   }
}

pub fn test_text_inking(
   renderer_data_in: &mut ZRendererData,
   state_data: &mut ZNodeStateData,
   in_data: &[ZPiece],
   _out_data: &mut [ZPiece],
) {
   let color: &ZColor = in_data[1].get_color().unwrap();
   let location: &CoordReal2D = in_data[3].get_coord2d().unwrap();

   let renderer_data: &mut SvgRendererData =
      renderer_data_in.as_mut().unwrap().downcast_mut::<SvgRendererData>().unwrap();

   let text_node_data: &mut SvgTextNode =
      state_data.as_mut().unwrap().downcast_mut::<SvgTextNode>().unwrap();

   let context: &Context = &renderer_data.main_context;
   let text_layout: &Layout = &text_node_data.sample_text_layout;

   // context.set_source_rgb(0.0, 0.0, 7.0);
   // assert_eq!(color, &ZColor::Rgb(0.0, 0.0, 0.7));
   set_source_color(context, color);
   context.move_to(location.0, location.1);
   pangocairo::show_layout(context, text_layout);
   // For now, set to ugly color to avoid accidentally reusing selected color.
   context.set_source_rgb(0.0, 1.0, 0.0);
}

pub fn test_circle_inking(
   renderer_data_in: &mut ZRendererData,
   _state_data: &mut ZNodeStateData,
   in_data: &[ZPiece],
   _out_data: &mut [ZPiece],
) {
   let center: &CoordReal2D = in_data[0].get_coord2d().unwrap();
   let radius: f64 = in_data[1].get_real().unwrap();
   let color: &ZColor = in_data[2].get_color().unwrap();

   let renderer_data: &mut SvgRendererData =
      renderer_data_in.as_mut().unwrap().downcast_mut::<SvgRendererData>().unwrap();

   let context: &Context = &renderer_data.main_context;

   set_source_color(context, color);
   context.move_to(center.0 + radius, center.1);
   context.arc(center.0, center.1, radius, 0.0 * PI, 2.0 * PI);
   context.stroke().unwrap();
   context.set_source_rgb(0.0, 1.0, 0.0);
}

pub fn register_svg_library(registry: &mut ZRegistry) {
   registry.register_new(
      ZNodeRegistrationBuilder::default().name("Test text".to_string()).build().unwrap(),
   );
}

#[derive(Default)]
pub struct RenderSvg {}

impl Renderer for RenderSvg {
   fn setup_render_to_stream<W: Write + 'static>(
      &self,
      zgraph: &mut ZMachine,
      out_stream: W,
   ) -> Result<(), ZGraphError> {
      if zgraph.renderer_data.is_some() {
         return Err(ZGraphError::DuplicateRendererSetup);
      }

      let canvas_width = 320.0; //  overall.canvas_width,
      let canvas_height = 240.0; // overall.canvas_height
      let x_offset = 40.0; //       overall.canvas_x_offset
      let y_offset = 50.0; //       overall.canvas_y_offset

      let mut surface = SvgSurface::for_stream(canvas_width, canvas_height, out_stream).unwrap();
      surface.set_document_unit(SvgUnit::Pt);
      let context = cairo::Context::new(&surface).unwrap();
      context.translate(x_offset, y_offset);

      let renderer_data: Option<Box<dyn Any>> = Some(Box::new(SvgRendererData {
         main_surface: surface,
         main_context: context,
         debug_lines: Vec::<DebugLine>::new(),
      }));

      zgraph.renderer_data = renderer_data;
      Ok(())
   }

   fn finish_renderer(
      &self,
      zgraph: &mut ZMachine,
   ) -> Result<Result<Box<dyn core::any::Any>, cairo::StreamWithError>, ZGraphError> {
      if zgraph.renderer_data.is_none() {
         return Err(ZGraphError::FinishNoRenderer);
      }

      let renderer_data: &mut SvgRendererData =
         zgraph.renderer_data.as_mut().unwrap().downcast_mut::<SvgRendererData>().unwrap();

      let surface: &SvgSurface = &renderer_data.main_surface;

      surface.flush();
      let mut result = surface.finish_output_stream();

      if result.is_ok() {
         {
            let borrowed_writer: &mut Vec<u8> =
               result.as_mut().unwrap().as_mut().downcast_mut::<Vec<u8>>().unwrap();
            for d in &renderer_data.debug_lines {
               match d {
                  DebugLine::SimpleString(s) => {
                     borrowed_writer.write_fmt(format_args!("<!-- {} -->\n", s)).unwrap()
                  }
               }
            }
         }
      }
      zgraph.renderer_data = None;

      Ok(result)
   }
}

pub trait Renderer {
   fn setup_render_to_stream<W: Write + 'static>(
      &self,
      zgraph: &mut ZMachine,
      out_stream: W,
   ) -> Result<(), ZGraphError>;

   fn finish_renderer(
      &self,
      zgraph: &mut ZMachine,
   ) -> Result<Result<Box<dyn core::any::Any>, cairo::StreamWithError>, ZGraphError>;
}

pub fn register_renderer_library(registry: &mut ZRegistry) {
   let graphdef_one_node: &'static str = r#"
{
  "name": "Subgraph with just one node",
  "inputs": [
    [ "sg_color", "Color" ],
    [ "sg_center", "Coord2D" ],
    [ "sg_radius", "Real" ]
  ],
  "nodes": [
    {
      "name": "actual_circle",
      "element": "Test circle",
      "edges": [
        { "src_node": "inputs",
          "connections": [
            [ "sg_color", "color" ],
            [ "sg_center", "center" ],
            [ "sg_radius", "radius" ]
          ] }
      ]
    }
  ],
  "output_ports": [
    [ "placeholder", "actual_circle", "void" ]
  ]
}
"#;

   registry.register_new(
      ZNodeRegistrationBuilder::default()
         .name("Test text".to_string())
         .calculation_fn(test_text_calculation)
         .inking_fn(test_text_inking)
         .ports_dest_copy(vec![
            PortPieceTyped("text".to_string(), ZPieceType::Text),
            PortPieceTyped("color".to_string(), ZPieceType::Color),
            PortPieceTyped("font style".to_string(), ZPieceType::FontStyle),
            PortPieceTyped("location".to_string(), ZPieceType::Coord2D),
         ])
         .build()
         .unwrap(),
   );
   registry.register_new(
      ZNodeRegistrationBuilder::default()
         .name("Test circle".to_string())
         .inking_fn(test_circle_inking)
         .ports_dest_copy(vec![
            PortPieceTyped("center".to_string(), ZPieceType::Coord2D),
            PortPieceTyped("radius".to_string(), ZPieceType::Real),
            PortPieceTyped("color".to_string(), ZPieceType::Color),
         ])
         .build()
         .unwrap(),
   );
   let _deserialized_one_node = from_str::<ZGraphDef>(graphdef_one_node).unwrap();
   registry.register_new(
      ZNodeRegistrationBuilder::default()
         .name("Test circle compound".to_string())
         // .graph_def(deserialized_one_node)
         .inking_fn(test_circle_inking)
         .ports_dest_copy(vec![
            PortPieceTyped("center".to_string(), ZPieceType::Coord2D),
            PortPieceTyped("radius".to_string(), ZPieceType::Real),
            PortPieceTyped("color".to_string(), ZPieceType::Color),
         ])
         .build()
         .unwrap(),
   );
}
