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

#include "base_dir/patinon/exploratory/abc/grammys/dual_parsing.h"

#include <iostream>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "base_dir/absl/debugging/leak_check.h"
#include "base_dir/absl/memory/memory.h"
#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/grammys/PvnLexer.h"
#include "base_dir/patinon/exploratory/abc/grammys/QvlParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/SvtParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/customized_lexing.h"
#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
#include "base_dir/patinon/exploratory/abc/grammys/pvn_token.h"

namespace patinon {
namespace pvn_parsing {

// ANTLRInputStream input(prepared_input_string);
// input_string_structureLexer lexer(&input);
// CommonTokenStream tokens(&lexer);
// input_string_structureParser parser(&tokens);
//   try {
//     // This is the actual antlr parse.
//     antlr4::tree::ParseTree* root = parser.start();
//
//     // Now we can walk the tree and generate the tree in our format.
//     OurTree* result = visitor.visit(root);
//     return absl::WrapUnique(result);
//   } catch (antlr4::ParseCancellationException) {
//     LOG(ERROR) << "Parse cancelled: " << input_string;
//     return std::unique_ptr<OurTree>(nullptr);
//   } catch (...) {
//     // Not sure if ParseCancellationException catches everything.
//     LOG(ERROR) << "Unexpected exception: " << input_string;
//     return std::unique_ptr<OurTree>(nullptr);
//   }

int PatinonParserGroup::CreateLexer(const string& infile) {
  std::ifstream in_stream;
  in_stream.open(infile, std::ifstream::in);
  if (in_stream.fail()) {
    std::cerr << "Failed to open input file." << std::endl;
    return 1;
  }

  input = absl::MakeUnique<antlr4::ANTLRInputStream>(in_stream);
  lexer = absl::MakeUnique<CustomizedPvnLexer>(input.get());

  lexer_decision_to_dfa = std::move(
      lexer->getInterpreter<antlr4::atn::LexerATNSimulator>()->_decisionToDFA);
  lexer->setInterpreter(new antlr4::atn::LexerATNSimulator(
      lexer.get(), lexer->getATN(), lexer_decision_to_dfa,
      lexer_shared_context_cache));

  return 0;
}

void PatinonParserGroup::LexerSetSalientMaster() {
  lexer->textual_factory = std::make_unique<SalientModeContextFactory>(
      PvnLexer::DEFAULT_TOKEN_CHANNEL, PvnLexer::SALIENT);
  lexer->code_factory = std::make_unique<QuarrelModeContextFactory>(
      PvnLexer::SECONDARY_CHANNEL, PvnLexer::QUARREL);

  lexer->PushEnter(TextualSubGenre::kMaster, nullptr,
                   lexer->textual_factory.get());
}

void PatinonParserGroup::LexerSetQuarrelMaster() {
  lexer->textual_factory = std::make_unique<SalientModeContextFactory>(
      PvnLexer::SECONDARY_CHANNEL, PvnLexer::SALIENT);
  lexer->code_factory = std::make_unique<QuarrelModeContextFactory>(
      PvnLexer::DEFAULT_TOKEN_CHANNEL, PvnLexer::QUARREL);

  lexer->PushEnter(TextualSubGenre::kMaster, nullptr,
                   lexer->code_factory.get());
}

int PatinonParserGroup::DualLex() {
  incommon_tokens = absl::MakeUnique<antlr4::CommonTokenStream>(
      lexer.get(), lexer->textual_factory->GetChannel());
  {
    absl::LeakCheckDisabler disabler;
    incommon_tokens->fill();
  }

  // Create token source and stream for Salient by extracting list of tokens
  // from the in-common stream.
  salient_token_list.resize(0);
  for (const auto token : incommon_tokens->getTokens()) {
    // Note that this copies the tokens.

    CustomizedToken* c_token = dynamic_cast<CustomizedToken*>(token);
    if (c_token != nullptr) {
      salient_token_list.push_back(absl::MakeUnique<CustomizedToken>(*c_token));
    } else {
      salient_token_list.push_back(
          absl::MakeUnique<antlr4::CommonToken>(token));
    }
  }
  if (salient_token_list.empty()) {
    std::cerr << "Empty set of tokens";
    return 99;
  }
  salient_token_source =
      absl::MakeUnique<antlr4::ListTokenSource>(std::move(salient_token_list));
  salient_tokens = absl::MakeUnique<antlr4::CommonTokenStream>(
      salient_token_source.get(), lexer->textual_factory->GetChannel());

  // Create token source and stream for Quarrel by extracting list of tokens
  // from the in-common stream.
  quarrel_token_list.resize(0);
  for (const auto token : incommon_tokens->getTokens()) {
    // Note that this copies the tokens.

    CustomizedToken* c_token = dynamic_cast<CustomizedToken*>(token);
    if (c_token != nullptr) {
      quarrel_token_list.push_back(absl::MakeUnique<CustomizedToken>(*c_token));
    } else {
      quarrel_token_list.push_back(
          absl::MakeUnique<antlr4::CommonToken>(token));
    }
  }
  if (quarrel_token_list.empty()) {
    std::cerr << "Empty set of tokens";
    return 99;
  }
  quarrel_token_source =
      absl::MakeUnique<antlr4::ListTokenSource>(std::move(quarrel_token_list));
  quarrel_tokens = absl::MakeUnique<antlr4::CommonTokenStream>(
      quarrel_token_source.get(), lexer->code_factory->GetChannel());

  return 0;
}

int PatinonParserGroup::DualParse() {
  salient_parser = absl::MakeUnique<SvtParser>(incommon_tokens.get());
  // The following may be unnecessary. Once we have a lot of examples to parse,
  // and can run santizers over them, we can revert to simpler parser calling.
  std::vector<antlr4::dfa::DFA> salient_decisionToDFA = std::move(
      salient_parser->getInterpreter<antlr4::atn::ParserATNSimulator>()
          ->decisionToDFA);
  antlr4::atn::PredictionContextCache salient_sharedContextCache =
      antlr4::atn::PredictionContextCache();
  salient_parser->setInterpreter(new antlr4::atn::ParserATNSimulator(
      salient_parser.get(), salient_parser->getATN(), salient_decisionToDFA,
      salient_sharedContextCache));

  salient_tree = salient_parser->salientTop();

  quarrel_parser = absl::MakeUnique<QvlParser>(quarrel_tokens.get());
  quarrel_tree = quarrel_parser->quarrelTop();

  return 0;
}

}  // namespace pvn_parsing
}  // namespace patinon
