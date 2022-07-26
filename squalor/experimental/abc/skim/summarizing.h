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

#ifndef BASE_DIR_PATINON_EXPLORATORY_ABC_SKIM_SUMMARIZING_H_
#define BASE_DIR_PATINON_EXPLORATORY_ABC_SKIM_SUMMARIZING_H_

#include <cstddef>
#include <deque>
#include <string>
#include <utility>
#include <vector>

#include "base_dir/absl/container/flat_hash_map.h"
#include "base_dir/absl/strings/escaping.h"
#include "base_dir/absl/strings/str_join.h"
#include "base_dir/absl/strings/str_replace.h"
#include "base_dir/absl/strings/str_split.h"
#include "base_dir/absl/strings/string_view.h"
#include "base_dir/absl/strings/strip.h"
#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/grammys/PvnLexer.h"
#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
#include "base_dir/patinon/exploratory/abc/grammys/pvn_token.h"
#include "base_dir/patinon/exploratory/abc/skim/summarizing.h"

namespace patinon {
namespace pvn_parsing {

namespace util {

inline bool IsWhitespaceToken(const antlr4::Token& token) {
  switch (token.getType()) {
    case PvnLexer::SINGLE_NEWLINE:
    case PvnLexer::MULTI_NEWLINE:
      return true;
    default:
      return false;
  }
}

inline string ReplaceAndTrim(absl::string_view input) {
  static constexpr const char nbsp[] = "&nbsp;";

  absl::string_view::size_type pos = 0;
  std::deque<absl::string_view> result_chain;
  while (pos < input.size()) {
    absl::string_view::size_type new_pos = input.find_first_of('\\', pos);
    if ((new_pos == absl::string_view::npos) ||
        (new_pos == (input.size() - 1))) {
      result_chain.push_back(input.substr(pos, input.size() - pos));
      break;
    }

    // '\' character found at new_pos. First push preceding substring.
    if ((new_pos - 1) > pos) {
      result_chain.push_back(input.substr(pos, new_pos - 1 - pos));
    }

    if (input.at(new_pos + 1) == ' ') {
      result_chain.push_back(nbsp);
    } else {
      result_chain.push_back(input.substr(new_pos, 2));
    }

    pos = new_pos + 2;
  }
  string result;
  absl::CUnescape(absl::StrJoin(result_chain, ""), &result);
  absl::StrReplaceAll({{"\n", " "},
                       {"\r", " "},
                       {"\t", " "},
                       {"\f", " "},
                       {"&", "&amp;"},
                       {"<", "&lt;"},
                       {">", "&gt;"}},
                      &result);

  result = absl::StrJoin(absl::StrSplit(result, ' ', absl::SkipEmpty()), " ");
  result = string(absl::StripPrefix(absl::StripSuffix(result, " "), " "));

  return result;
}

inline void GetAllTokensRecursive(antlr4::ParserRuleContext* ctx,
                                  std::vector<antlr4::Token*>* tokens) {
  for (auto child_tree : ctx->children) {
    antlr4::tree::TerminalNode* tnode =
        dynamic_cast<antlr4::tree::TerminalNode*>(child_tree);
    if (tnode != nullptr) {
      antlr4::Token* symbol = tnode->getSymbol();
      tokens->push_back(symbol);
    } else {
      antlr4::ParserRuleContext* child_context =
          dynamic_cast<antlr4::ParserRuleContext*>(child_tree);
      if (child_context != nullptr) {
        GetAllTokensRecursive(child_context, tokens);
      }
    }
  }
}

inline std::vector<antlr4::Token*> GetAllTokens(
    antlr4::ParserRuleContext* ctx) {
  std::vector<antlr4::Token*> tokens;
  GetAllTokensRecursive(ctx, &tokens);
  return tokens;
}

inline string GetTrimmedAllTokens(antlr4::ParserRuleContext* ctx) {
  std::vector<antlr4::Token*> tokens = GetAllTokens(ctx);

  std::vector<string> tokens_text;
  tokens_text.reserve(tokens.size());
  for (auto t : tokens) {
    // This does not eliminate duplicate whitespace, but reduces it.
    tokens_text.push_back(IsWhitespaceToken(*t) ? " " : t->getText());
  }
  return ReplaceAndTrim(absl::StrJoin(tokens_text, " "));
}

}  // namespace util

struct Qualifier {
  string left_side;
  string separator;
  string right_side;
};

struct Heading {
  static constexpr const char kDefaultFileId[] = "Not set";
  static constexpr const int kTocHeadingLevel = -1;
  // Later augment with a enumeration that says what kind of level-0, such as
  // "Module" or "Title".
  //
  // For now level 0 is the file title.
  int level;
  antlr4::tree::TerminalNode* terminal_node;
  string heading_text;
  // Often file_id is filename: taken together, the pair of this and the line
  // number must be unique for each heading entry.
  //
  // In the current code we do not set the file_id, but want it here as a
  // placeholder.
  int line_number = -1;
  string file_id = kDefaultFileId;
  int heading_number;
  string anchor_id;  // Uniquified anchor.
  std::vector<Qualifier> qualifiers;
  absl::flat_hash_map<string, int> left_side_to_qualifier_index;
};

struct WalkerTransition {
  size_t token_index = kInvalidTokenIndex;
  ParsingGenre destination_genre = ParsingGenre::kNone;
  TextualSubGenre destination_subgenre = TextualSubGenre::kNone;
};

struct SummarizerResults {
  static constexpr const int kNoDetectedTitle = -1;

