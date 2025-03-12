package org.example;
import java.io.IOException;
import java.util.List;
import java.util.Optional;

import org.example.TypedAstBuilder.Function;

import wasm_builder.Func;

sealed interface TypedAstNode {
};
sealed interface TypedStatement extends TypedAstNode {
	void toWasmCode(Func func, TypedAstBuilder builder) throws IOException;
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
	public void toWasmCode(wasm_builder.Func func, TypedAstBuilder builder) throws IOException {
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
	public void toWasmCode(wasm_builder.Func func, TypedAstBuilder builder) throws IOException {
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
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
		lhs.toWasmCode(func, builder);
		rhs.toWasmCode(func, builder);
		op.toWasmCode(func);
	}
		
};
record TypedFncallArgs(List<TypedExpression> args) implements TypedAstNode {};
record TypedFncall(String name, Function type, Optional<TypedFncallArgs> params) implements TypedExpression {
	@Override
	public Type getType() {
		return type.getReturnType();
	}

	@Override
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
		if(this.params.isPresent()) {
			for(TypedExpression arg : this.params.get().args()) {
				System.out.println(arg.toString());
				arg.toWasmCode(func, builder);
			}
		}
		int func_id = builder.getGlobalFunctionId(this.type) - 1;
		System.out.printf("Func id: %d\n", func_id); 
		System.out.println("call!");
		func.emitCall(func_id);
	}
};

record TypedVarDecl(TypedId id, Type type, Optional<TypedExpression> expr) implements TypedStatement {

	@Override
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
		func.emitGlobalGet(0);
		func.emitLocalSet(id.sym().id());

		func.emitGlobalGet(0);
		func.emitConst(4);
		func.emitAdd();
		func.emitGlobalSet(0);

		if(expr.isPresent()) {
			func.emitLocalGet(id.sym().id());
			expr.get().toWasmCode(func, builder);
			func.emitStore();
		}

	}
	
};
record TypedAssign(TypedId id, TypedExpression expr) implements TypedStatement {
	@Override
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
		func.emitLocalGet(id.sym().id());
		expr.toWasmCode(func, builder);
		func.emitStore();
	}
};

record TypedBlock(List<TypedStatement> statements) implements TypedStatement {

	@Override
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
		func.emitBlock();
		func.emitBlockType();
		for(TypedStatement s : statements) {
			s.toWasmCode(func, builder);
		}
		func.emitEnd();
	}
};

record TypedParam(TypedId id, Type type) implements TypedAstNode{};
record TypedParams(List<TypedParam> params) implements TypedAstNode {};
record TypedFndecl(String id, Optional<Params> params, Type returnType, List<TypedStatement> block) implements TypedStatement {

	@Override
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
		
		throw new UnsupportedOperationException("Unimplemented method 'toWasmCode'");
	}};

record TypedExternFndecl(ExternFndecl decl) implements TypedStatement {

	@Override
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
		// TODO Auto-generated method stub
		throw new UnsupportedOperationException("Unimplemented method 'toWasmCode'");
	}
}
record TypedReturn(TypedExpression expr) implements TypedStatement {

	@Override
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
		expr.toWasmCode(func, builder);
		func.emitEnd();
	}};
record TypedWhile(TypedExpression expr, TypedBlock block) implements TypedStatement {

	@Override
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
		// TODO Auto-generated method stub
		throw new UnsupportedOperationException("Unimplemented method 'toWasmCode'");
	}

};
record TypedCond(TypedExpression cond, TypedBlock ifBlock, Optional<TypedBlock> elseBlock) implements TypedStatement {
	@Override
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
		cond.toWasmCode(func, builder);
			
		func.emitIf();
		func.emitBlockType();
		
		for(var s : ifBlock.statements()) {
			s.toWasmCode(func, builder);
		}
		if(elseBlock.isPresent()) {
			func.emitElse();
			for(var s : elseBlock.get().statements()) {
				s.toWasmCode(func, builder);
			}
		}
		func.emitEnd();
	}
}

