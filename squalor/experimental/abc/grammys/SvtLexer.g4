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

lexer grammar SvtLexer;

import CommonCustomLexer;

// ====================================================
// Code customization.

@members {
inline static int ListTypeToPseudoToken(SvtListType list_type) {
  switch (list_type) {
    case SvtListType::kBullet:
      return PvnLexer::PSEUDO_LIST_BULLET;
    case SvtListType::kArabic:
      return PvnLexer::PSEUDO_LIST_ARABIC;
    case SvtListType::kLowerAlpha:
      return PvnLexer::PSEUDO_LIST_LOWER_ALPHA;
    case SvtListType::kUpperAlpha:
      return PvnLexer::PSEUDO_LIST_UPPER_ALPHA;
    case SvtListType::kLowerRoman:
      return PvnLexer::PSEUDO_LIST_LOWER_ROMAN;
    case SvtListType::kUpperRoman:
      return PvnLexer::PSEUDO_LIST_UPPER_ROMAN;
    case SvtListType::kListBreak:
      return PvnLexer::PSEUDO_LIST_BREAK;
    case SvtListType::kNone:
    default:
      return PvnLexer::NONE_TOKEN;
  }
}

// inline static SvtListType PseudoTokenToListType(int token_type) {
//   switch (token_type) {
//     case PvnLexer::PSEUDO_LIST_BULLET:
//       return SvtListType::kBullet;
//     case PvnLexer::PSEUDO_LIST_ARABIC:
//       return SvtListType::kArabic;
//     case PvnLexer::PSEUDO_LIST_LOWER_ALPHA:
//       return SvtListType::kLowerAlpha;
//     case PvnLexer::PSEUDO_LIST_UPPER_ALPHA:
//       return SvtListType::kUpperAlpha;
//     case PvnLexer::PSEUDO_LIST_LOWER_ROMAN:
//       return SvtListType::kLowerRoman;
//     case PvnLexer::PSEUDO_LIST_UPPER_ROMAN:
//       return SvtListType::kUpperRoman;
//     case PvnLexer::NONE_TOKEN:
//     default:
//       return SvtListType::kNone;
//   }
// }

  // Return the Salient subgenre of the current token, that is based on the
  // number of semicolons after skipping initial whitespace.
  //
  // Note that all uses of this could be replaced by per-alt predicates on
  // fixed values of TextualSubGenre.
  TextualSubGenre GetTokenSubgenre() {
  const auto token_scoped_copy = getText();
    const absl::string_view token_text(token_scoped_copy);
    if (token_text.empty()) {
      return TextualSubGenre::kMaster;
    }

    absl::string_view::size_type first_non_whitespace = token_text.find_first_not_of(" \t\n\r");
    if (first_non_whitespace == absl::string_view::npos) {
      return TextualSubGenre::kMaster;  // Whitespace-only case.
    }

    absl::string_view::size_type after_last_semicolon = token_text.find_first_not_of(';', first_non_whitespace);
    if (after_last_semicolon == absl::string_view::npos) {
      after_last_semicolon = token_text.size();
    }

    switch (after_last_semicolon - first_non_whitespace) {
    case 0:
      return TextualSubGenre::kMaster;
    case 1:
      return TextualSubGenre::kTextualRight;
    case 2:
      return TextualSubGenre::kTextualIndent;
    case 3:
      return TextualSubGenre::kTextualLeft;
    default:
      return TextualSubGenre::kNone;
    }
  }

  std::unique_ptr<antlr4::Token> nextToken() override {
    PvnLexer* pvn_lexer = dynamic_cast<PvnLexer*>(this);
    PVN_CHECK(pvn_lexer != nullptr);
    pvn_lexer->prev_token_type = pvn_lexer->curr_token_type;

    // Since these methods get the next token, which might not be of the
    // "current" context, this is not straightforward. If we want per-context
    // next-token methods, we will have to be more clever about immediate
    // dispatch logic, or use some kind of chain of hooks (which would not be
    // too complex to implement).
    std::unique_ptr<antlr4::Token> next_token =
        current_textual.SpecializedNextToken(this);
    pvn_lexer->curr_token_type = next_token->getType();

    return next_token;
  }

  inline size_t GetAuxiliaryTokenType() const { return auxiliary_token_type_; }
  inline void SetAuxiliaryTokenType(size_t auxiliary_token_type) {
    auxiliary_token_type_ = auxiliary_token_type;
  }

  // Simple reset when a token is used up.
  //
  // This should only set values to defaults, but can do so selectively.
  inline void TokenConsumeReset() {
    auxiliary_token_type_ = kInvalidTokenIndex;
  }

 // private:
  size_t auxiliary_token_type_ = kInvalidTokenIndex;

  SalientModeContext current_textual;
  // Invariant: If non-empty, pending_tokens' back must not be of logical
  // whitespace type.
  std::deque<std::unique_ptr<antlr4::Token>> pending_tokens;
}

