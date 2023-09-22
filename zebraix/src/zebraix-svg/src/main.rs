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

extern crate getopts;
extern crate render_svg;

use getopts::Options;

use std::env;

// use zebraix_serde::gen_serde::generated_with_native::zebraix_graph;

fn print_usage(program: &str, opts: Options) {
   let brief = format!("Usage: {} FILE [options]", program);
   print!("{}", opts.usage(&brief));
}

fn main() {
   let args: Vec<String> = env::args().collect();
   let program = args[0].clone();

   let mut opts = Options::new();
   opts.optopt("o", "", "set output file name", "NAME");
   opts.optflag("h", "help", "print this help menu");
   let matches = match opts.parse(&args[1..]) {
      Ok(m) => m,
      Err(f) => {
         panic!("{}", f.to_string())
      }
   };
   if matches.opt_present("h") {
      print_usage(&program, opts);
      return;
   }
   let _output = matches.opt_str("o");
   let _input = if !matches.free.is_empty() {
      matches.free[0].clone()
   } else {
      print_usage(&program, opts);
      return;
   };

   // let mut n = zebraix_graph::Node::new();
   // n.set_prime_rank(25);

   // assert_eq!(n.get_prime_rank(), 25);

   // let file = OpenOptions::new()
   //    .write(true)
   //    .create(true)
   //    .truncate(true)
   //    .open("pango_cairo_experiment.svg")
   //    .unwrap();
   // write_sample_to_file(file).unwrap();

   println!("Hello, world!");
}
