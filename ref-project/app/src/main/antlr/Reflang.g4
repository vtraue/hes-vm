grammar Reflang;
@header {package org.example;}
// Parser
program : statement* EOF;

statement:
				   vardecl
        |  assign
        |  stmtExpr 
    		|  fndecl
        |  cond
        |  block
        |  while
        |  return
        ;

stmtExpr : e = expr ';' ;
vardeclt :  name=varname ':' t=type ('=' init_expr=expr)? ';' ;
vardecl	  : name=varname ':=' init_expr=expr ';' ;
assign  :  name=varname '=' init_expr=expr ';' ;
varname : name=ID;
fndecl 	: FN name=varname '(' decl_params=params? ')' '->' ret_type=type decl_block=block;

param 	: name=varname ':' t=type;
params	: first = param (',' rest+=param)* ;

return  : 'return' expr ';' ;
fncall  : name=varname '(' args? ')' ;
args    :  first = expr (',' rest += expr)* ;
block	  : '{' statements += statement* '}';
while   :  'while' '(' expr ')' block ;
cond    :  'if' '(' cond_expr = expr ')' if_block = block ('else' else_block = block)? ;

type    : TYPE_INT #TInt 
				| TYPE_STRING #TString 
				| TYPE_BOOL #TBool ;

expr 		: fncall #fnc
				|  lhs = expr '*' rhs = expr # Mult
        |  lhs = expr '/' rhs = expr # Div
        |  lhs = expr '+' rhs = expr # Add
        |  lhs = expr '-' rhs = expr # Sub
        |  lhs = expr '==' rhs = expr # Eq
        |  lhs = expr '!=' rhs = expr # Neq
        |  lhs = expr '>' rhs = expr # Gt
        |  lhs = expr '>=' rhs = expr # Ge
        |  lhs = expr '<' rhs = expr # Lt
        |  lhs = expr '<=' rhs = expr # Le
				| varname # Id 
				| NUMBER # LiteralNmb
				| STRING # LiteralStr
				| bool_literal #LiteralBool
				| '(' expr ')' # Paren
				;
bool_literal : TRUE #LiteralTrue | FALSE #LiteralFalse;

FN		  : 'fn';
// Lexer
TYPE_INT		  : 'int'; 
TYPE_STRING		: 'string'; 
TYPE_BOOL		  : 'bool';

TRUE : 'true'  ;
FALSE : 'false' ;

ID      :  [a-z][a-zA-Z0-9_]* ;
NUMBER  :  [0-9]+ ;


STRING  :  '"' (~[\n\r"])* '"' ;


COMMENT :  '#' ~[\n\r]* -> skip ;
WS      :  [ \t\n\r]+ -> skip ;

