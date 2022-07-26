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

#include "base_dir/patinon/exploratory/abc/format/reform_utils.h"

#include <cstddef>
#include <iomanip>
#include <sstream>
#include <stack>
#include <string>
#include <vector>

#include "base_dir/absl/memory/memory.h"
#include "base_dir/absl/strings/str_format.h"
#include "base_dir/absl/strings/str_replace.h"
#include "base_dir/absl/strings/str_split.h"
#include "base_dir/absl/strings/string_view.h"
#include "base_dir/absl/strings/substitute.h"
#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/grammys/PvnLexer.h"
#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
#include "base_dir/patinon/exploratory/abc/grammys/pvn_token.h"

namespace patinon {
namespace pvn_parsing {

namespace util {

ParsingGenre ChooseModeFromFileExtension(absl::string_view filename) {
  const auto extension_index = filename.find_last_of('.');

  if (extension_index == filename.npos) {
    return ParsingGenre::kNone;
  }

  const auto extension = filename.substr(extension_index + 1);
  if (extension == "pvn") {
    return ParsingGenre::kQuarrel;
  } else if (extension == "svt") {
    return ParsingGenre::kSalient;
  } else {
    return ParsingGenre::kNone;
  }
}

string ChannelDescription(int channel, const PvnLexer& lexer) {
  string logical_name = "Other";
  if (channel == lexer.code_factory->GetChannel()) {
    logical_name = "QUARREL";
  } else if (channel == lexer.textual_factory->GetChannel()) {
    logical_name = "SALIENT";
  }
  string physical_name;
  switch (channel) {
    case PvnLexer::DEFAULT_TOKEN_CHANNEL:
      physical_name = "DEFAULT";
      break;
    case CustomizedToken::HIDDEN_CHANNEL:
      physical_name = "HIDDEN";
      break;
    case PvnLexer::SECONDARY_CHANNEL:
      physical_name = "SECONDARY";
      break;
    default:
      physical_name = "UNKNOWN CHANNEL";
      break;
  }

  return absl::StrFormat("%s / %s channel", logical_name, physical_name);
}

// Version of Trees::toStringTree that handles parens more sensibly.
string TreesToStringTree(antlr4::tree::ParseTree* t,
                         const std::vector<string>& ruleNames) {
  using antlr4::tree::ParseTree;
  using antlr4::tree::Trees;

  std::string temp =
      AntlrcppEscapeWhitespace(Trees::getNodeText(t, ruleNames), false);
  if (t->children.empty()) {
    return temp;
  }

  std::stringstream ss;
  ss << "(" << temp << ' ';

  // Implement the recursive walk as iteration to avoid trouble with deep
  // nesting.
  std::stack<size_t> stack;
  size_t childIndex = 0;
  ParseTree* run = t;
  while (childIndex < run->children.size()) {
    if (childIndex > 0) {
      ss << ' ';
    }
    ParseTree* child = run->children[childIndex];
    temp =
        AntlrcppEscapeWhitespace(Trees::getNodeText(child, ruleNames), false);
    if (!child->children.empty()) {
      // Go deeper one level.
      stack.push(childIndex);
      run = child;
      childIndex = 0;
      ss << "(" << temp << " ";
    } else {
      ss << temp;
      while (++childIndex == run->children.size()) {
        if (!stack.empty()) {
          // Reached the end of the current level. See if we can step up from
          // here.
          childIndex = stack.top();
          stack.pop();
          run = run->parent;
          ss << ")";
        } else {
          break;
        }
      }
    }
  }

  ss << ")";
  return ss.str();
}

// Non-const-ness of tokens is not ideal.
void DebugLexerTokens(absl::string_view channel_name,
                      antlr4::CommonTokenStream* tokens_ptr,
                      const PvnLexer& lexer, std::ofstream& out_stream) {
  antlr4::CommonTokenStream& tokens = *tokens_ptr;

  // out_stream << "Tokens in " << channel_name << " channel:\n\n";

  for (const auto token : tokens.getTokens()) {
    const string display_name =
        lexer.getVocabulary().getDisplayName(token->getType());
    auto reset_width = out_stream.width();
    out_stream << std::setfill(' ') << std::setw(24)
               << absl::StrReplaceAll(absl::Substitute("$0:", display_name),
                                      {
                                          {"\n", "\\n"},
                                          {"\t", "\\t"},
                                          {"\r", "\\r"},
                                      })
               << std::setw(reset_width)
               // << absl::StrFormat("%4d /%3d /%3d: ", token->getTokenIndex(),
               //                    token->getLine(),
               //                    token->getCharPositionInLine())
               << absl::StrFormat("%3d /%3d: ", token->getLine(),
                                  token->getCharPositionInLine())
               << std::setfill('.') << std::setw(24)
               << absl::StrReplaceAll(
                      absl::Substitute(" \"$0\"", token->getText()),
                      {
                          {"\n", "\\n"},
                          {"\t", "\\t"},
                          {"\r", "\\r"},
                      })
               << " :" << std::setw(30)
               << ChannelDescription(token->getChannel(), lexer)
               << std::setw(reset_width) << " :"
               << CustomizedToken::TokenAnomalyString(token) << std::endl;
  }
  // out_stream << std::endl;
}

// Non-const-ness of tokens is not ideal.
void DebugLexerPassThrough(antlr4::CommonTokenStream* tokens_ptr,
                           std::ofstream& out_stream) {
  for (const auto token : tokens_ptr->getTokens()) {
    if (token->getType() != XPathLexer::EOF) {
      out_stream << token->getText();
    }
  }
}

void DebugSimpleParseTree(antlr4::Parser* parser_ptr,
                          antlr4::ParserRuleContext* tree,
                          std::ofstream& out_stream) {
  antlr4::Parser& parser = *parser_ptr;

  out_stream << "TREE:\n";

  string tree_parens = TreesToStringTree(tree, parser.getRuleNames());
  int num_replacements;
  do {
    num_replacements = absl::StrReplaceAll(
        {
            {"   ", " <SP> "},
            {"<SP> <SP>", "<SP><SP><SP>"},
            {"<SP>  ", "<SP><SP> "},
        },
        &tree_parens);
  } while (num_replacements > 0);
  tree_parens = absl::StrReplaceAll(tree_parens, {{"(", "( "}, {")", " )"}});
  std::vector<std::string> tree_vector =
      absl::StrSplit(tree_parens, ' ', absl::SkipWhitespace());

  static constexpr char kIndentStr[] = "   ";
  int nesting_level = 0;
  TreePiece prev_piece = TreePiece::PIECE;
  for (const string& tree_piece : tree_vector) {
    TreePiece new_piece =
        tree_piece == "("
            ? TreePiece::OPEN
            : (tree_piece == ")" ? TreePiece::CLOSE : TreePiece::PIECE);
    if (new_piece == TreePiece::PIECE) {
      new_piece =
          tree_piece == "POPEN"
              ? TreePiece::OPEN
              : (tree_piece == "PCLOSE" ? TreePiece::CLOSE : TreePiece::PIECE);
    }

    if (!((new_piece == prev_piece) ||
          ((prev_piece == TreePiece::OPEN) &&
           (new_piece == TreePiece::PIECE)) ||
          ((prev_piece == TreePiece::PIECE) &&
           (new_piece == TreePiece::CLOSE)))) {
      if (nesting_level > 0) {
        out_stream << std::endl;
      }
      for (int i = 0; i < nesting_level; ++i) {
        out_stream << kIndentStr;
      }
    }
    if (tree_piece == "(") {
      out_stream << "(  ";
      ++nesting_level;
    } else if (tree_piece == ")") {
      out_stream << "  )";
      --nesting_level;
    } else if (tree_piece == "POPEN") {
      out_stream << "POPEN  ";
      // ++nesting_level;
    } else if (tree_piece == "PCLOSE") {
      out_stream << "  PCLOSE";
      // --nesting_level;
    } else {
      if (prev_piece == TreePiece::PIECE) {
        out_stream << " ";
      }
      out_stream << tree_piece;
    }
    prev_piece = new_piece;
  }
  out_stream << std::endl;
}

}  // namespace util

}  // namespace pvn_parsing
}  // namespace patinon
