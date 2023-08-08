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

// use crate::jaywalk_graph::zgraph_base::ZNodeTypeFinder;
use crate::jaywalk_graph::zgraph_base::PortDataVec;
use crate::jaywalk_graph::zgraph_base::PortPieceTyped;
use crate::jaywalk_graph::zgraph_base::ZGraphError;
use crate::jaywalk_graph::zgraph_base::ZNodeTypeFinder;
use crate::jaywalk_graph::zgraph_base::ZPiece;
use crate::jaywalk_graph::zgraph_base::ZPieceType;
use crate::jaywalk_graph::zgraph_graphdef::ZGraphDef;
use crate::jaywalk_graph::zgraph_graphdef::ZLinkPort;
use crate::jaywalk_graph::zgraph_graphdef::ZNodeDef;
use crate::jaywalk_graph::zgraph_node::PortDataCopier;
use crate::jaywalk_graph::zgraph_node::VoidFilter;
use crate::jaywalk_graph::zgraph_node::ZNode;
use crate::jaywalk_graph::zgraph_registry::ZNodeCategory;
use crate::jaywalk_graph::zgraph_registry::ZNodeRegistration;
use crate::jaywalk_graph::zgraph_registry::ZRegistry;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::rc::Rc;

#[inline(always)]
fn is_leaf_node(subnode_type: &Rc<ZNodeRegistration>) -> bool {
   subnode_type.category != ZNodeCategory::Subgraph
}

impl ZNode {
   pub fn reregister_user_graph_input_porting(
      realized_node_wrapped: &Rc<RefCell<ZNode>>,
      inputs: &Vec<PortPieceTyped>,
   ) -> Result<(), ZGraphError> {
      // CCC Top-level only.
      let top_node_borrowed: &mut ZNode = &mut realized_node_wrapped.borrow_mut();

      // Create custom type "registration" with source port typing.
      //
      let subnode_type: &ZNodeRegistration = top_node_borrowed.node_type.as_ref();
      let mut replacement_registration: ZNodeRegistration = subnode_type.clone();
      assert!(replacement_registration.ports_src_copy.is_empty());
      for input in inputs {
         replacement_registration.ports_src_copy.push(input.clone());
      }
      top_node_borrowed.node_type = Rc::new(replacement_registration);
      Ok(())
   }

   pub fn seed_user_graph_outputs(
      filter: VoidFilter,
      realized_node_wrapped: &Rc<RefCell<ZNode>>,
      output_ports: &Vec<ZLinkPort>,
      null_node: &Rc<RefCell<ZNode>>,
      floating_port_data: &PortDataVec,
   ) -> Result<(), ZGraphError> {
      let realized_node: &mut ZNode = &mut realized_node_wrapped.borrow_mut();

      PortDataCopier::create_copiers_for_link_ports(
         filter,
         output_ports,
         null_node,
         floating_port_data,
         &realized_node.data_ports_dest_copy,
         &mut realized_node.data_copiers_dest_copy,
         &mut realized_node.data_copiers_src_copy,
      );
      Ok(())
   }

   // self is the enclosing subgraph.
   pub fn create_inbound_copiers_for_leaf_node(
      &mut self,
      node_num: usize,
      node_defs: &Vec<ZNodeDef>,
      _registry: &ZRegistry,
      null_node: &Rc<RefCell<ZNode>>,
      floating_port_data: &PortDataVec,
      inbetween_copiers_src_copy: &mut Vec<Rc<RefCell<PortDataCopier>>>,
   ) -> Result<(), ZGraphError> {
      let subnode: &mut ZNode = &mut self.subgraph_nodes[node_num].borrow_mut();
      assert!(node_defs[node_num].is_precompiled);

      assert!(subnode.data_copiers_dest_copy.is_empty());
      subnode.data_copiers_dest_copy.clear();
      // let mut inbetween_copiers_src_copy = Vec::<Rc<RefCell<PortDataCopier>>>::default();
      PortDataCopier::create_copiers_for_link_ports(
         VoidFilter::All,
         &node_defs[node_num].all_links,
         null_node,
         floating_port_data,
         floating_port_data,
         &mut subnode.data_copiers_dest_copy,
         inbetween_copiers_src_copy,
      );
      Ok(())
   }

