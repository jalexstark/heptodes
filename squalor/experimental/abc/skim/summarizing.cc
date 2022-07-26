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

#include "base_dir/patinon/exploratory/abc/skim/summarizing.h"

#include <string>
#include <utility>
#include <vector>

#include "base_dir/absl/container/flat_hash_map.h"
#include "base_dir/absl/memory/memory.h"
#include "base_dir/absl/strings/match.h"
#include "base_dir/absl/strings/str_replace.h"
#include "base_dir/absl/strings/str_split.h"
#include "base_dir/absl/strings/string_view.h"
#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/grammys/SvtParser.h"
#include "base_dir/patinon/exploratory/abc/skim/coarse_listeners.h"

namespace patinon {
namespace pvn_parsing {

namespace util {

string MakeAnchor(string text) {
  absl::StrReplaceAll({{" ", "_"}, {",", ""}, {"\"", ""}, {"'", ""}}, &text);
  return text;
}

Heading GetHeading(SvtParser::HeadingContext* ctx,
                   CoarseSkimSalientListener* skimmer) {
  Heading heading;
  heading.level = 0;
  heading.terminal_node = ctx->TITLE();
  if (heading.terminal_node == nullptr) {
    heading.terminal_node = ctx->HEADING();
    heading.level = 1;
  }
  if (heading.terminal_node == nullptr) {
    heading.terminal_node = ctx->TOC();
    heading.level = Heading::kTocHeadingLevel;
  }

  if (heading.terminal_node == nullptr) {
    return heading;
  }

  heading.line_number = heading.terminal_node->getSymbol()->getLine();

  if (heading.level == 1) {
    // auto* tn = ;
    string heading_start = heading.terminal_node->getText();
    int level_adjust = absl::StartsWith(heading_start, "=") ? 3 : 1;
    heading.level = heading_start.size() - level_adjust;
  }

  if (heading.level != Heading::kTocHeadingLevel) {
    antlr4::ParserRuleContext* paraBlock =
        ctx->headingContent()->paraBlockText();
    heading.heading_text = GetTrimmedAllTokens(paraBlock);
  } else {
    heading.heading_text = "Table of Contents";
  }

  heading.anchor_id =
      skimmer->MakeUniqueAnchor(MakeAnchor(heading.heading_text));

  antlr4::tree::TerminalNode* qualifier_node = nullptr;
  auto qualifier = ctx->qualifier();
  if (qualifier != nullptr) {
    qualifier_node = qualifier->LOCATION_AND_QUALIFIERS();
  }

  if (qualifier_node != nullptr) {
    std::vector<std::string> qualifier_split = absl::StrSplit(
        qualifier_node->getText(), absl::ByAnyChar(" \t\n"), absl::SkipEmpty());

    heading.qualifiers.resize(qualifier_split.size());
    for (int i = 0; i < qualifier_split.size(); ++i) {
      absl::string_view qualifier = qualifier_split[i];
      auto separator_pos = qualifier.find_first_of("#=");
      string left_side;
      if (separator_pos == absl::string_view::npos) {
        heading.qualifiers[i].left_side = string(qualifier);
      } else {
        const char separator_char = qualifier.at(separator_pos);
        heading.qualifiers[i].separator = separator_char;
        if (separator_pos == 0) {
          // For now assume that (separator_char == '#').
          left_side = "anchor";
        } else {
          left_side = string(qualifier.substr(0, separator_pos));
        }
        heading.qualifiers[i].left_side = left_side;
        if ((separator_pos + 1) < qualifier.size()) {
          heading.qualifiers[i].right_side = string(qualifier.substr(
              separator_pos + 1, qualifier.size() - (separator_pos + 1)));
        }
      }
      heading.left_side_to_qualifier_index.insert(std::make_pair(left_side, i));
    }
  }

  const auto anchor_iter = heading.left_side_to_qualifier_index.find("anchor");
  if (anchor_iter != heading.left_side_to_qualifier_index.end()) {
    heading.anchor_id = heading.qualifiers[anchor_iter->second].right_side;
  }

  heading.heading_number = skimmer->GetAndIncrementHeadingCounter();
  return heading;
}

}  // namespace util

}  // namespace pvn_parsing
}  // namespace patinon
