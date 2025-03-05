import java.util.List;
import java.util.Optional;

sealed interface AstNode {};
sealed interface Statement extends AstNode {};
sealed interface Expression extends Statement {};

enum Type implements AstNode {
	String,
	Bool,
	Int,
} 

enum BinopType {
	Mul,
	Div,
	Add,
	Sub,
	Eq,
	Neq,
	Ge,
	Gt,
	Lt,
	Le
}
record Id(String name) implements Expression {};

record BoolLiteral(boolean lit) implements Expression {};
record StringLiteral(String literal) implements Expression {};
record IntLiteral(int literal) implements Expression {};

record BinOp(Expression lhs, BinopType op, Expression rhs) implements Expression {};

record FncallArgs(List<Expression> args) implements AstNode {};

record Fncall(Id id, Optional<FncallArgs> params) implements Expression {};

record VarDecl(Id id, Type type, Optional<Expression> expr) implements Statement {}
record Assign(Id id, Expression expr) implements Statement {};
record Block(List<Statement> statements) implements Statement {};
record Params(List<Param> params) implements AstNode {};
record Param(Id id, Type type) implements AstNode {} ;
record Fndecl(Id id, Optional<Params> params, Type returnType, Block block) implements Statement {}; 
record Return(Expression expr) implements Statement {} ;
record While(Expression expr, Block block) implements Statement {} ;
record Cond(Expression cond, Block ifBlock, Optional<Block> elseBlock) implements Statement {};