   pub fn create_src_data_for_node(
      wrapped_subnode: &Rc<RefCell<ZNode>>,
      null_node: &Rc<RefCell<ZNode>>,
   ) -> Result<(), ZGraphError> {
      let subnode: &mut ZNode = &mut wrapped_subnode.borrow_mut();
      let subnode_type: &ZNodeRegistration = subnode.node_type.as_ref();

      if !subnode.is_active {
         return Ok(());
      }
      {
         let src_ports_types: &Vec<PortPieceTyped> = &subnode_type.ports_src_copy;
         let src_data_copiers: &mut Vec<Rc<RefCell<PortDataCopier>>> =
            &mut subnode.data_copiers_src_copy;

         {
            let src_ports_data: &mut Vec<ZPiece> = &mut subnode.data_ports_src_copy.borrow_mut();
            assert!(src_ports_data.is_empty());
            src_ports_data.clear();
            src_ports_data.reserve(src_ports_types.len());
            for port_type in src_ports_types {
               src_ports_data.push(ZPiece::piece_data_default_for_piece_type(&port_type.1));
               eprintln!(
                  "   src data element type and type of data {:?} and {:?}",
                  port_type.1,
                  ZPieceType::get_piece_type_from_data(&src_ports_data[src_ports_data.len() - 1])
               );
            }
         }

         let mut port_names = Vec::<&String>::default();
         let mut port_name_map = HashMap::<String, usize>::default();
         port_names.reserve(src_ports_types.len());

         for (i, entry_type) in src_ports_types.iter().enumerate() {
            port_names.push(&entry_type.0);
            port_name_map.insert(entry_type.0.clone(), i);
         }
         eprintln!("Create src data for {}", subnode.name);
         for wrapped_copier in src_data_copiers {
            let mut copier: &mut PortDataCopier = &mut wrapped_copier.borrow_mut();

            eprintln!(
               "copier port {}, {}",
               copier.port_def.as_ref().unwrap().src_node_name,
               copier.port_def.as_ref().unwrap().src_port
            );
            // At this stage no copier should have unresolved source.
            assert!(!Rc::ptr_eq(&copier.src_node, null_node));

            if !Rc::ptr_eq(wrapped_subnode, &copier.src_node) {
               eprintln!(
                  "Copier source node not matching: {} vs {}",
                  copier.src_node.borrow().name,
                  subnode.name
               );
            }
            assert!(Rc::ptr_eq(wrapped_subnode, &copier.src_node));
            copier.src_port_data = subnode.data_ports_src_copy.clone();
            let port_name: &String = &copier.port_def.as_ref().unwrap().src_port;

            if copier.port_def.as_ref().unwrap().is_void {
               assert_eq!(copier.src_index, PortDataCopier::VOID_SENTINEL);
            } else {
               let gotten = port_name_map.get(port_name);
               assert!(
                  gotten.is_some(),
                  "Unable to find port \"{}\" in src node named \"{}\" for destination \"{}\"",
                  &port_name,
                  subnode.name,
                  "Unknown"
               );
               copier.src_index = *gotten.unwrap();
            }
         }
      }
      Ok(())
   }