@header {
#include <deque>

#include "base_dir/absl/strings/string_view.h"
}


// ====================================================
// Lexer setup.

tokens {
   LINE_JOIN, INDENT_CONTINUATION, ITEM_START_FIRST, ITEM_START_SUCCEEDING,
   SINGLE_NEWLINE, MULTI_NEWLINE, SVT_DEDENT, SVT_INDENT, LIST_BREAK_ACTUAL,
   EXTRA_ORDINARY_CHAIN,
   // Disambiguated delimiters (open token same as close).
   DOUBLE_BOLD_OPEN, DOUBLE_BOLD_CLOSE, DOUBLE_EMPH_OPEN, DOUBLE_EMPH_CLOSE,
   // List pseudo-tokens.
   PSEUDO_LIST_BULLET, PSEUDO_LIST_ARABIC, PSEUDO_LIST_LOWER_ALPHA,
   PSEUDO_LIST_UPPER_ALPHA, PSEUDO_LIST_LOWER_ROMAN, PSEUDO_LIST_UPPER_ROMAN,
   PSEUDO_LIST_BREAK
}


// ====================================================
// Default mode: Essentially empty.

FAKE_TOKEN : 'Lexer should never be used in default mode.';

// ====================================================
mode SALIENT;

// S_CODE_ALT_BLOCK
//    : TRIPLE_TT_PLAIN {
//       PushEnter(TextualSubGenre::kNone, &current_textual, code_factory.get());
//    }
//    -> type(ENTER_CODE)
//    ;

S_CODE_ALT_BLOCK
   : TRIPLE_TT_PLAIN {
      current_textual.set_pending_triple(TripleTransitions::kQuarrelBlock);
   }
   -> type(PENDING_ENTER_CODE)
   ;

Q_CODE_BLOCK_RETURN
   : TRIPLE_BACKSLASH
   {
         PopEnter();
   }
   -> type(LEAVE_TEXTUAL)
   ;


// '#' prefix.
//
// In URLs, for example, '#' is special.

LOCATION_AND_QUALIFIERS
   :  (LOCATION_ANCHOR | QUALIFIER_CHAIN)+
   ;

LOCATION_ANCHOR
   : '#' ORDINARY_IDENTIFIER { current_textual.get_in_ref_context() }?
   ;

TITLE
   : '#title' | '===='
   ;

HEADING
   : '#' '#'+
   | '=====' '='+
   | '#' '='+
   | '#=++'
   ;

TOC: '#toc';

CONTROL_IDENTIFIER
   : '#' ORDINARY_IDENTIFIER
   ;

// Double-char toggles, some paired open/close.
//

DOUBLE_MATH: '$$';
DOUBLE_TT: '``';
DOUBLE_SQUOTE: '\'\'';
DOUBLE_DQUOTE: '""';
DOUBLE_BOLD: '**'
   {
      setType(
         (current_textual.styling_flags & StylingFlags::kDoubleBold) == StylingFlags::kNone ?
         DOUBLE_BOLD_OPEN : DOUBLE_BOLD_CLOSE);
      current_textual.styling_flags = current_textual.styling_flags ^ StylingFlags::kDoubleBold;
   };
DOUBLE_EMPH: '//'
   {
      setType(
         (current_textual.styling_flags & StylingFlags::kDoubleEmph) == StylingFlags::kNone ?
         DOUBLE_EMPH_OPEN : DOUBLE_EMPH_CLOSE);
      current_textual.styling_flags = current_textual.styling_flags ^ StylingFlags::kDoubleEmph;
   };
DOUBLE_ULINE: '__';        // Should be disabled in semi-verbatim, math.
DOUBLE_STRIKE: '~~';
DOUBLE_SEMI_VERB: '%%';

