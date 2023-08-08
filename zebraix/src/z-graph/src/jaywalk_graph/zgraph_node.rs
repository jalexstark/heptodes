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
use crate::jaywalk_graph::zgraph_base::ZPiece;
use crate::jaywalk_graph::zgraph_graphdef::PresetPiece;
use crate::jaywalk_graph::zgraph_graphdef::ZLinkPort;
use crate::jaywalk_graph::zgraph_registry::ZNodeRegistration;
use crate::jaywalk_graph::ZRegistry;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Eq)]
pub enum VoidFilter {
   All,
   VoidOnly,
   NonVoid,
}

// Floatable version of edge copier
pub struct PortDataCopier {
   pub src_node: Rc<RefCell<ZNode>>,
   pub src_port_data: PortDataVec,
   pub src_index: usize,
   pub dest_port_data: PortDataVec,
   pub dest_index: usize,
   pub port_def: Option<ZLinkPort>,
}

// The `src_copiers_vec` is often a temporary parking spot.
impl PortDataCopier {
   pub const VOID_SENTINEL: usize = usize::MAX;
   pub const FLOATING_SENTINEL: usize = usize::MAX - 1;

   pub fn create_copiers_for_link_ports(
      filter: VoidFilter,
      output_ports: &Vec<ZLinkPort>,
      null_node: &Rc<RefCell<ZNode>>,
      floating_port_data: &PortDataVec,
      dest_port_data: &PortDataVec,
      dest_copiers_vec: &mut Vec<Rc<RefCell<PortDataCopier>>>,
      src_copiers_vec: &mut Vec<Rc<RefCell<PortDataCopier>>>,
   ) {
      // In ordinary mode of operation, for a user graph, we create one copier for each output.
      for port_def in output_ports {
         if (filter == VoidFilter::VoidOnly && !port_def.is_void)
            || (filter == VoidFilter::NonVoid && port_def.is_void)
         {
            continue;
         }
         let sentinel_index = if port_def.is_void {
            PortDataCopier::VOID_SENTINEL
         } else {
            PortDataCopier::FLOATING_SENTINEL
         };

         let expanded_graph_data_copier = Rc::new(RefCell::new(PortDataCopier {
            src_node: null_node.clone(),
            src_port_data: floating_port_data.clone(),
            src_index: PortDataCopier::FLOATING_SENTINEL,
            dest_port_data: dest_port_data.clone(),
            dest_index: sentinel_index,
            port_def: Some(port_def.clone()),
         }));
         dest_copiers_vec.push(expanded_graph_data_copier.clone());
         src_copiers_vec.push(expanded_graph_data_copier);
      }
   }
}

#[allow(dead_code)]
pub struct ZNode {
   pub name: String,
   pub node_type: Rc<ZNodeRegistration>,
   pub node_type_finder: Option<ZNodeTypeFinder>,
   pub node_state_data: ZNodeStateData,

   pub data_copiers_src_copy: Vec<Rc<RefCell<PortDataCopier>>>,
   pub data_copiers_dest_copy: Vec<Rc<RefCell<PortDataCopier>>>,
   pub data_copiers_src_void_port: Vec<Rc<RefCell<PortDataCopier>>>,
   pub data_ports_src_copy: PortDataVec,
   pub data_ports_dest_copy: PortDataVec,

   pub subgraph_nodes: Vec<Rc<RefCell<ZNode>>>, // Prolly init with 0 reserve.
   pub subgraph_node_map: Option<HashMap<String, usize>>,
   pub disaggregation_nodes: Vec<Option<Rc<RefCell<ZNode>>>>,

   pub is_active: bool,

   pub preset_data: Vec<PresetPiece>,
}

impl ZNode {
   // This assumes that the finder is for a valid type, such as subgraph.
   pub fn new_basic_node(
      name: &str,
      preset_data: &[PresetPiece],
      finder: &ZNodeTypeFinder, // Use a simple finder, not a string.
      registry: &ZRegistry,
   ) -> Rc<RefCell<ZNode>> {
      let node_reg: &Rc<ZNodeRegistration> = registry.find(finder).unwrap();

      Rc::new(RefCell::new(ZNode {
         name: name.to_string(),
         node_state_data: None,
         node_type: node_reg.clone(),
         node_type_finder: None,
         data_copiers_src_copy: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
         data_copiers_dest_copy: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
         data_copiers_src_void_port: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
         data_ports_src_copy: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
         data_ports_dest_copy: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
         subgraph_nodes: Vec::<Rc<RefCell<ZNode>>>::default(),
         subgraph_node_map: None,
         is_active: false,
         preset_data: preset_data.to_vec(),
         disaggregation_nodes: Vec::<Option<Rc<RefCell<ZNode>>>>::default(),
      }))
   }

   pub fn new_null_node(name: &str, registry: &ZRegistry) -> Rc<RefCell<ZNode>> {
      ZNode::new_basic_node(
         name,
         &Vec::<PresetPiece>::new(),
         &ZNodeTypeFinder::NullNodeType,
         registry,
      )
   }
}
