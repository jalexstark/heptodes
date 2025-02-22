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

// This file was an initial test of the ability to test and of basic
// build and of the existence of capabilities (text to SVG).
//
// Tests should be removed as their capabilities are replicated in
// more meaningful tests.

extern crate goldenfile;

use diag_golden::JsonGoldenTest;
use goldenfile::Mint;
use regex::Regex;
use std::env;
use std::io::Write;
use std::panic;

fn run_json_file_test(mint_dir: &str, input_filestem: &str, output_filestem: &str) {
   let mut json_golden = JsonGoldenTest::new(mint_dir, output_filestem);
   let input_data = json_golden.read_to_string(input_filestem);
   json_golden.writeln_as_bytes(&input_data);
}

// Change to relative path for test-data.

#[test]
fn test_json_file() {
   run_json_file_test(
      "tests/goldenfiles/",
      "tests/test-data/simple_json_sample",
      "json_simple_file",
   );
}

#[test]
fn test_json_simple() {
   let mut json_golden = JsonGoldenTest::new("tests/goldenfiles/", "json_simple_string");

   let in_text = r#"{
    "top-level": {
        "entry": "value",
	    "nested": {    "array": ["Tutti", "Frutty"]
                    }
            }
}"#;
   json_golden.writeln_as_bytes(in_text);
}

// Deliberate test of failure, and disable golden file update.
//
// This mostly works, but does print alarming terminal output if tested with `--nocapture`.
#[test]
fn test_json_failure() {
   {
      let mut json_golden = JsonGoldenTest::new("tests/goldenfiles/", "json_failure_test_data");

      let in_text = "{ \"k\" : 1 }";
      json_golden.writeln_as_bytes(in_text);
   }
   let update_var = env::var("UPDATE_GOLDENFILES");
   if !(update_var.is_ok() && update_var.unwrap() == "1") {
      let failure_result = panic::catch_unwind(|| {
         let mut json_golden = JsonGoldenTest::new("tests/goldenfiles/", "json_failure_test_data");

         let in_text = "{ \"k\" : 0 }";
         json_golden.writeln_as_bytes(in_text);
      });

      match failure_result {
         Ok(()) => {
            panic!("JSON test should have failed, but erroneously passed.");
         }
         Err(error) => {
            let mut mint = Mint::new("tests/goldenfiles");
            let mut error_mint = mint.new_goldenfile("json_failure_test_error.txt").unwrap();

            let diff_result = if let Some(s) = error.downcast_ref::<&str>() {
               (*s).to_string()
            } else if let Some(s) = error.downcast_ref::<String>() {
               s.clone()
            } else {
               "Unhandled panic result".to_string()
            };

            let diff_result_intermediate =
               Regex::new("^[^\n]*\n").unwrap().replace(&diff_result, "");

            let cleaned_diff_result =
               Regex::new("[^[:word:][:punct:][:space:]{}]\\[[[:word:]][[:word:]]?m")
                  .unwrap()
                  .replace_all(&diff_result_intermediate, "")
                  .to_string();
            writeln!(error_mint, "{cleaned_diff_result}").unwrap();
         }
      }
   } else {
      let mut mint = Mint::new("tests/goldenfiles");
      let mut error_mint = mint.new_goldenfile("json_failure_test_error.txt").unwrap();

      let error_text = "  left: `\"{\\n  \\\"k\\\": 1\\n}\"`\n right: `\"{\\n  \\\"k\\\": 0\\n}\"`\n\nDifferences (-left|+right):\n {\n-  \"k\": 1\n+  \"k\": 0\n }\n\n";
      writeln!(error_mint, "{error_text}").unwrap();
   }

   {
      let mut json_golden = JsonGoldenTest::new("tests/goldenfiles/", "json_failure_test_data");

      let in_text = "{ \"k\" : 1 }";
      json_golden.writeln_as_bytes(in_text);
   }
}
