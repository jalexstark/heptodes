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

use crate::check_panic_with_path;
use goldenfile::Mint;
use serde_json::to_string_pretty;
use serde_json::Value;
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
// differ, would result in churn in golden files.
pub struct SvgGoldenTest {
   _mint_dir: String,
   _output_filename: String,
   _mint: Mint,
   pub out_stream: fs::File,
}

// SvgGoldenWriteable may change, but will have trait Write.
pub type SvgGoldenWriteable = Vec<u8>;
// pub type SvgGoldenResult = Result<Box<dyn core::any::Any>, cairo::StreamWithError>;
pub type SvgGoldenBoxedContent = Box<dyn core::any::Any>;

impl SvgGoldenTest {
   #[allow(clippy::missing_panics_doc)]
   #[must_use]
   pub fn new(mint_dir: &str, golden_filestem: &str) -> Self {
      let mut mint = Mint::new(mint_dir);
      let full_golden_filename = format!("{golden_filestem}.svg");
      let out_stream = mint.new_goldenfile(full_golden_filename.clone()).unwrap();

      Self {
         _mint_dir: mint_dir.to_string(),
         _output_filename: full_golden_filename,
         _mint: mint,
         out_stream,
      }
   }

   // input_filestem can also create
   pub fn read_to_string(&mut self, input_filestem: &str) -> String {
      let full_input_path_string = input_filestem.to_string() + ".svg";
      let full_input_path = Path::new(&full_input_path_string);

      check_panic_with_path(
         std::fs::read_to_string(full_input_path),
         "opening input-data file",
         full_input_path,
      )
   }

   #[allow(clippy::missing_panics_doc)]
   pub fn writeln_as_bytes(&mut self, result: &str) {
      // let bytes_amount = self.out_stream.write(result.as_bytes()).unwrap();
      // assert!(bytes_amount == result.as_bytes().len());
      Self::filter_result(Box::new(result.as_bytes()), &self.out_stream);
      let bytes_amount_nl = self.out_stream.write(b"\n").unwrap();
      assert!(bytes_amount_nl == 1);
   }

   #[must_use]
   pub const fn get_raw_writeable(&self) -> SvgGoldenWriteable {
      Vec::<u8>::new()
   }

   #[allow(clippy::missing_panics_doc)]
   pub fn handover_result(&self, golden_writeable: SvgGoldenBoxedContent) {
      let unboxed_result = &**golden_writeable.downcast::<Vec<u8>>().unwrap();
      Self::filter_result(unboxed_result, &self.out_stream);
   }

   // Replace surface ID with generic ID, since this is changeable in tests.
   #[allow(clippy::missing_panics_doc)]
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
   _output_filename: String,
   _mint: Mint,
   pub out_stream: fs::File,
}

#[allow(clippy::missing_panics_doc)]
impl JsonGoldenTest {
   pub fn new(mint_dir: &str, golden_filestem: &str) -> Self {
      let mut mint = Mint::new(mint_dir);
      let full_golden_filename = format!("{golden_filestem}.json");
      let out_stream = mint
         .new_goldenfile_with_differ(full_golden_filename.clone(), Box::new(Self::custom_diff))
         .unwrap();

      Self {
         mint_dir: mint_dir.to_string(),
         _output_filename: full_golden_filename,
         _mint: mint,
         out_stream,
      }
   }

   // input_filestem can also create
   pub fn read_to_string(&mut self, input_filestem: &str) -> String {
      let full_input_path_string = input_filestem.to_string() + ".json";
      let full_input_path = Path::new(&full_input_path_string);

      check_panic_with_path(
         std::fs::read_to_string(full_input_path),
         "opening input-data file",
         full_input_path,
      )
   }

   pub fn writeln_as_bytes(&mut self, result: &str) {
      let bytes_amount = self.out_stream.write(result.as_bytes()).unwrap();
      assert!(bytes_amount == result.len());
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

      let data_old_result = fs::read(old);
      let data_old = match data_old_result {
         Ok(string_result) => string_result,
         Err(error) => panic!("Problem opening golden-data file {old:?}: {error:?}"),
      };
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
