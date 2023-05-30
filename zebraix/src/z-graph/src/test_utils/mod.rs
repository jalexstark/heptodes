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

// At time of coding, Rust has a bug that cannot cope with test-only emptiness.
// #![cfg(test)]

extern crate goldenfile;

use goldenfile::Mint;

use std::fs::File;
use std::io::{BufRead, Read, Write}; // , BufReader
                                     // use std::path::Path;

pub struct SvgGoldenTest {
   pub _mint_dir: String,
   pub _output_filename: String,
   pub _mint: Mint,
   pub out_stream: File,
}

pub type SvgGoldenResult = Result<Box<dyn core::any::Any>, cairo::StreamWithError>;
// SvgGoldenWriteable may change, but has trait Write.
pub type SvgGoldenWriteable = Vec<u8>;

impl SvgGoldenTest {
   pub fn new(mint_dir: &str, output_filename: &str) -> SvgGoldenTest {
      let mut mint = Mint::new(mint_dir);
      let out_stream = mint.new_goldenfile(output_filename).unwrap();

      SvgGoldenTest {
         _mint_dir: mint_dir.to_string(),
         _output_filename: output_filename.to_string(),
         _mint: mint,
         out_stream,
      }
   }

   pub fn get_raw_writeable(&self) -> SvgGoldenWriteable {
      Vec::<u8>::new()
   }

   pub fn handover_result(&self, golden_writeable: SvgGoldenResult) {
      let unboxed_result = &**golden_writeable.unwrap().downcast::<Vec<u8>>().unwrap();
      Self::filter_result(unboxed_result, &self.out_stream);
   }

   // Replace surface ID with generic ID, since this is changeable in tests.
   pub fn filter_result<R: Read, W: Write>(boxed_sample_svg: R, mut out_stream: W) {
      let line_reader = std::io::BufReader::new(boxed_sample_svg);

      for l in line_reader.lines() {
         let line = l.unwrap();
         if line.starts_with(r#"<g id="surface"#) {
            writeln!(out_stream, r#"<g id="surfaceXXXX">"#).unwrap();
         } else {
            out_stream.write_all(line.as_bytes()).unwrap();
            out_stream.write_all(b"\n").unwrap();
         }
      }
   }
}
