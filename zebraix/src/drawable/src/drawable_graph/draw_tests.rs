// Copyright 2022 Google LLC
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

extern crate goldenfile;
extern crate jaywalk_drawable;
extern crate json5;

use goldenfile::Mint;
use std::fs::File;
// use std::io::{BufRead, BufReader, Read, Write};
use std::io::{Read, Write};
// use std::path::Path;

use jaywalk_drawable::drawable_graph::DCircle;
use jaywalk_drawable::drawable_graph::DrawableGraph;
use jaywalk_drawable::drawable_graph::ZebraixGraph;

pub fn add(a: i32, b: i32) -> i32 {
   a + b
}

#[cfg(test)]
mod tests {
   // Note this useful idiom: importing names from outer (for mod tests) scope.
   use super::*;

   #[test]
   fn test_add() {
      assert_eq!(add(1, 2), 3);
   }
}

fn run_graph_json_test(input_filename: &str, output_filename: &str, args: &[&str]) {
   let mut string_args = Vec::new();

   string_args.reserve(args.len());
   for a in args.iter() {
      string_args.push(a.to_string());
   }

   let input_full_path = format!("src/drawable_graph/golden_inputs/{}", input_filename);

   let mut in_text = String::new();
   let mut inputfile = File::open(input_full_path).unwrap();
   inputfile.read_to_string(&mut in_text).unwrap();

   let deserialized = json5::from_str::<DrawableGraph>(&in_text).unwrap();

   // assert!(deserialized.graph.nodes.len() == 3, "Incorrect number of nodes.");

   let mut mint = Mint::new("src/drawable_graph/golden_outputs");
   let out_file = mint.new_goldenfile(output_filename).unwrap();

   filtered_graph_write_sample_to_file(out_file, &deserialized);
}

fn run_circles_json_test(input_filename: &str, output_filename: &str, args: &[&str]) {
   let mut string_args = Vec::new();

   string_args.reserve(args.len());
   for a in args.iter() {
      string_args.push(a.to_string());
   }

   let input_full_path = format!("src/drawable_graph/golden_inputs/{}", input_filename);

   let mut in_text = String::new();
   let mut inputfile = File::open(input_full_path).unwrap();
   inputfile.read_to_string(&mut in_text).unwrap();

   let deserialized = json5::from_str::<Vec<DCircle>>(&in_text).unwrap();

   // assert!(deserialized.graph.nodes.len() == 3, "Incorrect number of nodes.");

   let mut mint = Mint::new("src/drawable_graph/golden_outputs");
   let out_file = mint.new_goldenfile(output_filename).unwrap();

   filtered_circle_write_sample_to_file(out_file, &deserialized);
}

fn run_old_json_test(input_filename: &str, output_filename: &str, args: &[&str]) {
   let mut string_args = Vec::new();

   string_args.reserve(args.len());
   for a in args.iter() {
      string_args.push(a.to_string());
   }

   let input_full_path = format!("src/drawable_graph/golden_inputs/{}", input_filename);

   let mut in_text = String::new();
   let mut inputfile = File::open(input_full_path).unwrap();
   inputfile.read_to_string(&mut in_text).unwrap();

   let deserialized = json5::from_str::<ZebraixGraph>(&in_text).unwrap();

   assert!(deserialized.graph.nodes.len() == 3, "Incorrect number of nodes.");
   // let inbound_serialized = zebraix_serialized::read_file(File::open(input_full_path).unwrap()).unwrap();

   // assert!(
   //     inbound_serialized.get_nodes()[0].get_label_text() == "Four legs",
   //     "First entry not four legs"
   // );

   let mut mint = Mint::new("src/drawable_graph/golden_outputs");
   let out_file = mint.new_goldenfile(output_filename).unwrap();

   filtered_zebraix_write_sample_to_file(out_file, &deserialized);
}

// Replace surface ID with generic ID, since this is changeable in tests.
fn filtered_zebraix_write_sample_to_file<W: Write>(mut out_stream: W, graph: &ZebraixGraph) {
   // let intervening_writer = Vec::<u8>::new();
   // let line_reader = std::io::BufReader::new(&**intervening_writer);

   let out_data = json5::to_string(&graph).unwrap();
   writeln!(out_stream, "{}", out_data).unwrap();
}

// Replace surface ID with generic ID, since this is changeable in tests.
fn filtered_graph_write_sample_to_file<W: Write>(mut out_stream: W, graph: &DrawableGraph) {
   // let intervening_writer = Vec::<u8>::new();
   // let line_reader = std::io::BufReader::new(&**intervening_writer);

   let out_data = json5::to_string(&graph).unwrap();
   writeln!(out_stream, "{}", out_data).unwrap();
}

// Replace surface ID with generic ID, since this is changeable in tests.
fn filtered_circle_write_sample_to_file<W: Write>(mut out_stream: W, graph: &[DCircle]) {
   // let intervening_writer = Vec::<u8>::new();
   // let line_reader = std::io::BufReader::new(&**intervening_writer);

   let out_data = json5::to_string(&graph).unwrap();
   writeln!(out_stream, "{}", out_data).unwrap();
}

#[test]
fn test_old_json_sphinx() {
   run_old_json_test("sphinx.json", "sphinx_ranks.json", &["--label_with_ranks"]);
}

#[test]
fn test_graph_json_sphinx() {
   run_graph_json_test("graphs_00.json", "graphs_00_converted.json", &["--label_with_ranks"]);
}

#[test]
fn test_circles_json_sphinx() {
   run_circles_json_test("circles_00.json", "circles_00_converted.json", &["--label_with_ranks"]);
}