// As Creole, {{Image.jpg|title}}.
DOUBLE_OPEN_IMAGE: '{{';    // Should be disabled in semi-verbatim, math.
DOUBLE_CLOSE_IMAGE: '}}';   // Should be disabled in semi-verbatim, math.

// Triple-char toggles.
//

TRIPLE_MATH: '$$$';
TRIPLE_SEMI_VERB: '%%%';
TRIPLE_RESERVED: '[[[' | ']]]';

// Note: Only allow Quarrel block code within master-SubGenre Salient.
TRIPLE_TT_PLAIN: '```';
// TRIPLE_TT_SPEC: '```' ORDINARY_IDENTIFIER;
TRIPLE_BACKSLASH : '\\\\\\';


// In a figure, a description-type list item provides the caption, with the
// shortened caption as the "term" being described.
TRIPLE_OPEN_HIDE_PLAIN: '{{{';
TRIPLE_OPEN_EXT_SPEC: '{{{' ORDINARY_IDENTIFIER;
TRIPLE_CLOSE_HIDE_EXT: '}}}';

TRIPLE_OPEN_FIGURE: '(((';
TRIPLE_CLOSE_FIGURE: ')))';

// Dashing.
//

EN_DASH: '--';
EM_DASH: '---';
HRULE: '----' '-'+;

// Refs, anchors, footnotes, citations.
//
// Like Creole:
//    Link to [[wikipage]]
//    [[link_address|link text]]
REF_OPEN:
   '[[' {
      current_textual.set_in_ref_context(true);  }
   ;
REF_CLOSE:
   ']]' {
      current_textual.set_in_ref_context(false);  }
   ;
QUALIFIER_OPEN:
   WS* '{' {
      current_textual.set_in_ref_context(true);  }
   ;
QUALIFIER_CLOSE:
   WS* '}' {
      current_textual.set_in_ref_context(false);  }
   ;

// LINK_SEP: '|'  { current_textual.get_in_ref_context() }?;
// SINGLE_BAR: '|'  { !current_textual.get_in_ref_context() }?;
// EQUALS: '=';
// BAR_HASH: '|#';  // Anchor in link, or placement of anchor.

// Tables and grids.
//
// In Salient, all tables and grids must begin with a header line.
//
// The first line, the final line should have the same form. Internally, use
// the |====| form between header and content.
// |====|====|... are book-style tables (headers) and dividing content.
// |----|----|... are grid-style (headers) and for inserting lines or spanning.
//
// Width specifications default to ch (officially width of '0')?
// |------| means centred.
// |----->| means right-aligned.
// |<-----| means left-aligned.
// |<----30---->| means justified (aligns both sides).
// |----.3| means align decimal point, allow for up to 3.
// |------||------| indicates double vertical division.
//
// Inserted lines and/or spanning.
// |----x----x----| or
// |    x    | indicate column spans for the //previous// row.
//
// Continuation lines.
// : continued cell content : continued cell content : ... :

// Itemization and indented blocks.
//

// ITEM_PREFIXED_ALT_0
//    : ITEM_PREFIX (ITEM_PREFIX | ORDINARY_IDENTIFIER)*
//       {  current_textual.IsAtWsMarker(this)  }?
//       {  QQQQQQ if (current_textual.get_effective_start_col() != tokenStartCharPositionInLine) {
//             token_anomaly = TokenAnomaly::kNotAtLineStart;
//          }  }
//       -> type(ITEM_START_INDENT)
//    ;

ITEM_PREFIXED_ALT_ITEM
   : ((WS* ITEM_TOKEN)
      | (WS* BLOCK_INDENTATION WS* ITEM_TOKEN)
      )
      {  current_textual.IsAtWsMarker(this)  }?
      {
         // QQQQQQ if (current_textual.get_effective_start_col() != tokenStartCharPositionInLine) {
         //    // It would be good to reinstate logic, but would have to account for
         //    // tokens that can chain at beginning of line.
         //    // token_anomaly = TokenAnomaly::kNotAtLineStart;
         // }
         setType(current_textual.UpdateListNesting(getText(), this));
         current_textual.MoveWsMarker(this, false);  // Choosing false was a bit of a guess.
      }
      // -> type(ITEM_START_INDENT)
   ;


