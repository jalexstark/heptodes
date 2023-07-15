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
use crate::jaywalk_graph::zgraph_base::PortPieceTyped;
use crate::jaywalk_graph::zgraph_base::ZGraphError;
use crate::jaywalk_graph::zgraph_base::ZPiece;
use crate::jaywalk_graph::zgraph_base::ZPieceType;
use crate::jaywalk_graph::zgraph_graphdef::ZGraphDef;
use crate::jaywalk_graph::zgraph_graphdef::ZNodeDef;
use crate::jaywalk_graph::zgraph_graphdef::ZPortDef;
use crate::jaywalk_graph::zgraph_node::PortDataCopier;
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
      for input in inputs {
         replacement_registration.ports_src_copy.push(input.clone());
      }
      top_node_borrowed.node_type = Rc::new(replacement_registration);
      Ok(())
   }

   pub fn seed_user_graph_outputs(
      realized_node_wrapped: &Rc<RefCell<ZNode>>,
      output_ports: &Vec<ZPortDef>,
      null_node: &Rc<RefCell<ZNode>>,
      floating_port_data: &PortDataVec,
   ) -> Result<(), ZGraphError> {
      let realized_node: &mut ZNode = &mut realized_node_wrapped.borrow_mut();

      // In ordinary mode of operation, for a user graph, we create one copier for each output.
      assert!(realized_node.data_copiers_dest_copy.is_empty());
      for port_def in output_ports {
         let expanded_graph_data_copier = Rc::new(RefCell::new(PortDataCopier {
            src_node: null_node.clone(),
            src_port_data: floating_port_data.clone(),
            src_index: PortDataCopier::VOID_SENTINEL,
            dest_port_data: realized_node.data_ports_dest_copy.clone(),
            dest_index: PortDataCopier::VOID_SENTINEL,
            port_def: Some(port_def.clone()),
         }));
         realized_node.data_copiers_dest_copy.push(expanded_graph_data_copier.clone());
         // The "src copiers" are a temporary parking spot.
         realized_node.data_copiers_src_copy.push(expanded_graph_data_copier);
         realized_node.is_active = true;
         eprintln!("Port attachment: {}", port_def.0);
      }
      Ok(())
   }

   pub fn finish_leaf_node(
      &mut self,
      node_num: usize,
      n_def: &ZNodeDef,
      _registry: &ZRegistry,
      null_node: &Rc<RefCell<ZNode>>,
      floating_port_data: &PortDataVec,
   ) -> Result<(), ZGraphError> {
      let subnode: &mut ZNode = &mut self.subgraph_nodes[node_num].borrow_mut();

      assert!(subnode.data_copiers_dest_copy.is_empty());
      subnode.data_copiers_dest_copy.clear();

      for edge in &n_def.edges {
         eprintln!("   Adding subnode dependency on: {}", edge.src_node);
         let is_internal_src_node: bool = edge.src_node != "input";
         let src_node_znode: &Rc<RefCell<ZNode>> = if is_internal_src_node {
            let node_num: usize =
               *self.subgraph_node_map.as_ref().unwrap().get(&edge.src_node).unwrap();
            &self.subgraph_nodes[node_num]
         } else {
            eprintln!("      !! Edge source is not internal");
            null_node
         };

         for connection in &edge.connections {
            // Name among source nodes's outputs, name among dest node's inputs.
            eprintln!(
               "      Adding data copier: {}:{} to {}:{}",
               edge.src_node, connection.0, n_def.name, connection.1
            );

            // Only leaf nodes actually create copiers (plus
            // the very outer user-graph realization).
            let edges_copier = Rc::new(RefCell::new(PortDataCopier {
               src_node: src_node_znode.clone(),
               src_port_data: floating_port_data.clone(),
               src_index: PortDataCopier::VOID_SENTINEL,
               dest_port_data: floating_port_data.clone(),
               dest_index: PortDataCopier::VOID_SENTINEL,
               port_def: Some(ZPortDef(
                  connection.1.clone(),
                  edge.src_node.clone(),
                  connection.0.clone(),
               )),
            }));
            subnode.data_copiers_dest_copy.push(edges_copier.clone());

            if is_internal_src_node {
               let mut src_node_znode_bmut = src_node_znode.borrow_mut();
               src_node_znode_bmut.data_copiers_src_copy.push(edges_copier);
               src_node_znode_bmut.is_active = true;
            } else {
               self.data_copiers_src_copy.push(edges_copier);
               self.is_active = true;
            }
         }
      }
      Ok(())
   }

   pub fn create_src_data_for_node(
      wrapped_subnode: &Rc<RefCell<ZNode>>,
      null_node: &Rc<RefCell<ZNode>>,
   ) -> Result<(), ZGraphError> {
      let subnode: &mut ZNode = &mut wrapped_subnode.borrow_mut();
      let subnode_type: &ZNodeRegistration = subnode.node_type.as_ref();

      if subnode.data_copiers_src_copy.is_empty() {
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
            }
         }

         let mut port_names = Vec::<&String>::default();
         let mut port_name_map = HashMap::<String, usize>::default();
         port_names.reserve(src_ports_types.len());

         for (i, entry_type) in src_ports_types.iter().enumerate() {
            port_names.push(&entry_type.0);
            port_name_map.insert(entry_type.0.clone(), i);
         }
         for wrapped_copier in src_data_copiers {
            let mut copier: &mut PortDataCopier = &mut wrapped_copier.borrow_mut();

            assert!(!Rc::ptr_eq(&copier.src_node, null_node));

            assert!(Rc::ptr_eq(wrapped_subnode, &copier.src_node));
            copier.src_port_data = subnode.data_ports_src_copy.clone();
            let port_name: &String = &copier.port_def.as_ref().unwrap().2;

            if port_name == "void" {
               copier.src_index = PortDataCopier::VOID_SENTINEL;
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

      if subnode.data_copiers_src_copy.is_empty() {
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
            }
         }

         let mut port_names = Vec::<&String>::default();
         let mut port_name_map = HashMap::<String, usize>::default();
         port_names.reserve(dest_ports_types.len());

         for (i, entry_type) in dest_ports_types.iter().enumerate() {
            port_names.push(&entry_type.0);
            port_name_map.insert(entry_type.0.clone(), i);
         }
         for wrapped_copier in dest_data_copiers {
            let mut copier: &mut PortDataCopier = &mut wrapped_copier.borrow_mut();

            assert!(!Rc::ptr_eq(&copier.src_node, null_node));

            // assert!(Rc::ptr_eq(&self.subgraph_nodes[node_num], &copier.dest_node));
            copier.dest_port_data = subnode.data_ports_dest_copy.clone();
            let port_name: &String = &copier.port_def.as_ref().unwrap().0;

            if port_name == "void" {
               copier.dest_index = PortDataCopier::VOID_SENTINEL;
            } else {
               let gotten = port_name_map.get(port_name);
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
                        &port_name,
                        &src_name,
                        subnode.name,
                   );
               }
               copier.dest_index = *gotten.unwrap();
            }

            if copier.src_index != PortDataCopier::VOID_SENTINEL {
               // Check src and dest port element types.
               let src_piece_type = ZPieceType::get_piece_type_from_data(
                  &copier.src_port_data.borrow()[copier.src_index],
               );
               let dest_piece_type = ZPieceType::get_piece_type_from_data(
                  &copier.dest_port_data.borrow()[copier.dest_index],
               );
               eprintln!(
                  "Copier has src type {:?} and dest type {:?}",
                  src_piece_type, dest_piece_type
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

      if subnode.data_copiers_src_copy.is_empty() {
         return Ok(());
      }
      if subnode_type.category != ZNodeCategory::PresetData {
         return Ok(());
      }
      for (i, preset_item) in subnode.preset_data.iter().enumerate() {
         let src_ports_data: &mut Vec<ZPiece> = &mut subnode.data_ports_src_copy.borrow_mut();
         src_ports_data[i] = preset_item.1.clone();
      }
      Ok(())
   }

   /// BBB
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

            assert_eq!(subnode.is_active, !subnode.data_copiers_src_copy.is_empty());
            if subnode.data_copiers_src_copy.is_empty() {
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

   pub fn build_out_subgraph(
      top_node: &Rc<RefCell<ZNode>>,
      graph_def: &ZGraphDef,
      registry: &ZRegistry,
      null_node: &Rc<RefCell<ZNode>>,
      floating_port_data: &PortDataVec,
      is_toplevel: bool,
   ) -> Result<(), ZGraphError> {
      {
         // Create vector of nodes for subgraph.
         let top_node_borrowed: &mut ZNode = &mut top_node.borrow_mut();

         let node_defs: &Vec<ZNodeDef> = &graph_def.nodes;
         let subgraph_size = node_defs.len();
         let null_noderegistration: &Rc<ZNodeRegistration> = registry.get_null_noderegistration();

         // 1st pass: set up minimal vector of realized nodes.
         let mut node_map_size: usize = top_node_borrowed.subgraph_nodes.len();
         assert_eq!(node_map_size, 0);
         top_node_borrowed.subgraph_nodes.reserve_exact(subgraph_size);
         let mut subgraph_node_map = HashMap::<String, usize>::default();

         for n_def in node_defs {
            top_node_borrowed.subgraph_nodes.push(Rc::new(RefCell::new(ZNode {
               name: n_def.name.clone(),
               node_state_data: None,
               node_type: null_noderegistration.clone(),
               node_type_finder: None,
               data_copiers_src_copy: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
               data_copiers_dest_copy: Vec::<Rc<RefCell<PortDataCopier>>>::default(),
               data_ports_src_copy: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
               data_ports_dest_copy: Rc::new(RefCell::new(Vec::<ZPiece>::default())),
               subgraph_nodes: Vec::<Rc<RefCell<ZNode>>>::default(),
               subgraph_node_map: None,
               is_active: false,
               preset_data: n_def.preset_data.clone(),
            })));

            assert!(!subgraph_node_map.contains_key(&n_def.name));
            subgraph_node_map.insert(n_def.name.clone(), node_map_size);
            node_map_size += 1;
         }
         assert_eq!(node_map_size, subgraph_size);
         top_node_borrowed.subgraph_node_map = Some(subgraph_node_map);
      }

      // Inter-pass replumbing of copiers out of subgraph. Replumb to internal nodes for which
      // each is outbound.
      {
         let top_node_borrowed: &mut ZNode = &mut top_node.borrow_mut();

         let &mut subgraph_node_map = &mut top_node_borrowed.subgraph_node_map.as_ref().unwrap();
         let mut direct_in_out_copiers = Vec::<Rc<RefCell<PortDataCopier>>>::default();
         for external_copier in &top_node_borrowed.data_copiers_src_copy {
            let borrow_hold: &mut PortDataCopier = &mut external_copier.borrow_mut();
            let port_def: &ZPortDef = borrow_hold.port_def.as_ref().unwrap();

            let connects_to_internal: bool = port_def.1 != "input";
            if connects_to_internal {
               // Port name, src node name, src port name.
               let node_num: usize = *subgraph_node_map.get(&port_def.1).unwrap();
               let src_node_znode: &Rc<RefCell<ZNode>> =
                  &top_node_borrowed.subgraph_nodes[node_num];
               {
                  let mut src_node_znode_bmut = src_node_znode.borrow_mut();
                  src_node_znode_bmut.data_copiers_src_copy.push(external_copier.clone());
                  src_node_znode_bmut.is_active = true;
               }
               borrow_hold.src_node = src_node_znode.clone();
            } else {
               direct_in_out_copiers.push(external_copier.clone());
            }
         }
         top_node_borrowed.data_copiers_src_copy = direct_in_out_copiers; // Finish "move" of copiers to their internal nodes.
      }

      // 2nd pass vector of created nodes and node_defs matches before and after.
      {
         let top_node_borrowed: &mut ZNode = &mut top_node.borrow_mut();

         let node_defs: &Vec<ZNodeDef> = &graph_def.nodes;
         let subgraph_size = node_defs.len();

         for node_num in (0..subgraph_size).rev() {
            let n_def = &node_defs[node_num];

            eprintln!("Processing subnode: {}", n_def.name);

            let subnode_type: &Rc<ZNodeRegistration> = registry.find(&n_def.element).unwrap();
            assert_eq!(
               node_num,
               *top_node_borrowed.subgraph_node_map.as_ref().unwrap().get(&n_def.name).unwrap()
            );

            {
               let subnode: &mut ZNode =
                  &mut top_node_borrowed.subgraph_nodes[node_num].borrow_mut();

               if subnode.data_copiers_src_copy.is_empty() {
                  continue;
               }
               subnode.node_type = subnode_type.clone();
               subnode.node_type_finder = Some(n_def.element.clone());
            }

            if is_leaf_node(subnode_type) {
               top_node_borrowed.finish_leaf_node(
                  node_num,
                  n_def,
                  registry,
                  null_node,
                  floating_port_data,
               )?;
            } else {
               eprintln!(
                  "Apply input remapping from all top_node_borrowed.data_copiers_src_copy for {}",
                  n_def.name
               );
            }
         }
      }

      if is_toplevel {
         // CCC Top-level only.
         //
         // This could be applied to any subgraph, but the remaining copiers-src should be empty.

         let top_node_borrowed: &ZNode = &top_node.borrow();
         for copier in &top_node_borrowed.data_copiers_src_copy {
            let new_src_node = &mut copier.borrow_mut();
            assert!(Rc::ptr_eq(&new_src_node.src_node, null_node));
            new_src_node.src_node = top_node.clone();
         }
      }

      Ok(())
   }

   pub fn realize_from_usergraph_graphdef(
      top_node: &Rc<RefCell<ZNode>>,
      graph_def: &ZGraphDef,
      registry: &ZRegistry,
      null_node: &Rc<RefCell<ZNode>>,
      floating_port_data: &PortDataVec,
   ) -> Result<(), ZGraphError> {
      Self::build_out_subgraph(
         top_node,
         graph_def,
         registry,
         null_node,
         floating_port_data,
         /*is_toplevel=*/ true,
      )?;

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
         let top_node_borrowed: &mut ZNode = &mut top_node.borrow_mut();
         assert_eq!(graph_def.output_ports.len(), top_node_borrowed.data_copiers_dest_copy.len());

         for (i, wrapped_copier) in top_node_borrowed.data_copiers_dest_copy.iter().enumerate() {
            let port_def = &graph_def.output_ports[i];
            let copier: &mut PortDataCopier = &mut wrapped_copier.borrow_mut();
            assert_eq!(port_def.0, copier.port_def.as_ref().unwrap().0);
         }
      }

      {
         // CCC Top-level only.
         //
         // Figure types of outer-graph outputs from inputs into their copiers, which must now be resolved.

         // Recreate custom type "registration" with outer graph's destination port typing.

         // This is not intended to be fast.
         let top_node_borrowed: &mut ZNode = &mut top_node.borrow_mut();

         let subnode_type: &ZNodeRegistration = top_node_borrowed.node_type.as_ref();
         let mut replacement_registration: ZNodeRegistration = subnode_type.clone();

         replacement_registration
            .ports_dest_copy
            .reserve(top_node_borrowed.data_copiers_dest_copy.len());

         for wrapped_copier in &top_node_borrowed.data_copiers_dest_copy {
            let copier: &mut PortDataCopier = &mut wrapped_copier.borrow_mut();

            let piece_type = if copier.src_index == PortDataCopier::VOID_SENTINEL {
               // This is a legitimate path.
               ZPieceType::Void
            } else if Rc::ptr_eq(&copier.src_node, top_node) {
               let src_node: &ZNode = top_node_borrowed;
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
               .push(PortPieceTyped(copier.port_def.as_ref().unwrap().0.clone(), piece_type));
         }
         top_node_borrowed.node_type = Rc::new(replacement_registration);
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

         let top_node_borrowed: &mut ZNode = &mut top_node.borrow_mut();

         // Begin: Debugging of unsourced copiers.
         for wrapped_copier in &top_node_borrowed.data_copiers_dest_copy {
            let copier: &PortDataCopier = &wrapped_copier.borrow();

            assert!(!Rc::ptr_eq(&copier.src_node, null_node));
         }
         for wrapped_copier in &top_node_borrowed.data_copiers_src_copy {
            let copier: &PortDataCopier = &wrapped_copier.borrow_mut();
            let &port_def = &copier.port_def.as_ref().unwrap();
            eprintln!("direct copier: {}, {}, {}", port_def.0, port_def.1, port_def.2);
         }
         // End: Debugging of unsourced copiers.
      }

      // A late-late pass could merge adjacent copiers.

      Ok(())
   }
}
