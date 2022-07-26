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

#ifndef BASE_DIR_PATINON_EXPLORATORY_ABC_FORMAT_REFORM_LISTENERS_H_
#define BASE_DIR_PATINON_EXPLORATORY_ABC_FORMAT_REFORM_LISTENERS_H_

#include <iosfwd>
#include <memory>
#include <utility>

// Break coarse listener dependency with extraction to structure, and depend
// instead on skim/summarizing.h.

#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/format/base_handler.h"
#include "base_dir/patinon/exploratory/abc/format/reform_handlers.h"
#include "base_dir/patinon/exploratory/abc/grammys/QvlParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/QvlParserBaseListener.h"
#include "base_dir/patinon/exploratory/abc/grammys/SvtParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/SvtParserBaseListener.h"
#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
#include "base_dir/patinon/exploratory/abc/skim/coarse_listeners.h"
#include "base_dir/patinon/exploratory/abc/skim/enhanced_parse_tree_property.h"
#include "base_dir/patinon/exploratory/abc/skim/summarizing.h"

namespace patinon {
namespace pvn_parsing {

enum class ListLayout {
  kCompact,
  kSpacious,
};

class SalientToHtmlListener : public SvtParserBaseListener {
 public:
  SalientToHtmlListener(
      std::unique_ptr<SummarizerResults> summarizer_results,
      std::unique_ptr<AltParseTreeProperty<CoarseProperties>> coarse_properties,
      antlr4::CommonTokenStream* salient_tokens, std::ofstream& out_stream)
      : SvtParserBaseListener(),
        summarizer_results_(std::move(summarizer_results)),
        handler_(new SalientToHtmlHandler(
            out_stream, std::move(coarse_properties),
            {util::SpecificAnyWhitespaceMap(), util::AnySpecificWhitespaceMap(),
             util::SpecificSpecificWhitespaceMap()},
            salient_tokens)) {}

  void enterSalientTop(SvtParser::SalientTopContext* ctx) override;
  void exitSalientTop(SvtParser::SalientTopContext* ctx) override;
  void enterHeading(SvtParser::HeadingContext* ctx) override;
  void enterParaBlock(SvtParser::ParaBlockContext* ctx) override;

  void enterEitherList(SvtParser::EitherListContext* ctx) override;
  void exitEitherList(SvtParser::EitherListContext* ctx) override;

  void enterLinearContent(SvtParser::LinearContentContext* ctx) override;
  void exitLinearContent(SvtParser::LinearContentContext* ctx) override;

  void enterListItem(SvtParser::ListItemContext* ctx) override;
  void exitListItem(SvtParser::ListItemContext* ctx) override;

  void enterEveryRule(antlr4::ParserRuleContext* ctx) override;
  void exitEveryRule(antlr4::ParserRuleContext* ctx) override;
  void visitTerminal(antlr4::tree::TerminalNode* ctx) override;

  SummarizerResults* GetSummarizerResults() {
    return summarizer_results_.get();
  }
  SalientToHtmlHandler* GetHandler() { return handler_.get(); }

 private:
  bool DisableAutoSubtreeForRule(antlr4::ParserRuleContext* ctx);

  std::unique_ptr<SummarizerResults> summarizer_results_;
  std::unique_ptr<SalientToHtmlHandler> handler_;
  int disableNormalSubtreeContent = 0;
};

class QuarrelToHtmlListener : public QvlParserBaseListener {
 public:
  QuarrelToHtmlListener(
      std::unique_ptr<SummarizerResults> summarizer_results,
      std::unique_ptr<AltParseTreeProperty<CoarseProperties>> coarse_properties,
      antlr4::CommonTokenStream* quarrel_tokens, std::ofstream& out_stream)
      : QvlParserBaseListener(),
        summarizer_results_(std::move(summarizer_results)),
        handler_(new QuarrelToHtmlHandler(
            out_stream, std::move(coarse_properties),
            {util::SpecificAnyWhitespaceMap(), util::AnySpecificWhitespaceMap(),
             util::SpecificSpecificWhitespaceMap()},
            quarrel_tokens)) {}