   pub fn create_dest_data_for_node(
      wrapped_subnode: &Rc<RefCell<ZNode>>,
      null_node: &Rc<RefCell<ZNode>>,
   ) -> Result<(), ZGraphError> {
      let subnode: &mut ZNode = &mut wrapped_subnode.borrow_mut();
      let subnode_type: &ZNodeRegistration = subnode.node_type.as_ref();

      if !subnode.is_active {
         return Ok(());
      }
      {
         let dest_ports_types: &Vec<PortPieceTyped> = &subnode_type.ports_dest_copy;
         let dest_data_copiers: &mut Vec<Rc<RefCell<PortDataCopier>>> =
            &mut subnode.data_copiers_dest_copy;
         {
            let dest_ports_data: &mut Vec<ZPiece> = &mut subnode.data_ports_dest_copy.borrow_mut();
            assert!(dest_ports_data.is_empty());
            dest_ports_data.clear();
            dest_ports_data.reserve(dest_ports_types.len());
            for port_type in dest_ports_types {
               dest_ports_data.push(ZPiece::piece_data_default_for_piece_type(&port_type.1));
               eprintln!(
                  "   dest data element type and type of data {:?} and {:?}",
                  port_type.1,
                  ZPieceType::get_piece_type_from_data(&dest_ports_data[dest_ports_data.len() - 1])
               );
            }
         }

         let mut port_names = Vec::<&String>::default();
         let mut port_name_map = HashMap::<String, usize>::default();
         port_names.reserve(dest_ports_types.len());

         for (i, entry_type) in dest_ports_types.iter().enumerate() {
            port_names.push(&entry_type.0);
            port_name_map.insert(entry_type.0.clone(), i);
         }
         eprintln!("Create dest data for {}", subnode.name);
         for wrapped_copier in dest_data_copiers {
            let mut copier: &mut PortDataCopier = &mut wrapped_copier.borrow_mut();

            if Rc::ptr_eq(&copier.src_node, null_node) {
               eprintln!(
                  "Null src invalid this late {}, {}, {}",
                  copier.port_def.as_ref().unwrap().name,
                  copier.port_def.as_ref().unwrap().src_node_name,
                  copier.port_def.as_ref().unwrap().src_port
               );
            }
            assert!(!Rc::ptr_eq(&copier.src_node, null_node));
            // assert!(
            //    copier.port_def.as_ref().unwrap().is_void
            //       || !Rc::ptr_eq(&copier.src_node, null_node)
            // );

            // assert!(Rc::ptr_eq(&self.subgraph_nodes[node_num], &copier.dest_node));
            copier.dest_port_data = subnode.data_ports_dest_copy.clone();
            let dest_port_name: &String = &copier.port_def.as_ref().unwrap().name;
            // let src_port_name: &String = &copier.port_def.as_ref().unwrap().src_port;

            if copier.port_def.as_ref().unwrap().is_void {
               // copier.dest_index = PortDataCopier::VOID_SENTINEL;
               assert_eq!(copier.dest_index, PortDataCopier::VOID_SENTINEL);
            } else {
               let gotten = port_name_map.get(dest_port_name);
               if gotten.is_none() {
                  let src_name_try = copier.src_node.try_borrow();
                  let src_name: String = if let Ok(src_name_node) = src_name_try {
                     src_name_node.name.clone()
                  } else {
                     "Not sure (".to_string() + &subnode.name + "???)"
                  };
                  assert!(
                        gotten.is_some(),
                        "Unable to find port \"{}\" in dest for src node named \"{}\" for destination \"{}\"",
                        &dest_port_name,
                        &src_name,
                        subnode.name,
                   );
               }
               copier.dest_index = *gotten.unwrap();
            }

            if !copier.port_def.as_ref().unwrap().is_void {
               assert_ne!(copier.src_index, PortDataCopier::VOID_SENTINEL);
               // Check src and dest port element types.
               let src_piece_type = ZPieceType::get_piece_type_from_data(
                  &copier.src_port_data.borrow()[copier.src_index],
               );
               let dest_piece_type = ZPieceType::get_piece_type_from_data(
                  &copier.dest_port_data.borrow()[copier.dest_index],
               );
               eprintln!(
                  "Copier has src type {:?} and dest type {:?}, indices {}, {}",
                  src_piece_type, dest_piece_type, copier.src_index, copier.dest_index,
               );
               assert!(src_piece_type == dest_piece_type);
            }
         }
      }
      Ok(())
   }

   pub fn populate_preset_data_for_node(
      wrapped_subnode: &Rc<RefCell<ZNode>>,
   ) -> Result<(), ZGraphError> {
      let subnode: &mut ZNode = &mut wrapped_subnode.borrow_mut();
      let subnode_type: &ZNodeRegistration = subnode.node_type.as_ref();

      eprintln!("   Preset data population for node {}", subnode.name);

      if subnode.data_copiers_src_copy.is_empty() {
         return Ok(());
      }
      if subnode_type.category != ZNodeCategory::PresetData {
         return Ok(());
      }
      for (i, preset_item) in subnode.preset_data.iter().enumerate() {
         let src_ports_data: &mut Vec<ZPiece> = &mut subnode.data_ports_src_copy.borrow_mut();
         src_ports_data[i] = preset_item.1.clone();
         eprintln!(
            "      Preset data has apparent type {:?}",
            ZPieceType::get_piece_type_from_data(&src_ports_data[i])
         );
      }
      Ok(())
   }

   pub fn add_tree_dag_nodes(
      toposort_nodes: &mut LinkedList<Rc<RefCell<ZNode>>>,
      subgraph_node: &Rc<RefCell<ZNode>>,
   ) {
      let subgraph_node_borrowed: &mut ZNode = &mut subgraph_node.borrow_mut();

      let subgraph_size = subgraph_node_borrowed.subgraph_nodes.len();

      for node_num in 0..subgraph_size {
         let subnode_unborrowed: &Rc<RefCell<ZNode>> =
            &subgraph_node_borrowed.subgraph_nodes[node_num];
         let is_leaf_node_outside: bool;
         {
            let subnode: &mut ZNode = &mut subnode_unborrowed.borrow_mut();

            assert_eq!(is_leaf_node(&subnode.node_type), subnode.subgraph_nodes.is_empty());

            assert_eq!(
               subnode.is_active,
               !subnode.data_copiers_src_copy.is_empty()
                  || !subnode.data_copiers_src_void_port.is_empty()
                  || !subnode.subgraph_nodes.is_empty()
            );
            if !subnode.is_active {
               continue;
            }

            is_leaf_node_outside = is_leaf_node(&subnode.node_type);
         }

         if is_leaf_node_outside {
            toposort_nodes.push_back(subnode_unborrowed.clone());
         } else {
            ZNode::add_tree_dag_nodes(toposort_nodes, subnode_unborrowed);
         }
      }

      toposort_nodes.push_back(subgraph_node.clone());
   }

