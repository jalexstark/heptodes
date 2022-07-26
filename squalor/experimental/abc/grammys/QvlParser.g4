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

parser grammar QvlParser;

options { tokenVocab=PvnLexer; }

// The following is inserted right at the top of the header.
@header {
// The generated lexer must be included via the custom lexer header, because
// the lexer members depend on the custom token factory.
#include "base_dir/patinon/exploratory/abc/grammys/pvn_token.h"
#include "base_dir/patinon/exploratory/misc/check_macros.h"
}

@members {
   bool IsTokenAtStatement() {
      CustomizedToken* token= dynamic_cast<CustomizedToken *>(_input->LT(1));
      if (token != nullptr) {
         return token->supplement_.is_quarrel_statement;
      }
      return false;
   }
}

// Interleaving dual walkers assume that every non-EOF terminal node is within
// a parse-tree node.
quarrelTop
   : (module | gClass| funProc)* EOF
   | EOF
   ;

module
   : {IsTokenAtStatement()}? MODULE identifier
   ;

gClass
   : {IsTokenAtStatement()}? CLASS identifier openStmt ({IsTokenAtStatement()}? classPiece)* closeStmt
   ;

classPiece
   : (FUN | PROC) identifier POPEN identifierList? PCLOSE ( DIR_RESULT identifier qualifiers? )? statementSeq
   | identifier qualifiers?
   ;

funProc
   : (FUN | PROC) identifier POPEN identifierList? PCLOSE ( DIR_RESULT identifier qualifiers? )? statementSeq
   ;

identifierList
   : identifier qualifiers?
   | identifier qualifiers? ',' identifierList
   ;

contractSeq: contractClause+;

contractClause
   : contractPrefix identifier qualifiers?
   ;

identifier: Q_IDENTIFIER;

qualifiers
   : singleQualifier+
   ;

singleQualifier
   : ':' identifier
   ;

statementSeq
   : openStmt ({IsTokenAtStatement()}? contractSeq)? ({IsTokenAtStatement()}? statement)* closeStmt
   ;

statementSeqCont
   : openStmt ({IsTokenAtStatement()}? contractSeq)? ({IsTokenAtStatement()}? statement)* spliceStmt
   ;

openStmt: OPEN_STMT;
contractPrefix: CONTRACT_UNARY;
closeStmt: CLOSE_STMT;
spliceStmt: CLOSE_STMT;

// statement: {IsTokenAtStatement()}? statement_bol;

statement
   : identifier qualifiers? DIR_RESULT expression
   | IF expression statementSeq
   | IF expression statementSeqCont ELSE statementSeq
   ;

expression
   : constant
   | identifier
   | expression MISC_OP expression
   | POPEN expression PCLOSE
   | LAMBDA POPEN identifierList? PCLOSE statementSeq
   ;


// POPEN: POPEN;
//
// PCLOSE: PCLOSE;

constant
   : Q_CONSTANT
   ;

