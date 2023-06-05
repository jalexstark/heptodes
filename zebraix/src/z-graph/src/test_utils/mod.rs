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
use std::io::{BufRead, Read, Write};

// Helper that handles a golden (Mint) test for SVG output.
//
// Some parts of SVG are "random", such as the surface ID.  The
// strategy is to filter the output.  The alternative, modifying the
// diff, would result in churn in golden files.
pub struct SvgGoldenTest {
   pub _mint_dir: String,
   pub _output_filename: String,
   pub _mint: Mint,
   pub out_stream: File,
}

// SvgGoldenWriteable may change, but will have trait Write.
pub type SvgGoldenWriteable = Vec<u8>;
pub type SvgGoldenResult = Result<Box<dyn core::any::Any>, cairo::StreamWithError>;

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

// Helper that handles a golden (Mint) test for Json output.
//
// This is mainly a convenience class, and it can be used to check
// idempotency (perhaps with additional default values in output) of
// deserialization and serialization. It assumes that output is
// pretty-printed, and adds a newline.
pub struct JsonGoldenTest {
   pub mint_dir: String,
   pub input_filename: String,
   pub _output_filename: String,
   pub _mint: Mint,
   pub out_stream: File,
}

impl JsonGoldenTest {
   pub fn new(mint_dir: &str, input_filename: &str, output_filename: &str) -> JsonGoldenTest {
      let mut mint = Mint::new(mint_dir);
      let out_stream = mint.new_goldenfile(output_filename).unwrap();

      JsonGoldenTest {
         mint_dir: mint_dir.to_string(),
         input_filename: input_filename.to_string(),
         _output_filename: output_filename.to_string(),
         _mint: mint,
         out_stream,
      }
   }

   pub fn read_to_string(&mut self) -> String {
      let input_full_path = self.mint_dir.clone() + &self.input_filename;

      std::fs::read_to_string(input_full_path).unwrap()
   }

   pub fn provide_result(&mut self, result: &str) {
      let bytes_amount = self.out_stream.write(result.as_bytes()).unwrap();
      assert!(bytes_amount == result.as_bytes().len());
      let bytes_amount_nl = self.out_stream.write(b"\n").unwrap();
      assert!(bytes_amount_nl == 1);
   }
}
