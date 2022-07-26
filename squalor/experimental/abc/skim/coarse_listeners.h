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

#ifndef BASE_DIR_PATINON_EXPLORATORY_ABC_SKIM_COARSE_LISTENERS_H_
#define BASE_DIR_PATINON_EXPLORATORY_ABC_SKIM_COARSE_LISTENERS_H_

#include <cstddef>
#include <deque>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "base_dir/absl/container/flat_hash_set.h"
#include "base_dir/absl/strings/str_format.h"
#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/grammys/QvlParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/QvlParserBaseListener.h"
#include "base_dir/patinon/exploratory/abc/grammys/SvtParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/SvtParserBaseListener.h"
#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
#include "base_dir/patinon/exploratory/abc/skim/enhanced_parse_tree_property.h"
#include "base_dir/patinon/exploratory/abc/skim/summarizing.h"

namespace patinon {
namespace pvn_parsing {

class CoarseSkimSalientListener;

namespace util {

// Ugh! Anchor generation logic should be a (potentially optional) part of the
// skimming process.
Heading GetHeading(SvtParser::HeadingContext* ctx,
                   CoarseSkimSalientListener* skimmer);

inline std::pair<string, int> GetHeadingId(Heading heading) {
  return std::make_pair(heading.file_id, heading.line_number);
}

inline std::pair<string, int> GetHeadingId(SvtParser::HeadingContext* ctx) {
  antlr4::tree::TerminalNode* terminal_node = ctx->TITLE();
  if (terminal_node == nullptr) {
    terminal_node = ctx->HEADING();
  }
  if (terminal_node == nullptr) {
    terminal_node = ctx->TOC();
  }

  // Really an error.
  if (terminal_node == nullptr) {
    return std::make_pair(Heading::kDefaultFileId, 0);
  }

  return std::make_pair(Heading::kDefaultFileId,
                        terminal_node->getSymbol()->getLine());
}

}  // namespace util

class CoarseSkimSalientListener : public SvtParserBaseListener {
 public:
  explicit CoarseSkimSalientListener()
      : SvtParserBaseListener(),
        coarse_properties_(
            std::make_unique<AltParseTreeProperty<CoarseProperties>>()),
        summarizer_results_(std::make_unique<SummarizerResults>()),
        primary_nest_level_(0) {}

  void enterHeading(SvtParser::HeadingContext* ctx) override;

  void ReprocessHeadings();
  void visitTerminal(antlr4::tree::TerminalNode* ctx) override;
  void enterEveryRule(antlr4::ParserRuleContext* ctx) override;
  void exitEveryRule(antlr4::ParserRuleContext* ctx) override;

  inline std::unique_ptr<SummarizerResults> TakeSummarizerResults() {
    return std::move(summarizer_results_);
  }
  inline std::unique_ptr<AltParseTreeProperty<CoarseProperties>>
  TakeCoarseProperties() {
    return std::move(coarse_properties_);
  }

  // Make the heading's anchor before incrementing the counter.
  inline string MakeUniqueAnchor(string first_try) {
    auto insertion_pair = anchors_used_.insert(first_try);
    if (insertion_pair.second) {
      return first_try;
    } else {
      string uniquified = absl::StrFormat("%s_%d", first_try, heading_counter_);
      anchors_used_.insert(uniquified);
      return uniquified;
    }
  }

  inline int GetAndIncrementHeadingCounter() {
    int counter = heading_counter_;
    ++heading_counter_;
    return counter;
  }

 private:
  std::unique_ptr<AltParseTreeProperty<CoarseProperties>> coarse_properties_;
  std::unique_ptr<SummarizerResults> summarizer_results_;
  std::deque<Heading> heading_list_;  // Used only in construction.
  int heading_counter_ = 0;
  absl::flat_hash_set<string>
      anchors_used_;  // For uniquifying, construction only.
  TextualSubGenre tokenwise_sub_genre_ = TextualSubGenre::kNone;
  int primary_nest_level_;
};

class CoarseSkimQuarrelListener : public QvlParserBaseListener {
 public:
  explicit CoarseSkimQuarrelListener()
      : QvlParserBaseListener(),
        coarse_properties_(
            std::make_unique<AltParseTreeProperty<CoarseProperties>>()),
        summarizer_results_(std::make_unique<SummarizerResults>()),
        statement_nest_level_(0),
        cumulative_statement_level_(0),
        expression_nest_level_(0),
        nesting_stack_(),
        is_within_closures_(false) {}

