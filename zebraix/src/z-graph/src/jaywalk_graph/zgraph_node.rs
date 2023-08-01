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

use crate::jaywalk_graph::zgraph_base::PortDataVec;
use crate::jaywalk_graph::zgraph_base::ZNodeStateData;
use crate::jaywalk_graph::zgraph_base::ZNodeTypeFinder;
use crate::jaywalk_graph::zgraph_graphdef::PresetPiece;
use crate::jaywalk_graph::zgraph_graphdef::ZLinkPort;
use crate::jaywalk_graph::zgraph_registry::ZNodeRegistration;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Floatable version of edge copier
pub struct PortDataCopier {
   pub src_node: Rc<RefCell<ZNode>>,
   pub src_port_data: PortDataVec,
   pub src_index: usize,
   pub dest_port_data: PortDataVec,
   pub dest_index: usize,
   pub port_def: Option<ZLinkPort>,
}

impl PortDataCopier {
   pub const VOID_SENTINEL: usize = usize::MAX;
   pub const FLOATING_SENTINEL: usize = usize::MAX - 1;
}

#[allow(dead_code)]
pub struct ZNode {
   pub name: String,
   pub node_type: Rc<ZNodeRegistration>,
   pub node_type_finder: Option<ZNodeTypeFinder>,
   pub node_state_data: ZNodeStateData,

   pub data_copiers_src_copy: Vec<Rc<RefCell<PortDataCopier>>>,
   pub data_copiers_dest_copy: Vec<Rc<RefCell<PortDataCopier>>>,
   pub data_ports_src_copy: PortDataVec,
   pub data_ports_dest_copy: PortDataVec,

   pub subgraph_nodes: Vec<Rc<RefCell<ZNode>>>, // Prolly init with 0 reserve.
   pub subgraph_node_map: Option<HashMap<String, usize>>,

   pub is_active: bool,

   pub preset_data: Vec<PresetPiece>,
}
