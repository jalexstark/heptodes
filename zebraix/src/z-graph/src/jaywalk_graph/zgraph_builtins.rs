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
use crate::jaywalk_graph::zgraph_base::ZNodeStateData;
use crate::jaywalk_graph::zgraph_base::ZNodeTypeFinder;
use crate::jaywalk_graph::zgraph_base::ZOptionBox;
use crate::jaywalk_graph::zgraph_base::ZPiece;
use crate::jaywalk_graph::zgraph_base::ZPieceType;
use crate::jaywalk_graph::zgraph_base::ZRendererData;
use crate::jaywalk_graph::zgraph_base::ZTextStyle;
use crate::jaywalk_graph::zgraph_registry::ZNodeCategory;
use crate::jaywalk_graph::zgraph_registry::ZNodeRegistration;
use crate::jaywalk_graph::zgraph_registry::ZNodeRegistrationBuilder;
use crate::jaywalk_graph::zgraph_registry::ZRegistry;
use std::rc::Rc;

pub fn text_style_agg_calculation(
   _renderer_data_in: &mut ZRendererData,
   _state_data: &mut ZNodeStateData,
   in_data: &[ZPiece],
   out_data: &mut [ZPiece],
) {
   let font_style: &ZFontStyle = in_data[0].get_font_style().unwrap();
   let color: &ZColor = in_data[1].get_color().unwrap();

   let text_style: &mut ZTextStyle = out_data[0].get_mut_text_style().unwrap();
   text_style.font_style = font_style.clone();
   text_style.color = color.clone();
}

pub fn text_style_disagg_calculation(
   _renderer_data_in: &mut ZRendererData,
   _state_data: &mut ZNodeStateData,
   in_data: &[ZPiece],
   out_data: &mut [ZPiece],
) {
   let text_style: &ZTextStyle = in_data[0].get_text_style().unwrap();

   let font_style: &mut ZFontStyle = out_data[0].get_mut_font_style().unwrap();
   *font_style = text_style.font_style.clone();
   let color: &mut ZColor = out_data[1].get_mut_color().unwrap();
   *color = text_style.color.clone();
}

pub fn font_style_disagg_calculation(
   _renderer_data_in: &mut ZRendererData,
   _state_data: &mut ZNodeStateData,
   in_data: &[ZPiece],
   out_data: &mut [ZPiece],
) {
   let font_style: &ZFontStyle = in_data[0].get_font_style().unwrap();

   let size: &mut f64 = out_data[0].get_mut_real().unwrap();
   *size = font_style.size;
   let family: &mut String = out_data[1].get_mut_text().unwrap();
   *family = font_style.family.clone();
   let language: &mut ZOptionBox = out_data[2].get_mut_option_box().unwrap();
   *language = font_style.language.clone();
}

pub fn coord2d_disagg_calculation(
   _renderer_data_in: &mut ZRendererData,
   _state_data: &mut ZNodeStateData,
   in_data: &[ZPiece],
   out_data: &mut [ZPiece],
) {
   let coord2d: &CoordReal2D = in_data[0].get_coord2d().unwrap();

   let coord_0: &mut f64 = out_data[0].get_mut_real().unwrap();
   *coord_0 = coord2d.0;
   let coord_1: &mut f64 = out_data[1].get_mut_real().unwrap();
   *coord_1 = coord2d.1;
}

pub fn register_builtin_library(registry: &mut ZRegistry) {
   registry
      .register_new(ZNodeRegistrationBuilder::default().name("Group".to_string()).build().unwrap());
   registry
      .register_new(ZNodeRegistrationBuilder::default().name("Input".to_string()).build().unwrap());
   registry.register_new(
      ZNodeRegistrationBuilder::default().name("Output".to_string()).build().unwrap(),
   );
   registry.register_new(
      ZNodeRegistrationBuilder::default()
         .name("Preset data".to_string())
         .category(ZNodeCategory::PresetData)
         .build()
         .unwrap(),
   );
   registry.register_new(
      ZNodeRegistrationBuilder::default()
         .name("text_style_agg".to_string())
         .calculation_fn(text_style_agg_calculation)
         .ports_dest_copy(vec![
            PortPieceTyped("font style".to_string(), ZPieceType::FontStyle),
            PortPieceTyped("color".to_string(), ZPieceType::Color),
         ])
         .ports_src_copy(vec![PortPieceTyped("text style".to_string(), ZPieceType::TextStyle)])
         .category(ZNodeCategory::Aggregator)
         .build()
         .unwrap(),
   );
   registry.register_new(
      ZNodeRegistrationBuilder::default()
         .name("text_style_disagg".to_string())
         .calculation_fn(text_style_disagg_calculation)
         .ports_dest_copy(vec![PortPieceTyped("text style".to_string(), ZPieceType::TextStyle)])
         .ports_src_copy(vec![
            PortPieceTyped("font style".to_string(), ZPieceType::FontStyle),
            PortPieceTyped("color".to_string(), ZPieceType::Color),
         ])
         .category(ZNodeCategory::Disaggregator)
         .build()
         .unwrap(),
   );
   registry.register_new(
      ZNodeRegistrationBuilder::default()
         .name("font_style_disagg".to_string())
         .calculation_fn(font_style_disagg_calculation)
         .ports_dest_copy(vec![PortPieceTyped("font style".to_string(), ZPieceType::FontStyle)])
         .ports_src_copy(vec![
            PortPieceTyped("size".to_string(), ZPieceType::Real),
            PortPieceTyped("family".to_string(), ZPieceType::Text),
            PortPieceTyped("language".to_string(), ZPieceType::OptionBox),
         ])
         .category(ZNodeCategory::Disaggregator)
         .build()
         .unwrap(),
   );
   registry.register_new(
      ZNodeRegistrationBuilder::default()
         .name("coord2d_disagg".to_string())
         .calculation_fn(coord2d_disagg_calculation)
         .ports_dest_copy(vec![PortPieceTyped("coord".to_string(), ZPieceType::Coord2D)])
         .ports_src_copy(vec![
            PortPieceTyped("0".to_string(), ZPieceType::Real),
            PortPieceTyped("1".to_string(), ZPieceType::Real),
         ])
         .category(ZNodeCategory::Disaggregator)
         .build()
         .unwrap(),
   );
}

impl ZPieceType {
   pub fn get_disaggregator_registration_for_piece_type(
      registry: &ZRegistry,
      piece_type: &ZPieceType,
   ) -> Option<Rc<ZNodeRegistration>> {
      let disagg_name: &str = match piece_type {
         ZPieceType::Void => "Null",
         ZPieceType::Integer => "Null",
         ZPieceType::Real => "Null",
         ZPieceType::Unit => "Null",
         ZPieceType::Text => "Null",
         //
         ZPieceType::Color => "Null",
         ZPieceType::FontStyle => "font_style_disagg",
         ZPieceType::TextStyle => "text_style_disagg",
         //
         ZPieceType::Coord2D => "coord2d_disagg",
         //
         ZPieceType::OptionBox => "Null",
      };

      if disagg_name == "Null" {
         eprintln!("Unable to handle disaggregation of {:?} at present", piece_type);
         return None;
      }

      let finder = ZNodeTypeFinder::ByString(disagg_name.to_string());

      let gotten = registry.find(&finder);
      assert!(
         gotten.is_ok(),
         "Could not find element type in registry, element name \"{}\"",
         finder.get_descriptive_name()
      );
      Some(gotten.unwrap().clone())
   }
}
