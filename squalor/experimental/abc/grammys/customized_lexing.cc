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

#include "base_dir/patinon/exploratory/abc/grammys/customized_lexing.h"

#include <cstddef>
#include <memory>
#include <string>
#include <utility>

#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/grammys/PvnLexer.h"
#include "base_dir/patinon/exploratory/abc/grammys/pvn_token.h"

namespace patinon {
namespace pvn_parsing {

TokenAnomaly CustomizedPvnLexer::TokenTokenAnomaly(size_t token_type) {
  switch (token_type) {
    case PvnLexer::INDENT_CONTINUATION:
    case PvnLexer::ITEM_START_FIRST:
    case PvnLexer::ITEM_START_SUCCEEDING:
    case PvnLexer::LIST_BREAK_ACTUAL:
      return token_anomaly;
    default:
      return TokenAnomaly::kNone;
  }
}

std::unique_ptr<antlr4::CommonToken> CustomizedTokenFactory::create(
    std::pair<antlr4::TokenSource*, antlr4::CharStream*> source, size_t type,
    const std::string& text, size_t channel, size_t start, size_t stop,
    size_t line, size_t charPositionInLine) {
  // Specialization in this override.
  if (channel != CustomizedToken::HIDDEN_CHANNEL) {
    channel = once_channel_;
  }
  once_channel_ = custom_channel_;

  TokenAnomaly token_anomaly = TokenAnomaly::kNone;
  bool is_quarrel_statement = false;
  size_t auxiliary_token_type = kInvalidTokenIndex;

  if (CustomizedPvnLexer* custom_lexer =
          dynamic_cast<CustomizedPvnLexer*>(source.first)) {
    token_anomaly = custom_lexer->TokenTokenAnomaly(type);
    is_quarrel_statement =
        custom_lexer->current_code.IsAtGNewStatementMarker(custom_lexer);
    auxiliary_token_type = custom_lexer->GetAuxiliaryTokenType();

    custom_lexer->TokenConsumeReset();
  }

  // Because C++ is an RAII language, we have to copy code in order to
  // customize:
  //
  // antlr4::CommonTokenFactory::create(source, type, text, channel, start,
  //                                    stop, line, charPositionInLine);

  std::unique_ptr<CustomizedToken> t(
      new CustomizedToken(source, type, channel, start, stop));
  t->setLine(line);
  t->setCharPositionInLine(charPositionInLine);
  if (!text.empty()) {
    t->setText(text);
  } else if (copyText && source.second != nullptr) {
    t->setText(source.second->getText(antlr4::misc::Interval(start, stop)));
  }

  // Insert customized fields.
  t->supplement_.token_anomaly = token_anomaly;
  t->supplement_.is_quarrel_statement = is_quarrel_statement;
  t->supplement_.auxiliary_token_type = auxiliary_token_type;

  return t;
}

}  // namespace pvn_parsing
}  // namespace patinon
