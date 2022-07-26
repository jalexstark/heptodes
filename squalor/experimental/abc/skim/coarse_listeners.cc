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

#include "base_dir/patinon/exploratory/abc/skim/coarse_listeners.h"

#include <algorithm>
#include <cstddef>
#include <deque>
#include <iostream>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "base_dir/absl/container/flat_hash_map.h"
#include "base_dir/absl/memory/memory.h"
#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/grammys/PvnLexer.h"
#include "base_dir/patinon/exploratory/abc/grammys/QvlParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/SvtParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
#include "base_dir/patinon/exploratory/abc/skim/enhanced_parse_tree_property.h"
#include "base_dir/patinon/exploratory/abc/skim/summarizing.h"

namespace patinon {
namespace pvn_parsing {

void CoarseSkimSalientListener::enterEveryRule(antlr4::ParserRuleContext* ctx) {
  switch (ctx->getRuleIndex()) {
    case SvtParser::RuleListItem:
      ++primary_nest_level_;
      break;
    default:
      break;
  }

  // This has a silly name because we would like to correct this in the future.
  CoarseProperties cpp_makes_moving_fragile;
  cpp_makes_moving_fragile.sub_genre = tokenwise_sub_genre_;
  cpp_makes_moving_fragile.statement_nest_level = primary_nest_level_;
  coarse_properties_->put(ctx, cpp_makes_moving_fragile);
}

void CoarseSkimSalientListener::exitEveryRule(antlr4::ParserRuleContext* ctx) {
  switch (ctx->getRuleIndex()) {
    case SvtParser::RuleListItem:
      --primary_nest_level_;
      break;
    default:
      break;
  }
  // This has a silly name because we would like to correct this in the future.
  CoarseProperties cpp_makes_moving_fragile = coarse_properties_->get(ctx);
  cpp_makes_moving_fragile.sub_genre = tokenwise_sub_genre_;
  cpp_makes_moving_fragile.statement_nest_level = primary_nest_level_;
  coarse_properties_->put(ctx, cpp_makes_moving_fragile);
}

void CoarseSkimSalientListener::visitTerminal(antlr4::tree::TerminalNode* ctx) {
  if ((ctx->getSymbol()->getType() == PvnLexer::LEAVE_CODE) ||
      (ctx->getSymbol()->getType() == PvnLexer::ENTER_TEXTUAL)) {
    if (ctx->getText() == ";;;") {
      tokenwise_sub_genre_ = TextualSubGenre::kTextualLeft;
    } else if (ctx->getText() == ";;") {
      tokenwise_sub_genre_ = TextualSubGenre::kTextualIndent;
    } else if (ctx->getText() == ";") {
      tokenwise_sub_genre_ = TextualSubGenre::kTextualRight;
    } else {
      tokenwise_sub_genre_ = TextualSubGenre::kMaster;
    }

    summarizer_results_->quarrel_to_salient_transitions.push_back(
        {ctx->getSymbol()->getTokenIndex(), ParsingGenre::kSalient,
         tokenwise_sub_genre_});
  }

  // This has a silly name because we would like to correct this in the future.
  CoarseProperties cpp_makes_moving_fragile;
  cpp_makes_moving_fragile.sub_genre = tokenwise_sub_genre_;
  cpp_makes_moving_fragile.statement_nest_level = primary_nest_level_;
  coarse_properties_->put(ctx, cpp_makes_moving_fragile);

  if ((ctx->getSymbol()->getType() == PvnLexer::NEWLINE_ENTER_CODE) ||
      (ctx->getSymbol()->getType() == PvnLexer::LEAVE_TEXTUAL)) {
    tokenwise_sub_genre_ = TextualSubGenre::kMaster;

    summarizer_results_->salient_to_quarrel_transitions.push_back(
        {ctx->getSymbol()->getTokenIndex(), ParsingGenre::kQuarrel,
         tokenwise_sub_genre_});
  }
}

void CoarseSkimSalientListener::enterHeading(SvtParser::HeadingContext* ctx) {
  heading_list_.push_back(util::GetHeading(ctx, this));
}

void CoarseSkimSalientListener::ReprocessHeadings() {
  int current_nesting = 0;
  summarizer_results_->heading_vector.resize(0);
  summarizer_results_->heading_vector.reserve(heading_list_.size());
  for (const auto& h : heading_list_) {
    // It is fine to move up one nesting level automatically.
    ++current_nesting;
    if (current_nesting > h.level) {
      current_nesting = h.level;
    }
    while (current_nesting < h.level) {
      Heading dummy_heading;
      dummy_heading.level = current_nesting;
      dummy_heading.heading_text = "Heading missing at this level";
      summarizer_results_->heading_vector.push_back(dummy_heading);
      ++current_nesting;
    }
    summarizer_results_->heading_vector.push_back(h);
    // Also insert into hash map.

    int new_heading_index = summarizer_results_->heading_vector.size() - 1;

    summarizer_results_->heading_indices[util::GetHeadingId(h)] =
        new_heading_index;

    if (h.level == 0) {
      if (summarizer_results_->title_heading_index ==
          SummarizerResults::kNoDetectedTitle) {
        summarizer_results_->title_heading_index = new_heading_index;
      } else {
        std::cerr << "More than one title-level heading (module, doc title)";
      }
    }
  }
  Heading dummy_heading;
  dummy_heading.level = 0;
  dummy_heading.heading_text = "Convenience trailing level-0 heading.";
  summarizer_results_->heading_vector.push_back(dummy_heading);
}

void CoarseSkimQuarrelListener::ReprocessHeadings() {}

void CoarseSkimQuarrelListener::enterExpression(
    QvlParser::ExpressionContext* ctx) {
  ++expression_nest_level_;
}
void CoarseSkimQuarrelListener::ManagedExitExpression() {
  expression_nest_level_ = std::max(0, expression_nest_level_ - 1);
}

void CoarseSkimQuarrelListener::enterOpenStmt(QvlParser::OpenStmtContext* ctx) {
  if (expression_nest_level_ != 0) {
    nesting_stack_.emplace_front(statement_nest_level_, expression_nest_level_);
    statement_nest_level_ = 0;
    expression_nest_level_ = 0;
  } else {
    ++statement_nest_level_;
  }
  ++cumulative_statement_level_;
}

void CoarseSkimQuarrelListener::ManagedExitCloseStmt() {
  if (statement_nest_level_ != 0) {
    --statement_nest_level_;
  } else if (!nesting_stack_.empty()) {
    const auto pair = nesting_stack_.front();
    nesting_stack_.pop_front();
    statement_nest_level_ = pair.first;
    expression_nest_level_ = pair.second;
  }
  --cumulative_statement_level_;
}

void CoarseSkimQuarrelListener::visitTerminal(antlr4::tree::TerminalNode* ctx) {
  is_within_closures_ = false;
  // This has a silly name because we would like to correct this in the future.
  CoarseProperties cpp_makes_moving_fragile;
  cpp_makes_moving_fragile.is_closure = false;
  cpp_makes_moving_fragile.statement_nest_level = statement_nest_level_;
  cpp_makes_moving_fragile.cumulative_statement_level =
      cumulative_statement_level_;
  cpp_makes_moving_fragile.expression_nest_level = expression_nest_level_;
  cpp_makes_moving_fragile.nesting_stack_depth = nesting_stack_.size();
  coarse_properties_->put(ctx, cpp_makes_moving_fragile);
}

void CoarseSkimQuarrelListener::enterEveryRule(antlr4::ParserRuleContext* ctx) {
  // This has a silly name because we would like to correct this in the future.
  CoarseProperties cpp_makes_moving_fragile;
  coarse_properties_->put(ctx, cpp_makes_moving_fragile);
  is_within_closures_ = false;
}

void CoarseSkimQuarrelListener::exitEveryRule(antlr4::ParserRuleContext* ctx) {
  bool is_closure = false;
  switch (ctx->getRuleIndex()) {
    case QvlParser::RuleCloseStmt:
    case QvlParser::RuleSpliceStmt:
      ManagedExitCloseStmt();
      is_closure = true;
      break;
    case QvlParser::RuleExpression:
      ManagedExitExpression();
      is_closure = true;
      break;
    default:
      break;
  }

  is_within_closures_ = is_within_closures_ || is_closure;
  // This has a silly name because we would like to correct this in the future.
  CoarseProperties cpp_makes_moving_fragile = coarse_properties_->get(ctx);
  cpp_makes_moving_fragile.is_closure = is_within_closures_;
  cpp_makes_moving_fragile.statement_nest_level = statement_nest_level_;
  cpp_makes_moving_fragile.cumulative_statement_level =
      cumulative_statement_level_;
  cpp_makes_moving_fragile.expression_nest_level = expression_nest_level_;
  cpp_makes_moving_fragile.nesting_stack_depth = nesting_stack_.size();
  coarse_properties_->put(ctx, cpp_makes_moving_fragile);
}

// void IterativeParseTreeWalker::walk(...) const {
//
//   <Vars that are now class members>
//
//   while (currentNode != nullptr) {
//     <Code section A>
//
//     do {
//       <Inner code section B>
//     } while (currentNode != nullptr);
//   }
// }
//
// And we want to replace
// walker.walk();
// with
// do {} while (iterating_walker.WalkStep() != ParsingGenre::kNone);
//
// and we do this by using in_outer_walk_==true to indicate iff section A
// should be executed on the next call to WalkStep().
bool StepwiseParseTreeWalker::WalkStep() {
  using antlr4::tree::ErrorNode;
  using antlr4::tree::TerminalNode;

  if (current_node_ == nullptr) {
    return false;
  }

  if (in_outer_walk_) {
    in_outer_walk_ = false;

    // Pre-order visit
    if (antlrcpp::is<ErrorNode*>(current_node_)) {
      listener_->visitErrorNode(dynamic_cast<ErrorNode*>(current_node_));
    } else if (antlrcpp::is<TerminalNode*>(current_node_)) {
      listener_->visitTerminal(static_cast<TerminalNode*>(current_node_));
    } else {
      enterRule(listener_, current_node_);
    }

    // Listener is allowed to construct a vector of children during enterRule()
    // and then provide it unchanged between then and exitRule();

    std::vector<antlr4::tree::ParseTree*>* children =
        GetChildren(current_node_);

    // Move down to first child, if it exists.
    if (!children->empty()) {
      node_stack_.push_back(current_node_);
      index_stack_.push_back(current_index_);
      current_index_ = 0;
      current_node_ = (*children)[0];
      in_outer_walk_ = true;
    }

  } else {
    // Inner loop does:
    // No child nodes, so walk tree.
    {
      // Each inner loop iteration does:
      // post-order visit
      if (!antlrcpp::is<TerminalNode*>(current_node_)) {
        exitRule(listener_, current_node_);
      }

      // No parent, so no siblings.
      if (node_stack_.empty()) {
        current_node_ = nullptr;
        current_index_ = 0;
        in_outer_walk_ = true;
        return false;
        // Maybe need to account for final switch to other grammar, but no need
        // for bookkeeping, because this walker's tree will now be null.
      } else {
        ++current_index_;
        std::vector<antlr4::tree::ParseTree*>* children =
            GetChildren(node_stack_.back());

        if (children->size() > current_index_) {
          // Move to next sibling if possible.
          current_node_ = (*children)[current_index_];
          in_outer_walk_ = true;
          return true;
        } else {
          // No next sibling, so move up.
          current_node_ = node_stack_.back();
          node_stack_.pop_back();
          current_index_ = index_stack_.back();
          index_stack_.pop_back();
        }
      }
    }
  }
  return (current_node_ != nullptr);
}

void StepwiseParseTreeWalker::Walk() {
  while (WalkStep()) {
  }
}

std::vector<antlr4::tree::ParseTree*>* IteratingWalker::GetChildren(
    antlr4::tree::ParseTree* node) {
  return &node->children;
}

const WalkerTransition& IteratingWalker::PreDepartureStep() {
  antlr4::tree::ParseTree* current_node = GetCurrentNode();
  if (current_node == nullptr) {
    walker_genre_state_.destination_genre = ParsingGenre::kNone;
  } else if (IsInOuterWalk()) {
    // This, in cooperation with the caller, effectively suspends this walker
    // and switches processing to the destination mode.
    if ((next_transition_index_index_ < walker_transitions_.size()) &&
        (walker_transitions_[next_transition_index_index_].token_index <
         current_node->getSourceInterval().a)) {
      ++next_transition_index_index_;

      walker_genre_state_ =
          walker_transitions_[next_transition_index_index_ - 1];
    }
  }

  return walker_genre_state_;
}

const WalkerTransition& IteratingWalker::WalkStep() {
  bool walk_result = StepwiseParseTreeWalker::WalkStep();
  if (!walk_result) {
    walker_genre_state_.destination_genre = ParsingGenre::kNone;
  }
  return walker_genre_state_;
}

}  // namespace pvn_parsing

}  // namespace patinon
