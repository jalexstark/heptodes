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

lexer grammar CommonCustomLexer;

// ====================================================
// Code customization.

@members {
   LexerCustomization* GetLexerCustomization() {
      return &lexer_customization_;
   }

  void PushEnter(
      TextualSubGenre sub_genre, AbstractModeContext *suspending_mode_context,
      AbstractModeContextFactory *mode_context_factory) {
    if (suspending_mode_context != nullptr) {
      auto suspending_context_copy = absl::WrapUnique(suspending_mode_context->clone());
      mode_context_stack.push_back(std::move(suspending_context_copy));
    }
    std::unique_ptr<AbstractModeContext> new_mode_context = mode_context_factory->Create();
    new_mode_context->HandleEntry(sub_genre, this, GetLexerCustomization());
    ConsumeModeContext(std::move(new_mode_context));
  }

  void PopEnter() {
    if (mode_context_stack.empty()) {
       PVN_CHECK(false);  // << "Attempt to pop an empty mode context stack.";
       return;
    }
    std::unique_ptr<AbstractModeContext> popped_context = std::move(mode_context_stack.back());
    mode_context_stack.pop_back();
    popped_context->HandleEntry(TextualSubGenre::kNone, this, GetLexerCustomization());
    ConsumeModeContext(std::move(popped_context));
  }

  std::vector<std::unique_ptr<AbstractModeContext>> mode_context_stack;

  std::unique_ptr<AbstractModeContextFactory> textual_factory;
  std::unique_ptr<AbstractModeContextFactory> code_factory;


  TokenAnomaly token_anomaly = TokenAnomaly::kNone;
  LexerCustomization lexer_customization_;
  ParsingGenre parsing_genre;
  int curr_token_type = NONE_TOKEN;
  int prev_token_type = NONE_TOKEN;
}

// The following is inserted right at the top of the header.
@header {
// The generated lexer must be included via the custom lexer header, because
// the lexer members depend on the custom token factory.

#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
#include "base_dir/patinon/exploratory/abc/grammys/pvn_token.h"
#include "base_dir/patinon/exploratory/misc/check_macros.h"
#include "base_dir/absl/memory/memory.h"
}


// ====================================================
// Lexer setup.

tokens {
   NONE_TOKEN, ENTER_TEXTUAL, LEAVE_TEXTUAL, PENDING_ENTER_CODE, NEWLINE_ENTER_CODE, LEAVE_CODE,
   ENTER_VULGAR, LEAVE_VULGAR
}


// ====================================================
// Default mode: Essentially empty.

FAKE_TOKEN : 'Lexer should never be used in default mode.';


