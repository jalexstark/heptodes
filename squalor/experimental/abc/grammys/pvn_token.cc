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

#include "base_dir/patinon/exploratory/abc/grammys/pvn_token.h"

#include "base_dir/patinon/exploratory/abc/grammys/PvnLexer.h"
#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
// #include "base_dir/patinon/exploratory/abc/grammys/SvtLexer.h"

namespace patinon {
namespace pvn_parsing {

using patinon::pvn_parsing::PvnLexer;

inline static bool IsLogicalWhitespace(size_t token_type) {
  switch (token_type) {
    case PvnLexer::LINE_JOIN:
    case PvnLexer::SINGLE_NEWLINE:
    case PvnLexer::MULTI_NEWLINE:
    case PvnLexer::WS_CHAIN:
      return true;
    default:
      return false;
  }
}

inline static bool IsImpliedIndenter(size_t token_type) {
  switch (token_type) {
    case PvnLexer::ITEM_START_FIRST:
    case PvnLexer::ITEM_START_SUCCEEDING:
    case PvnLexer::LIST_BREAK_ACTUAL:
      return true;
    default:
      return false;
  }
}

// Superset of IsListTokenChainWithNext. List tokens that are allowed to be
// empty are list-like but do not chain.
inline static bool IsListLikeToken(size_t token_type) {
  switch (token_type) {
    case PvnLexer::ITEM_START_FIRST:
    case PvnLexer::ITEM_START_SUCCEEDING:
    case PvnLexer::LIST_BREAK_ACTUAL:
    case PvnLexer::INDENT_CONTINUATION:
    case PvnLexer::SVT_INDENT:
    case PvnLexer::SVT_DEDENT:
      return true;
    default:
      return false;
  }
}

// A token that can be part of a contiguous chain of item nesting.
// Subset of IsListLikeToken.
inline static bool IsListTokenChainWithNext(size_t token_type) {
  switch (token_type) {
    case PvnLexer::ITEM_START_FIRST:
    case PvnLexer::ITEM_START_SUCCEEDING:
    case PvnLexer::INDENT_CONTINUATION:
    case PvnLexer::SVT_INDENT:
    case PvnLexer::SVT_DEDENT:
      return true;
    default:
      return false;
  }
}

inline static bool TriggersReindentationNow(size_t token_type) {
  switch (token_type) {
    case PvnLexer::MULTI_NEWLINE:  // Really? This will get confused with
                                   // multiple newlines.
    // case ITEM_START_INDENT:
    case PvnLexer::INDENT_CONTINUATION:
    case PvnLexer::LEAVE_TEXTUAL:
    case PvnLexer::PENDING_ENTER_CODE:
    case PvnLexer::EOF:
      return true;
    default:
      return false;
  }
}

inline static bool TriggersSubsequentReindentation(size_t token_type) {
  switch (token_type) {
    case PvnLexer::ENTER_TEXTUAL:
    case PvnLexer::LEAVE_CODE:
      return true;
    default:
      return false;
  }
}

// If ForcesZeroIndent() then TriggersReindentationNow() true and
// IsLogicalWhitespace() false.
inline static bool ForcesZeroIndent(size_t token_type) {
  switch (token_type) {
    case PvnLexer::LEAVE_TEXTUAL:
    case PvnLexer::PENDING_ENTER_CODE:
    case PvnLexer::EOF:
      return true;
    default:
      return false;
  }
}

// Assumes that token type is one of {ITEM_START_FIRST, ITEM_START_SUCCEEDING,
// INDENT_CONTINUATION}.
static SvtListType ExtractListType(const absl::string_view token_text) {
  const int tmp_pos = token_text.find_first_of('@');
  if (tmp_pos != absl::string_view::npos) {
    return SvtListType::kBullet;
  }

  const int colon_pos = token_text.find_first_of(':');
  if ((colon_pos == absl::string_view::npos) ||
      (colon_pos == (token_text.length() - 1))) {
    // const auto token_text_str = std::string(token_text);
    // PVN_CHECK_EQ(token_text_str, "Whoops");
    return SvtListType::kNone;
  }
  const char char_after_colon = token_text[colon_pos + 1];
  if (('0' <= char_after_colon) && (char_after_colon <= '9')) {
    return SvtListType::kArabic;
  } else if (('a' <= char_after_colon) && (char_after_colon < 'i')) {
    return SvtListType::kLowerAlpha;
  } else if (('i' <= char_after_colon) && (char_after_colon <= 'x')) {
    return SvtListType::kLowerRoman;
  } else if (('A' <= char_after_colon) && (char_after_colon < 'I')) {
    return SvtListType::kUpperAlpha;
  } else if (('I' <= char_after_colon) && (char_after_colon <= 'X')) {
    return SvtListType::kUpperRoman;
  } else if (char_after_colon == '%') {
    // PVN_CHECK(false);
    return SvtListType::kListBreak;
  } else {
    PVN_CHECK_EQ(char_after_colon, 'a');  // Deliberate failure, 'a' = 97.
    return SvtListType::kNone;
  }
}

// Basically count initial '|' characters, skipping whitespace.
//
// This appears to have become a little too complicated, and can be refactored
// later once there are plenty of regression tests.
static int CalculateIndentation(const absl::string_view token_text,
                                size_t token_type) {
  switch (token_type) {
    case PvnLexer::ITEM_START_FIRST:
    case PvnLexer::ITEM_START_SUCCEEDING:
    case PvnLexer::LIST_BREAK_ACTUAL:
    case PvnLexer::INDENT_CONTINUATION:
      break;
    default:
      return 0;
  }
  int curr_pos = 0;
  int indentation = 0;
  while (true) {
    if (curr_pos == token_text.size()) {
      break;
    }
    int new_pos = token_text.find_first_not_of(" \t", curr_pos);
    if (new_pos == absl::string_view::npos) {
      break;
    }
    curr_pos = new_pos;
    int after_pos = token_text.find_first_not_of('|', curr_pos);
    if (after_pos == curr_pos) {
      break;
    }
    if (after_pos == absl::string_view::npos) {
      after_pos = token_text.size();
    }
    indentation += after_pos - curr_pos;
    curr_pos = after_pos;
  }
  // switch (token_type) {
  // case ITEM_START_INDENT:
  //   ++indentation;
  //   break;
  // default:
  //   break;
  // }
  return indentation;
}

//   inline antlr4::Token*
//   PeekBack(std::deque<std::unique_ptr<antlr4::Token>>* token_deque) {
//     std::unique_ptr<antlr4::Token> popped_token =
//         std::move(token_deque->back());
//     token_deque->pop_back();

//     antlr4::Token *peeked_ptr = popped_token.get();
//     token_deque->push_back(std::move(popped_token));

//     return peeked_ptr;
//   }

std::unique_ptr<antlr4::Token> SalientModeContext::SpecializedNextToken(
    antlr4::Lexer* lexer) {
  PvnLexer* svt_lexer = dynamic_cast<PvnLexer*>(lexer);
  PVN_CHECK(svt_lexer != nullptr);
  PVN_CHECK(svt_lexer->pending_tokens.empty() ||
            !IsLogicalWhitespace(svt_lexer->pending_tokens.back()->getType()));

  if (svt_lexer->pending_tokens.empty()) {
    do {
      std::unique_ptr<antlr4::Token> new_token(
          lexer->antlr4::Lexer::nextToken());

      reindenting_required = reindenting_required ||
                             TriggersReindentationNow(new_token->getType());
      PVN_CHECK_EQ(last_emitted_implied_indenter,
                   IsImpliedIndenter(svt_lexer->prev_token_type));
      if (IsImpliedIndenter(new_token->getType()) &&
          !last_emitted_implied_indenter) {
        reindenting_required = true;
      }

      // if (reindenting_required && !IsListLikeToken(new_token->getType()) &&
      //     !IsLogicalWhitespace(new_token->getType())) {
      if (!IsListLikeToken(new_token->getType()) &&
          !IsLogicalWhitespace(new_token->getType())) {
        // std::cerr << "$$ Resetting nesting depth, type: "
        //           << new_token->getType() << std::endl;

        // NO LONGER PERTINENT:
        // No longer true, or never was if item is empty (just "@\n", say).
        // PVN_CHECK_GE(nested_list_types.size(), nesting_depth_this_line);

        // if (nested_list_types.size() > nesting_depth_this_line) {
        //   nested_list_types.resize(nesting_depth_this_line);
        // }
        nesting_depth_this_line = 0;
      }
      // else {
      //   std::cerr << "$$ Skipping, type: " << new_token->getType() <<
      //   std::endl;
      // }

      svt_lexer->pending_tokens.push_back(std::move(new_token));
    } while (IsLogicalWhitespace(svt_lexer->pending_tokens.back()->getType()));
    PVN_CHECK(
        !svt_lexer->pending_tokens.empty() &&
        !IsLogicalWhitespace(svt_lexer->pending_tokens.back()->getType()));

    int requested_indentation;
    if (reindenting_required) {
      antlr4::Token* back_token = svt_lexer->pending_tokens.back().get();
      if (ForcesZeroIndent(back_token->getType())) {
        requested_indentation = 0;
      } else {
        requested_indentation =
            CalculateIndentation(back_token->getText(), back_token->getType());
      }
    } else {
      requested_indentation = current_indentation;
    }

    // if (requested_indentation < nesting_depth_this_line) {
    //   std::cerr << "$$ Resetting nesting depth, type: "
    //             << svt_lexer->pending_tokens.back().get()->getType()
    //             << std::endl;
    //   nesting_depth_this_line = requested_indentation;
    //   PVN_CHECK_GE(nested_list_types.size(), nesting_depth_this_line);
    //   if (nested_list_types.size() > nesting_depth_this_line) {
    //     nested_list_types.resize(nesting_depth_this_line);
    //   }
    // }

    // If any dedents, and first token and not preceded by list end, then add.

    // Dedent insertion.
    while (current_indentation > requested_indentation) {
      --current_indentation;
      antlr4::Token* front_token = svt_lexer->pending_tokens.front().get();

      std::unique_ptr<CustomizedToken> new_token(new CustomizedToken(
          std::pair(front_token->getTokenSource(),
                    front_token->getInputStream()),
          PvnLexer::SVT_DEDENT, front_token->getChannel(),
          front_token->getStartIndex(), front_token->getStartIndex() - 1));
      new_token->setTokenIndex(front_token->getTokenIndex());
      new_token->setLine(front_token->getLine());
      new_token->setCharPositionInLine(front_token->getCharPositionInLine());

      svt_lexer->pending_tokens.push_front(std::move(new_token));
    }
    // Indent insertion.
    while (current_indentation < requested_indentation) {
      ++current_indentation;
      antlr4::Token* back_token = svt_lexer->pending_tokens.back().get();

      std::unique_ptr<CustomizedToken> new_token(new CustomizedToken(
          std::pair(back_token->getTokenSource(), back_token->getInputStream()),
          PvnLexer::SVT_INDENT, back_token->getChannel(),
          back_token->getStartIndex(), back_token->getStartIndex() - 1));
      new_token->setTokenIndex(back_token->getTokenIndex());
      new_token->setLine(back_token->getLine());
      new_token->setCharPositionInLine(back_token->getCharPositionInLine());

      // Push a token, just before the back.
      svt_lexer->pending_tokens.insert(--svt_lexer->pending_tokens.end(),
                                       std::move(new_token));
    }

    antlr4::Token* back_token = svt_lexer->pending_tokens.back().get();
    reindenting_required =
        TriggersSubsequentReindentation(back_token->getType());

    last_emitted_implied_indenter = IsImpliedIndenter(back_token->getType());
    if (last_emitted_implied_indenter) {
      ++current_indentation;
    }
    // nesting_depth_this_line = current_indentation;
    if (nested_list_types.size() > current_indentation) {
      nested_list_types.resize(current_indentation);
    }
  }

  std::unique_ptr<antlr4::Token> popped_token =
      std::move(svt_lexer->pending_tokens.front());
  svt_lexer->pending_tokens.pop_front();
  // if (!IsListLikeToken(popped_token->getType())) {
  //   std::cerr << "$$ Resetting nesting depth, type: " <<
  //   popped_token->getType()
  //             << std::endl;
  //   PVN_CHECK_GE(nested_list_types.size(), nesting_depth_this_line);
  //   if (nested_list_types.size() > nesting_depth_this_line) {
  //     nested_list_types.resize(nesting_depth_this_line);
  //   }
  //   nesting_depth_this_line = 0;
  // } else {
  //   std::cerr << "$$ Skipping, type: " << popped_token->getType() <<
  //   std::endl;
  // }
  return popped_token;
}

int SalientModeContext::UpdateListNesting(std::string token_text,
                                          // size_t token_type,
                                          antlr4::Lexer* lexer) {
  PvnLexer* svt_lexer = dynamic_cast<PvnLexer*>(lexer);
  PVN_CHECK(svt_lexer != nullptr);

  // const int indent_increment = CalculateIndentation(token_text, token_type);

  // Get new token type from token text.
  const SvtListType new_list_type = ExtractListType(token_text);

  int list_chaining_type = PvnLexer::NONE_TOKEN;

  // std::cerr << "$$" << lexer->getLine();

  if (IsListTokenChainWithNext(svt_lexer->prev_token_type)) {
    // PROBLEM HERE WITH EMPTY ITEMS.MAYBE SHOULD

    // std::cerr << "**" << token_text << "**\n";

    // Should be able to check and report error if indent increments by > 1.
    ++nesting_depth_this_line;
    nested_list_types.push_back(new_list_type);
    PVN_CHECK_EQ(nested_list_types.size(), nesting_depth_this_line);
    if (new_list_type == SvtListType::kListBreak) {
      list_chaining_type = PvnLexer::LIST_BREAK_ACTUAL;
    } else {
      list_chaining_type = PvnLexer::ITEM_START_FIRST;
    }
  } else {
    // std::cerr << "++" << token_text << "**\n";

    // Add 1 because the indent calculator does not count the terminal list
    // token.
    int indent_increment =
        CalculateIndentation(token_text, PvnLexer::ITEM_START_FIRST);
    if (new_list_type != SvtListType::kNone) {
      ++indent_increment;
    }
    // if (indent_increment == 0) {
    //   std::cerr << "*** " << token_text << " ***\n";
    // }
    PVN_CHECK_GT(indent_increment, 0);

    nesting_depth_this_line += indent_increment;

    if (nesting_depth_this_line <= nested_list_types.size()) {
      PVN_CHECK_EQ(nesting_depth_this_line, indent_increment);
      // Shorten "stack" of list types.
      nested_list_types.resize(indent_increment);
      if (new_list_type == SvtListType::kNone) {
        list_chaining_type = PvnLexer::INDENT_CONTINUATION;
      } else if (nested_list_types[indent_increment - 1] == new_list_type) {
        list_chaining_type = PvnLexer::ITEM_START_SUCCEEDING;
      } else if (new_list_type == SvtListType::kListBreak) {
        list_chaining_type = PvnLexer::LIST_BREAK_ACTUAL;
        // PVN_CHECK(false);
        nested_list_types[indent_increment - 1] = new_list_type;
      } else {
        // We could get rid of this and require list restart tokens all the
        // time. This logic is complicated, and we could then drop the
        // nested_list_types vector and just track the depth (currently the
        // size of the vector).
        list_chaining_type = PvnLexer::ITEM_START_FIRST;
        nested_list_types[indent_increment - 1] = new_list_type;
      }
    } else {
      // This does not hold.
      // PVN_CHECK_EQ(nesting_depth_this_line, indent_increment);

      // Garbage: Should check indent_increment == (nesting_depth_this_line +
      // 1). Should check not non-item indent continuation. if (new_list_type ==
      // SvtListType::kNone) {
      //   std::cerr << "$$$ " << nested_list_types.size() << "---"
      //             << nesting_depth_this_line << "---" << lexer->getLine()
      //             << ":::" << token_text << " ***\n";
      // }
      PVN_CHECK_NE(new_list_type, SvtListType::kNone);
      PVN_CHECK_NE(new_list_type, SvtListType::kListBreak);
      nested_list_types.resize(indent_increment, new_list_type);
      list_chaining_type = PvnLexer::ITEM_START_FIRST;
    }
  }
  svt_lexer->SetAuxiliaryTokenType(
      PvnLexer::ListTypeToPseudoToken(new_list_type));
  return list_chaining_type;
}

// @Override
// public Token nextToken() {
//     while (pendingTokens.isEmpty()) {
//         Token token = super.nextToken();
//         switch (token.getType()) {
//         case INDENT:
//             // handle indent here. to skip this token, simply don't add
//             // anything to the pendingTokens queue and super.nextToken()
//             // will be called again.
//             break;

//         case DEDENT:
//             // handle indent here. to skip this token, simply don't add
//             // anything to the pendingTokens queue and super.nextToken()
//             // will be called again.
//             break;

//         default:
//             pendingTokens.add(token);
//             break;
//         }
//     }

//     return pendingTokens.poll();
// }

}  // namespace pvn_parsing

}  // namespace patinon