ITEM_PREFIXED_ALT_INDENT
   : WS *BLOCK_INDENTATION
      {  current_textual.IsAtWsMarker(this)  }?
      {
         // QQQQQQ if (current_textual.get_effective_start_col() != tokenStartCharPositionInLine) {
         //    // It would be good to reinstate logic, but would have to account for
         //    // tokens that can chain at beginning of line.
         //    // token_anomaly = TokenAnomaly::kNotAtLineStart;
         // }

         setType(current_textual.UpdateListNesting(getText(), this));
         current_textual.MoveWsMarker(this, false);  // Choosing false was a bit of a guess.
      }
      // -> type(INDENT_CONTINUATION)
   ;

ITEM_PREFIXED_ALT_OTHER
   : ((ITEM_PREFIX (ITEM_PREFIX | ORDINARY_IDENTIFIER)*)
      | (BLOCK_INDENTATION)
      | (ITEM_TOKEN)
      | (BLOCK_INDENTATION WS* ITEM_TOKEN)
      )
      {  !current_textual.IsAtWsMarker(this)  }?
      -> type(EXTRA_ORDINARY_CHAIN)
   ;

// TODO: Clean up.
// TODO: Make note that lexer should never emit 2 successive
// EXTRA_ORDINARY_CHAINs.
//
// TODO: Also note that from point of view of Salient, stretches of whitespace
// are significant, but not their form. For example, each can be replaced with a
// single space.
EXTRA_ORDINARY_CHAIN_USUAL
   :( WS* ORDINARY_PIECE (WS | EXTRA_ORDINARY_PIECE)* EXTRA_ORDINARY_PIECE
      // {  !current_textual.get_in_ref_context()  }?
   | WS* ORDINARY_PIECE)
      {  !current_textual.get_in_ref_context()  }?
      -> type(EXTRA_ORDINARY_CHAIN)
   ;

QUALIFIER_CHAIN
   : WS* ORDINARY_PIECE_QUAL+
      {  current_textual.get_in_ref_context()  }?
   ;

fragment
EXTRA_ORDINARY_PIECE
   : ORDINARY_PIECE ITEM_PREFIX?
   ;

// Design rule: tokens with newlines must end in a newline. These must correctly
// set at_logical_newline Boolean so that zero-column tests work.

// Design rule: Whitespace is meaningful in prefix situations, so do not consume
// trailing whitespace.

NEWLINE_SEMI_COLONS
   : WS* S_NEWLINE_CHARS  WS* (';;; ' | ';; ' | '; ' )
      { current_textual.get_sub_genre() == GetTokenSubgenre() }?
      {
         current_textual.MoveWsMarker(this, true);  }
   -> type(SINGLE_NEWLINE)
   ;

MULTILINE_SEMI_COLONS
   : (   (WS* S_NEWLINE_CHARS WS* ';;;' ( WS* S_NEWLINE_CHARS  WS* ';;; ')+)
       | (WS* S_NEWLINE_CHARS WS* ';;' ( WS* S_NEWLINE_CHARS  WS* ';; ')+)
       | (WS* S_NEWLINE_CHARS WS* ';' ( WS* S_NEWLINE_CHARS  WS* '; ')+)     )
      { current_textual.get_sub_genre() == GetTokenSubgenre() }?
      {
         current_textual.MoveWsMarker(this, true);  }
   -> type(MULTI_NEWLINE)
   ;

// Handle blank lines at end of a Salient comment section.
MULTILINE_SEMI_COLONS_NOT_FINAL
   :  (WS* S_NEWLINE_CHARS
          (   (WS* ';;;' WS* S_NEWLINE_CHARS)+
            | (WS* ';;' WS* S_NEWLINE_CHARS)+
            | (WS* ';' WS* S_NEWLINE_CHARS)+    )
   )
   { current_textual.get_sub_genre() == GetTokenSubgenre() }?
   {
         PopEnter();
   }
   -> type(LEAVE_TEXTUAL)
   ;

NEWLINE_PENDING
   : WS* S_NEWLINE_CHARS
      { current_textual.get_pending_triple() == TripleTransitions::kQuarrelBlock }?
      {
        current_textual.set_pending_triple(TripleTransitions::kNone);
        PushEnter(TextualSubGenre::kNone, &current_textual, code_factory.get());
      }
   -> type(NEWLINE_ENTER_CODE)
   ;

