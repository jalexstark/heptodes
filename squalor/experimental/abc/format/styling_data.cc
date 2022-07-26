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

#include <cstddef>
#include <string>
#include <utility>

#include "base_dir/absl/container/flat_hash_map.h"
#include "base_dir/absl/memory/memory.h"
#include "base_dir/absl/strings/string_view.h"
#include "base_dir/patinon/exploratory/abc/format/base_handler.h"
#include "base_dir/patinon/exploratory/abc/grammys/PvnLexer.h"
#include "base_dir/patinon/exploratory/abc/grammys/QvlParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/SvtParser.h"

namespace patinon {
namespace pvn_parsing {

namespace util {

// IMPORTANT: The actual content must be static, because string views get
// passed around.
//
// In the future this should go into an html file so that the content can be
// linted, then imported as embedded data and chopped up, such as with
// BEGIN_HEAD_START... END_HEAD_START.
std::unique_ptr<FormatContentMap> CreateHtmlPairMap() {
  std::unique_ptr<FormatContentMap> new_map =
      absl::MakeUnique<FormatContentMap>();
  constexpr const char kHtmlHeadStart[] =
      "<!DOCTYPE html>\n"
      "<html xmlns=\"http://www.w3.org/1999/xhtml\" lang=\"\" xml:lang=\"\">\n"
      "<head>\n"
      "  <meta charset=\"utf-8\" />\n"
      "  <meta name=\"generator\" content=\"salient\" />\n"
      "  <link "
      "href=\"https://fonts.googleapis.com/"
      "css?family=Gentium+Basic:400,400i,700|IBM+Plex+Mono:400,400i,600,600i|"
      "IBM+Plex:400,400i,600,600i&"
      "display=swap&subset=latin-ext\" rel=\"stylesheet\">\n"
      "<link rel=\"stylesheet\" href=\"salient-basic.css\">\n"
      "  <title>$0</title>\n"
      "  <style>\n"
      "  </style>\n"
      "</head>\n"
      "<body>\n"
      "<header>\n"
      "<h1 class=\"title\">$0</h1>\n"
      "</header>\n";
  constexpr const char kHtmlHeadFinish[] =
      "</body>\n"
      "</html>\n";

  using util::CustomCategoryId;
  using util::MakeContentMapKey;
  using util::ManagedStringViewPair;
  using util::ManagementStyle;
  using util::RuleCategory;
  new_map->emplace(MakeContentMapKey(RuleCategory::kCustom,
                                     CustomCategoryId::kDocumentOuter),
                   ManagedStringViewPair(ManagementStyle::kStatic,
                                         kHtmlHeadStart, kHtmlHeadFinish));
  new_map->emplace(
      MakeContentMapKey(RuleCategory::kCustom,
                        CustomCategoryId::kDocumentSection),
      ManagedStringViewPair(ManagementStyle::kTemporary,
                            "<section id='$1' class='level$0'>\n", ""));
  new_map->emplace(MakeContentMapKey(RuleCategory::kCustom,
                                     CustomCategoryId::kDocumentHeading),
                   ManagedStringViewPair(ManagementStyle::kTemporary,
                                         "<H$0>$1</H$0>\n", ""));

  new_map->emplace(
      MakeContentMapKey(RuleCategory::kCustom, CustomCategoryId::kTocEnclosure),
      ManagedStringViewPair(
          ManagementStyle::kTemporary,
          "<nav class='toc'><div class='text-narrowing'><span "
          "class='sidepiece'>",
          // "<div class='toc-bordered'>\n"
          "\n</span></div></nav>\n"));
  new_map->emplace(
      MakeContentMapKey(RuleCategory::kCustom, CustomCategoryId::kTocList),
      ManagedStringViewPair(ManagementStyle::kTemporary, "\n$0<ul>",
                            "\n$0</ul>"));
  new_map->emplace(
      MakeContentMapKey(RuleCategory::kCustom, CustomCategoryId::kTocItem),
      ManagedStringViewPair(ManagementStyle::kTemporary,
                            "\n$0<li><a target='_self' href='#$1'>$2</a>",
                            "</li>"));
  // new_map->emplace(
  //     MakeContentMapKey(RuleCategory::kCustom, CustomCategoryId::kListOuter),
  //     ManagedStringViewPair(ManagementStyle::kTemporary, "\n<ul class='$0
  //     $1'>",
  //                           "\n</ul>"));
  new_map->emplace(
      MakeContentMapKey(RuleCategory::kCustom, CustomCategoryId::kItemInner),
      ManagedStringViewPair(ManagementStyle::kTemporary, "<li class='$0'>",
                            "\n</li>"));

  new_map->emplace(
      MakeContentMapKey(RuleCategory::kSalientParser, SvtParser::RulePara),
      ManagedStringViewPair(ManagementStyle::kTemporary, "<p>\n", "\n</p>\n"));

  new_map->emplace(
      MakeContentMapKey(RuleCategory::kSalientParser,
                        SvtParser::RuleListItemParaPiece),
      ManagedStringViewPair(ManagementStyle::kTemporary, "<p>\n", "\n</p>\n"));

  new_map->emplace(
      MakeContentMapKey(RuleCategory::kLexerToken, PvnLexer::DOUBLE_BOLD_OPEN),
      ManagedStringViewPair(ManagementStyle::kTemporary, "<b>", "</b>"));
  new_map->emplace(
      MakeContentMapKey(RuleCategory::kLexerToken, PvnLexer::DOUBLE_EMPH_OPEN),
      ManagedStringViewPair(ManagementStyle::kTemporary, "<em>", "</em>"));

  new_map->emplace(
      MakeContentMapKey(RuleCategory::kCustom,
                        CustomCategoryId::kListSimpleItemClass),
      ManagedStringViewPair(ManagementStyle::kTemporary, "item-simple", ""));
  new_map->emplace(
      MakeContentMapKey(RuleCategory::kCustom,
                        CustomCategoryId::kListCompactItemClass),
      ManagedStringViewPair(ManagementStyle::kTemporary, "item-compact", ""));
  new_map->emplace(
      MakeContentMapKey(RuleCategory::kCustom,
                        CustomCategoryId::kListBlockItemClass),
      ManagedStringViewPair(ManagementStyle::kTemporary, "item-spacious", ""));
  new_map->emplace(
      MakeContentMapKey(RuleCategory::kCustom,
                        CustomCategoryId::kListCompactListClass),
      ManagedStringViewPair(ManagementStyle::kTemporary, "list-compact", ""));
  new_map->emplace(
      MakeContentMapKey(RuleCategory::kCustom,
                        CustomCategoryId::kListBlockListClass),
      ManagedStringViewPair(ManagementStyle::kTemporary, "list-spacious", ""));
  new_map->emplace(
      MakeContentMapKey(RuleCategory::kCustom,
                        CustomCategoryId::kListAttachedClass),
      ManagedStringViewPair(ManagementStyle::kTemporary, "attached-list", ""));
  new_map->emplace(
      MakeContentMapKey(RuleCategory::kCustom,
                        CustomCategoryId::kListDetachedClass),
      ManagedStringViewPair(ManagementStyle::kTemporary, "detached-list", ""));
  new_map->emplace(MakeContentMapKey(RuleCategory::kLexerToken,
                                     PvnLexer::PSEUDO_LIST_BULLET),
                   ManagedStringViewPair(
                       ManagementStyle::kTemporary,
                       "\n<ul class='$0 $1 bullet-unnumbered'>", "\n</ul>"));
  new_map->emplace(
      MakeContentMapKey(RuleCategory::kLexerToken,
                        PvnLexer::PSEUDO_LIST_ARABIC),
      ManagedStringViewPair(ManagementStyle::kTemporary,
                            "\n<ol class='$0 $1 arabic-numbered'>", "\n</ol>"));
  new_map->emplace(MakeContentMapKey(RuleCategory::kLexerToken,
                                     PvnLexer::PSEUDO_LIST_LOWER_ALPHA),
                   ManagedStringViewPair(
                       ManagementStyle::kTemporary,
                       "\n<ol class='$0 $1 lower-alpha-numbered'>", "\n</ol>"));
  new_map->emplace(MakeContentMapKey(RuleCategory::kLexerToken,
                                     PvnLexer::PSEUDO_LIST_UPPER_ALPHA),
                   ManagedStringViewPair(
                       ManagementStyle::kTemporary,
                       "\n<ol class='$0 $1 upper-alpha-numbered'>", "\n</ol>"));
  new_map->emplace(MakeContentMapKey(RuleCategory::kLexerToken,
                                     PvnLexer::PSEUDO_LIST_LOWER_ROMAN),
                   ManagedStringViewPair(
                       ManagementStyle::kTemporary,
                       "\n<ol class='$0 $1 lower-roman-numbered'>", "\n</ol>"));
  new_map->emplace(MakeContentMapKey(RuleCategory::kLexerToken,
                                     PvnLexer::PSEUDO_LIST_UPPER_ROMAN),
                   ManagedStringViewPair(
                       ManagementStyle::kTemporary,
                       "\n<ol class='$0 $1 upper-roman-numbered'>", "\n</ol>"));

  return new_map;
}

const absl::flat_hash_map<size_t, int>& SpecificAnyWhitespaceMap() {
  static const auto& specific_any_whitespace_map =
      *new absl::flat_hash_map<size_t, int>({
          // Quarrel.
          {PvnLexer::COLON, 0},
          {PvnLexer::POPEN, 0},
          // Salient.
          {PvnLexer::QUALIFIER_OPEN, 0},
          {PvnLexer::INDENT_CONTINUATION, 2},
          {PvnLexer::ITEM_START_FIRST, 2},
          {PvnLexer::LIST_BREAK_ACTUAL, 2},
          {PvnLexer::ITEM_START_SUCCEEDING, 2},
          {PvnLexer::EXTRA_ORDINARY_CHAIN, 0},
          {PvnLexer::SINGLY_ORDINARY, 0},
          // , {PvnLexer::BLANK_LINE, 0}
      });

  return specific_any_whitespace_map;
}

const absl::flat_hash_map<size_t, int>& AnySpecificWhitespaceMap() {
  static const auto& any_specific_whitespace_map =
      *new absl::flat_hash_map<size_t, int>({// Quarrel.
                                             {PvnLexer::PCLOSE, 0},
                                             // Salient.
                                             {PvnLexer::QUALIFIER_CLOSE, 0}});

  return any_specific_whitespace_map;
}

const absl::flat_hash_map<std::pair<size_t, size_t>, int>&
SpecificSpecificWhitespaceMap() {
  static const auto& specific_specific_whitespace_map =
      *new absl::flat_hash_map<std::pair<size_t, size_t>, int>({
          {{PvnLexer::CLOSE_STMT, PvnLexer::CLOSE_STMT}, 2},
          {{PvnLexer::Q_IDENTIFIER, PvnLexer::POPEN}, 0},
          {{PvnLexer::EXTRA_ORDINARY_CHAIN, PvnLexer::EXTRA_ORDINARY_CHAIN}, 1},
      });

  return specific_specific_whitespace_map;
}

}  // namespace util

}  // namespace pvn_parsing
}  // namespace patinon
