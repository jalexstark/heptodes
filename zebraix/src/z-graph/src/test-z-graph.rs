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
use serde_json::to_string_pretty;

use z_graph::jaywalk_graph::zgraph_base::RenderSvg;
use z_graph::jaywalk_graph::zgraph_base::Renderer;
use z_graph::jaywalk_graph::zgraph_base::ZGraphDef;
use z_graph::jaywalk_graph::zgraph_base::ZMachine;
use z_graph::jaywalk_graph::ZebraixGraph;
use z_graph::render_svg::write_sample_to_write;
use z_graph::test_utils::JsonGoldenTest;
use z_graph::test_utils::SvgGoldenTest;

fn run_json_test(mint_dir: &str, input_filename: &str, output_filename: &str) {
   let input_full_path = format!("test-files/golden-inputs/{}", input_filename);
   let svg_golden = SvgGoldenTest::new(mint_dir, output_filename);

   let in_text = std::fs::read_to_string(input_full_path).unwrap();

   let deserialized = from_str::<ZGraphDef>(&in_text).unwrap();

   let mut z_graph = ZMachine::new();
   let svg_renderer = RenderSvg::default();

   z_graph.provide_graph_def(&deserialized).unwrap();

   z_graph.transition_to_deffed().unwrap();
   svg_renderer.setup_render_to_stream(&mut z_graph, svg_golden.get_raw_writeable()).unwrap();
   z_graph.transition_to_constructed().unwrap();
   z_graph.transition_to_calculated().unwrap();
   z_graph.transition_to_inked().unwrap();
   let raw_result = svg_renderer.finish_renderer(&mut z_graph).unwrap();
   z_graph.transition_to_finished().unwrap();

   svg_golden.handover_result(raw_result);
}

#[test]
fn test_json_sphinx() {
   run_json_test("test-files/golden-svgs", "simple_graph.json", "simple_graph.svg");
}

// Retire once ZGraph subsumes ZebraixGraph.
fn run_json_test_old(input_filename: &str, output_filename: &str, args: &[&str]) {
   let mut string_args = Vec::new();

   string_args.reserve(args.len());
   for a in args.iter() {
      string_args.push(a.to_string());
   }

   let input_full_path = format!("test-files/golden-inputs/{}", input_filename);

   let in_text = std::fs::read_to_string(input_full_path).unwrap();

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

// Retire once ZGraph subsumes ZebraixGraph.
#[test]
fn test_json_sphinx_old() {
   run_json_test_old("sphinx.json", "sphinx_ranks.svg", &["--label_with_ranks"]);
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

fn run_idem_test(mint_dir: &str, input_filename: &str, output_filename: &str) {
   let mut json_golden = JsonGoldenTest::new(mint_dir, input_filename, output_filename);

   let in_text = json_golden.read_to_string();

   let deserialized = from_str::<ZGraphDef>(&in_text).unwrap();

   let serialized = to_string_pretty::<ZGraphDef>(&deserialized).unwrap();

   json_golden.provide_result(&serialized);
}

#[test]
fn test_idem_simple() {
   run_idem_test("test-files/golden-inputs/", "simple_graph.json", "simple_graph.json");
}
