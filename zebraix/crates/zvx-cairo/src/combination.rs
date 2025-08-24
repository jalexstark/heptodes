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
use std::io::Write;
use zvx_docagram::diagram::SpartanDiagram;

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
