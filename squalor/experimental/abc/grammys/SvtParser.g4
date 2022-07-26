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

parser grammar SvtParser;

options { tokenVocab=PvnLexer; }

@members {
  inline static int CheckListType(int token_type) {
    switch (token_type) {
      case PSEUDO_LIST_BULLET:
      case PSEUDO_LIST_ARABIC:
      case PSEUDO_LIST_LOWER_ALPHA:
      case PSEUDO_LIST_UPPER_ALPHA:
      case PSEUDO_LIST_LOWER_ROMAN:
      case PSEUDO_LIST_UPPER_ROMAN:
        return token_type;
      case NONE_TOKEN:
      default:
        PVN_CHECK(false);
        // PVN_CHECK_EQ(token_type, NONE_TOKEN);
        return NONE_TOKEN;
    }
  }

  // Probably could take any ParserRuleContext.
  const TokenSupplement& ObtainSupplement(antlr4::ParserRuleContext * ctx) {
    antlr4::Token* first_token = ctx->getStart();

    CustomizedToken* custom_token = dynamic_cast<CustomizedToken*>(first_token);
    PVN_CHECK_NE(custom_token, nullptr);
    return custom_token->supplement_;
  }
}

@header {
#include "base_dir/patinon/exploratory/abc/grammys/pvn_token.h"
#include "base_dir/patinon/exploratory/abc/grammys/genres.h"
}
// -----------------
// Structural rules.

salientTop
   :  entitySeq eof
   |  salientInCode eof
   |  eof
   ;

eof
   : blankLine? SINGLE_NEWLINE* EOF
   ;

blankLine
   : SINGLE_NEWLINE SINGLE_NEWLINE  // With luck we can removed this, and detect multiplicities reliably.
   | MULTI_NEWLINE
   ;

salientInCode
   : (SINGLE_NEWLINE* enterSalient entitySeq? SINGLE_NEWLINE* exitSalient)+
   ;

entitySeq
   // This is really
   // SINGLE_NEWLINE* topEntity? ((blankLine | codeInSalient)+ topEntity?)* SINGLE_NEWLINE*
   // but requiring at least one top entity or codeInSalient.
   : SINGLE_NEWLINE* topEntity ((blankLine | codeInSalient)+ topEntity?)* SINGLE_NEWLINE*
   | SINGLE_NEWLINE* ((blankLine | codeInSalient)+ topEntity?)+ SINGLE_NEWLINE*
   ;

codeInSalient
   : SINGLE_NEWLINE* enterCode salientInCode? exitCode SINGLE_NEWLINE*
   ;

enterCode
   : PENDING_ENTER_CODE NEWLINE_ENTER_CODE
   ;

exitCode
   : LEAVE_CODE
   ;

enterSalient
   : ENTER_TEXTUAL
   ;

exitSalient
   : LEAVE_TEXTUAL
   ;

// -----------------
// Entities.

// para is between blank lines (or interrupting syntax switch), paraBlock is
// logical chunk within. For instance, an item starts a new paraBlock.
topEntity
   : para
   | heading
   | detachedList
   ;

// This probably means we put <p>...</p> around a bit of Code.
para
   : (paraBlock) (paraBlock | listItem)*   // THIS FEELS UGLY.
   | paraBlock (inBetween paraBlock)+
   ;

detachedList
   : eitherList { $eitherList.ctx->attachment = SvtListAttachment::kDetached; }
   ;

attachedList
   : eitherList { $eitherList.ctx->attachment = SvtListAttachment::kAttached; }
   ;

eitherList locals [
      size_t list_type_pseudo_token = NONE_TOKEN,
      SvtListAttachment attachment = SvtListAttachment::kNone,
      SvtListCompactness list_compactness = SvtListCompactness::kNone,
   ]
   // Block list if first inter-item separation is block, sloppy about rest.
   : (listItem (blockSep listItem) ((inBetween | blockSep) listItem)*)
   {
     const TokenSupplement& supplement = ObtainSupplement($ctx);
     $list_type_pseudo_token = CheckListType(supplement.auxiliary_token_type);
     $list_compactness = SvtListCompactness::kBlock;
   }
   | (
   // Treat single-item list as compact list.
   listItem
   // Compact list if first 2 items are juxtaposed, sloppy about rest.
     | (listItem (inBetween listItem) ((inBetween | blockSep) listItem)*))
   {
     const TokenSupplement& supplement = ObtainSupplement($ctx);
     $list_type_pseudo_token = CheckListType(supplement.auxiliary_token_type);
     $list_compactness = SvtListCompactness::kCompact;
   }
   ;

