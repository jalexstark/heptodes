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

pub mod gen_protos;
// pub mod zebraix_proto;

// // use protobuf::Message;
// use gen_protos::generated_with_native::zebraix_graph;
// use protobuf::Message;
// use std::io::Read;

// pub fn read_file<R: Read + 'static>(
//     mut in_stream: R,
// ) -> Result<zebraix_graph::ZebraixGraph, protobuf::error::ProtobufError> {
//     let r = protobuf::parse_from_reader::<zebraix_graph::ZebraixGraph>(&mut in_stream);

//     // parse_from_bytes::<graph::ZebraixGraph>(&out_bytes).unwrap();

//     return r;
// }

// pub fn read_file<R: Read + 'static>(
//     &mut in_stream: &mut R,
// ) -> Result<zebraix_graph::ZebraixGraph, protobuf::error::ProtobufError> {
//     let mut coded_stream = protobuf::CodedInputStream::new(&mut in_stream);

//     let mut g = zebraix_graph::ZebraixGraph::new();
//     let r = g.merge_from(&mut coded_stream);

//     return r;
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
