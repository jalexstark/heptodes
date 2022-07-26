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

use std::env;
use std::fs;

use protobuf_codegen_pure::Customize;
use std::path::Path;

extern crate protobuf_codegen_pure;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let generated_proto_dir = format!("{}/proto_gencode", out_dir);

    if Path::new(&generated_proto_dir).exists() {
        fs::remove_dir_all(&generated_proto_dir).unwrap();
    }
    fs::create_dir(&generated_proto_dir).unwrap();

    protobuf_codegen_pure::Codegen::new()
        .customize(Customize { gen_mod_rs: Some(true), ..Default::default() })
        .out_dir(generated_proto_dir)
        .inputs(&["src/protos/zebraix_graph.proto"])
        .include("src/protos")
        // .run_from_script();
        .run()
        .expect("protoc");
}
