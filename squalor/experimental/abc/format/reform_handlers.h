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

#ifndef BASE_DIR_PATINON_EXPLORATORY_ABC_FORMAT_REFORM_HANDLERS_H_
#define BASE_DIR_PATINON_EXPLORATORY_ABC_FORMAT_REFORM_HANDLERS_H_

#include <memory>
#include <ostream>
#include <string>
#include <utility>
#include <vector>

#include "base_dir/absl/container/flat_hash_map.h"
#include "base_dir/absl/memory/memory.h"
#include "base_dir/absl/strings/str_format.h"
#include "base_dir/absl/strings/string_view.h"
#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/format/base_handler.h"
#include "base_dir/patinon/exploratory/abc/skim/enhanced_parse_tree_property.h"
#include "base_dir/patinon/exploratory/abc/skim/summarizing.h"

namespace patinon {
namespace pvn_parsing {

class SalientToHtmlHandler : public MiddleHandler {
 public:
  SalientToHtmlHandler(std::ofstream& out_stream_x,
                       std::unique_ptr<AltParseTreeProperty<CoarseProperties>>
                           coarse_properties_x,
                       WhitespaceMaps whitespace_maps_x,
                       antlr4::CommonTokenStream* tokens_x)
      : MiddleHandler(out_stream_x, std::move(coarse_properties_x),
                      whitespace_maps_x, util::CreateHtmlPairMap(), tokens_x) {}

  void OutputLines(bool full_flush, const WalkerTransition& next_genre_state);

  inline void CloseSectionsTo(int new_level) {
    ++current_section_depth;
    while (current_section_depth > new_level) {
      out_stream << absl::StrFormat("</section>\n");
      --current_section_depth;
    }
  }
};

class QuarrelToHtmlHandler : public MiddleHandler {
 public:
  QuarrelToHtmlHandler(std::ofstream& out_stream_x,
                       std::unique_ptr<AltParseTreeProperty<CoarseProperties>>
                           coarse_properties_x,
                       WhitespaceMaps whitespace_maps_x,
                       antlr4::CommonTokenStream* tokens_x)
      : MiddleHandler(out_stream_x, std::move(coarse_properties_x),
                      whitespace_maps_x, util::CreateHtmlPairMap(), tokens_x) {}

  void OutputLines(bool full_flush, const WalkerTransition& next_genre_state);
};

class QuarrelReformatHandler : public MiddleHandler {
  static constexpr int kMaxInitialLineTokens = 144;

 public:
  QuarrelReformatHandler(std::ofstream& out_stream_x,
                         std::unique_ptr<AltParseTreeProperty<CoarseProperties>>
                             coarse_properties_x,
                         WhitespaceMaps whitespace_maps_x,
                         antlr4::CommonTokenStream* tokens_x)
      : MiddleHandler(out_stream_x, std::move(coarse_properties_x),
                      whitespace_maps_x,
                      absl::MakeUnique<util::FormatContentMap>(), tokens_x) {
    output_pieces.reserve(kMaxInitialLineTokens + 2);
  }

  void OutputLines(bool full_flush, const WalkerTransition& next_genre_state);

 private:
  int CalcUnadjustedIndent(const CoarseProperties& coarse_properties_choice);
  void ProcessPending();
  int ProcessIndent();
  void ProcessBlankLines();
};

class SalientReformatHandler : public MiddleHandler {
  static constexpr int kMaxInitialLineTokens = 144;

 public:
  SalientReformatHandler(std::ofstream& out_stream_x,
                         std::unique_ptr<AltParseTreeProperty<CoarseProperties>>
                             coarse_properties_x,
                         WhitespaceMaps whitespace_maps_x,
                         antlr4::CommonTokenStream* tokens_x)
      : MiddleHandler(out_stream_x, std::move(coarse_properties_x),
                      whitespace_maps_x,
                      absl::MakeUnique<util::FormatContentMap>(), tokens_x) {
    output_pieces.reserve(kMaxInitialLineTokens + 2);
  }

  void OutputLines(bool full_flush, const WalkerTransition& next_genre_state);
  void FinishUp() {}
  void FullBeginning(SummarizerResults* summarizer_results) {}

 private:
  void ProcessPending();
  int ProcessIndent();
  void ProcessBlankLines();
};

}  // namespace pvn_parsing
}  // namespace patinon

#endif  // BASE_DIR_PATINON_EXPLORATORY_ABC_FORMAT_REFORM_HANDLERS_H_