  WalkerTransition outer_genre_state;
  absl::flat_hash_map<std::pair<string, int>, int> heading_indices;
  std::vector<Heading> heading_vector;
  // Normally the title-level heading (level 0) is the first (0 index);
  int title_heading_index = kNoDetectedTitle;
  // From line number to heading index.
  absl::flat_hash_map<int, int> heading_by_line;
  std::vector<WalkerTransition> quarrel_to_salient_transitions;
  std::vector<WalkerTransition> salient_to_quarrel_transitions;
};

// Coarse properties are those that are primarily assigned during a coarse skim
// analysis, perhaps with an additional reprocessing step on the final results
// or to combine results.
//
// The properties are mechanistically associated with parse tree nodes, bur are
// primarily associated with tokens (terminal parse nodes). For safety and
// consistency every parse node should have an entry, and so the skim parsers
// should assign on enterEveryRule() and visitTerminal().
//
// Properties are often changed by a token, and the meaning of the content is
// for the most part determined by token's context. Suppose that we indent the
// content but not the first line of a block comment. There are three relevant
// parse tree walks: the begin-comment token ("/*" in C), the end-comment token
// ("*/" in C), and the common block-comment rule node for which the tokens are
// the first and last children. We could update the running indentation (in the
// tree-walk object) at the beginning of the enterBlockComment() method and at
// the end of the exitBlockComment() method. (Largely equivalently, we could do
// so in enterEveryRule() and exitEveryRule().) Alternatively we could perform
// either of these updates in visitTerminal(). If so, we must be careful to
// increase the indent before creating the properties for the begin-comment
// token, and/or decreasing after property creation for the end-comment.
//
// Properties are passed through to the output processors of reformatters. These
// processors need to know the context before a token is encountered. For
// example, the indentation just preceding a begin-block-comment token would be
// the indentation outside of the block comment. This is associated with the
// token preceding the begin-comment token. Therefore output handling uses the
// "preceding" and "at" properties for each token.
struct CoarseProperties {
  bool is_closure = false;
  int statement_nest_level;
  int cumulative_statement_level;
  int expression_nest_level;
  int nesting_stack_depth;
  TextualSubGenre sub_genre = TextualSubGenre::kNone;
};

}  // namespace pvn_parsing
}  // namespace patinon

#endif  // BASE_DIR_PATINON_EXPLORATORY_ABC_SKIM_SUMMARIZING_H_
