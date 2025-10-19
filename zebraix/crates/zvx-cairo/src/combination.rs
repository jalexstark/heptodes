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

use crate::render::CairoSpartanRender;
use std::error::Error;
use std::io::Write;
use zvx_docagram::diagram::DrawableDiagram;
use zvx_docagram::diagram::SpartanPreparation;
use zvx_drawable::interface::ZvxRenderEngine;

// This may seem odd, but is Rust-inspired. The diagram and the renderer can be separately
// borrowed with different mutability.
// #[derive(Debug)]
pub struct CairoSpartanCombo {
   pub drawable_diagram: DrawableDiagram,
   pub render_engine: Box<dyn ZvxRenderEngine>,
}

impl CairoSpartanCombo {
   #[allow(clippy::missing_errors_doc)]
   #[allow(clippy::missing_panics_doc)]
   pub fn render_diagram(&mut self) -> Result<Box<dyn core::any::Any>, Box<dyn Error>> {
      self.render_engine.render_drawables(&self.drawable_diagram.drawables)
   }

   #[allow(clippy::missing_panics_doc)]
   pub fn create_for_stream<W: Write + 'static>(
      out_stream: W,
      preparation: &SpartanPreparation,
   ) -> Self {
      Self {
         drawable_diagram: DrawableDiagram { prep: preparation.clone(), drawables: vec![] },
         render_engine: CairoSpartanRender::create_for_stream(
            out_stream,
            &preparation.canvas_layout,
            &preparation.diagram_choices,
         ),
      }
   }
}