  void enterQuarrelTop(QvlParser::QuarrelTopContext* ctx) override;
  void exitQuarrelTop(QvlParser::QuarrelTopContext* ctx) override;
  void enterEveryRule(antlr4::ParserRuleContext* ctx) override;
  void exitEveryRule(antlr4::ParserRuleContext* ctx) override;
  void visitTerminal(antlr4::tree::TerminalNode* ctx) override;

  SummarizerResults* GetSummarizerResults() {
    return summarizer_results_.get();
  }
  QuarrelToHtmlHandler* GetHandler() { return handler_.get(); }

 private:
  std::unique_ptr<SummarizerResults> summarizer_results_;
  std::unique_ptr<QuarrelToHtmlHandler> handler_;
};

class QuarrelReformatListener : public QvlParserBaseListener {
 public:
  QuarrelReformatListener(
      std::unique_ptr<SummarizerResults> summarizer_results,
      std::unique_ptr<AltParseTreeProperty<CoarseProperties>> coarse_properties,
      antlr4::CommonTokenStream* quarrel_tokens, std::ofstream& out_stream)
      : QvlParserBaseListener(),
        summarizer_results_(std::move(summarizer_results)),
        handler_(new QuarrelReformatHandler(
            out_stream, std::move(coarse_properties),
            {util::SpecificAnyWhitespaceMap(), util::AnySpecificWhitespaceMap(),
             util::SpecificSpecificWhitespaceMap()},
            quarrel_tokens)) {}

  void visitTerminal(antlr4::tree::TerminalNode* ctx) override;
  void enterStatement(QvlParser::StatementContext* ctx) override;
  void exitEveryRule(antlr4::ParserRuleContext* ctx) override;

  SummarizerResults* GetSummarizerResults() {
    return summarizer_results_.get();
  }
  QuarrelReformatHandler* GetHandler() { return handler_.get(); }

 private:
  // Output lines if needed in places such as at the beginning of terminal
  // (token processing).
  void MaybeOutputLines(antlr4::tree::TerminalNode* ctx);

  std::unique_ptr<SummarizerResults> summarizer_results_;
  std::unique_ptr<QuarrelReformatHandler> handler_;
};

class SalientReformatListener : public SvtParserBaseListener {
 public:
  SalientReformatListener(
      std::unique_ptr<SummarizerResults> summarizer_results,
      std::unique_ptr<AltParseTreeProperty<CoarseProperties>> coarse_properties,
      antlr4::CommonTokenStream* salient_tokens, std::ofstream& out_stream)
      : SvtParserBaseListener(),
        summarizer_results_(std::move(summarizer_results)),
        handler_(new SalientReformatHandler(
            out_stream, std::move(coarse_properties),
            {util::SpecificAnyWhitespaceMap(), util::AnySpecificWhitespaceMap(),
             util::SpecificSpecificWhitespaceMap()},
            salient_tokens)) {}

  void visitTerminal(antlr4::tree::TerminalNode* ctx) override;
  void enterEveryRule(antlr4::ParserRuleContext* ctx) override;
  void exitEveryRule(antlr4::ParserRuleContext* ctx) override;

  SummarizerResults* GetSummarizerResults() {
    return summarizer_results_.get();
  }
  SalientReformatHandler* GetHandler() { return handler_.get(); }

 private:
  // Output lines if needed in places such as at the beginning of terminal
  // (token processing).
  void MaybeOutputLines(antlr4::tree::TerminalNode* ctx);

