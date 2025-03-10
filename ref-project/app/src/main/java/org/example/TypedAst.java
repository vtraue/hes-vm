package org.example;
import java.util.List;
import java.util.Optional;

sealed interface TypedAstNode {};
sealed interface TypedStatement extends TypedAstNode {
};
sealed interface TypedExpression extends TypedStatement{
	Type getType();
};

record TypedId(String name, TypedAstBuilder.Symbol sym) implements TypedExpression {
	@Override
	public Type getType() {
		return sym.type();
	}
};
record TypedLiteral(Literal lit, Type t) implements TypedExpression {
	@Override
	public Type getType() {
		return t;
	}
};
record TypedBinOP(TypedExpression lhs, BinopType op, TypedExpression rhs) implements TypedExpression {
	@Override
	public Type getType() {
		return lhs.getType();
	}
	
};
record TypedFncallArgs(List<TypedExpression> args) implements TypedAstNode {};
record TypedFncall(TypedId id, Optional<TypedFncallArgs> params, Type returnType) implements TypedExpression {
	@Override
	public Type getType() {
		return returnType;
	}
};
record TypedVarDecl(TypedId id, Type type, Optional<TypedExpression> expr) implements TypedStatement {
};
record TypedAssign(TypedId id, TypedExpression expr) implements TypedStatement {};
record TypedBlock(List<TypedStatement> statements) implements TypedStatement {};
record TypedParam(TypedId id, Type type) implements TypedAstNode{};
record TypedParams(List<TypedParam> params) implements TypedAstNode {};
record TypedFndecl(String id, Optional<Params> params, Type returnType, TypedBlock block) implements TypedStatement {};
record TypedReturn(TypedExpression expr) implements TypedStatement {};
record TypedWhile(TypedExpression expr, TypedBlock block) implements TypedStatement {};
record TypedCond(TypedExpression cond, TypedBlock ifBlock, Optional<TypedBlock> elseBlock) implements TypedStatement {}




