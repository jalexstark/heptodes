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

#ifndef BASE_DIR_PATINON_EXPLORATORY_ABC_FORMAT_BASE_HANDLER_H_
#define BASE_DIR_PATINON_EXPLORATORY_ABC_FORMAT_BASE_HANDLER_H_

#include <sys/types.h>

#include <cstddef>
#include <iosfwd>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "base_dir/absl/container/flat_hash_map.h"
#include "base_dir/absl/strings/string_view.h"
#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/grammys/PvnLexer.h"
#include "base_dir/patinon/exploratory/abc/skim/enhanced_parse_tree_property.h"
#include "base_dir/patinon/exploratory/abc/skim/summarizing.h"
#include "base_dir/patinon/exploratory/misc/check_macros.h"

namespace patinon {
namespace pvn_parsing {

static constexpr int kNormalMaxLineChars = 96;
static constexpr int kTextualRightCommentColumn = 72;
static constexpr int kExtendedMaxLineChars = 120;
static constexpr int kColumnCommentWidth =
    kExtendedMaxLineChars - kTextualRightCommentColumn;

namespace util {

enum class ManagementStyle {
  kNone,
  kStatic,
  kTemporary,
};

class ManagedStringViewPair
    : public std::pair<absl::string_view, absl::string_view> {
 public:
  // Make inline?
  ManagedStringViewPair(ManagementStyle style, absl::string_view first_view,
                        absl::string_view second_view) {
    if (style == ManagementStyle::kStatic) {
      first = first_view;
      second = second_view;
    } else {
      opt_first_string = absl::make_unique<std::string>(first_view);
      opt_second_string = absl::make_unique<std::string>(second_view);
      first = absl::string_view(*opt_first_string);
      second = absl::string_view(*opt_second_string);
    }
  }
  ManagedStringViewPair()
      : ManagedStringViewPair(ManagementStyle::kStatic, kDummyPairFirst,
                              kDummyPairSecond) {}

 private:
  static constexpr const char kDummyPairFirst[] = "";
  static constexpr const char kDummyPairSecond[] =
      "USE OF MISSING CONTENT MAP ENTRY";

  std::unique_ptr<std::string> opt_first_string;
  std::unique_ptr<std::string> opt_second_string;
};

enum class RuleCategory {
  kNone = 0,
  kCustom,
  kLexerToken,
  kQuarrelParser,
  kSalientParser,
  kRostrumParser,
};

enum class CustomCategoryId {
  kInvalid,
  kDocumentOuter,
  kDocumentSection,
  kDocumentHeading,
  kTocEnclosure,
  kTocList,
  kTocItem,
  // kListOuter,
  kItemInner,
  kListSimpleItemClass,
  kListCompactItemClass,
  kListBlockItemClass,
  kListCompactListClass,
  kListBlockListClass,
  kListAttachedClass,
  kListDetachedClass,
};

inline std::pair<RuleCategory, int> MakeContentMapKey(RuleCategory category,
                                                      int token_or_rule) {
  PVN_DCHECK_NE(category, RuleCategory::kCustom);
  return std::make_pair(category, token_or_rule);
}

inline std::pair<RuleCategory, int> MakeContentMapKey(
    RuleCategory category, CustomCategoryId custom_id) {
  PVN_DCHECK_EQ(category, RuleCategory::kCustom);
  return std::make_pair(category, static_cast<const int>(custom_id));
}

class FormatContentMap
    : public absl::flat_hash_map<std::pair<RuleCategory, int>,
                                 ManagedStringViewPair> {
 public:
  // This getter drops the ownership management and adds optional's has_value(),
  // cleaning up C++'s unordered hash map interface.
  std::optional<const std::pair<absl::string_view, absl::string_view>> Get(
      const std::pair<RuleCategory, int> key) const {
    const auto finding = this->find(key);
    if (finding == this->end()) {
      return std::nullopt;
    } else {
      return std::optional<
          const std::pair<absl::string_view, absl::string_view>>(
          finding->second);
    }
  }
};

std::unique_ptr<FormatContentMap> CreateHtmlPairMap();

const absl::flat_hash_map<size_t, int>& SpecificAnyWhitespaceMap();

const absl::flat_hash_map<size_t, int>& AnySpecificWhitespaceMap();

const absl::flat_hash_map<std::pair<size_t, size_t>, int>&
SpecificSpecificWhitespaceMap();

}  // namespace util

struct OutputPiece {
  string text;
  int token_index = -1;
  CoarseProperties coarse_properties_before;
  CoarseProperties coarse_properties_at;
  size_t token_type = PvnLexer::NONE_TOKEN;
};

struct WhitespaceMaps {
  const absl::flat_hash_map<size_t, int>& specific_any_whitespace_map;
  const absl::flat_hash_map<size_t, int>& any_specific_whitespace_map;
  const absl::flat_hash_map<std::pair<size_t, size_t>, int>&
      specific_specific_whitespace_map;
};

struct InterModeFormatting {
  int one_line_max_length = 0;
  int multi_line_max_length = 0;
  bool append_newline = true;
  int residual_column_position = 0;
  int contextual_indent_column =
      0;  // For example, the Quarrel indent for double-semi Salient comments.
  ssize_t destination_token_index = -1;
};

// We output a newline and process pending output for two basic reasons: (a)
// a statement ends, or (b) we encounter something like a "class" declaration.
//
// Note that many of these patterns can be processed directly in the
// visitTerminal() method, and the flush reason is not needed to retain state.
enum class FlushReason {
  kNone,            // Many reasons for flushing are processed immediately.
  kCloseStatement,  // Close statements can be merged, so flushing is deferred
                    // until the first non-close token.
  kOpenPattern,     // "Top" level statements such as class declarations might
                 // need to be processed on the syntactic entry rather than the
                 // token itself.
};

struct OutputHandler {
  OutputHandler(std::ofstream& out_stream_x,
                std::unique_ptr<AltParseTreeProperty<CoarseProperties>>
                    coarse_properties_x,
                WhitespaceMaps whitespace_maps_x,
                std::unique_ptr<util::FormatContentMap> content_pair_map_x,
                antlr4::CommonTokenStream* tokens_x)
      : out_stream(out_stream_x),
        output_pieces(0),
        pending_char_count(0),
        coarse_properties(std::move(coarse_properties_x)),
        prevailing_properties(),
        whitespace_maps(whitespace_maps_x),
        content_pair_map(std::move(content_pair_map_x)),
        tokens(tokens_x),
        current_section_depth(0),
        inter_mode_formatting(),
        entry_genre_state(),
        flush_at_next_token(FlushReason::kNone) {}