  void ReprocessHeadings();

  inline std::unique_ptr<SummarizerResults> TakeSummarizerResults() {
    return std::move(summarizer_results_);
  }
  inline std::unique_ptr<AltParseTreeProperty<CoarseProperties>>
  TakeCoarseProperties() {
    return std::move(coarse_properties_);
  }

  void enterExpression(QvlParser::ExpressionContext* ctx) override;
  void enterOpenStmt(QvlParser::OpenStmtContext* ctx) override;
  // In order to ensure correct sequencing, we directly call some listener
  // methods from the generic rule methods.
  void visitTerminal(antlr4::tree::TerminalNode* ctx) override;
  void enterEveryRule(antlr4::ParserRuleContext* ctx) override;
  void exitEveryRule(antlr4::ParserRuleContext* ctx) override;

 private:
  void ManagedExitExpression();
  void ManagedExitCloseStmt();
  std::unique_ptr<AltParseTreeProperty<CoarseProperties>> coarse_properties_;
  std::unique_ptr<SummarizerResults> summarizer_results_;
  // std::deque<Heading> heading_list_;  // Used only in construction.
  // int heading_counter_ = 0;
  // absl::flat_hash_set<string>
  //     anchors_used_;  // For uniquifying, construction only.
  int statement_nest_level_;
  int cumulative_statement_level_;
  int expression_nest_level_;
  std::deque<std::pair<int, int>> nesting_stack_;
  bool is_within_closures_;
};

class StepwiseParseTreeWalker : public antlr4::tree::ParseTreeWalker {
 public:
  StepwiseParseTreeWalker(antlr4::tree::ParseTreeListener* listener,
                          antlr4::tree::ParseTree* t)
      : antlr4::tree::ParseTreeWalker(),
        listener_(listener),
        current_node_(t) {}
  virtual ~StepwiseParseTreeWalker() {}

  // virtual void walk() const override;
  // Note that the brokenness of C++ means that (around C++17) we cannot name
  // this function "walk()" to match IterativeParseTreeWalker.
  void Walk();
  // The typical function for this is to return node->children(), but it can
  // be customized for special nodes.
  virtual std::vector<antlr4::tree::ParseTree*>* GetChildren(
      antlr4::tree::ParseTree* node) {
    return &node->children;
  }

 protected:
  antlr4::tree::ParseTree* GetCurrentNode() const { return current_node_; }
  bool IsInOuterWalk() const { return in_outer_walk_; }

  bool WalkStep();

 private:
  // Members that are local variables in IterativeParseTreeWalker::walk().
  antlr4::tree::ParseTreeListener* listener_;
  antlr4::tree::ParseTree* current_node_;

  std::vector<antlr4::tree::ParseTree*> node_stack_;
  std::vector<size_t> index_stack_;

  size_t current_index_ = 0;
  // The outer walk control is state additional to that in
  // IterativeParseTreeWalker.
  bool in_outer_walk_ = true;
};

class IteratingWalker : public StepwiseParseTreeWalker {
 public:
  IteratingWalker(WalkerTransition walker_genre_state,
                  antlr4::tree::ParseTreeListener* listener,
                  antlr4::tree::ParseTree* t)
      : StepwiseParseTreeWalker(listener, t),
        walker_genre_state_(walker_genre_state) {}
  ~IteratingWalker() override {}

  // Returns next parsing mode, of kNone if completely done.
  const WalkerTransition& PreDepartureStep();
  const WalkerTransition& WalkStep();
  std::vector<antlr4::tree::ParseTree*>* GetChildren(
      antlr4::tree::ParseTree* node) override;

  // This could be changed to AddTransitions if multiple sets are supported in
  // the future.
  void SetTransitions(std::vector<WalkerTransition> walker_transitions) {
    walker_transitions_ = walker_transitions;
  }

  const WalkerTransition& GetGenreState() const { return walker_genre_state_; }
  void SetGenreState(const WalkerTransition& walker_genre_state) {
    walker_genre_state_ = walker_genre_state;
  }

 private:
  WalkerTransition walker_genre_state_;
  std::vector<WalkerTransition> walker_transitions_;
  int next_transition_index_index_ = 0;
};

}  // namespace pvn_parsing

}  // namespace patinon

#endif  // BASE_DIR_PATINON_EXPLORATORY_ABC_SKIM_COARSE_LISTENERS_H_
