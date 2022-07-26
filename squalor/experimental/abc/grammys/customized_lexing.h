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

#ifndef BASE_DIR_PATINON_EXPLORATORY_ABC_GRAMMYS_CUSTOMIZED_LEXING_H_
#define BASE_DIR_PATINON_EXPLORATORY_ABC_GRAMMYS_CUSTOMIZED_LEXING_H_

#include <cstddef>
#include <memory>

#include "base_dir/java/antlr4/v4_7_1/Cpp/src/antlr4-runtime.h"
#include "base_dir/patinon/exploratory/abc/grammys/PvnLexer.h"
#include "base_dir/patinon/exploratory/abc/grammys/pvn_token.h"

namespace patinon {
namespace pvn_parsing {

// We need this because Lexer.setTokenFactory is broken, but it ends up being
// convenient anyway.
class CustomizedPvnLexer : public PvnLexer {
 public:
  explicit CustomizedPvnLexer(antlr4::CharStream* input) : PvnLexer(input) {
    lexer_customization_.custom_token_factory =
        std::make_shared<CustomizedTokenFactory>();
    _factory = lexer_customization_.custom_token_factory;
  }

  TokenAnomaly TokenTokenAnomaly(size_t type);
};

}  // namespace pvn_parsing
}  // namespace patinon

#endif  // BASE_DIR_PATINON_EXPLORATORY_ABC_GRAMMYS_CUSTOMIZED_LEXING_H_