  std::unique_ptr<SummarizerResults> summarizer_results_;
  std::unique_ptr<SalientReformatHandler> handler_;
};

template <class QuarrelListenerType, class SalientListenerType>
void PerformConversion(WalkerTransition outer_genre_state,
                       antlr4::ParserRuleContext* quarrel_tree,
                       antlr4::ParserRuleContext* salient_tree,
                       antlr4::CommonTokenStream* quarrel_tokens,
                       antlr4::CommonTokenStream* salient_tokens,
                       std::ofstream& out_stream) {
  std::unique_ptr<AltParseTreeProperty<CoarseProperties>>
      quarrel_coarse_properties;
  std::unique_ptr<AltParseTreeProperty<CoarseProperties>>
      salient_coarse_properties;
  std::unique_ptr<SummarizerResults> quarrel_summarizer_results;
  std::unique_ptr<SummarizerResults> salient_summarizer_results;

  {
    CoarseSkimQuarrelListener quarrel_summarizer_listener;
    antlr4::tree::ParseTreeWalker summarizer_walker;
    summarizer_walker.walk(&quarrel_summarizer_listener, quarrel_tree);
    quarrel_summarizer_listener.ReprocessHeadings();

    quarrel_coarse_properties =
        quarrel_summarizer_listener.TakeCoarseProperties();
    quarrel_summarizer_results =
        quarrel_summarizer_listener.TakeSummarizerResults();
    quarrel_summarizer_results->outer_genre_state = outer_genre_state;
  }

  CoarseSkimSalientListener salient_summarizer_listener;
  {
    antlr4::tree::ParseTreeWalker walker;
    walker.walk(&salient_summarizer_listener, salient_tree);
    salient_summarizer_listener.ReprocessHeadings();

    salient_coarse_properties =
        salient_summarizer_listener.TakeCoarseProperties();
    salient_summarizer_results =
        salient_summarizer_listener.TakeSummarizerResults();
    salient_summarizer_results->outer_genre_state = outer_genre_state;
  }

  SalientListenerType salient_conversion_listener(
      std::move(salient_summarizer_results),
      std::move(salient_coarse_properties), salient_tokens, out_stream);
  QuarrelListenerType quarrel_conversion_listener(
      std::move(quarrel_summarizer_results),
      std::move(quarrel_coarse_properties), quarrel_tokens, out_stream);

  IteratingWalker quarrel_iterating_walker(
      outer_genre_state, &quarrel_conversion_listener, quarrel_tree);
  IteratingWalker salient_iterating_walker(
      outer_genre_state, &salient_conversion_listener, salient_tree);
  quarrel_iterating_walker.SetTransitions(
      salient_conversion_listener.GetSummarizerResults()
          ->quarrel_to_salient_transitions);
  salient_iterating_walker.SetTransitions(
      salient_conversion_listener.GetSummarizerResults()
          ->salient_to_quarrel_transitions);

  WalkerTransition current_genre_state = outer_genre_state;
  InterModeFormatting inter_mode_format_transfer;
  do {
    while (current_genre_state.destination_genre == ParsingGenre::kQuarrel) {
      quarrel_conversion_listener.GetHandler()->inter_mode_formatting =
          inter_mode_format_transfer;
      quarrel_conversion_listener.GetHandler()->entry_genre_state =
          current_genre_state;

      quarrel_iterating_walker.SetGenreState(current_genre_state);
      current_genre_state = quarrel_iterating_walker.PreDepartureStep();

      if (current_genre_state.destination_genre == ParsingGenre::kQuarrel) {
        current_genre_state = quarrel_iterating_walker.WalkStep();
      }
      if (current_genre_state.destination_genre != ParsingGenre::kQuarrel) {
        quarrel_conversion_listener.GetHandler()->OutputLines(
            true /*=grammar_flush*/, current_genre_state);
      }
      // We pass current_genre_state on to the next walker: it was updated as we
      // went along.
      inter_mode_format_transfer =
          quarrel_conversion_listener.GetHandler()->inter_mode_formatting;
    }

    while (current_genre_state.destination_genre == ParsingGenre::kSalient) {
      salient_conversion_listener.GetHandler()->inter_mode_formatting =
          inter_mode_format_transfer;
      salient_conversion_listener.GetHandler()->entry_genre_state =
          current_genre_state;

      salient_iterating_walker.SetGenreState(current_genre_state);
      current_genre_state = salient_iterating_walker.PreDepartureStep();

      if (current_genre_state.destination_genre == ParsingGenre::kSalient) {
        current_genre_state = salient_iterating_walker.WalkStep();
      }
      if (current_genre_state.destination_genre != ParsingGenre::kSalient) {
        salient_conversion_listener.GetHandler()->OutputLines(
            true /*=grammar_flush*/, current_genre_state);
      }
      // We pass current_genre_state on to the next walker: it was updated as we
      // went along.
      inter_mode_format_transfer =
          salient_conversion_listener.GetHandler()->inter_mode_formatting;
    }
  } while (current_genre_state.destination_genre != ParsingGenre::kNone);
}

}  // namespace pvn_parsing
}  // namespace patinon

#endif  // BASE_DIR_PATINON_EXPLORATORY_ABC_FORMAT_REFORM_LISTENERS_H_