paraBlock
   // : paraBlockText | paraBlockPlusMore  // Use paraBlockContent?
   : paraBlockContent
   ;

listItem locals [
       SvtListCompactness item_compactness = SvtListCompactness::kNone]
   // Compact set of items if first 2 paras are juxtaposed, sloppy about rest.
   : (paraBlockItem listItemParaPiece?
        (inBetween logicalIndent listItemParaPiece?)
          ((inBetween | blockSep) logicalIndent listItemParaPiece?)* SVT_DEDENT)
   { $item_compactness = SvtListCompactness::kCompact; }
   //
   // Needed - yes, if unnested list? But be careful not to generalize this and allow it to fit at doc top level.
   // : paraBlockItem listItemParaPiece? (blockSep listItemParaPiece?)+
   //
   // Blocked item if first inter-para separation is block, sloppy about rest.
   |  (paraBlockItem listItemParaPiece? (blockSep logicalIndent listItemParaPiece?) ((inBetween | blockSep) logicalIndent listItemParaPiece?)* SVT_DEDENT)
   { $item_compactness = SvtListCompactness::kBlock; }
   //
   // Simple item is a single run of content or none.
   | (paraBlockItem logicalIndent paraBlockContent SVT_DEDENT  // Nested list.
       | paraBlockItem paraBlockContent SVT_DEDENT
       | paraBlockItem SVT_DEDENT)
   { $item_compactness = SvtListCompactness::kSimple; }
   ;

listItemParaPiece
   : paraBlockContent
   ;

// // A key feature of paraBlock_following and paraBlockSelfStanding is that
// // they can be treated in one process, such as for whitespace reduction and
// // line-breaking.

// paraBlock_following
//    : EXTRA_ORDINARY_CHAIN* (inBetween EXTRA_ORDINARY_CHAIN+)*
//    ;

// paraBlockSelfStanding
//    : EXTRA_ORDINARY_CHAIN+ (inBetween EXTRA_ORDINARY_CHAIN+)*
   // ;

// This is what makes a paragraph, but without the <p>...</p>.
paraBlockContent
   : paraBlockText | paraBlockPlusMore
   ;

// Paragraph-like content that might not need to be in separate paragraph.
paraBlockText
   : (inBetween? linearContent)+
   ;

// Paragraph-like content that might not need to be in separate paragraph.
linearContent locals[ size_t content_opening = NONE_TOKEN ]
   : EXTRA_ORDINARY_CHAIN //+ (inBetween EXTRA_ORDINARY_CHAIN)*
   | SINGLY_ORDINARY
   | DOUBLE_BOLD_OPEN paraBlockText? inBetween? DOUBLE_BOLD_CLOSE
     { $content_opening = DOUBLE_BOLD_OPEN; }
   | DOUBLE_EMPH_OPEN paraBlockText? inBetween? DOUBLE_EMPH_CLOSE
     { $content_opening = DOUBLE_EMPH_OPEN; }
   ;

// DOUBLE_TT: '``';
// DOUBLE_SQUOTE: '\'\'';
// DOUBLE_DQUOTE: '""';
// DOUBLE_BOLD: '**';
// DOUBLE_EMPH: '//';
// DOUBLE_ULINE: '__';        // Should be disabled in semi-verbatim, math.
// DOUBLE_STRIKE: '~~';

paraBlockPlusMore
   : paraBlockText (inBetween? attachedList inBetween? paraBlockText)* inBetween? attachedList
   | paraBlockText (inBetween? attachedList inBetween? paraBlockText)*
   | detachedList
   ;

inBetween
   : (LINE_JOIN|WS_CHAIN)* SINGLE_NEWLINE (LINE_JOIN|WS_CHAIN)*
   | (LINE_JOIN|WS_CHAIN)+
   ;

blockSep
   : (LINE_JOIN|WS_CHAIN)* MULTI_NEWLINE (LINE_JOIN|WS_CHAIN)*
   // | (LINE_JOIN|WS_CHAIN)+
   ;

// heading
//    : (HEADING | TITLE) paraBlockSelfStanding qualifier?
//    | TOC qualifier?
//    ;

heading
   : (HEADING | TITLE) headingContent qualifier?
   | TOC qualifier?
   ;

headingContent
   : paraBlockText
   ;

qualifier
   : QUALIFIER_OPEN LOCATION_AND_QUALIFIERS QUALIFIER_CLOSE
   ;

paraBlockItem
   : ITEM_START_FIRST
   | ITEM_START_SUCCEEDING
   ;

// EDIT

logicalIndent
   : INDENT_CONTINUATION
   ;