   pub fn get_or_find_node_type<'a>(
      node: &'a Rc<RefCell<ZNode>>,
      node_def: &'a ZNodeDef,
      registry: &'a ZRegistry,
   ) -> Rc<ZNodeRegistration> {
      let gotten = registry.find(&node_def.element);
      assert!(
         gotten.is_ok(),
         "Could not find element type in registry, element name \"{}\"",
         node_def.element.get_descriptive_name()
      );

      let subnode_type: &Rc<ZNodeRegistration> = gotten.unwrap();

      {
         let subnode: &mut ZNode = &mut node.borrow_mut();

         subnode.disaggregation_nodes.resize(subnode_type.ports_src_copy.len(), None);
         subnode.node_type = subnode_type.clone();
         subnode.node_type_finder = Some(node_def.element.clone());
         subnode.node_type.clone()
      }
   }

   // Move the attachment point of a copier (passed in wrapped and alread-borrowed form) to its
   // destination node within a subgraph.  The process will, for subfield copiers, be iterative.
   //
   // (a) For void copiers, the destination attachment will be the final destination.  (b) Once
   // a copier hits the subgraph "inputs", that is it derives from the subgraph, it stops.  (c)
   // Copiers that access subfields, when hitting an aggregation node, will follow that node's
   // inputs.  (d) Copiers that access subfields will otherwise need the insertion of a
   // disaggregator.
   #[allow(clippy::too_many_arguments)]
   #[allow(clippy::needless_return)]
   pub fn reattach_copier(
      copier_wrapped: &Rc<RefCell<PortDataCopier>>,
      copier_borrowed: &mut PortDataCopier,
      enclosing_data_copiers_src_copy: &mut Vec<Rc<RefCell<PortDataCopier>>>,
      enclosing_data_copiers_src_void_port: &mut Vec<Rc<RefCell<PortDataCopier>>>,
      enclosing_is_active: &mut bool,
      subgraph_node_map: &HashMap<std::string::String, usize>,
      subgraph_nodes: &Vec<Rc<RefCell<ZNode>>>,
      subgraph_node_defs: &Vec<ZNodeDef>,
      registry: &ZRegistry,
   ) {
      let port_def: &ZLinkPort = copier_borrowed.port_def.as_ref().unwrap();

      if port_def.is_graph_input {
         if port_def.is_void {
            // Final destination.
            // copier_borrowed.src_node = enclosing_znode.clone();
            copier_borrowed.src_index = PortDataCopier::VOID_SENTINEL;
            // Note that this clones the reference to the copier.

            enclosing_data_copiers_src_void_port.push(copier_wrapped.clone());
            // MMM should only apply is_active if toplevel.
            *enclosing_is_active = true;
         } else {
            enclosing_data_copiers_src_copy.push(copier_wrapped.clone());
            // MMM should only apply is_active if toplevel.
            *enclosing_is_active = true;
         }
      } else {
         let node_num: usize = *subgraph_node_map.get(&port_def.src_node_name).unwrap();
         let src_node_znode: &Rc<RefCell<ZNode>> = &subgraph_nodes[node_num];
         let n_def = &subgraph_node_defs[node_num];

         if port_def.has_subfield() {
            let attachment_node_reg: Rc<ZNodeRegistration> =
               Self::get_or_find_node_type(src_node_znode, n_def, registry);

            if attachment_node_reg.category == ZNodeCategory::Aggregator {
               let upstream_ports: &Vec<ZLinkPort> = &n_def.all_links;
               let port_def: &mut ZLinkPort = copier_borrowed.port_def.as_mut().unwrap();
               // Handle port name by reference to match code elsewhere.
               let target_port_name_str = port_def.pop();
               let target_port_name = &target_port_name_str;

               let mut found = false;
               for link in upstream_ports {
                  if link.name == *target_port_name {
                     found = true;
                     ZLinkPort::merge_upstream_into_down(link, port_def);
                     break;
                  }
               }
               assert!(found);
            } else {
               panic!("not yet implemented to auto-insert disaggs {}", attachment_node_reg.name);
            }

            // This is a tail recursive call with same arguments. The effect is iterative,
            // moving up any auto-disaggregation nodes and updating the `port_def` each
            // step.
            Self::reattach_copier(
               copier_wrapped,
               copier_borrowed,
               enclosing_data_copiers_src_copy,
               enclosing_data_copiers_src_void_port,
               enclosing_is_active,
               subgraph_node_map,
               subgraph_nodes,
               subgraph_node_defs,
               registry,
            );
            // Explicit return to ensure that this is tail recursive and not accidentally
            // modified otherwise.
            return;
         } else {
            let mut src_node_znode_bmut = src_node_znode.borrow_mut();

            src_node_znode_bmut.is_active = true;

            copier_borrowed.src_node = src_node_znode.clone();
            if port_def.is_void {
               // Final destination.
               copier_borrowed.src_index = PortDataCopier::VOID_SENTINEL;
               // Note that this clones the reference to the copier.
               src_node_znode_bmut.data_copiers_src_void_port.push(copier_wrapped.clone());
            } else {
               // Note that this clones the reference to the copier.
               src_node_znode_bmut.data_copiers_src_copy.push(copier_wrapped.clone());
            }
            // *src_node_copier_borrowed = src_node_znode.clone();
         }
      }
   }

   pub fn terminate_voids_and_collect_remainder(
      in_place_copiers_src_copy: &mut Vec<Rc<RefCell<PortDataCopier>>>,
      data_copiers_src_void_port: &mut Vec<Rc<RefCell<PortDataCopier>>>,
      non_void_copiers_src_copy: &mut Vec<Rc<RefCell<PortDataCopier>>>,
   ) {
      let clone_of_subnode_data_copiers_src_copy = in_place_copiers_src_copy.clone();
      in_place_copiers_src_copy.clear();

      for copier_wrapped in &clone_of_subnode_data_copiers_src_copy {
         let copier_borrowed: &PortDataCopier = &mut copier_wrapped.borrow();
         eprintln!(
            "Src from this node {}, {}, {}",
            copier_borrowed.port_def.as_ref().unwrap().name,
            copier_borrowed.port_def.as_ref().unwrap().src_node_name,
            copier_borrowed.port_def.as_ref().unwrap().src_port,
         );
         assert_eq!(
            copier_borrowed.src_index == PortDataCopier::VOID_SENTINEL,
            copier_borrowed.port_def.as_ref().unwrap().is_void
         );
         if copier_borrowed.port_def.as_ref().unwrap().is_void {
            data_copiers_src_void_port.push(copier_wrapped.clone());
            // in_place_copiers_src_copy.push(copier_wrapped.clone());
            assert!(
               !copier_borrowed.port_def.as_ref().unwrap().is_void,
               "Contractually, this likely should no longer occur."
            );
            continue;
         }

         non_void_copiers_src_copy.push(copier_wrapped.clone());
      }
   }

   pub fn merge_subgraph_ports_into_copiers(
      non_void_copiers_src_copy: &Vec<Rc<RefCell<PortDataCopier>>>,
      upstream_ports: &Vec<ZLinkPort>,
   ) {
      for copier_wrapped in non_void_copiers_src_copy {
         let copier_borrowed: &mut PortDataCopier = &mut copier_wrapped.borrow_mut();
         assert_eq!(copier_borrowed.src_index, PortDataCopier::FLOATING_SENTINEL);
         assert_eq!(copier_borrowed.dest_index, PortDataCopier::FLOATING_SENTINEL);

         let port_def: &mut ZLinkPort = copier_borrowed.port_def.as_mut().unwrap();
         let target_port_name = &port_def.src_port;

         let mut found = false;
         for link in upstream_ports {
            if link.name == *target_port_name {
               found = true;
               ZLinkPort::merge_upstream_into_down(link, port_def);
               break;
            }
         }

         assert!(
            found,
            "Unable to find connection for {}",
            copier_borrowed.port_def.as_ref().unwrap().src_port
         );
      }
   }

   pub fn build_out_subgraph(
      top_node: &Rc<RefCell<ZNode>>,
      graph_def: &ZGraphDef,
      registry: &ZRegistry,
      null_node: &Rc<RefCell<ZNode>>,
      floating_port_data: &PortDataVec,
      _is_toplevel: bool,
   ) -> Result<(), ZGraphError> {
      ZNode::seed_user_graph_outputs(
         VoidFilter::VoidOnly,
         top_node,
         &graph_def.output_ports_as_links,
         null_node,
         floating_port_data,
      )?;

      {
         // Create: (a) Shell ZNodes for every node in the subgraph graphdef.  (b) A hash map
         // of names to indices, which index nodes in the graphdef and ZNodes.
         //
         // The shell nodes are given copies of preset data. The amount of extra work and
         // storage might be small, and it avoids needing the graphdef later as would be the
         // case if that data were populated lazily.
         assert!(graph_def.is_precompiled);

         // Create vector of nodes for subgraph.
         let enclosing_borrowed: &mut ZNode = &mut top_node.borrow_mut();

         let node_defs: &Vec<ZNodeDef> = &graph_def.nodes;
         let subgraph_size = node_defs.len();

         // 1st pass: set up minimal vector of realized nodes.
         let mut node_map_size: usize = enclosing_borrowed.subgraph_nodes.len();
         assert_eq!(node_map_size, 0);
         enclosing_borrowed.subgraph_nodes.reserve_exact(subgraph_size);
         let mut subgraph_node_map = HashMap::<String, usize>::default();

         for n_def in node_defs {
            assert!(n_def.is_precompiled);

            enclosing_borrowed.subgraph_nodes.push(ZNode::new_basic_node(
               &n_def.name[..],
               &n_def.preset_data,
               &ZNodeTypeFinder::NullNodeType,
               registry,
            ));

            assert!(!subgraph_node_map.contains_key(&n_def.name));
            subgraph_node_map.insert(n_def.name.clone(), node_map_size);
            node_map_size += 1;
         }
         assert_eq!(node_map_size, subgraph_size);
         enclosing_borrowed.subgraph_node_map = Some(subgraph_node_map);
      }

      // Handle external copiers, that is those with destinations outside the graph and that
      // source from this subgraph's outputs.
      {
         let enclosing_borrowed: &mut ZNode = &mut top_node.borrow_mut();
         let node_defs: &Vec<ZNodeDef> = &graph_def.nodes;
         let &mut subgraph_node_map = &mut enclosing_borrowed.subgraph_node_map.as_ref().unwrap();

         let copiers_src_copy_moved = enclosing_borrowed.data_copiers_src_copy.clone();
         enclosing_borrowed.data_copiers_src_copy.clear();
         for copier_wrapped in &copiers_src_copy_moved {
            let copier_borrowed: &mut PortDataCopier = &mut copier_wrapped.borrow_mut();

            // Reattach a copier to an internal source node, or another source by following
            // subfield extraction.
            Self::reattach_copier(
               copier_wrapped,
               copier_borrowed,
               &mut enclosing_borrowed.data_copiers_src_copy,
               &mut enclosing_borrowed.data_copiers_src_void_port,
               &mut enclosing_borrowed.is_active,
               subgraph_node_map,
               &enclosing_borrowed.subgraph_nodes,
               node_defs,
               registry,
            );
         }
      }

      // Process nodes from sink back to source
      {
         let enclosing_borrowed: &mut ZNode = &mut top_node.borrow_mut();

         let node_defs: &Vec<ZNodeDef> = &graph_def.nodes;
         let subgraph_size = node_defs.len();

         for node_num in (0..subgraph_size).rev() {
            let n_def = &node_defs[node_num];

            if !enclosing_borrowed.subgraph_nodes[node_num].borrow().is_active {
               continue;
            }

            let subnode_type: Rc<ZNodeRegistration> = Self::get_or_find_node_type(
               &enclosing_borrowed.subgraph_nodes[node_num],
               n_def,
               registry,
            );

            //
            let mut outward_copiers_src_copy = Vec::<Rc<RefCell<PortDataCopier>>>::default();

            if is_leaf_node(&subnode_type) {
               enclosing_borrowed.create_inbound_copiers_for_leaf_node(
                  node_num,
                  node_defs,
                  registry,
                  null_node,
                  floating_port_data,
                  &mut outward_copiers_src_copy,
               )?;
            } else {
               eprintln!("++> Recursing for {}", n_def.name);
               {
                  // Pre-process inbound connections to this node before expansion.

                  let subnode: &mut ZNode =
                     &mut enclosing_borrowed.subgraph_nodes[node_num].borrow_mut();
                  let mut non_void_copiers_src_copy = Vec::<Rc<RefCell<PortDataCopier>>>::default();

                  // (A) Process and separate out void copiers the src to this node's subgraph,
                  // because they do not get mapped *inwards*.
                  Self::terminate_voids_and_collect_remainder(
                     &mut subnode.data_copiers_src_copy,
                     &mut subnode.data_copiers_src_void_port,
                     &mut non_void_copiers_src_copy,
                  );

                  // (B) Map non-void copiers through the *output* link ports for this node's subgraph.
                  let upstream_ports: &Vec<ZLinkPort> =
                     &subnode_type.graph_def.output_ports_as_links;
                  Self::merge_subgraph_ports_into_copiers(
                     &non_void_copiers_src_copy,
                     upstream_ports,
                  );

                  // (C) Before subgraph recursion we do not chase the copiers into the
                  // subgraph. The recursion does this after creating its nodes.
                  //
                  subnode.data_copiers_src_copy.append(&mut non_void_copiers_src_copy);
               }
               Self::build_out_subgraph(
                  &enclosing_borrowed.subgraph_nodes[node_num],
                  &subnode_type.graph_def,
                  registry,
                  null_node,
                  floating_port_data,
                  /*is_toplevel=*/ false,
               )?;
               eprintln!("<-- Up return for {}", n_def.name);

               {
                  // Replumb all "input" internal copiers in the current node's subgraph through
                  // the links to the node's inputs.

                  let subnode: &mut ZNode =
                     &mut enclosing_borrowed.subgraph_nodes[node_num].borrow_mut();

                  // (A) Process and separate out void copiers the src to this node's subgraph,
                  // because they do not get mapped *outwards*.
                  Self::terminate_voids_and_collect_remainder(
                     &mut subnode.data_copiers_src_copy,
                     &mut subnode.data_copiers_src_void_port,
                     &mut outward_copiers_src_copy,
                  );
                  enclosing_borrowed
                     .data_copiers_src_copy
                     .append(&mut subnode.data_copiers_src_copy);

                  // (B) Map non-void copiers through the *input* link ports for this node's subgraph.
                  let upstream_ports: &Vec<ZLinkPort> = &n_def.all_links;
                  Self::merge_subgraph_ports_into_copiers(
                     &outward_copiers_src_copy,
                     upstream_ports,
                  );
               }
            }

            // (C) Attach copiers to src nodes within the graph, chasing subfields as
            // needed.
            for copier_wrapped in &outward_copiers_src_copy {
               let copier_borrowed: &mut PortDataCopier = &mut copier_wrapped.borrow_mut();

               Self::reattach_copier(
                  copier_wrapped,
                  copier_borrowed,
                  &mut enclosing_borrowed.data_copiers_src_copy,
                  &mut enclosing_borrowed.data_copiers_src_void_port,
                  &mut enclosing_borrowed.is_active,
                  enclosing_borrowed.subgraph_node_map.as_ref().unwrap(),
                  &enclosing_borrowed.subgraph_nodes,
                  &node_defs,
                  registry,
               );
            }
            {
               // let enclosing_borrowed: &mut ZNode = &mut top_node.borrow_mut();
               for copier_wrapped in &enclosing_borrowed.data_copiers_src_void_port {
                  let copier_borrowed: &mut PortDataCopier = &mut copier_wrapped.borrow_mut();
                  copier_borrowed.src_node = top_node.clone();
               }
            }
         }
      }

      // Not true: void copiers are left attached to nested subgraph sources.
      // if !is_toplevel {
      //    let enclosing_borrowed: &ZNode = &top_node.borrow();
      //    assert!(enclosing_borrowed.data_copiers_src_copy.is_empty());
      // }

      Ok(())
   }

   pub fn realize_from_usergraph_graphdef(
      top_node: &Rc<RefCell<ZNode>>,
      graph_def: &ZGraphDef,
      registry: &ZRegistry,
      null_node: &Rc<RefCell<ZNode>>,
      floating_port_data: &PortDataVec,
   ) -> Result<(), ZGraphError> {
      assert!(graph_def.is_precompiled);

      Self::build_out_subgraph(
         top_node,
         graph_def,
         registry,
         null_node,
         floating_port_data,
         /*is_toplevel=*/ true,
      )?;

      // if is_toplevel {
      // This could be applied to any subgraph, but the remaining copiers-src should be empty or void.
      {
         let enclosing_borrowed: &mut ZNode = &mut top_node.borrow_mut();

         for copier in &enclosing_borrowed.data_copiers_src_copy {
            let copier_borrowed = &mut copier.borrow_mut();
            // assert!(Rc::ptr_eq(&copier_borrowed.src_node, null_node));
            assert!(copier_borrowed.port_def.as_ref().unwrap().is_graph_input);
            copier_borrowed.src_node = top_node.clone();
            if copier_borrowed.port_def.as_ref().unwrap().is_void {
               copier_borrowed.src_index = PortDataCopier::VOID_SENTINEL;
            }
         }
      }
      // }

      let mut toposort_nodes = LinkedList::<Rc<RefCell<ZNode>>>::new();
      Self::add_tree_dag_nodes(&mut toposort_nodes, top_node);

      // Populate type information with custom "node registration" for preset data nodes. This
      // could probably be folded in to later loops for efficiency, but is performed separately
      // for now for semantic clarity.
      {
         for wrapped_subnode in &toposort_nodes {
            let subnode: &mut ZNode = &mut wrapped_subnode.borrow_mut();
            let subnode_type: &ZNodeRegistration = subnode.node_type.as_ref();

            if subnode_type.category == ZNodeCategory::PresetData {
               let mut replacement_registration: ZNodeRegistration = subnode_type.clone();
               // Replace output port typing.
               for preset_item in &subnode.preset_data {
                  replacement_registration.ports_src_copy.push(PortPieceTyped(
                     preset_item.0.clone(),
                     ZPieceType::get_piece_type_from_data(&preset_item.1),
                  ));
               }
               subnode.node_type = Rc::new(replacement_registration);
            }
         }
      }

      // Late pass, after pruning optimizations, is to create port data vectors.
      {
         // Copiers src data ports.

         for wrapped_subnode in &toposort_nodes {
            ZNode::create_src_data_for_node(wrapped_subnode, null_node)?;
         }
      }

      {
         // CCC Top-level only.
         //
         // Check consistency.
         let enclosing_borrowed: &mut ZNode = &mut top_node.borrow_mut();
         assert_eq!(
            graph_def.output_ports_as_links.len(),
            enclosing_borrowed.data_copiers_dest_copy.len()
         );

         for (i, wrapped_copier) in enclosing_borrowed.data_copiers_dest_copy.iter().enumerate() {
            let port_def = &graph_def.output_ports_as_links[i];
            let copier: &mut PortDataCopier = &mut wrapped_copier.borrow_mut();
            assert_eq!(port_def.name, copier.port_def.as_ref().unwrap().name);
         }
      }

      {
         // CCC Top-level only.
         //
         // Figure types of outer-graph outputs from inputs into their copiers, which must now be resolved.

         // Recreate custom type "registration" with outer graph's destination port typing.

         // This is not intended to be fast.
         let enclosing_borrowed: &mut ZNode = &mut top_node.borrow_mut();

         let subnode_type: &ZNodeRegistration = enclosing_borrowed.node_type.as_ref();
         let mut replacement_registration: ZNodeRegistration = subnode_type.clone();

         replacement_registration
            .ports_dest_copy
            .reserve(enclosing_borrowed.data_copiers_dest_copy.len());

         for wrapped_copier in &enclosing_borrowed.data_copiers_dest_copy {
            let copier: &mut PortDataCopier = &mut wrapped_copier.borrow_mut();

            if copier.src_index == PortDataCopier::FLOATING_SENTINEL {
               eprintln!(
                  "Floating still {}, {}, {}",
                  copier.port_def.as_ref().unwrap().name,
                  copier.port_def.as_ref().unwrap().src_node_name,
                  copier.port_def.as_ref().unwrap().src_port
               );
            }
            assert_ne!(copier.src_index, PortDataCopier::FLOATING_SENTINEL);
            assert!(!Rc::ptr_eq(&copier.src_node, null_node));
            assert_eq!(
               Rc::ptr_eq(&copier.src_node, top_node),
               copier.port_def.as_ref().unwrap().is_graph_input
            );
            assert_eq!(
               copier.src_index == PortDataCopier::VOID_SENTINEL,
               copier.port_def.as_ref().unwrap().is_void
            );
            let piece_type = if copier.src_index == PortDataCopier::VOID_SENTINEL {
               // This is a legitimate path.
               ZPieceType::Void
            } else if Rc::ptr_eq(&copier.src_node, top_node) {
               let src_node: &ZNode = enclosing_borrowed;
               ZPieceType::get_piece_type_from_data(
                  &src_node.data_ports_src_copy.borrow()[copier.src_index],
               )
            } else {
               let wrapped_src_node = copier.src_node.borrow();
               let src_node: &ZNode = &wrapped_src_node;
               let x = ZPieceType::get_piece_type_from_data(
                  &src_node.data_ports_src_copy.borrow()[copier.src_index],
               );
               x
            };

            replacement_registration
               .ports_dest_copy
               .push(PortPieceTyped(copier.port_def.as_ref().unwrap().name.clone(), piece_type));
         }
         enclosing_borrowed.node_type = Rc::new(replacement_registration);
      }

      // It is likely possible to have only src_data_copiers as Vec<Box<PortDataCopier>> and at
      // this point move the copiers to similar dest_data_copiers. However, such a
      // simplification should only be done when we have established the desirability of
      // invoking copiers as sub-iteration.
      {
         for wrapped_subnode in &toposort_nodes {
            // Copiers dest data ports.
            ZNode::create_dest_data_for_node(wrapped_subnode, null_node)?;

            // Fill preset data.
            ZNode::populate_preset_data_for_node(wrapped_subnode)?;
         }
      }

      {
         // Debug / check consistency.

         let enclosing_borrowed: &mut ZNode = &mut top_node.borrow_mut();

         // Begin: Debugging of unsourced copiers.
         for wrapped_copier in &enclosing_borrowed.data_copiers_dest_copy {
            let copier: &PortDataCopier = &wrapped_copier.borrow();

            assert!(!Rc::ptr_eq(&copier.src_node, null_node));
         }
         for wrapped_copier in &enclosing_borrowed.data_copiers_src_copy {
            let copier: &PortDataCopier = &wrapped_copier.borrow_mut();
            let &port_def = &copier.port_def.as_ref().unwrap();
            eprintln!(
               "direct copier: {}, {}, {}",
               port_def.name, port_def.src_node_name, port_def.src_port
            );
         }
         // End: Debugging of unsourced copiers.
      }

      // A late-late pass could merge adjacent copiers.

      Ok(())
   }
}
