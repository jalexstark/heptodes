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

#ifndef BASE_DIR_PATINON_EXPLORATORY_ABC_GRAMMYS_PVN_TOKEN_H_
#define BASE_DIR_PATINON_EXPLORATORY_ABC_GRAMMYS_PVN_TOKEN_H_

#include <cstddef>
#include <forward_list>
#include <limits>
#include <memory>
#include <string>
#include <utility>

#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
#include "base_dir/patinon/exploratory/misc/check_macros.h"

namespace patinon {
namespace pvn_parsing {

constexpr size_t kInvalidTokenIndex = std::numeric_limits<size_t>::max() - 3;

// Used to annotate a token even if we have parsed OK. For example, some tokens
// should be at the start of a line.
enum class TokenAnomaly {
  kNone,
  kNotAtLineStart,
  kAtLineStart,
};

inline std::string TokenAnomalyToString(TokenAnomaly ta) {
  switch (ta) {
    case TokenAnomaly::kAtLineStart:
      return "Token anomaly at line start";
    case TokenAnomaly::kNotAtLineStart:
      return "Token anomaly NOT at line start";
    default:
    case TokenAnomaly::kNone:
      return "(No token anomaly)";
  }
}

struct TokenSupplement {
  TokenAnomaly token_anomaly = TokenAnomaly::kNone;
  bool is_quarrel_statement = false;
  size_t auxiliary_token_type = kInvalidTokenIndex;
};

class CustomizedToken : public antlr4::CommonToken {
 public:
  CustomizedToken(std::pair<antlr4::TokenSource*, antlr4::CharStream*> source,
                  size_t type, size_t channel, size_t start, size_t stop)
      : antlr4::CommonToken(source, type, channel, start, stop) {}

  static inline std::string TokenAnomalyString(
      const antlr4::Token* const token) {
    if (const CustomizedToken* const customized_token =
            dynamic_cast<const CustomizedToken* const>(token)) {
      return TokenAnomalyToString(customized_token->supplement_.token_anomaly);
    } else {
      PVN_DCHECK(false);
      return "Unrecognized anomaly";
    }
  }
  TokenSupplement supplement_;
};

// Would be good to restructure so that this is inline. For that, the lexer
// class would have to be defined first. However, the lexer needs to own a copy.
class CustomizedTokenFactory : public antlr4::CommonTokenFactory {
  std::unique_ptr<antlr4::CommonToken> create(
      std::pair<antlr4::TokenSource*, antlr4::CharStream*> source, size_t type,
      const std::string& text, size_t channel, size_t start, size_t stop,
      size_t line, size_t charPositionInLine) override;

 public:
  void SetImmediateCustomChannel(size_t channel) {
    once_channel_ = channel;
    custom_channel_ = channel;
  }
  void SetDeferredCustomChannel(size_t channel) { custom_channel_ = channel; }

  size_t once_channel_;
  size_t custom_channel_;
};

struct LexerCustomization {
  // Probably prefer unique_ptr and then be able to put custom lexer class
  // before custom token facory.
  Ref<CustomizedTokenFactory> custom_token_factory;
};

class AbstractModeContext {
 public:
  // Make virtual?
  AbstractModeContext(size_t channel, int lexer_mode)
      : channel_(channel), lexer_mode_(lexer_mode) {}

  // Contract: Passing kNone for sub_genre means that it should be disregarded.
  // The genre information is already set when reentering a mode context.
  virtual void HandleEntry(TextualSubGenre sub_genre, antlr4::Lexer* lexer,
                           LexerCustomization* customization) = 0;

  virtual AbstractModeContext* clone() const = 0;

  virtual ~AbstractModeContext() {}

 protected:
  size_t channel_;
  int lexer_mode_;
};

class AbstractModeContextFactory {
 public:
  // Make virtual?
  AbstractModeContextFactory(size_t channel, int lexer_mode)
      : channel_(channel), lexer_mode_(lexer_mode) {}

  virtual std::unique_ptr<AbstractModeContext> Create() = 0;

  virtual ~AbstractModeContextFactory() {}

  size_t GetChannel() const { return channel_; }

 protected:
  size_t channel_;
  int lexer_mode_;
};

class QuarrelModeContext : public AbstractModeContext {
 public:
  QuarrelModeContext() : AbstractModeContext(0, -1) {}

 public:
  QuarrelModeContext(size_t channel, int lexer_mode)
      : AbstractModeContext(channel, lexer_mode) {}
  friend class QuarrelModeContextFactory;

 public:
  QuarrelModeContext* clone() const override {
    return new QuarrelModeContext(*this);
  }

  ~QuarrelModeContext() override {}

  inline void NestStatements() {
    if (q_parens_nesting_ != 0) {
      nesting_stack_.emplace_front(q_statement_nesting_, q_parens_nesting_);
      q_statement_nesting_ = 0;
      q_parens_nesting_ = 0;
    } else {
      ++q_statement_nesting_;
    }
  }

  inline void DeNestStatements() {
    if (q_statement_nesting_ != 0) {
      --q_statement_nesting_;
    } else if (!nesting_stack_.empty()) {
      const auto pair = nesting_stack_.front();
      nesting_stack_.pop_front();
      q_statement_nesting_ = pair.first;
      q_parens_nesting_ = pair.second;
    }
  }

  inline void ClampedDecr_q_parens_nesting() {
    q_parens_nesting_ = std::max(0, q_parens_nesting_ - 1);
  }
  inline void Incr_q_parens_nesting() {
    q_parens_nesting_ = std::max(0, q_parens_nesting_ - 1);
  }
  inline int get_q_parens_nesting() const { return q_parens_nesting_; }

