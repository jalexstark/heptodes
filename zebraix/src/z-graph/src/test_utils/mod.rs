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

use goldenfile::Mint;
// use json5::from_str;
// use serde_json::from_slice;
use serde_json::to_string_pretty;
use serde_json::Value;
// use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use testdir::testdir;

// Helper that handles a golden (Mint) test for SVG output.
//
// Some parts of SVG are "random", such as the surface ID.  The
// strategy is to filter the output.  The alternative, modifying the
// diff, would result in churn in golden files.
pub struct SvgGoldenTest {
   pub _mint_dir: String,
   pub _output_filename: String,
   pub _mint: Mint,
   pub out_stream: fs::File,
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
   pub fn filter_result<R: io::Read, W: io::Write>(boxed_sample_svg: R, mut out_stream: W) {
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
//
// The input is passed through a formatter, discarding comments and
// with one "thing" per line.  This enables the user to format their
// input as they see fit, without clever diffing of whitespace.
pub struct JsonGoldenTest {
   pub mint_dir: String,
   pub input_filename: String,
   pub _output_filename: String,
   pub _mint: Mint,
   pub out_stream: fs::File,
}

impl JsonGoldenTest {
   pub fn new(mint_dir: &str, input_filename: &str, output_filename: &str) -> JsonGoldenTest {
      let mut mint = Mint::new(mint_dir);
      // let out_stream = mint.new_goldenfile(output_filename).unwrap();
      let out_stream =
         mint.new_goldenfile_with_differ(output_filename, Box::new(Self::custom_diff)).unwrap();

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

   fn custom_diff(old: &Path, new: &Path) {
      let dir: PathBuf = testdir!();
      let mut reformatted_old = dir.join(old.file_name().unwrap());
      reformatted_old.set_file_name(
         reformatted_old.file_stem().unwrap().to_str().unwrap().to_owned()
            + "-old."
            + reformatted_old.extension().unwrap().to_str().unwrap(),
      );
      let mut reformatted_new = dir.join(new.file_name().unwrap());
      reformatted_new.set_file_name(
         reformatted_new.file_stem().unwrap().to_str().unwrap().to_owned()
            + "-new."
            + reformatted_new.extension().unwrap().to_str().unwrap(),
      );

      let data_old = fs::read(old).unwrap();
      // Parse the old-path file into serde_json::Value.
      let value_old: Value = serde_json::from_slice(&data_old).unwrap();
      // Write old-path JSON into pretty-print.
      let serialized_old = to_string_pretty::<Value>(&value_old).unwrap();
      fs::write(&reformatted_old, serialized_old).unwrap();
      assert!(reformatted_old.exists());

      // We replace the test-generated pretty print with that passed
      // through "Value" so that the ordering is lexicographic.
      let data_new = fs::read(new).unwrap();
      // Parse the new-path file into serde_json::Value.
      let value_new: Value = serde_json::from_slice(&data_new).unwrap();
      // Write new-path JSON into pretty-print.
      let serialized_new = to_string_pretty::<Value>(&value_new).unwrap();
      fs::write(&reformatted_new, serialized_new).unwrap();
      assert!(reformatted_new.exists());

      goldenfile::differs::text_diff(&reformatted_old, &reformatted_new);
   }
}
