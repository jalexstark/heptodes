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

#include "base_dir/patinon/exploratory/abc/format/reform_handlers.h"

#include <sys/types.h>

#include <algorithm>
#include <cstddef>
#include <iostream>
#include <iterator>
#include <string>
#include <utility>
#include <vector>

#include "strings/util.h"  // strcount
#include "base_dir/absl/container/flat_hash_map.h"
#include "base_dir/absl/memory/memory.h"
#include "base_dir/absl/strings/string_view.h"
#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/format/base_handler.h"
#include "base_dir/patinon/exploratory/abc/grammys/PvnLexer.h"
#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
#include "base_dir/patinon/exploratory/abc/grammys/pvn_token.h"
#include "base_dir/patinon/exploratory/abc/skim/summarizing.h"
#include "base_dir/patinon/exploratory/misc/check_macros.h"

namespace patinon {
namespace pvn_parsing {

void CommonReformatter::AppendPiece(string text, size_t token_type,
                                    ssize_t token_index,
                                    antlr4::tree::ParseTree* ctx,
                                    OutputHandler* output_handler) {
  const WhitespaceMaps& whitespace_maps = output_handler->whitespace_maps;
  const CoarseProperties subsequent_properties =
      output_handler->coarse_properties->get(ctx);
  // Append intra-token whitespace.
  if (!output_handler->output_pieces.empty()) {
    int intra_chars = 1;
    const auto sa_override = whitespace_maps.specific_any_whitespace_map.find(
        output_handler->output_pieces.back().token_type);
    if (sa_override != whitespace_maps.specific_any_whitespace_map.end()) {
      intra_chars = sa_override->second;
    }
    const auto as_override =
        whitespace_maps.any_specific_whitespace_map.find(token_type);
    if (as_override != whitespace_maps.any_specific_whitespace_map.end()) {
      intra_chars = as_override->second;
    }
    const auto ss_override =
        whitespace_maps.specific_specific_whitespace_map.find(
            {output_handler->output_pieces.back().token_type, token_type});
    if (ss_override != whitespace_maps.specific_specific_whitespace_map.end()) {
      intra_chars = ss_override->second;
    }
    if (intra_chars > 0) {
      output_handler->output_pieces.push_back(OutputPiece(
          {string(intra_chars, ' '), -1, output_handler->prevailing_properties,
           output_handler->prevailing_properties,
           PvnLexer::Q_IMPUTED_WHITESPACE}));
      output_handler->pending_char_count += intra_chars;
    }
  }

  // Append new piece.
  output_handler->output_pieces.push_back(
      {text, static_cast<int>(token_index),
       output_handler->prevailing_properties, subsequent_properties,
       token_type});
  output_handler->pending_char_count += text.size();
}

void CommonReformatter::DropFrontN(int n, OutputHandler* output_handler) {
  // Require n <= output_pieces.size(), but handle case where it is not.
  if (n >= output_handler->output_pieces.size()) {
    output_handler->pending_char_count = 0;
    output_handler->output_pieces.resize(0);
    return;
  }

  for (int i = 0; i < n; ++i) {
    output_handler->pending_char_count -=
        output_handler->output_pieces[i].text.size();
  }

  output_handler->output_pieces.erase(
      output_handler->output_pieces.begin(),
      output_handler->output_pieces.begin() + n);
}

void QuarrelReformatHandler::ProcessBlankLines() {
  int token_index = -1;
  for (const auto& piece : output_pieces) {
    token_index = piece.token_index;
    if (token_index != -1) {
      break;
    }
  }
  int num_newlines = 0;
  // Handle special case such as Q_STMT_NEWLINE, ENTER_TEXTUAL, in which
  // case we reach here with empty set of output_pieces, and might have just
  // Quarrel blank lines between Salient pieces.
  if (token_index == -1) {
    token_index = inter_mode_formatting.destination_token_index + 1;
  }

  for (int i = token_index - 1; i >= 0; --i) {
    size_t token_type = tokens->get(i)->getType();
    bool loop_done = false;
    switch (token_type) {
      case PvnLexer::Q_STMT_NEWLINE:
        ++num_newlines;
        break;
      case PvnLexer::NEWLINE_ENTER_CODE:
      case PvnLexer::LEAVE_TEXTUAL:
        if (tokens->get(i)->getText() == "\n") {
          ++num_newlines;
        }
        loop_done = true;
        break;
      case PvnLexer::LEAVE_CODE:
      case PvnLexer::ENTER_TEXTUAL:
        // For now, no newline insertions.
        break;
      case PvnLexer::Q_WS:
        break;
      default:
        loop_done = true;
        break;
    }
    if (loop_done) {
      // Simulate situation.
      break;
    }
  }
  if (num_newlines > 1) {
    out_stream << string(num_newlines - 1, '\n');
  }
}

void SalientReformatHandler::ProcessBlankLines() {
  if (tokens->getNumberOfOnChannelTokens() == 0) {
    // This should be meaningless, but gives the tokens object a kick. Without
    // this the subsequent calls to get(i) can out-of-range fail.
    return;
  }
  int token_index = -1;
  for (const auto& piece : output_pieces) {
    token_index = piece.token_index;
    if (token_index != -1) {
      break;
    }
  }
  int num_newlines = 0;
  // Handle special case such as Q_STMT_NEWLINE, ENTER_TEXTUAL, in which
  // case we reach here with empty set of output_pieces, and might have just
  // Quarrel blank lines between Salient pieces.
  if (token_index == -1) {
    if (inter_mode_formatting.destination_token_index != kInvalidTokenIndex) {
      token_index = inter_mode_formatting.destination_token_index + 1;
    }
  }

  for (int i = token_index - 1; i >= 0; --i) {
    size_t token_type = tokens->get(i)->getType();
    bool loop_done = false;
    switch (token_type) {
      case PvnLexer::Q_STMT_NEWLINE:
        break;
      case PvnLexer::SINGLE_NEWLINE:
        ++num_newlines;
        break;
      case PvnLexer::MULTI_NEWLINE:
        // In the input there can be more than 2 new lines.
        num_newlines += 2;
        break;
      case PvnLexer::NEWLINE_ENTER_CODE:
      case PvnLexer::LEAVE_TEXTUAL:
        num_newlines += strcount(tokens->get(i)->getText(), '\n');
        if ((i > 0) &&
            ((tokens->get(i - 1)->getType() == PvnLexer::LEAVE_CODE) ||
             (tokens->get(i - 1)->getType() == PvnLexer::ENTER_TEXTUAL))) {
          // Handle completely empty comment, which never has a line ended by
          // content.
          ++num_newlines;
        }
        loop_done = true;
        break;
      case PvnLexer::ENTER_TEXTUAL:
      case PvnLexer::LEAVE_CODE:
        if (entry_genre_state.destination_subgenre !=
            TextualSubGenre::kMaster) {
          ++num_newlines;
        }
        loop_done = true;
        break;
      case PvnLexer::Q_WS:
        break;
      default:
        loop_done = true;
        break;
    }
    if (loop_done) {
      // Simulate situation.
      break;
    }
  }

  if (num_newlines > 1) {
    if (entry_genre_state.destination_subgenre == TextualSubGenre::kMaster) {
      out_stream << string(num_newlines - 1, '\n');
    } else {
      string inter_string = "";
      for (int i = 0; i < num_newlines - 1; ++i) {
        ProcessIndent();
        inter_mode_formatting.residual_column_position = 0;
        out_stream << inter_string << std::endl;
      }
    }
  }
}

int QuarrelReformatHandler::CalcUnadjustedIndent(
    const CoarseProperties& coarse_properties_choice) {
  int indent_chars = 3 * (coarse_properties_choice.cumulative_statement_level +
                          coarse_properties_choice.expression_nest_level);

  if (coarse_properties_choice.expression_nest_level > 0) {
    indent_chars += 3;
  }

  return indent_chars;
}

int QuarrelReformatHandler::ProcessIndent() {
  if (output_pieces.empty()) {
    return 0;
  }

  CoarseProperties& coarse_properties_choice =
      output_pieces[0].coarse_properties_before;
  for (const auto& piece : output_pieces) {
    if (piece.coarse_properties_at.is_closure) {
      coarse_properties_choice = piece.coarse_properties_at;
    } else if (piece.token_type != PvnLexer::Q_IMPUTED_WHITESPACE) {
      break;
    }
  }

  int statement_adjust = 0;

  if (!output_pieces.empty()) {
    if (output_pieces[0].token_type == PvnLexer::OPEN_STMT) {
      statement_adjust = 1;
    }
    switch (output_pieces.front().token_type) {
      case PvnLexer::CONTRACT_UNARY:
        statement_adjust = 1;
        break;
      default:
        break;
    }
  }

  int indent_chars = std::max(
      0, CalcUnadjustedIndent(coarse_properties_choice) - 3 * statement_adjust);
  out_stream << string(indent_chars, ' ');
  inter_mode_formatting.residual_column_position += indent_chars;

  return indent_chars;
}

int SalientReformatHandler::ProcessIndent() {
  int total_chars = 0;

  switch (prevailing_properties.sub_genre) {
    case TextualSubGenre::kTextualRight: {
      const int pad_chars = kTextualRightCommentColumn -
                            inter_mode_formatting.residual_column_position;
      if (pad_chars > 0) {
        out_stream << string(pad_chars, ' ');
      }
      out_stream << ";";
      total_chars += 1;
      break;
    }
    case TextualSubGenre::kTextualIndent: {
      const int pad_chars = inter_mode_formatting.contextual_indent_column;
      if (pad_chars > 0) {
        out_stream << string(pad_chars, ' ');
      }
      out_stream << ";;";
      total_chars += 2;
      break;
    }
    case TextualSubGenre::kTextualLeft:
      out_stream << ";;;";
      total_chars += 3;
      break;
    default:
      break;
  }

  int indent_chars = 0;
  if (!output_pieces.empty()) {
    CoarseProperties& coarse_properties =
        output_pieces[0].coarse_properties_before;
    indent_chars = 3 * coarse_properties.statement_nest_level;
  }
  // The handler's prevailing_properties could be used for indentation if
  // appropriate.
  if (indent_chars > 0) {
    out_stream << string(indent_chars, ' ');
  }
  total_chars += indent_chars;

  return total_chars;
}

void QuarrelReformatHandler::ProcessPending() {
  ProcessBlankLines();
  if (output_pieces.empty()) {
    return;
  }
  ProcessIndent();

  int strings_to_output = output_pieces.size();
  int first_piece = 0;
  if ((strings_to_output > 0) &&
      (output_pieces[0].token_type == PvnLexer::Q_IMPUTED_WHITESPACE)) {
    first_piece = 1;
    pending_char_count -= output_pieces[0].text.size();
  }

  for (int i = first_piece; i < strings_to_output; ++i) {
    out_stream << output_pieces[i].text;
    pending_char_count -= output_pieces[i].text.size();
    inter_mode_formatting.residual_column_position +=
        output_pieces[i].text.size();
  }
  if (inter_mode_formatting.append_newline) {
    out_stream << std::endl;
    inter_mode_formatting.residual_column_position = 0;
  }
  inter_mode_formatting.contextual_indent_column =
      CalcUnadjustedIndent(prevailing_properties);

  // Check needed that char count is zero.
  output_pieces.resize(0);
  pending_char_count = 0;
}

void SalientReformatHandler::ProcessPending() {
  ProcessBlankLines();
  if (output_pieces.empty()) {
    return;
  }

  int processed_indent = ProcessIndent();
  // out_stream << "CCC";
  if (entry_genre_state.destination_subgenre != TextualSubGenre::kMaster) {
    out_stream << " ";
    ++processed_indent;
  }

  // At least one token.
  // Can break line end where there is not whitespace.
  // If last token within line limit is whitespace, drop.
  // If next token after the last one output is whitespace, drop.
  // const std::vector<OutputPiece>& output_pieces = output_pieces;
  int strings_to_output = output_pieces.size();
  int first_piece = 0;
  if ((strings_to_output > 0) &&
      (output_pieces[0].token_type == PvnLexer::Q_IMPUTED_WHITESPACE)) {
    first_piece = 1;
    pending_char_count -= output_pieces[0].text.size();
  }
  if (strings_to_output <= first_piece) {
    CommonReformatter::DropFrontN(first_piece, this);
    return;
  }
  std::vector<absl::string_view> text_pieces;
  text_pieces.reserve(strings_to_output);
  int end_piece = first_piece;  // Treat [first_piece, end_piece) as half-open.
  int consumed_size = 0;

  text_pieces.push_back(output_pieces[end_piece].text);
  consumed_size += output_pieces[end_piece].text.size();
  ++end_piece;

  while ((strings_to_output > end_piece) &&
         ((consumed_size + output_pieces[end_piece].text.size()) <=
          (inter_mode_formatting.multi_line_max_length - processed_indent))) {
    text_pieces.push_back(output_pieces[end_piece].text);
    consumed_size += output_pieces[end_piece].text.size();
    ++end_piece;
  }
  // Distinguish handling of whitespace around end of sequence.
  int consumed_end_piece = end_piece;
  // Delete whitespace at end of sequence.
  if (output_pieces[end_piece - 1].token_type ==
      PvnLexer::Q_IMPUTED_WHITESPACE) {
    --end_piece;
    text_pieces.pop_back();
  }
  // Prepare to drop whitespace that follows sequence, since we will be
  // inserting a newline.
  if ((strings_to_output > consumed_end_piece) &&
      (output_pieces[consumed_end_piece].token_type ==
       PvnLexer::Q_IMPUTED_WHITESPACE)) {
    consumed_size += output_pieces[end_piece].text.size();
    ++consumed_end_piece;
  }

  for (int i = first_piece; i < end_piece; ++i) {
    out_stream << output_pieces[i].text;
  }
  if (inter_mode_formatting.append_newline) {
    out_stream << std::endl;
    inter_mode_formatting.residual_column_position = 0;
  }

  CommonReformatter::DropFrontN(consumed_end_piece, this);
}

void QuarrelReformatHandler::OutputLines(
    bool full_flush, const WalkerTransition& next_genre_state) {
  if (!full_flush) {
    // In this present version we only do larger flushes, not incremental
    // popping and output. In the long run we want to output more as we go, not
    // least when encountering errors. This feature can be implemented once we
    // have a large body of examples to process. For now, we make sure that we
    // output fairly often, such as after statements and between paragraphs.
    return;
  }

  flush_at_next_token = FlushReason::kNone;

  inter_mode_formatting.one_line_max_length = kNormalMaxLineChars;
  inter_mode_formatting.multi_line_max_length = kNormalMaxLineChars;
  inter_mode_formatting.append_newline = true;
  inter_mode_formatting.destination_token_index = next_genre_state.token_index;

  if ((next_genre_state.destination_genre == ParsingGenre::kSalient) &&
      (next_genre_state.destination_subgenre ==
       TextualSubGenre::kTextualRight)) {
    inter_mode_formatting.multi_line_max_length =
        kTextualRightCommentColumn - 1;
    inter_mode_formatting.append_newline = false;
  }

  bool some_output = !output_pieces.empty();

  // Note that full_flush == true.
  ProcessPending();  // This will call ProcessBlankLines() if queue empty.
  while (!output_pieces.empty()) {
    ProcessPending();
  }
  if (some_output &&
      (next_genre_state.destination_genre != ParsingGenre::kNone) &&
      (next_genre_state.destination_genre !=
       entry_genre_state.destination_genre)) {
    PVN_CHECK(output_pieces.empty());
    ProcessBlankLines();
  }
}

void SalientReformatHandler::OutputLines(
    bool full_flush, const WalkerTransition& next_genre_state) {
  if (!full_flush) {
    // In this present version we only do larger flushes, not incremental
    // popping and output. In the long run we want to output more as we go, not
    // least when encountering errors. This feature can be implemented once we
    // have a large body of examples to process. For now, we make sure that we
    // output fairly often, such as after statements and between paragraphs.
    return;
  }

  flush_at_next_token = FlushReason::kNone;

  inter_mode_formatting.one_line_max_length = kNormalMaxLineChars;
  inter_mode_formatting.multi_line_max_length = kNormalMaxLineChars;
  inter_mode_formatting.append_newline = true;
  inter_mode_formatting.destination_token_index = next_genre_state.token_index;

  if ((entry_genre_state.destination_genre != ParsingGenre::kNone) &&
      (entry_genre_state.destination_subgenre ==
       TextualSubGenre::kTextualRight)) {
    inter_mode_formatting.one_line_max_length = kColumnCommentWidth - 2;
    inter_mode_formatting.multi_line_max_length = kColumnCommentWidth;
  } else if ((entry_genre_state.destination_genre != ParsingGenre::kNone) &&
             (entry_genre_state.destination_subgenre ==
              TextualSubGenre::kTextualIndent)) {
    int column_width =
        kNormalMaxLineChars - inter_mode_formatting.contextual_indent_column;
    inter_mode_formatting.one_line_max_length = column_width;
    inter_mode_formatting.multi_line_max_length = column_width;
  }

  bool some_output = !output_pieces.empty();

  // Note that full_flush == true.
  ProcessPending();  // This will call ProcessBlankLines() if queue empty.
  while (!output_pieces.empty()) {
    ProcessPending();
  }
  if (some_output &&
      (next_genre_state.destination_genre != ParsingGenre::kNone) &&
      (next_genre_state.destination_genre !=
       entry_genre_state.destination_genre)) {
    PVN_CHECK(output_pieces.empty());
    ProcessBlankLines();
  }
}

void QuarrelToHtmlHandler::OutputLines(
    bool full_flush, const WalkerTransition& next_genre_state) {
  if (!full_flush) {
    // In this present version we only do larger flushes, not incremental
    // popping and output. In the long run we want to output more as we go, not
    // least when encountering errors. This feature can be implemented once we
    // have a large body of examples to process. For now, we make sure that we
    // output fairly often, such as after statements and between paragraphs.
    return;
  }
  const int first_piece = 0;
  const int end_piece = output_pieces.size();

  for (int i = first_piece; i < end_piece; ++i) {
    out_stream << output_pieces[i].text;
  }

  CommonReformatter::DropFrontN(end_piece, this);
}

void SalientToHtmlHandler::OutputLines(
    bool full_flush, const WalkerTransition& next_genre_state) {
  if (!full_flush) {
    // In this present version we only do larger flushes, not incremental
    // popping and output. In the long run we want to output more as we go, not
    // least when encountering errors. This feature can be implemented once we
    // have a large body of examples to process. For now, we make sure that we
    // output fairly often, such as after statements and between paragraphs.
    return;
  }
  const int first_piece = 0;
  const int end_piece = output_pieces.size();

  for (int i = first_piece; i < end_piece; ++i) {
    out_stream << output_pieces[i].text;
  }

  CommonReformatter::DropFrontN(end_piece, this);
}

}  // namespace pvn_parsing
}  // namespace patinon