  inline void MoveGNewStatementMarker(antlr4::Lexer* lexer) {
    preceding_q_nl_marker_col_ = lexer->getCharPositionInLine();
    preceding_q_nl_marker_line_ = lexer->getLine();
  }

  inline bool IsAtGNewStatementMarker(antlr4::Lexer* lexer) {
    return (
        (lexer->tokenStartCharPositionInLine == preceding_q_nl_marker_col_) &&
        (lexer->tokenStartLine == preceding_q_nl_marker_line_));
  }

  void HandleEntry(TextualSubGenre sub_genre__unused, antlr4::Lexer* lexer,
                   LexerCustomization* customization) override {
    customization->custom_token_factory->SetDeferredCustomChannel(channel_);
    lexer->setMode(lexer_mode_);
    if (q_parens_nesting_ == 0) {
      MoveGNewStatementMarker(lexer);
    }
  }

  int preceding_q_nl_marker_col_ = 0;
  int preceding_q_nl_marker_line_ = 1;

  int q_statement_nesting_ = 0;
  int q_parens_nesting_ = 0;
  std::forward_list<std::pair<int, int>> nesting_stack_;
};

enum class TripleTransitions {
  kNone,
  kQuarrelBlock,
};

// class SvtLexer;

class SalientModeContext : public AbstractModeContext {
 public:
  SalientModeContext() : AbstractModeContext(0, -1) {}

 public:
  SalientModeContext(size_t channel, int lexer_mode)
      : AbstractModeContext(channel, lexer_mode) {}
  friend class SalientModeContextFactory;

 public:
  SalientModeContext* clone() const override {
    return new SalientModeContext(*this);
  }

  ~SalientModeContext() override {}

  // QQQQQQ inline void set_effective_start_col(int val) { effective_start_col_
  // = val; } QQQQQQ inline int get_effective_start_col() const { return
  // effective_start_col_;
  // }
  inline TextualSubGenre get_sub_genre() const { return sub_genre_; }
  inline void set_in_ref_context(bool val) { in_ref_context_ = val; }
  inline bool get_in_ref_context() const { return in_ref_context_; }
  inline void set_pending_triple(TripleTransitions val) {
    pending_triple_ = val;
  }
  inline TripleTransitions get_pending_triple() const {
    return pending_triple_;
  }

  inline void MoveWsMarker(antlr4::Lexer* lexer,
                           bool zero_effective_start_col) {
    if (zero_effective_start_col) {
      // QQQQQQ effective_start_col_ = 0;
    }
    preceding_ws_marker_col_ = lexer->getCharPositionInLine();
    preceding_ws_marker_line_ = lexer->getLine();
  }

  inline bool IsAtWsMarker(antlr4::Lexer* lexer) {
    return ((lexer->tokenStartCharPositionInLine == preceding_ws_marker_col_) &&
            (lexer->tokenStartLine == preceding_ws_marker_line_));
  }

  std::unique_ptr<antlr4::Token> SpecializedNextToken(antlr4::Lexer* lexer);
  int UpdateListNesting(std::string token_text, antlr4::Lexer* lexer);

  void HandleEntry(TextualSubGenre sub_genre, antlr4::Lexer* lexer,
                   LexerCustomization* customization) override {
    if (sub_genre != TextualSubGenre::kNone) {
      sub_genre_ = sub_genre;
    }
    customization->custom_token_factory->SetImmediateCustomChannel(channel_);
    lexer->setMode(lexer_mode_);
    in_ref_context_ = false;
  }

  // Start of physical line adjusted. Textual newlines must always set this. A
  // logical newline sets it to -1 so that a token never starts at the
  // beginning of a line.
  // int effective_start_col_ = 0;
  int preceding_ws_marker_col_ = -1;
  int preceding_ws_marker_line_ = -1;
  bool in_ref_context_ = false;
  TextualSubGenre sub_genre_ = TextualSubGenre::kNone;
  TripleTransitions pending_triple_ = TripleTransitions::kNone;

  int current_indentation = 0;
  // Salient within code might begin with an itemized list, requiring
  // indentation.
  bool reindenting_required = true;
  bool last_emitted_implied_indenter = false;

  // std::underlying_type<StylingFlags> styling_flags = StylingFlags::kNone;
  StylingFlags styling_flags = StylingFlags::kNone;

  // Nested list types may be bigger than nesting depth in the current line. A
  // new line steps through encountered list items and matches pattern of list
  // nesting.
  std::vector<SvtListType> nested_list_types;
  int nesting_depth_this_line = 0;
};

class QuarrelModeContextFactory : public AbstractModeContextFactory {
 public:
  QuarrelModeContextFactory() : QuarrelModeContextFactory(0, -1) {}
  QuarrelModeContextFactory(size_t channel, int lexer_mode)
      : AbstractModeContextFactory(channel, lexer_mode) {}

 public:
  ~QuarrelModeContextFactory() override {}

  std::unique_ptr<AbstractModeContext> Create() override {
    return std::make_unique<QuarrelModeContext>(channel_, lexer_mode_);
  }
};

class SalientModeContextFactory : public AbstractModeContextFactory {
 public:
  SalientModeContextFactory() : SalientModeContextFactory(0, -1) {}
  SalientModeContextFactory(size_t channel, int lexer_mode)
      : AbstractModeContextFactory(channel, lexer_mode) {}

 public:
  ~SalientModeContextFactory() override {}

  std::unique_ptr<AbstractModeContext> Create() override {
    return std::make_unique<SalientModeContext>(channel_, lexer_mode_);
  }
};

}  // namespace pvn_parsing
}  // namespace patinon

#endif  // BASE_DIR_PATINON_EXPLORATORY_ABC_GRAMMYS_PVN_TOKEN_H_