  std::ofstream& out_stream;
  std::vector<OutputPiece> output_pieces;
  int pending_char_count;
  std::unique_ptr<AltParseTreeProperty<CoarseProperties>> coarse_properties;
  CoarseProperties prevailing_properties;
  WhitespaceMaps whitespace_maps;
  std::unique_ptr<util::FormatContentMap> content_pair_map;
  antlr4::CommonTokenStream* tokens;
  int current_section_depth;
  InterModeFormatting inter_mode_formatting;
  WalkerTransition entry_genre_state;
  FlushReason flush_at_next_token;
};

// This class serves to add future (probably pure) virtual methods.
class MiddleHandler : public OutputHandler {
 public:
  MiddleHandler(std::ofstream& out_stream_x,
                std::unique_ptr<AltParseTreeProperty<CoarseProperties>>
                    coarse_properties_x,
                WhitespaceMaps whitespace_maps_x,
                std::unique_ptr<util::FormatContentMap> content_pair_map_x,
                antlr4::CommonTokenStream* tokens_x)
      : OutputHandler(out_stream_x, std::move(coarse_properties_x),
                      whitespace_maps_x, std::move(content_pair_map_x),
                      tokens_x) {}
};

struct CommonReformatter {
  // Append text, which in Quarrel is all the text in the token, but in Salient
  // will be part of the token if it is split up.
  static void AppendPiece(string text, size_t token_type, ssize_t token_index,
                          antlr4::tree::ParseTree* ctx,
                          OutputHandler* output_handler);
  static inline void AppendPiece(string text, antlr4::tree::TerminalNode* ctx,
                                 OutputHandler* output_handler) {
    AppendPiece(text, ctx->getSymbol()->getType(),
                ctx->getSymbol()->getTokenIndex(), ctx, output_handler);
  }
  static inline void AppendPieceFirstDecendant(string text,
                                               antlr4::tree::ParseTree* ctx,
                                               OutputHandler* output_handler) {
    antlr4::tree::ParseTree* traversal_node = ctx;
    while (!traversal_node->children.empty()) {
      traversal_node = traversal_node->children.front();
    }
    auto descendant_node =
        dynamic_cast<antlr4::tree::TerminalNode*>(traversal_node);
    if (descendant_node == nullptr) {
      PVN_CHECK_NE(descendant_node, nullptr);
      return;
    }
    AppendPiece(text, descendant_node->getSymbol()->getType(),
                descendant_node->getSymbol()->getTokenIndex(), ctx,
                output_handler);
  }
  static inline void AppendPieceLastDecendant(string text,
                                              antlr4::tree::ParseTree* ctx,
                                              OutputHandler* output_handler) {
    antlr4::tree::ParseTree* traversal_node = ctx;
    while (!traversal_node->children.empty()) {
      traversal_node = traversal_node->children.back();
    }
    auto descendant_node =
        dynamic_cast<antlr4::tree::TerminalNode*>(traversal_node);
    if (descendant_node == nullptr) {
      PVN_CHECK_NE(descendant_node, nullptr);
      return;
    }
    AppendPiece(text, descendant_node->getSymbol()->getType(),
                descendant_node->getSymbol()->getTokenIndex(), ctx,
                output_handler);
  }

  static void DropFrontN(int n, OutputHandler* output_handler);
};

}  // namespace pvn_parsing
}  // namespace patinon

#endif  // BASE_DIR_PATINON_EXPLORATORY_ABC_FORMAT_BASE_HANDLER_H_
