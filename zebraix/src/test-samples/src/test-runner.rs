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

extern crate goldenfile;
extern crate render_svg;

use json5::from_str;

use goldenfile::Mint;
use render_svg::write_sample_to_file;
use render_svg::write_spline_test_to_file;
use render_svg::SplineTest;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

use jaywalk_base::jaywalk_graph::ZebraixGraph;

fn run_json_test(input_filename: &str, output_filename: &str, args: &[&str]) {
   let mut string_args = Vec::new();

   string_args.reserve(args.len());
   for a in args.iter() {
      string_args.push(a.to_string());
   }

   let input_full_path = format!("tests/golden-inputs/{}", input_filename);

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

   let mut mint = Mint::new("tests/golden-svgs");
   let out_file = mint.new_goldenfile(output_filename).unwrap();

   filtered_write_sample_to_file(out_file);
}

#[test]
fn test_json_sphinx() {
   run_json_test("sphinx.json", "sphinx_ranks.svg", &["--label_with_ranks"]);
}

// Check if two SVG files, with given path names, have meaningful difference.
//
// Importantly, this disregards differences in surface ID.
fn custom_diff(old: &Path, new: &Path) {
   let mut old_file = BufReader::new(File::open(old).unwrap());
   let mut new_file = BufReader::new(File::open(new).unwrap());

   loop {
      let mut old_line = String::new();
      let mut new_line = String::new();
      let old_len = old_file.read_line(&mut old_line).unwrap();
      let new_len = new_file.read_line(&mut new_line).unwrap();
      if old_len == 0 {
         if new_len == 0 {
            // Both file reads ended at this point: no material difference.
            return;
         } else {
            break; // Short old file.
         };
      } else if new_len == 0 {
         break; // Short new file.
      }
      if old_line.starts_with(r#"<g id="surface"#) {
         if new_line.starts_with(r#"<g id="surface"#) {
            continue;
         } else {
            break; // Skippable old line mismatch with new.
         }
      } else if new_line.starts_with(r#"<g id="surface"#) {
         break; // Skippable new line mismatch with old.
      }

      if new_line != old_line {
         break;
      }
   }

   // Need to panic.
   goldenfile::differs::text_diff(old, new);
   panic!("Unreachable panic: custom differ finds diff but not text differ.");
}

#[test]
//
// This test recovers the stream after use by Cairo. At present this is not necessarily useful.
// The output of SVG Cairo cannot be inserted into, say, an html document without stripping the
// XML header. Nonetheless this might be useful if the output were written to a buffer.
fn test() {
   let mut mint = Mint::new("tests/golden-svgs");
   let file = mint.new_goldenfile_with_differ("recapture_demo.svg", Box::new(custom_diff)).unwrap();

   let mut file_relinquished =
      write_sample_to_file(file).unwrap().downcast::<std::fs::File>().unwrap();
   writeln!(file_relinquished, "These lines demonstrate recapture of file from Cairo.").unwrap();
}

// Replace surface ID with generic ID, since this is changeable in tests.
fn filtered_write_sample_to_file<W: Write>(mut out_stream: W) {
   let intervening_writer = Vec::<u8>::new();
   let relinquished_writer =
      write_sample_to_file(intervening_writer).unwrap().downcast::<Vec<u8>>().unwrap();

   let line_reader = std::io::BufReader::new(&**relinquished_writer);
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

   let _input_full_path = format!("tests/golden-inputs/{}", input_filename);
   // let inbound_serialized = zebraix_serialized::read_file(File::open(input_full_path).unwrap()).unwrap();

   // assert!(
   //     inbound_serialized.get_nodes()[0].get_label_text() == "Four legs",
   //     "First entry not four legs"
   // );

   let mut mint = Mint::new("tests/golden-svgs");
   let out_file = mint.new_goldenfile(output_filename).unwrap();

   filtered_write_sample_to_file(out_file);
}

#[test]
fn test_sphinx() {
   run_one_test("sphinx.pb.txt", "sphinx_ranks.svg", &["--label_with_ranks"]);
}

