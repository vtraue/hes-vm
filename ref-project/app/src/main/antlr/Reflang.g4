grammar Reflang;
@header {package org.example;}
// Parser
program : statement* EOF;

statement:
		   vardecl
		|  vardeclt
        |  assign
        |  derefAssign
        |  stmtExpr 
        |  export_fndecl
        |  import_fndecl
    	|  fndecl
        |  while
        |  break
        |  return
        ;

vardecl	: name=varname COLON_EQ init_expr=expr ';' ;
vardeclt: name=varname COLON t=type ('=' init_expr=expr)? ';' ;

structdecl: STRUCT name=varname '{' fieldTypeDecl '}' ;   

fieldTypeDecl: name=varname COLON t=type;
fieldDeclList: first = fieldTypeDecl (',' rest += fieldTypeDecl)*;
stmtExpr : e = expr ';' ;

varname : name=ID;

assign  : name=varname '=' init_expr=expr ';' ;
derefAssign : name=varname(POINTSTAR) '=' init_expr=expr';' ;  
import_fndecl : IMPORT env_name = ID FN name=varname '(' decl_params=params? ')' ('->' ret_type=type)? ';' ; 
export_fndecl:  EXPORT FN name=varname '(' decl_params=params? ')' ('->' ret_type=type)? decl_block=block ;
fndecl 	: FN name=varname '(' decl_params=params? ')' ('->' ret_type=type)? decl_block=block;

param 	: name=varname ':' t=type;
params	: first = param (',' rest+=param)* ;

break   : 'break' expr? ';' ;
return  : 'return' expr? ';' ;
fncall  : name=varname '(' args? ')' ;
args    :  first = expr (',' rest += expr)* ;
block	  : '{' statements += statement* '}';
while   :  'while' '(' expr ')' block ;
cond    :  'if' '(' cond_expr = expr ')' if_block = block ('else' else_block = block)? ;
pointerType: (parent=primitive_type)depth+=STAR+;  

type: pointerType 
      | primitive_type ; 

primitive_type: TYPE_INT #TInt 
		| TYPE_STRING #TString
		| TYPE_BOOL #TBool ; 

cast : 'cast''('t=type','src_expr = expr ')';
//cast : t=type '(' src_expr = expr ')';

expr 	:  fncall #fnc
        |  block #code_block
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
		| bool_literal #LiteralBool
		| '(' expr ')' # Paren
		| STRING # LiteralStr
        | AMPERSAND name=varname #Ref
        | (name=varname)(POINTSTAR) #Deref
		| cond #condExpr 
		| cast #castExpr;

bool_literal : TRUE #LiteralTrue | FALSE #LiteralFalse;

// Lexer
TYPE_INT		  : 'int'; 
TYPE_STRING		: 'string'; 
TYPE_BOOL		  : 'bool';

TRUE : 'true'  ;
FALSE : 'false' ;

COLON : ':';
COLON_EQ : ':=';
IMPORT	: 'import';
EXPORT 	: 'export';
FN		  : 'fn';
STAR    : '*';
RAW_DATA: 'raw_data';
AMPERSAND: '&';
POINTSTAR: '.*';

ID      :  [a-z][a-zA-Z0-9_]* ;
NUMBER  :  [0-9]+ ;

STRING  :  '"'(~[\n\r"])*'"';
STRUCT  : 'struct';

COMMENT :  '#' ~[\n\r]* -> skip ;
WS      :  [ \t\n\r]+ -> skip ;

