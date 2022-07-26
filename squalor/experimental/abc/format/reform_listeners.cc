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

#include "base_dir/patinon/exploratory/abc/format/reform_listeners.h"

#include <cstddef>
#include <iostream>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "base_dir/absl/base/port.h"
#include "base_dir/absl/container/flat_hash_map.h"
#include "base_dir/absl/memory/memory.h"
#include "base_dir/absl/strings/numbers.h"
#include "base_dir/absl/strings/str_format.h"
#include "base_dir/absl/strings/str_split.h"
#include "base_dir/absl/strings/string_view.h"
#include "base_dir/absl/strings/string_view_utils.h"
#include "base_dir/absl/strings/substitute.h"
#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/format/base_handler.h"
#include "base_dir/patinon/exploratory/abc/format/reform_handlers.h"
#include "base_dir/patinon/exploratory/abc/grammys/PvnLexer.h"
#include "base_dir/patinon/exploratory/abc/grammys/QvlParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/SvtParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
#include "base_dir/patinon/exploratory/abc/skim/coarse_listeners.h"
#include "base_dir/patinon/exploratory/abc/skim/enhanced_parse_tree_property.h"
#include "base_dir/patinon/exploratory/abc/skim/summarizing.h"

namespace patinon {
namespace pvn_parsing {

namespace util {

void OutputToc(const SummarizerResults& summarizer_results,
               const Heading& toc_heading, OutputHandler* handler) {
  std::ofstream& out_stream = handler->out_stream;
  const auto toc_enclosure = handler->content_pair_map->Get(MakeContentMapKey(
      util::RuleCategory::kCustom, util::CustomCategoryId::kTocEnclosure));
  const auto toc_list = handler->content_pair_map->Get(MakeContentMapKey(
      util::RuleCategory::kCustom, util::CustomCategoryId::kTocList));
  const auto toc_item = handler->content_pair_map->Get(MakeContentMapKey(
      util::RuleCategory::kCustom, util::CustomCategoryId::kTocItem));
  PVN_CHECK(toc_enclosure.has_value());
  PVN_CHECK(toc_list.has_value());
  PVN_CHECK(toc_item.has_value());

  constexpr int default_toc_depth = 3;
  out_stream << toc_enclosure->first;

  int toc_depth = default_toc_depth;
  const auto depth_iter =
      toc_heading.left_side_to_qualifier_index.find("toc_depth");
  if (depth_iter != toc_heading.left_side_to_qualifier_index.end()) {
    if (!absl::SimpleAtoi(toc_heading.qualifiers[depth_iter->second].right_side,
                          &toc_depth)) {
      toc_depth = default_toc_depth;
    }
  }

  int current_nesting = 0;
  std::vector<bool> pending_item_at_level(toc_depth, false);
  for (const auto& h : summarizer_results.heading_vector) {
    if ((h.level < 0) || (h.level > toc_depth)) {
      continue;
    }
    do {
      if ((current_nesting >= h.level) &&
          pending_item_at_level[current_nesting]) {
        out_stream << toc_item->second;
        pending_item_at_level[h.level] = false;
      }
      if (current_nesting < h.level) {
        out_stream << absl::Substitute(toc_list->first,
                                       std::string(3 * current_nesting, ' '));
        ++current_nesting;
      }
      if (current_nesting > h.level) {
        --current_nesting;
        out_stream << absl::Substitute(toc_list->second,
                                       std::string(3 * current_nesting, ' '));
      }
    } while (current_nesting != h.level);

    if (h.level > 0) {
      out_stream << absl::Substitute(toc_item->first,
                                     std::string(3 * current_nesting, ' '),
                                     h.anchor_id, h.heading_text);
      pending_item_at_level[h.level] = true;
    }
  }
  out_stream << toc_enclosure->second;
}

}  // namespace util

void QuarrelReformatListener::MaybeOutputLines(
    antlr4::tree::TerminalNode* ctx) {
  bool grammar_flush = false;
  switch (handler_->flush_at_next_token) {
    case FlushReason::kCloseStatement:
      if (ctx->getSymbol()->getType() != PvnLexer::CLOSE_STMT) {
        grammar_flush = true;
      }
      break;
    case FlushReason::kOpenPattern:
      grammar_flush = true;
      break;
    default:
      if (ctx->getSymbol()->getType() == PvnLexer::CLOSE_STMT) {
        grammar_flush = true;
      }
      break;
  }

  handler_->OutputLines(grammar_flush, WalkerTransition());
}

void QuarrelReformatListener::visitTerminal(antlr4::tree::TerminalNode* ctx) {
  MaybeOutputLines(ctx);

  if (ctx->getSymbol()->getType() == PvnLexer::EOF) {
    return;
  }

  CommonReformatter::AppendPiece(ctx->getText(), ctx, handler_.get());

  handler_->prevailing_properties = handler_->coarse_properties->get(ctx);
}

void QuarrelReformatListener::enterStatement(QvlParser::StatementContext* ctx) {
  const int pending_pieces = handler_->output_pieces.size();
  if (pending_pieces != 0) {
    handler_->flush_at_next_token = FlushReason::kOpenPattern;
  }
}

void QuarrelReformatListener::exitEveryRule(antlr4::ParserRuleContext* ctx) {
  if (!handler_->output_pieces.empty()) {
    switch (ctx->getRuleIndex()) {
      case QvlParser::RuleCloseStmt: {
        handler_->flush_at_next_token = FlushReason::kCloseStatement;
        break;
      }
      case QvlParser::RuleOpenStmt: {
        handler_->flush_at_next_token = FlushReason::kOpenPattern;
        break;
      }
      default:
        break;
    }
    handler_->output_pieces.back().coarse_properties_at =
        handler_->coarse_properties->get(ctx);
  }
  handler_->prevailing_properties = handler_->coarse_properties->get(ctx);
}

void SalientReformatListener::MaybeOutputLines(
    antlr4::tree::TerminalNode* ctx) {
  // FIRST. Flush pending output when required.
  bool grammar_flush = false;

  switch (handler_->flush_at_next_token) {
    case FlushReason::kCloseStatement:
      grammar_flush = true;
      break;
    case FlushReason::kOpenPattern:
      grammar_flush = true;
      break;
    default:
      break;
  }

  switch (ctx->getSymbol()->getType()) {
    case PvnLexer::PENDING_ENTER_CODE:
    case PvnLexer::NEWLINE_ENTER_CODE:  // Probably unnecessary.
      if (GetHandler()->entry_genre_state.destination_subgenre ==
          TextualSubGenre::kMaster) {
        grammar_flush = true;
      }
      break;
    case PvnLexer::LEAVE_TEXTUAL:
      if (GetHandler()->entry_genre_state.destination_subgenre !=
          TextualSubGenre::kNone) {
        grammar_flush = true;
      }
      break;
    default:
      break;
  }

  handler_->OutputLines(grammar_flush, WalkerTransition());
}

void SalientReformatListener::visitTerminal(antlr4::tree::TerminalNode* ctx) {
  MaybeOutputLines(ctx);

  size_t symbol_type = ctx->getSymbol()->getType();
  if (ctx->getSymbol()->getType() == PvnLexer::EOF) {
    return;
  }

  const CoarseProperties subsequent_properties =
      handler_->coarse_properties->get(ctx);

  // It does not seem necessary to check for NEWLINE_ENTER_CODE.
  bool skip_for_transition =
      (((symbol_type == PvnLexer::ENTER_TEXTUAL) ||
        (symbol_type == PvnLexer::LEAVE_CODE) ||
        (symbol_type == PvnLexer::LEAVE_TEXTUAL)) &&
       (subsequent_properties.sub_genre != TextualSubGenre::kMaster));

  // std::string reformed_text;
  switch (symbol_type) {
    case PvnLexer::SINGLE_NEWLINE:
    case PvnLexer::MULTI_NEWLINE:
    case PvnLexer::SVT_DEDENT:
      break;
    case PvnLexer::SINGLY_ORDINARY:
      CommonReformatter::AppendPiece(ctx->getText(), ctx, handler_.get());
      break;
    case PvnLexer::EXTRA_ORDINARY_CHAIN: {
      // reformed_text =
      // absl::StrJoin(absl::StrSplit(ctx->getText(), absl::ByAnyChar("
      // \t"),
      //                              absl::SkipEmpty()),
      //               "");
      const auto string_text = ctx->getText();
      const auto string_view_text = absl::string_view(string_text);
      auto string_vec = absl::StrSplit(string_view_text, absl::ByAnyChar(" \t"),
                                       absl::SkipEmpty());
      // Really chains should not have initial or terminal whitespace, because
      // that confuses formatting.
      const bool initial_whitespace =
          string_view_text.find_first_not_of(" \t") != 0;
      const bool termminal_whitespace =
          string_view_text.find_last_not_of(" \t") !=
          (string_view_text.size() - 1);

      const string empty_string;
      if (initial_whitespace) {
        CommonReformatter::AppendPiece(empty_string, ctx, handler_.get());
      }
      for (const auto& s : string_vec) {
        CommonReformatter::AppendPiece(string(s), ctx, handler_.get());
      }
      if (termminal_whitespace) {
        CommonReformatter::AppendPiece(empty_string, ctx, handler_.get());
      }
      break;
    }
    default:
      if (skip_for_transition) {
        break;
      }
      string token_text(ctx->getText());  // Forced copy.
      absl::string_view sv_token_text(token_text);
      strings::RemoveLeadingWhitespace(&sv_token_text);
      if (!sv_token_text.empty()) {
        // reformed_text = std::string(sv_token_text);
        CommonReformatter::AppendPiece(string(sv_token_text), ctx,
                                       handler_.get());
      }
      break;
  }
  // if (!reformed_text.empty()) {
  //   CommonReformatter::AppendPiece(reformed_text, ctx, handler_.get());
  // }
  handler_->prevailing_properties = subsequent_properties;
}

void SalientReformatListener::enterEveryRule(antlr4::ParserRuleContext* ctx) {
  switch (ctx->getRuleIndex()) {
    case SvtParser::RuleHeading:
    case SvtParser::RulePara:
    case SvtParser::RuleListItem:  // XXX This might need expansion.
      GetHandler()->flush_at_next_token = FlushReason::kOpenPattern;
      break;
    default:
      break;
  }
}

void SalientReformatListener::exitEveryRule(antlr4::ParserRuleContext* ctx) {
  // This might be used for updating prevailing properties to subsequent
  // properties, but that seems to work more logically and consistently when
  // done only on terminals.

  // const CoarseProperties subsequent_properties =
  //     handler_->coarse_properties->get(ctx);
  // The extra condition here suggests that the handling of prevailing
  // properties is fundamentally flawed.
  // if (ctx->getRuleIndex() != SvtParser::RuleSalientInQuarrel) {
  //   handler_->prevailing_properties = subsequent_properties;
  // }
}

void QuarrelToHtmlListener::enterQuarrelTop(QvlParser::QuarrelTopContext* ctx) {
  if (summarizer_results_->outer_genre_state.destination_genre !=
      ParsingGenre::kQuarrel) {
    return;
  }
  const auto html_head = handler_->content_pair_map->Get(MakeContentMapKey(
      util::RuleCategory::kCustom, util::CustomCategoryId::kDocumentOuter));
  PVN_CHECK(html_head.has_value());

  string doc_title =
      summarizer_results_->title_heading_index ==
              SummarizerResults::kNoDetectedTitle
          ? "MISSING TITLE"
          : summarizer_results_
                ->heading_vector[summarizer_results_->title_heading_index]
                .heading_text;

  handler_->out_stream << absl::Substitute(html_head->first, doc_title);
}

void QuarrelToHtmlListener::exitQuarrelTop(QvlParser::QuarrelTopContext* ctx) {
  if (summarizer_results_->outer_genre_state.destination_genre !=
      ParsingGenre::kQuarrel) {
    return;
  }
  const auto html_head = handler_->content_pair_map->Get(MakeContentMapKey(
      util::RuleCategory::kCustom, util::CustomCategoryId::kDocumentOuter));
  PVN_CHECK(html_head.has_value());

  handler_->out_stream << html_head->second;
}

void QuarrelToHtmlListener::enterEveryRule(antlr4::ParserRuleContext* ctx) {
  const auto content_pair =
      GetHandler()->content_pair_map->Get(MakeContentMapKey(
          util::RuleCategory::kQuarrelParser, ctx->getRuleIndex()));

  // Rules with explicit handling should not mix with automatic content
  // insertion.
  switch (ctx->getRuleIndex()) {
    case QvlParser::RuleQuarrelTop:
      PVN_CHECK(!content_pair.has_value());
      return;
    default:
      break;
  }

  if (content_pair.has_value()) {
    GetHandler()->out_stream << content_pair->first;
  }
}

void QuarrelToHtmlListener::exitEveryRule(antlr4::ParserRuleContext* ctx) {
  const auto content_pair =
      GetHandler()->content_pair_map->Get(MakeContentMapKey(
          util::RuleCategory::kQuarrelParser, ctx->getRuleIndex()));

  if (content_pair.has_value()) {
    GetHandler()->out_stream << content_pair->second;
  }
}

void QuarrelToHtmlListener::visitTerminal(antlr4::tree::TerminalNode* ctx) {
  bool grammar_flush = true;
  size_t symbol_type = ctx->getSymbol()->getType();

  handler_->OutputLines(grammar_flush, WalkerTransition());

  switch (symbol_type) {
    case PvnLexer::EOF:
      break;
    default:
      string token_text(ctx->getText());  // Forced copy.
      absl::string_view sv_token_text(token_text);
      // strings::RemoveLeadingWhitespace(&sv_token_text);
      if (!sv_token_text.empty()) {
        CommonReformatter::AppendPiece(string(sv_token_text), ctx,
                                       handler_.get());
      }
      break;
  }
  if (false || (symbol_type == PvnLexer::CLOSE_STMT)) {
    CommonReformatter::AppendPiece("\n", ctx, handler_.get());
  }
  handler_->OutputLines(grammar_flush, WalkerTransition());
}

inline bool SalientToHtmlListener::DisableAutoSubtreeForRule(
    antlr4::ParserRuleContext* ctx) {
  // Some rules, like headings, handle the content generation for their subtree
  // and so the terminal nodes should not push output automatically.
  switch (ctx->getRuleIndex()) {
    case SvtParser::RuleHeading:
      return true;
    default:
      return false;
  }
}

void SalientToHtmlListener::enterSalientTop(SvtParser::SalientTopContext* ctx) {
  if (summarizer_results_->outer_genre_state.destination_genre !=
      ParsingGenre::kSalient) {
    return;
  }
  const auto html_head = handler_->content_pair_map->Get(MakeContentMapKey(
      util::RuleCategory::kCustom, util::CustomCategoryId::kDocumentOuter));
  PVN_CHECK(html_head.has_value());

  string doc_title =
      summarizer_results_->title_heading_index ==
              SummarizerResults::kNoDetectedTitle
          ? "MISSING TITLE"
          : summarizer_results_
                ->heading_vector[summarizer_results_->title_heading_index]
                .heading_text;

  handler_->out_stream << absl::Substitute(html_head->first, doc_title);
}

void SalientToHtmlListener::exitSalientTop(SvtParser::SalientTopContext* ctx) {
  if (summarizer_results_->outer_genre_state.destination_genre !=
      ParsingGenre::kSalient) {
    return;
  }

  handler_->CloseSectionsTo(0);

  const auto html_head = handler_->content_pair_map->Get(MakeContentMapKey(
      util::RuleCategory::kCustom, util::CustomCategoryId::kDocumentOuter));
  PVN_CHECK(html_head.has_value());
  handler_->out_stream << html_head->second;
}

void SalientToHtmlListener::enterHeading(SvtParser::HeadingContext* ctx) {
  // Headings are unusual, in that we handle entirely from the document
  // summarizer results.
  Heading heading =
      GetSummarizerResults()->heading_vector
          [GetSummarizerResults()->heading_indices[util::GetHeadingId(ctx)]];
  if (heading.level == Heading::kTocHeadingLevel) {
    util::OutputToc(*GetSummarizerResults(), heading, GetHandler());
  } else if (heading.level > 0) {
    GetHandler()->CloseSectionsTo(heading.level);

    const auto section_pair = handler_->content_pair_map->Get(MakeContentMapKey(
        util::RuleCategory::kCustom, util::CustomCategoryId::kDocumentSection));
    PVN_CHECK(section_pair.has_value());
    const auto heading_pair = handler_->content_pair_map->Get(MakeContentMapKey(
        util::RuleCategory::kCustom, util::CustomCategoryId::kDocumentHeading));
    PVN_CHECK(heading_pair.has_value());

    GetHandler()->out_stream << absl::Substitute(
        section_pair->first, heading.level, heading.anchor_id);
    GetHandler()->out_stream << absl::Substitute(
        heading_pair->first, heading.level, heading.heading_text);
  }
}

inline const absl::string_view ObtainListStyle(
    const SalientToHtmlHandler& handler,
    const SvtParser::EitherListContext& ctx) {
  using util::CustomCategoryId;
  CustomCategoryId style_id = CustomCategoryId::kInvalid;
  switch (ctx.list_compactness) {
    case SvtListCompactness::kCompact:
      style_id = CustomCategoryId::kListCompactListClass;
      break;
    case SvtListCompactness::kBlock:
      style_id = CustomCategoryId::kListBlockListClass;
      break;
    case SvtListCompactness::kNone:
    default:
      style_id = CustomCategoryId::kInvalid;
      break;
  }

  const auto content_pair = handler.content_pair_map->Get(
      MakeContentMapKey(util::RuleCategory::kCustom, style_id));
  PVN_CHECK(content_pair.has_value());
  return content_pair->first;
}

inline const absl::string_view ObtainListContext(
    const SalientToHtmlHandler& handler,
    const SvtParser::EitherListContext& ctx) {
  using util::CustomCategoryId;
  CustomCategoryId style_id = CustomCategoryId::kInvalid;
  switch (ctx.attachment) {
    case SvtListAttachment::kAttached:
      style_id = CustomCategoryId::kListAttachedClass;
      break;
    case SvtListAttachment::kDetached:
      style_id = CustomCategoryId::kListDetachedClass;
      break;
    case SvtListAttachment::kNone:
    default:
      style_id = CustomCategoryId::kInvalid;
      break;
  }

  const auto content_pair = handler.content_pair_map->Get(
      MakeContentMapKey(util::RuleCategory::kCustom, style_id));
  PVN_CHECK(content_pair.has_value());
  return content_pair->first;
}

inline const std::pair<absl::string_view, absl::string_view>
ObtainListNumbering(const SalientToHtmlHandler& handler,
                    const SvtParser::EitherListContext& ctx) {
  const auto content_pair = handler.content_pair_map->Get(MakeContentMapKey(
      util::RuleCategory::kLexerToken, ctx.list_type_pseudo_token));
  PVN_CHECK(content_pair.has_value());
  return content_pair.value();
}

void SalientToHtmlListener::enterEitherList(SvtParser::EitherListContext* ctx) {
  const std::string list_style = std::string(ObtainListStyle(*handler_, *ctx));
  const std::string contextual_style =
      std::string(ObtainListContext(*handler_, *ctx));

  const auto content_pair = ObtainListNumbering(*handler_, *ctx);
  handler_->out_stream << absl::Substitute(content_pair.first, contextual_style,
                                           list_style);
  // handler_->out_stream << content_pair.first;
}

void SalientToHtmlListener::exitEitherList(SvtParser::EitherListContext* ctx) {
  const auto content_pair = ObtainListNumbering(*handler_, *ctx);
  handler_->out_stream << content_pair.second;
}

inline const absl::string_view ObtainItemStyle(
    const SalientToHtmlHandler& handler,
    const SvtParser::ListItemContext& ctx) {
  using util::CustomCategoryId;
  CustomCategoryId style_id = CustomCategoryId::kInvalid;
  switch (ctx.item_compactness) {
    case SvtListCompactness::kSimple:
      style_id = CustomCategoryId::kListSimpleItemClass;
      break;
    case SvtListCompactness::kCompact:
      style_id = CustomCategoryId::kListCompactItemClass;
      break;
    case SvtListCompactness::kBlock:
      style_id = CustomCategoryId::kListBlockItemClass;
      break;
    case SvtListCompactness::kNone:
    default:
      style_id = CustomCategoryId::kInvalid;
      break;
  }

  const auto content_pair = handler.content_pair_map->Get(
      MakeContentMapKey(util::RuleCategory::kCustom, style_id));
  PVN_CHECK(content_pair.has_value());
  return content_pair->first;
}

void SalientToHtmlListener::enterListItem(SvtParser::ListItemContext* ctx) {
  const auto content_pair = handler_->content_pair_map->Get(MakeContentMapKey(
      util::RuleCategory::kCustom, util::CustomCategoryId::kItemInner));
  PVN_CHECK(content_pair.has_value());

  const string item_style = std::string(ObtainItemStyle(*handler_, *ctx));

  handler_->out_stream << absl::Substitute(content_pair->first, item_style);
}

void SalientToHtmlListener::exitListItem(SvtParser::ListItemContext* ctx) {
  const auto content_pair = handler_->content_pair_map->Get(MakeContentMapKey(
      util::RuleCategory::kCustom, util::CustomCategoryId::kItemInner));
  PVN_CHECK(content_pair.has_value());
  handler_->out_stream << content_pair->second;
}

void SalientToHtmlListener::enterLinearContent(
    SvtParser::LinearContentContext* ctx) {
  if (ctx->content_opening == SvtParser::NONE_TOKEN) {
    return;
  }
  const auto content_pair = handler_->content_pair_map->Get(
      MakeContentMapKey(util::RuleCategory::kLexerToken, ctx->content_opening));
  PVN_CHECK(content_pair.has_value());
  handler_->out_stream << content_pair->first;
}
void SalientToHtmlListener::exitLinearContent(
    SvtParser::LinearContentContext* ctx) {
  if (ctx->content_opening == SvtParser::NONE_TOKEN) {
    return;
  }
  const auto content_pair = handler_->content_pair_map->Get(
      MakeContentMapKey(util::RuleCategory::kLexerToken, ctx->content_opening));
  PVN_CHECK(content_pair.has_value());
  handler_->out_stream << content_pair->second;
}

void SalientToHtmlListener::enterEveryRule(antlr4::ParserRuleContext* ctx) {
  const auto content_pair =
      GetHandler()->content_pair_map->Get(MakeContentMapKey(
          util::RuleCategory::kSalientParser, ctx->getRuleIndex()));

  // Rules with explicit handling should not mix with automatic content
  // insertion.
  switch (ctx->getRuleIndex()) {
    case SvtParser::RuleSalientTop:
    case SvtParser::RuleHeading:
    case SvtParser::RuleParaBlock:
      PVN_CHECK(!content_pair.has_value());
      break;
    default:
      break;
  }

  if (content_pair.has_value()) {
    GetHandler()->out_stream << content_pair->first;
  }

  // Disable content generation in subtree if this rule handles it.
  if (DisableAutoSubtreeForRule(ctx)) {
    ++disableNormalSubtreeContent;
  }
}

void SalientToHtmlListener::exitEveryRule(antlr4::ParserRuleContext* ctx) {
  // This matches the disabling in enterEveryRule(). Reenable content
  // generation in subtree if this rule handles it.
  if (DisableAutoSubtreeForRule(ctx)) {
    --disableNormalSubtreeContent;
    PVN_CHECK_GE(disableNormalSubtreeContent, 0);
  }

  const auto content_pair =
      GetHandler()->content_pair_map->Get(MakeContentMapKey(
          util::RuleCategory::kSalientParser, ctx->getRuleIndex()));

  if (content_pair.has_value()) {
    GetHandler()->out_stream << content_pair->second;
  }
}

void SalientToHtmlListener::enterParaBlock(SvtParser::ParaBlockContext* ctx) {
  // antlr4::ParserRuleContext* paraBlock = ctx->paraBlockContent();
  // GetHandler()->out_stream << util::GetTrimmedAllTokens(paraBlock) <<
  // std::endl;
}

void SalientToHtmlListener::visitTerminal(antlr4::tree::TerminalNode* ctx) {
  bool grammar_flush = true;
  size_t symbol_type = ctx->getSymbol()->getType();

  handler_->OutputLines(grammar_flush, WalkerTransition());

  if (disableNormalSubtreeContent == 0) {
    switch (symbol_type) {
      case PvnLexer::EXTRA_ORDINARY_CHAIN: {
        string token_text(ctx->getText());  // Forced copy.
        absl::string_view sv_token_text(token_text);
        // strings::RemoveLeadingWhitespace(&sv_token_text);
        if (!sv_token_text.empty()) {
          CommonReformatter::AppendPiece(string(sv_token_text), ctx,
                                         handler_.get());
        }
        break;
      }
      case PvnLexer::SVT_DEDENT:
      case PvnLexer::LINE_JOIN:
      case PvnLexer::SINGLE_NEWLINE:
      case PvnLexer::MULTI_NEWLINE:
      case PvnLexer::ENTER_TEXTUAL:
      case PvnLexer::EOF:
      default:
        break;
    }
  }
  if (false || (symbol_type == PvnLexer::ENTER_TEXTUAL)) {
    CommonReformatter::AppendPiece("\n", ctx, handler_.get());
  }
  handler_->OutputLines(grammar_flush, WalkerTransition());
}

}  // namespace pvn_parsing
}  // namespace patinon
