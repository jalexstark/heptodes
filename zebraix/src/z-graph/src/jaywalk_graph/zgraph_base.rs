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

// pub mod jaywalk_foundation;
// pub mod jaywalk_traiting;

extern crate derive_builder;

extern crate cairo;
extern crate pango;
extern crate pangocairo;

use crate::jaywalk_graph::jaywalk_foundation::is_default;
use crate::jaywalk_graph::jaywalk_traiting::is_mult_ident_f64;
use crate::jaywalk_graph::jaywalk_traiting::mult_ident_f64;
use crate::jaywalk_graph::JVec;
use cairo::Context;
use cairo::SvgSurface;
use cairo::SvgUnit;
use derive_builder::Builder;
use pango::Language;
use pango::Layout;
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use std::any::Any;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub enum Types {
   Void = 0,
   Integer,
   Real,
   Bool,
   Weighted,
   WeightedView,
   //
   Group,
   GraphOutput,
   GraphInput,
   BoxedText,
   Circle,
   TextLine,
}

#[derive(Serialize, Deserialize)]
pub enum ZDataByType {
   Void = 0,
   Dirty,
   Derived,
   Fit,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ZData {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

#[derive(Clone, Serialize, Deserialize, DefaultFromSerde)]
pub struct ZNodeTypeFinder {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ZNodeDef {
   pub name: String,
   #[serde(skip_serializing_if = "is_default")]
   pub description: Option<String>,

   // #[serde(skip_serializing_if = "is_default")]
   #[serde(default)]
   pub node_type: ZNodeTypeFinder,

   #[serde(skip_serializing_if = "is_default")]
   pub preset_data: Option<ZData>,
}

// impl EmptyVec for ZNodeDef {
//    type Item = ZNodeDef;

//    fn empty_vec() -> &'static Vec<ZNodeDef> {
//       static EMPTY_VEC: Vec<ZNodeDef> = Vec::<ZNodeDef>::new();
//       &EMPTY_VEC
//    }
// }

#[derive(Clone, Serialize, Deserialize)]
pub struct ZEdgeDef {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

// impl EmptyVec for ZEdgeDef {
//    type Item = ZEdgeDef;

//    fn empty_vec() -> &'static Vec<ZEdgeDef> {
//       static EMPTY_VEC: Vec<ZEdgeDef> = Vec::<ZEdgeDef>::new();
//       &EMPTY_VEC
//    }
// }

#[derive(Serialize, Deserialize)]
pub struct ZEdgeDataDef {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ZWeighted {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ZWeightedView {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ZFree {
   #[serde(skip_serializing_if = "is_mult_ident_f64")]
   #[serde(default = "mult_ident_f64")]
   pub field_c: f64,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZGraphDefCategory {
   // GraphDef is complete, connecting input and output, and expanded
   // on construction.
   UserGraph = 0,
   // Library: Similar to UserGraph.
   LibraryGraph,
   // Library-like element not expanded for sizing, only if not
   // supported by renderer.
   LibraryDrawable,
   // Element not expanded for sizing, in set of renderer-required.
   BuiltinDrawable,
   // Source that introduces new free variable, and outputs weighted
   // with unit weight.
   FreeWeighted,
   // LP-understood operator, single Weighted output.
   OperatorWeighted,
   // Not intended to have anything renderable. Cannot output
   // Weighted. Can output multiple non-Weighted.
   OperatorGeneral,
   // A general expression, which must expand to a sub-graph, even if
   // to single ExpressionGraph.
   ExpressionGraph,
   // An expression that Zebraix evaluates in one go. Cannot output
   // Weighted. Can write multiple outputs.
   ExpressionEvaluator,
   // Converts weighted to real, demanding finalization.
   Finalizer,
   // Sink that consumes one or more weighted and adds to objective
   // function.
   Objective,
   // Source that introduces preset value.
   Preset,
}

impl Default for ZGraphDefCategory {
   fn default() -> Self {
      ZGraphDefCategory::UserGraph
   }
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ZGraphDef {
   #[serde(default, skip_serializing_if = "is_default")]
   pub category: ZGraphDefCategory,

   pub name: String,
   #[serde(skip_serializing_if = "is_default")]
   pub description: Option<String>,

   #[serde(default)]
   pub nodes: JVec<ZNodeDef>,
   #[serde(default)]
   pub edges: JVec<ZEdgeDef>,
}

#[derive(PartialEq, Eq)]
pub enum ZMachineTypestate {
   Init = 0,
   Deffed,
   Constructed,
   Calculated,
   Inked,
   Finished,
}

pub struct SvgRendererData {
   pub main_surface: SvgSurface,
   pub main_context: Context,

   pub debug_lines: Vec<DebugLine>,
}

pub struct SvgTextNode {
   pub sample_text_layout: Layout,
}

type ZNodeStateData = Option<Box<dyn Any>>;
type ZRendererData = Option<Box<dyn Any>>;

pub struct ZMachine {
   pub typestate: ZMachineTypestate,
   pub has_graph_def: bool,

   pub graph_def: ZGraphDef,
   // // Mirroring ZGraphDef.
   // pub category: ZGraphDefCategory,
   // pub name: String,
   // pub description: Option<String>,
   // pub nodes: JVec<ZNodeDef>,
   // pub edges: JVec<ZEdgeDef>,

   // SVG renderer.
   pub renderer_data: ZRendererData,
   pub test_text_node_data: ZNodeStateData,

   // Node registry
   registry: ZRegistry,
}

#[derive(Debug)]
pub enum ZGraphError {
   IncorrectTypestateTransition,
   DuplicateGraphDef,
   MissingGraphDef,
   DuplicateRendererSetup,
   RendererForConstruction,
   FinishNoRenderer,
   UnfinishedRenderer,
}

#[derive(Debug)]
pub enum DebugLine {
   SimpleString(String),
}

impl ZMachine {
   pub fn new() -> Self {
      Self {
         typestate: ZMachineTypestate::Init,
         has_graph_def: false,

         graph_def: ZGraphDef::default(),
         // // ZGraphDef fields.
         // category: ZGraphDefCategory::default(),
         // name: String::default(),
         // description: Option::<String>::default(),
         // nodes: JVec::<ZNodeDef>::default(),
         // edges: JVec::<ZEdgeDef>::default(),

         // SVG renderer.
         renderer_data: None,
         // main_surface: Option::<SvgSurface>::default(),
         // main_context: Option::<Context>::default(),
         test_text_node_data: None,

         // Node registry
         registry: ZRegistry::default(),
      }
   }

   // User-level graphdef for overall drawing.
   pub fn provide_graph_def(&mut self, graph_def: &ZGraphDef) -> Result<(), ZGraphError> {
      if self.has_graph_def {
         return Err(ZGraphError::DuplicateGraphDef);
      }

      self.graph_def = graph_def.clone();
      self.has_graph_def = true;
      Ok(())
   }

   pub fn transition_to_deffed(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Init {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }
      if !self.has_graph_def {
         return Err(ZGraphError::MissingGraphDef);
      }
      self.typestate = ZMachineTypestate::Deffed;
      Ok(())
   }

   pub fn transition_to_constructed(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Deffed {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }
      if self.renderer_data.is_none() {
         return Err(ZGraphError::RendererForConstruction);
      }

      //

      test_text_construction(
         &mut self.renderer_data,
         &mut self.test_text_node_data,
         &ZData::default(),
         &mut ZData::default(),
      );

      //

      self.typestate = ZMachineTypestate::Constructed;
      Ok(())
   }

   pub fn transition_to_calculated(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Constructed {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }
      self.typestate = ZMachineTypestate::Calculated;
      Ok(())
   }

   pub fn transition_to_inked(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Calculated {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }

      //

      test_text_inking(
         &mut self.renderer_data,
         &mut self.test_text_node_data,
         &ZData::default(),
         &mut ZData::default(),
      );

      //

      self.typestate = ZMachineTypestate::Inked;
      Ok(())
   }

   pub fn transition_to_finished(&mut self) -> Result<(), ZGraphError> {
      if self.typestate != ZMachineTypestate::Inked {
         return Err(ZGraphError::IncorrectTypestateTransition);
      }
      if self.renderer_data.is_some() {
         return Err(ZGraphError::UnfinishedRenderer);
      }
      self.typestate = ZMachineTypestate::Finished;
      Ok(())
   }
}

impl Default for ZMachine {
   fn default() -> Self {
      Self::new()
   }
}

#[derive(Default, Builder)]
pub struct ZNodeRegistration {
   name: String,
   construction_fn: Option<ZNodeInvocationFn>,
   calculation_fn: Option<ZNodeInvocationFn>,
   inking_fn: Option<ZNodeInvocationFn>,
}

#[derive(Default)]
pub struct ZRegistry {
   pub node_registrations: HashMap<String, ZNodeRegistration>,
}

// pub enum ZNodeStateData {
//    None,
//    RendererNode(Box<dyn Any>),
// }

// impl Default for ZNodeStateData {
//    fn default() -> Self {
//       ZNodeStateData::None
//    }
// }

//     pub struct ZNodeStateData<'a> {
//    pub data: &'a mut Option<Box<dyn Any>>,
// }

// pub struct ZRendererData<'a> {
//    pub data: &'a mut Option<Box<dyn Any>>,
// }

type ZNodeInvocationFn = fn(&mut ZRendererData, &mut ZNodeStateData, &ZData, &mut ZData);

impl ZRegistry {
   pub fn register_new(&mut self, _node_registration: ZNodeRegistration) {}
}

fn test_text_construction(
   renderer_data_in: &mut ZRendererData,
   state_data: &mut ZNodeStateData,
   _in_data: &ZData,
   _out_data: &mut ZData,
) {
   let renderer_data: &mut SvgRendererData =
      renderer_data_in.as_mut().unwrap().downcast_mut::<SvgRendererData>().unwrap();

   let context: &Context = &renderer_data.main_context;

   // Create a single context, instead of using create_layout.  This
   // demonstrates avoiding lots of Pango contexts.
   let text_context = pangocairo::create_context(context);
   let text_layout = pango::Layout::new(&text_context);

   let k_label_font_size = 10.0;

   let mut font_description = pango::FontDescription::new();
   font_description.set_family("sans");
   font_description.set_absolute_size(k_label_font_size * pango::SCALE as f64);

   text_layout.set_font_description(Some(&font_description));
   text_layout.set_text("Hello world! pygq");
   text_layout.context().set_language(Some(&Language::from_string("en-US")));

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

   *state_data = Some(Box::new(SvgTextNode { sample_text_layout: text_layout }));
}

fn test_text_inking(
   renderer_data_in: &mut ZRendererData,
   state_data: &mut ZNodeStateData,
   _in_data: &ZData,
   _out_data: &mut ZData,
) {
   let renderer_data: &mut SvgRendererData =
      renderer_data_in.as_mut().unwrap().downcast_mut::<SvgRendererData>().unwrap();

   let text_node_data: &mut SvgTextNode =
      state_data.as_mut().unwrap().downcast_mut::<SvgTextNode>().unwrap();

   let context: &Context = &renderer_data.main_context;
   let text_layout: &Layout = &text_node_data.sample_text_layout;

   context.move_to(160.0 + 30.0, 120.0);
   context.arc(160.0, 120.0, 30.0, 0.0 * PI, 2.0 * PI);
   context.stroke().unwrap();

   context.set_source_rgb(0.0, 0.0, 1.0);

   context.move_to(120.0, 60.0);
   pangocairo::show_layout(context, text_layout);
}

pub fn register_builtin_library(registry: &mut ZRegistry) {
   registry.register_new(
      ZNodeRegistrationBuilder::default().name("Test text".to_string()).build().unwrap(),
   );
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
               &mut result.as_mut().unwrap().as_mut().downcast_mut::<Vec<u8>>().unwrap();
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
