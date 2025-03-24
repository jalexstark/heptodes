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

use zvx_golden::filtered::SvgGoldenTest;

fn run_svg_file_test(mint_dir: &str, input_filestem: &str, output_filestem: &str) {
   let mut svg_golden = SvgGoldenTest::new(mint_dir, output_filestem);
   let input_data = svg_golden.read_to_string(input_filestem);
   svg_golden.writeln_as_bytes(&input_data);
}

// Change to relative path for test-data.

#[test]
fn test_svg_file() {
   run_svg_file_test("tests/goldenfiles/", "tests/test-data/simple_svg_sample", "svg_simple_file");
}

#[test]
fn test_svg_simple() {
   let mut svg_golden = SvgGoldenTest::new("tests/goldenfiles/", "svg_simple_string");

   // Note that SVG files must not start with blank line.
   let in_text = r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
 width="320" height="240" viewBox="0 0 320 240">
<path fill="none" stroke-width="2" stroke-linecap="butt" stroke-linejoin="miter"
 stroke="rgb(0%, 0%, 0%)" stroke-opacity="1" stroke-miterlimit="10"
 d="M 190 120 C 190 136.570312 176.570312 150 160 150
    C 143.429688 150 130 136.570312 130 120 C 130 103.429688 143.429688 90 160 90
    C 176.570312 90 190 103.429688 190 120 "
 transform="matrix(1, 0, 0, 1, -15, -25)"/>
</svg>"#;

   svg_golden.writeln_as_bytes(in_text);
}