NEWLINE_WITHOUT_SEMI_NON_MASTER
   : WS* S_NEWLINE_CHARS
   { current_textual.get_sub_genre() != GetTokenSubgenre() }?
   {
         PopEnter();
   }
   -> type(LEAVE_TEXTUAL)
   ;

MULTI_NEWLINE_MASTER
   : WS* S_NEWLINE_CHARS ( WS* S_NEWLINE_CHARS )+
      { current_textual.get_sub_genre() == GetTokenSubgenre() }?
      {
         current_textual.MoveWsMarker(this, true);  }
   -> type(MULTI_NEWLINE)
   ;

SINGLE_NEWLINE_MASTER
   : WS* S_NEWLINE_CHARS
      { current_textual.get_sub_genre() == GetTokenSubgenre() }?
      {
         current_textual.MoveWsMarker(this, true);  }
   -> type(SINGLE_NEWLINE)
   ;

// A line join will not distinguish between single, double, or triple semis.
LINE_JOIN_NO_WS
   :  '\\' WS* S_NEWLINE_CHARS
      {  // QQQQQ current_textual.set_effective_start_col(-1);
         if (current_textual.IsAtWsMarker(this)) {  current_textual.MoveWsMarker(this, false);  } }
      -> type(LINE_JOIN)
   ;

fragment S_NEWLINE_CHARS
   : '\r'? '\n'
   | '\r'
   ;

// ':|' cont, ':#' enumerated, ':;' line comment (?), ':"' block quotation,
// ':^' footnote definition.
fragment ITEM_PREFIX
   : (':' [|#;^]? )
   | ITEM_TOKEN
   ;

fragment ITEM_TOKEN
   : '@' | '::' | ':"' | ':><' | ':' [0-9]+ | ':' [a-z]+ | ':' [A-Z]+
   | ':%' // List break.
   ;

fragment BLOCK_INDENTATION
   : INDENT_MARKER (WS* INDENT_MARKER)*
   ;

fragment INDENT_MARKER
   // : ('|' | ':' | ':|')
   : '|'
   ;

fragment
ORDINARY_PIECE
   : ORDINARY_SUBPIECE+ (WS ORDINARY_SUBPIECE+)*
   ;

fragment
ORDINARY_PIECE_QUAL
   : ORDINARY_SUBPIECE_QUAL+ (WS ORDINARY_SUBPIECE_QUAL+)*
   ;

fragment
ORDINARY_SUBPIECE
   : ORDINARY_CHAR+
   // : ORDINARY_CHAR+ (SINGLY_ORDINARY ORDINARY_CHAR+)* // SINGLY_ORDINARY?
   // | SINGLY_ORDINARY ORDINARY_CHAR+ (SINGLY_ORDINARY ORDINARY_CHAR+)* // SINGLY_ORDINARY?
   // | SINGLY_VERY_ORDINARY
   ;

fragment
ORDINARY_SUBPIECE_QUAL
   : ORDINARY_CHAR+ (SINGLY_ORDINARY ORDINARY_CHAR+)* // SINGLY_ORDINARY?
   | SINGLY_ORDINARY ORDINARY_CHAR+ (SINGLY_ORDINARY ORDINARY_CHAR+)* // SINGLY_ORDINARY?
   | SINGLY_VERY_ORDINARY
   ;

// TODO: Enable "__".
// TODO: Note that ORDINARY_IDENTIFIER is not for content, and can match
// double underscores.
fragment
ORDINARY_IDENTIFIER
   : ORDINARY_CHAR+ (SINGLY_ORDINARY_IDENTIFIER ORDINARY_CHAR+)*
   ;

fragment ORDINARY_CHAR
   : ~[-`~!@#$%^&*()_+={}|[\]\\:";'<>?,./ \t\r\n] // ' Reset code highlighting.
   ;

SINGLY_ORDINARY
   : ['`~$%^&()+=";<>*/]  // ' Reset code highlighting.
   | SINGLY_ORDINARY_IDENTIFIER
   | SINGLY_VERY_ORDINARY
   ;

fragment SINGLY_VERY_ORDINARY
   : [,.]
   | SINGLY_ORDINARY_IDENTIFIER
   ;


fragment SINGLY_ORDINARY_IDENTIFIER
   : [-_!?]
   ;

WS_CHAIN
   : WS+ {  current_textual.MoveWsMarker(this, false);  }
   ;

fragment WS
   : [ \t]
   ;
