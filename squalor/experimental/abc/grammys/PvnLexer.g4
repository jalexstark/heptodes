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

lexer grammar PvnLexer;

import SvtLexer, QvlLexer, CommonCustomLexer;

// ====================================================
// Code customization.

@members {
  void ConsumeModeContext(std::unique_ptr<AbstractModeContext> resuming_context) {
    auto textual_mode_try = dynamic_cast<SalientModeContext *>(resuming_context.get());
    auto code_mode_try = dynamic_cast<QuarrelModeContext *>(resuming_context.get());

    if (textual_mode_try != nullptr) {
      parsing_genre = std::move(ParsingGenre::kSalient);
      current_textual = *textual_mode_try;
    } else if (code_mode_try != nullptr) {
      parsing_genre = ParsingGenre::kQuarrel;
      current_code = *code_mode_try;
    } else {
      PVN_DCHECK(false);  // << "Popped mode context is not of a known derived class.";
    }
  }
}

@header {
}


// ====================================================
// Lexer setup.

tokens {
   Q_IMPUTED_WHITESPACE
}

channels {
  SECONDARY_CHANNEL
}


// ====================================================
// Default mode: Essentially empty.

FAKE_TOKEN : 'Lexer should never be used in default mode.';
