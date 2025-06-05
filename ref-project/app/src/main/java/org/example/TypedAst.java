package org.example;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;

import org.example.TypedAstBuilder.Function;

import wasm_builder.Func;
import wasm_builder.WasmValueType;

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
    if(!sym.local()) {
      func.emitLoad();
    }
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
      case StringLiteral l -> {
        func.builder.addStringData(Arrays.asList(l.literal()));
        func.emitConst(l.pointer());
      }
      case IntLiteral i -> func.emitConst(i.literal());
    }
  }
    
};
record TypedBinOP(TypedExpression lhs, BinopType op, TypedExpression rhs) implements TypedExpression {
  @Override
  public Type getType() {
    if(op.getKind() == BinopKind.Cmp) {
      return PrimitiveType.Bool;
    } else {
      return lhs.getType();
    }
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
    return type.getReturnType().orElse(PrimitiveType.Void);
  }

  @Override
  public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
    if(this.params.isPresent()) {
      for(TypedExpression arg : this.params.get().args()) {
        arg.toWasmCode(func, builder);
      }
    }
    int func_id = builder.getGlobalFunctionId(this.type);
    func.emitCall(func_id);
  }
};

record TypedDeref(TypedId id, Type t) implements TypedExpression {
	@Override
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
    id.toWasmCode(func, builder); 
    func.emitLoad();  
	}

	@Override
	public Type getType() {
    return t; 
	}
}

record TypedRef(TypedId id, PointerType t) implements TypedExpression {
	@Override
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
    if(!id.sym().local()) {
      func.emitLocalGet(id.sym().id);
    } else {
      System.out.println("Error!");
      //TODO: Error
    }
	}

	@Override
	public Type getType() {
    return this.t;
	}
}
record TypedBreak(Optional<TypedExpression> expr, Type t) implements TypedExpression {
  @Override
  public Type getType() {
    return expr.map(e -> e.getType()).orElse(PrimitiveType.Void);
  }
  @Override 
  public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
    if(expr.isPresent()) {
      expr.get().toWasmCode(func, builder); 
    }

    func.emitBr(0);
  }
}
record TypedVarDecl(TypedId id, Type type, Optional<TypedExpression> expr) implements TypedStatement {
  @Override
  public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
    if(id.sym().local()) {
      if(expr.isPresent()) {
        System.out.println("local var decl");
        expr.get().toWasmCode(func, builder);
        func.emitLocalSet(id.sym().id());
      }
    } else {
      System.out.println("not local var decl");
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
  }
  
};
record TypedAssign(TypedId id, TypedExpression expr) implements TypedStatement {
  @Override
  public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
    if(id.sym().local()) {
      expr.toWasmCode(func, builder);
      func.emitLocalSet(id.sym().id());
    } else {
      func.emitLocalGet(id.sym().id());
      expr.toWasmCode(func, builder);
      func.emitStore();
    }

  }
};

record TypedBlock(List<TypedStatement> statements, Type type) implements TypedExpression {
  @Override
  public Type getType() {
    return type;
  }

  @Override
  public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
    func.emitBlock();
    if(type == PrimitiveType.Void) {
      func.emitBlockType();
    } else {
      func.emitBlockType(type.toWasmValueType()); 
    }

    for(TypedStatement s : statements) {
      s.toWasmCode(func, builder);
    }
    func.emitEnd();
  }
};

record TypedParam(TypedId id, Type type) implements TypedAstNode {};
record TypedParams(List<TypedParam> params) implements TypedAstNode {};
record TypedFndecl(String id, Optional<Params> params, Optional<Type> returnType, boolean export, List<TypedStatement> block) implements TypedStatement {

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
record TypedReturn(Optional<TypedExpression> expr) implements TypedStatement {
  @Override
  public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
    if(expr.isPresent()) {
      expr.get().toWasmCode(func, builder);
    } 
    func.emitEnd();
  }};

record TypedWhile(TypedExpression expr, TypedBlock block) implements TypedStatement {
  @Override
  public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
    expr.toWasmCode(func, builder); 
    func.emitIf();
    func.emitBlockType();
    func.emitLoop();
    func.emitBlockType();
    for(var s: block.statements()) {
      s.toWasmCode(func, builder);
    }
    expr.toWasmCode(func, builder);
    func.emitBrIf(0);

    func.emitEnd();
    func.emitEnd();
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

record TypedDerefAssign(TypedId id, TypedExpression expr) implements TypedStatement {

	@Override
	public void toWasmCode(Func func, TypedAstBuilder builder) throws IOException {
    func.emitLocalGet(id.sym().id); 
    expr.toWasmCode(func, builder);
    func.emitStore();
    
	}
}
