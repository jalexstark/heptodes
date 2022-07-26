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

#ifndef BASE_DIR_PATINON_EXPLORATORY_ABC_GRAMMYS_DUAL_PARSING_H_
#define BASE_DIR_PATINON_EXPLORATORY_ABC_GRAMMYS_DUAL_PARSING_H_

#include <memory>
#include <string>
#include <vector>

#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/grammys/QvlParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/SvtParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/customized_lexing.h"

namespace patinon {
namespace pvn_parsing {

struct PatinonParserGroup {
 public:
  int CreateLexer(const string& infile);
  void LexerSetSalientMaster();
  void LexerSetQuarrelMaster();
  int DualLex();
  int DualParse();

  std::unique_ptr<antlr4::ANTLRInputStream> input;
  std::unique_ptr<CustomizedPvnLexer> lexer;
  std::unique_ptr<antlr4::CommonTokenStream> incommon_tokens;

  std::vector<std::unique_ptr<antlr4::Token>> salient_token_list;
  std::unique_ptr<antlr4::ListTokenSource> salient_token_source;
  std::unique_ptr<antlr4::CommonTokenStream> salient_tokens;
  std::unique_ptr<SvtParser> salient_parser;
  SvtParser::SalientTopContext* salient_tree;

  std::vector<std::unique_ptr<antlr4::Token>> quarrel_token_list;
  std::unique_ptr<antlr4::ListTokenSource> quarrel_token_source;
  std::unique_ptr<antlr4::CommonTokenStream> quarrel_tokens;
  std::unique_ptr<QvlParser> quarrel_parser;
  QvlParser::QuarrelTopContext* quarrel_tree;
  std::vector<antlr4::dfa::DFA> lexer_decision_to_dfa;
  antlr4::atn::PredictionContextCache lexer_shared_context_cache;
};

}  // namespace pvn_parsing
}  // namespace patinon

#endif  // BASE_DIR_PATINON_EXPLORATORY_ABC_GRAMMYS_DUAL_PARSING_H_
