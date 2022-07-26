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

lexer grammar QvlLexer;

import CommonCustomLexer;

// ====================================================
// Code customization.

@members {
  QuarrelModeContext current_code;
}

@header {
}


// ====================================================
// Lexer setup.

tokens {
   OPEN_STMT, CLOSE_STMT, Q_STMT_NEWLINE
}


// ====================================================
// Default mode: Essentially empty.

FAKE_TOKEN : 'Lexer should never be used in default mode.';


// ====================================================
mode QUARREL;

// Note that we want a verbatim section early in development, not least for
// copyright notices.

Q_TRIPLE_SEMI
   : ';;;' {
         PushEnter(TextualSubGenre::kTextualLeft, &current_code, textual_factory.get());
      }  -> type(ENTER_TEXTUAL)
   ;

Q_DOUBLE_SEMI
   : ';;' {
         PushEnter(TextualSubGenre::kTextualIndent, &current_code, textual_factory.get());
      }  -> type(ENTER_TEXTUAL)
   ;

Q_SINGLE_SEMI
   : ';' {
         PushEnter(TextualSubGenre::kTextualRight, &current_code, textual_factory.get());
      }  -> type(ENTER_TEXTUAL)
   ;

Q_TEXTUAL_BLOCK
   : '#\\\\\\' {
         PushEnter(TextualSubGenre::kMaster, &current_code, textual_factory.get());
      }  -> type(ENTER_TEXTUAL)
   ;

Q_TEXTUAL_BLOCK_RETURN
   : '```' {
         PopEnter();
      }  -> type(LEAVE_CODE)
   ;

Q_NEWLINE_ALT_0
   : Q_NEWLINE_CHARS
      {  current_code.get_q_parens_nesting() == 0  }?
      { current_code.MoveGNewStatementMarker(this);
      }
      -> type(Q_STMT_NEWLINE),
        channel(HIDDEN)
      ;

Q_NEWLINE_ALT_1
   : Q_NEWLINE_CHARS
      {  current_code.get_q_parens_nesting() != 0  }?
      -> type(Q_STMT_NEWLINE),
        channel(HIDDEN)
      ;

POPEN: '(' {  current_code.Incr_q_parens_nesting();  };
PCLOSE: ')'
      {  current_code.ClampedDecr_q_parens_nesting();  };

fragment Q_NEWLINE_CHARS
   : '\r'? '\n'
   | '\r'
   ;


Q_WS
   : [ \t]+ {
      if (current_code.IsAtGNewStatementMarker(this)) {current_code.MoveGNewStatementMarker(this); }
      } -> skip
   ;

Q_STMT_ALT_0
   : '|' {  !current_code.IsAtGNewStatementMarker(this)  }?
      {  current_code.NestStatements(); }
      -> type(OPEN_STMT);

Q_STMT_ALT_1
   : '|' { current_code.IsAtGNewStatementMarker(this)  }?
      {  current_code.DeNestStatements();
        current_code.MoveGNewStatementMarker(this);  }
      -> type(CLOSE_STMT);

   // identifier
   // keywords
   // parens
   // Q_STMT
   // string literal


Q_CONSTANT
   : Q_NUMBER (Q_E Q_SIGN? Q_NUMBER)?
   ;


fragment Q_NUMBER
   : ('0' .. '9') + ('.' ('0' .. '9') +)?
   ;


fragment Q_E
   : 'E' | 'e'
   ;


fragment Q_SIGN
   : ('+' | '-')
   ;


FUN: 'fun';
PROC: 'proc';
FOR: 'for';
LOOP: 'loop';
OVER: 'over' | 'through';
WHILE: 'while';
DO: 'do';
IF: 'if';
ELSE: 'else';
ELSEIF: 'elseif';
CSE: 'cse';
RAISE: 'raise';
HANDLE: 'handle';
MODULE: 'module';
CLASS: 'class';
LAMBDA: 'lambda';

CONTRACT_UNARY: '|->';
DIR_RESULT: ':=' | '->' | '<-';
MISC_OP: ARITH_OP | COMP_OP | BOOLEAN_OP | BITWISE_OP;

fragment
ARITH_OP: '*' | '+' | '-' | '/';
fragment
COMP_OP: '>' | '<' | '>=' | '<=' | '==' | '!==';
fragment
BOOLEAN_OP: '!' | '&&' | '||';
fragment
BITWISE_OP: '***' | '+++' | '!!!' | '///';

COLON
   : ':'
   ;

COMMA
   : ','
   ;

Q_IDENTIFIER
   : [a-zA-Z] [a-zA-Z0-9_]*
   ;

// Q_ANY_CHAR
//    : .
//    ;
