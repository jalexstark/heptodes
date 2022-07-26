/*
 * Copyright 2022 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef BASE_DIR_PATINON_EXPLORATORY_ABC_SKIM_ENHANCED_PARSE_TREE_PROPERTY_H_
#define BASE_DIR_PATINON_EXPLORATORY_ABC_SKIM_ENHANCED_PARSE_TREE_PROPERTY_H_

#include "base_dir/absl/container/node_hash_map.h"
#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/misc/check_macros.h"

namespace patinon {
namespace pvn_parsing {

// Safer version of ParseTreeProperty.
template <typename V>
class AltParseTreeProperty {
 public:
  V get(antlr4::tree::ParseTree* node) {
    const int count = annotations_.count(node);
    PVN_CHECK_NE(count, 0);
    return annotations_[node];
  }
  void put(antlr4::tree::ParseTree* node, V value) {
    annotations_[node] = value;
  }
  V removeFrom(antlr4::tree::ParseTree* node) {
    auto value = annotations_[node];
    annotations_.erase(node);
    return value;
  }

 protected:
  absl::node_hash_map<antlr4::tree::ParseTree*, V> annotations_;
};

}  // namespace pvn_parsing
}  // namespace patinon

#endif  // BASE_DIR_PATINON_EXPLORATORY_ABC_SKIM_ENHANCED_PARSE_TREE_PROPERTY_H_