// SVG_TESTS = [
//     # input, output/name, extra_args.
//     [
//         "sample",
//         "sphinx",  # Unused, except as dependency.
//         [
//             "--generate_sample_graph",
//         ],
//     ],
//     [
//         "bridge_three",
//         "bridge_three",
//         [
//             "--vanish_waypoints",
//         ],
//     ],
//     [
//         "bridge_three_disorderly",
//         "bridge_three_disorderly",
//         [
//             "--vanish_waypoints",
//         ],
//     ],
//     [
//         "bridge_two_a",
//         "bridge_two_a",
//         [
//             "--vanish_waypoints",
//         ],
//     ],
//     [
//         "bridge_two_b",
//         "bridge_two_b",
//         [
//             "--vanish_waypoints",
//         ],
//     ],
//     [
//         "bridge_two_c",
//         "bridge_two_c",
//         [
//             "--vanish_waypoints",
//         ],
//     ],
//     [
//         "bridge_waypoint",
//         "bridge_waypoint",
//         [],
//     ],
//     [
//         "complicated",
//         "complicated",
//         [],
//     ],
//     [
//         "config_error",
//         "config_error",
//         [
//             "--dump_inbound_graph=6",
//         ],
//     ],
//     [
//         "config_error_ranks",
//         "config_error_unadjusted",
//         [
//             "--label_with_ranks",
//         ],
//     ],
//     [
//         "fitness",
//         "fitness",
//         [
//             "--dump_inbound_graph=5",
//         ],
//     ],
//     [
//         "fruit_embedding",
//         "fruit_embedding",
//         [],
//     ],
//     [
//         "fruit_hierarchy",
//         "fruit_hierarchy",
//         [],
//     ],
//     [
//         "grid_16",
//         "grid_16",
//         [],
//     ],
//     [
//         "grid_16_on_grid",
//         "grid_16_on_grid",
//         [],
//     ],
//     [
//         "hierarchy_ranks",
//         "fruit_hierarchy",
//         [
//             "--label_with_ranks",
//         ],
//     ],
//     [
//         "inference",
//         "inference",
//         [],
//     ],
//     [
//         "inference_sink_only",
//         "inference_sink_only",
//         [],
//     ],
//     [
//         "inference_source_only",
//         "inference_source_only",
//         [],
//     ],
//     [
//         "long_citrus",
//         "long_citrus",
//         [],
//     ],
//     [
//         "on_grid",
//         "on_grid",
//         [
//             "--label_with_ranks",
//         ],
//     ],
//     [
//         "rank_labels",
//         "complicated",
//         [
//             "--label_with_ranks",
//         ],
//     ],
//     [
//         "sample_ticks",
//         "config_error",
//         [
//             "--draw_label_ticks",
//         ],
//     ],
//     [
//         "sphinx",
//         "sphinx",
//         [],
//     ],
//     [
//         "sphinx_ranks",
//         "sphinx",
//         [
//             "--label_with_ranks",
//         ],
//     ],
//     [
//         "traffic_dag",
//         "traffic_dag",
//         [],
//     ],
//     [
//         "traffic_fsm",
//         "traffic_fsm",
//         [],
//     ],
//     [
//         "cross_complex_01",
//         "cross_complex_01",
//         [],
//     ],
//     [
//         "cross_simple_01",
//         "cross_simple_01",
//         [],
//     ],
//     [
//         "cross_simple_02",
//         "cross_simple_02",
//         [],
//     ],
// ]

// Replace surface ID with generic ID, since this is changeable in tests.
fn filtered_write_spline_test_to_file<W: Write>(mut out_stream: W, spline_def: &SplineTest) {
   let intervening_writer = Vec::<u8>::new();
   let relinquished_writer = write_spline_test_to_file(intervening_writer, spline_def)
      .unwrap()
      .downcast::<Vec<u8>>()
      .unwrap();

   let line_reader = std::io::BufReader::new(&**relinquished_writer);
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

fn run_one_spline_test(spline_def: &SplineTest, output_filename: &str) {
   let mut mint = Mint::new("tests/golden-svgs");
   let out_file = mint.new_goldenfile(output_filename).unwrap();

   filtered_write_spline_test_to_file(out_file, spline_def);
}

#[test]
fn test_spline_0() {
   let spline_def = SplineTest { name: "Spline test 0" };
   run_one_spline_test(&spline_def, "spline_test_0.svg");
}
