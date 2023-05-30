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

// This file was an initial test of the ability to test and of basic
// build and of the existence of capabilities (text to SVG).
//
// Tests should be removed as their capabilities are replicated in
// more meaningful tests.

#![cfg(test)]

extern crate goldenfile;

use json5::from_str;

use std::fs::File;
use std::io::{Read}; // , BufReader
                                     // use std::path::Path;
use z_graph::jaywalk_graph::ZebraixGraph;
use z_graph::render_svg::write_sample_to_write;


use z_graph::test_utils::SvgGoldenTest;

fn run_json_test(input_filename: &str, output_filename: &str, args: &[&str]) {
   let mut string_args = Vec::new();

   string_args.reserve(args.len());
   for a in args.iter() {
      string_args.push(a.to_string());
   }

   let input_full_path = format!("test-files/golden-inputs/{}", input_filename);

   let mut in_text = String::new();
   let mut inputfile = File::open(input_full_path).unwrap();
   inputfile.read_to_string(&mut in_text).unwrap();

   let deserialized = from_str::<ZebraixGraph>(&in_text).unwrap();

   assert!(deserialized.graph.nodes.len() == 3, "Incorrect number of nodes.");
   // let inbound_serialized = zebraix_serialized::read_file(File::open(input_full_path).unwrap()).unwrap();

   // assert!(
   //     inbound_serialized.get_nodes()[0].get_label_text() == "Four legs",
   //     "First entry not four legs"
   // );

   let svg_golden = SvgGoldenTest::new("test-files/golden-svgs", output_filename);
   let raw_result = write_sample_to_write(svg_golden.get_raw_writeable());
   svg_golden.handover_result(raw_result);
}

#[test]
fn test_json_sphinx() {
   run_json_test("sphinx.json", "sphinx_ranks.svg", &["--label_with_ranks"]);
}

// Function to
//   * Open input file.
//   * Parse options vector.
//   * Convert from serialized to internal.
//   * Open golden file. (outer function)
//   * Create layout.
//   * Write to SVG. (outer function)

fn run_one_test(input_filename: &str, output_filename: &str, args: &[&str]) {
   let mut string_args = Vec::new();

   string_args.reserve(args.len());
   for a in args.iter() {
      string_args.push(a.to_string());
   }

   let _input_full_path = format!("test-files/golden-inputs/{}", input_filename);
   // let inbound_serialized = zebraix_serialized::read_file(File::open(input_full_path).unwrap()).unwrap();

   // assert!(
   //     inbound_serialized.get_nodes()[0].get_label_text() == "Four legs",
   //     "First entry not four legs"
   // );

   let svg_golden = SvgGoldenTest::new("test-files/golden-svgs", output_filename);
   let raw_result = write_sample_to_write(svg_golden.get_raw_writeable());
   svg_golden.handover_result(raw_result);
}

#[test]
fn test_sphinx() {
   run_one_test("sphinx.pb.txt", "sphinx_ranks.svg", &["--label_with_ranks"]);
}