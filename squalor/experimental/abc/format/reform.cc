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

// #include
// "base_dir/patinon/exploratory/abc/format/reform.h"

#include <iostream>
#include <memory>
#include <string>
#include <vector>

#include "base_dir/absl/flags/flag.h"
#include "base_dir/absl/flags/parse.h"
#include "base_dir/patinon/exploratory/abc/format/reform_listeners.h"
#include "base_dir/patinon/exploratory/abc/format/reform_utils.h"
#include "base_dir/patinon/exploratory/abc/grammys/QvlParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/SvtParser.h"
#include "base_dir/patinon/exploratory/abc/grammys/customized_lexing.h"
#include "base_dir/patinon/exploratory/abc/grammys/dual_parsing.h"
#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
#include "base_dir/patinon/exploratory/abc/skim/summarizing.h"

ABSL_FLAG(bool, debug_lexer_tokens, false, "Dump lexer tokens to output");
ABSL_FLAG(bool, debug_lexer_pass_through, false,
          "Check that merged lexer token channels reproduce original file");
ABSL_FLAG(bool, debug_inbound_quarrel, false,
          "Dump inbound Quarrel parse tree in simple lisp-like format");
ABSL_FLAG(bool, debug_inbound_salient, false,
          "Dump inbound Salient parse tree in simple lisp-like format");
ABSL_FLAG(bool, salient_to_html, false, "Convert Salient file to html");
ABSL_FLAG(bool, quarrel_to_html, false, "Convert Quarrel file to html");
ABSL_FLAG(bool, quarrel_reformat, false, "Reformat a Quarrel file");
ABSL_FLAG(bool, salient_reformat, false, "Reformat a Salient file");
ABSL_FLAG(std::string, in_file, "", "input file");
ABSL_FLAG(std::string, out_file, "", "output file");

namespace patinon {
namespace pvn_parsing {

namespace {

// Header declaration (old):
// int PipedDebug(const string& infile, const string& outfile);

int PipedDebug(const string& infile, const string& outfile) {
  std::ofstream out_stream;
  out_stream.open(outfile, std::ofstream::out);

  if (out_stream.fail()) {
    std::cerr << "Failed to open output file." << std::endl;
    return 1;
  }

  ParsingGenre parsing_genre = util::ChooseModeFromFileExtension(infile);
  if (parsing_genre == ParsingGenre::kNone) {
    std::cerr << "Failed to find parsing mode from file extension."
              << std::endl;
    return 1;
  }

  PatinonParserGroup parser_group;

  int lexer_retval = parser_group.CreateLexer(infile);
  if (lexer_retval != 0) {
    return lexer_retval;
  }

  // parser_group.lexer->fill();

  if (parsing_genre == ParsingGenre::kSalient) {
    parser_group.LexerSetSalientMaster();
  } else {
    parser_group.LexerSetQuarrelMaster();
  }

  int lexing_retval = parser_group.DualLex();
  if (lexing_retval != 0) {
    return lexing_retval;
  }

  if (absl::GetFlag(FLAGS_debug_lexer_tokens)) {
    util::DebugLexerTokens("EVERY", parser_group.incommon_tokens.get(),
                           *parser_group.lexer, out_stream);
    return 0;
  }

  if (absl::GetFlag(FLAGS_debug_lexer_pass_through)) {
    util::DebugLexerPassThrough(parser_group.incommon_tokens.get(), out_stream);
    return 0;
  }

  int parse_retval = parser_group.DualParse();
  if (parse_retval != 0) {
    return parse_retval;
  }

  if (absl::GetFlag(FLAGS_debug_inbound_quarrel)) {
    util::DebugSimpleParseTree(parser_group.quarrel_parser.get(),
                               parser_group.quarrel_tree, out_stream);
  }

  if (absl::GetFlag(FLAGS_debug_inbound_salient)) {
    util::DebugSimpleParseTree(parser_group.salient_parser.get(),
                               parser_group.salient_tree, out_stream);
  }

  WalkerTransition salient_outer(
      {0, ParsingGenre::kSalient, TextualSubGenre::kMaster});
  WalkerTransition quarrel_outer(
      {0, ParsingGenre::kQuarrel, TextualSubGenre::kNone});

  if (absl::GetFlag(FLAGS_salient_to_html)) {
    PerformConversion<QuarrelToHtmlListener, SalientToHtmlListener>(
        salient_outer, parser_group.quarrel_tree, parser_group.salient_tree,
        parser_group.quarrel_tokens.get(), parser_group.salient_tokens.get(),
        out_stream);
  }

  if (absl::GetFlag(FLAGS_quarrel_to_html)) {
    PerformConversion<QuarrelToHtmlListener, SalientToHtmlListener>(
        quarrel_outer, parser_group.quarrel_tree, parser_group.salient_tree,
        parser_group.quarrel_tokens.get(), parser_group.salient_tokens.get(),
        out_stream);
  }

  if (absl::GetFlag(FLAGS_quarrel_reformat)) {
    PerformConversion<QuarrelReformatListener, SalientReformatListener>(
        quarrel_outer, parser_group.quarrel_tree, parser_group.salient_tree,
        parser_group.quarrel_tokens.get(), parser_group.salient_tokens.get(),
        out_stream);
  }

  if (absl::GetFlag(FLAGS_salient_reformat)) {
    PerformConversion<QuarrelReformatListener, SalientReformatListener>(
        salient_outer, parser_group.quarrel_tree, parser_group.salient_tree,
        parser_group.quarrel_tokens.get(), parser_group.salient_tokens.get(),
        out_stream);
  }

  return 0;
}

}  // namespace

}  // namespace pvn_parsing
}  // namespace patinon

int main(int argc, char* argv[]) {
  std::vector<char*> positional_args = absl::ParseCommandLine(argc, argv);

  if (positional_args.size() != 1) {
    std::cerr << "Unrecognized extra arguments" << std::endl;
    return -1;
  }
  if (absl::GetFlag(FLAGS_in_file).empty()) {
    std::cerr << "Missing input file argument" << std::endl;
    return -1;
  }
  if (absl::GetFlag(FLAGS_out_file).empty()) {
    std::cerr << "Missing output file argument" << std::endl;
    return -1;
  }

  return patinon::pvn_parsing::PipedDebug(absl::GetFlag(FLAGS_in_file),
                                          absl::GetFlag(FLAGS_out_file));
}
