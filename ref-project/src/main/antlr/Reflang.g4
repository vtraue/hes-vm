grammar Reflang;

// Parser
program : statement* EOF;

statement:
				   vardecl
        |  assign
    		|  fndecl
        |  expr ';'
        |  block
        |  while
        |  cond
        |  return
        ;

vardecl :  name=ID ':' t=type ('=' init_expr=expr)? ';' ;
assign  :  name=ID '=' init_expr=expr ';' ;

fndecl 	: FN name=ID '(' decl_params=params? ')' '->' ret_type=type decl_block=block;

param 	: name=ID ':' t=type;
params	: first = param (',' rest+=param)* ;

return  : 'return' expr ';' ;
fncall  : name = ID '(' args? ')' ;
args    :  first = expr (',' rest += expr)* ;
block	  : '{' statements = statement* '}';
while   :  'while' '(' expr ')' block ;
cond    :  'if' '(' cond_expr = expr ')' if_block = block ('else' else_block = block)? ;

type    : TYPE_INT  #TInt 
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
				| ID # Id 
				| NUMBER # LiteralNmb
				| STRING # LiteralStr
				| bool_literal #LiteralBool
				| '(' expr ')' # Paren
				;
bool_literal : TRUE #LiteralTrue | FALSE #LiteralFalse;

// Lexer
ID      :  [a-z][a-zA-Z0-9_]* ;
NUMBER  :  [0-9]+ ;

TYPE_INT		  : 'int'; 
TYPE_STRING		: 'string'; 
TYPE_BOOL		  : 'bool';

STRING  :  '"' (~[\n\r"])* '"' ;

TRUE : 'true'  ;
FALSE : 'false' ;

COMMENT :  '#' ~[\n\r]* -> skip ;
WS      :  [ \t\n\r]+ -> skip ;
FN		  : 'fn';

