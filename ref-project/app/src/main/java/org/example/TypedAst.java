package org.example;
import java.io.IOException;
import java.util.List;
import java.util.Optional;

import wasm_builder.Func;

sealed interface TypedAstNode {
};
sealed interface TypedStatement extends TypedAstNode {
	void toWasmCode(wasm_builder.Func func) throws IOException;
};

sealed interface TypedExpression extends TypedStatement{
	Type getType();
};

record TypedId(String name, TypedAstBuilder.Symbol sym) implements TypedExpression {
	@Override
	public Type getType() {
		return sym.type();
	}

	@Override
	public void toWasmCode(wasm_builder.Func func) throws IOException {
		func.emitLocalGet(sym.id());	
		func.emitLoad();
	}
};
record TypedLiteral(Literal lit, Type t) implements TypedExpression {
	@Override
	public Type getType() {
		return t;
	}

	@Override
	public void toWasmCode(wasm_builder.Func func) throws IOException {
		switch(this.lit) {
			case BoolLiteral b -> func.emitConst(b.lit() ? 1 : 0);
			case StringLiteral _ -> func.emitConst(999);
			case IntLiteral i -> func.emitConst(i.literal());
		}
	}
		
};
record TypedBinOP(TypedExpression lhs, BinopType op, TypedExpression rhs) implements TypedExpression {
	@Override
	public Type getType() {
		return lhs.getType();
	}

	@Override
	public void toWasmCode(Func func) throws IOException {
		lhs.toWasmCode(func);
		rhs.toWasmCode(func);
		op.toWasmCode(func);
	}
		
};
record TypedFncallArgs(List<TypedExpression> args) implements TypedAstNode {};
record TypedFncall(TypedId id, Optional<TypedFncallArgs> params, Type returnType) implements TypedExpression {
	@Override
	public Type getType() {
		return returnType;
	}

	@Override
	public void toWasmCode(Func func) throws IOException {
		// TODO Auto-generated method stub
		throw new UnsupportedOperationException("Unimplemented method 'toWasmCode'");
	}
};

record TypedVarDecl(TypedId id, Type type, Optional<TypedExpression> expr) implements TypedStatement {

	@Override
	public void toWasmCode(Func func) throws IOException {
		func.emitGlobalGet(0);
		func.emitLocalSet(id.sym().id());

		func.emitGlobalGet(0);
		func.emitConst(4);
		func.emitAdd();
		func.emitGlobalSet(0);

		if(expr.isPresent()) {
			func.emitLocalGet(id.sym().id());
			expr.get().toWasmCode(func);
			func.emitStore();
		}

	}
	
};
record TypedAssign(TypedId id, TypedExpression expr) implements TypedStatement {
	@Override
	public void toWasmCode(Func func) throws IOException {
		func.emitLocalGet(id.sym().id());
		expr.toWasmCode(func);
		func.emitStore();
	}
};

record TypedBlock(List<TypedStatement> statements) implements TypedStatement {

	@Override
	public void toWasmCode(Func func) throws IOException {
		func.emitBlock();
		func.emitBlockType();
		for(TypedStatement s : statements) {
			s.toWasmCode(func);
		}
		func.emitEnd();
	}
};

record TypedParam(TypedId id, Type type) implements TypedAstNode{};
record TypedParams(List<TypedParam> params) implements TypedAstNode {};
record TypedFndecl(String id, Optional<Params> params, Type returnType, List<TypedStatement> block) implements TypedStatement {

	@Override
	public void toWasmCode(Func func) throws IOException {
		
		throw new UnsupportedOperationException("Unimplemented method 'toWasmCode'");
	}};

record TypedExternFndecl(ExternFndecl decl) implements TypedStatement {

	@Override
	public void toWasmCode(Func func) throws IOException {
		// TODO Auto-generated method stub
		throw new UnsupportedOperationException("Unimplemented method 'toWasmCode'");
	}
}
record TypedReturn(TypedExpression expr) implements TypedStatement {

	@Override
	public void toWasmCode(Func func) throws IOException {
		expr.toWasmCode(func);
		func.emitEnd();
	}};
record TypedWhile(TypedExpression expr, TypedBlock block) implements TypedStatement {

	@Override
	public void toWasmCode(Func func) throws IOException {
		// TODO Auto-generated method stub
		throw new UnsupportedOperationException("Unimplemented method 'toWasmCode'");
	}

};
record TypedCond(TypedExpression cond, TypedBlock ifBlock, Optional<TypedBlock> elseBlock) implements TypedStatement {
	@Override
	public void toWasmCode(Func func) throws IOException {
		cond.toWasmCode(func);
			
		func.emitIf();
		func.emitBlockType();
		
		for(var s : ifBlock.statements()) {
			s.toWasmCode(func);
		}
		if(elseBlock.isPresent()) {
			func.emitElse();
			for(var s : elseBlock.get().statements()) {
				s.toWasmCode(func);
			}
		}
		func.emitEnd();
	}
}

