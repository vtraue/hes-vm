package org.example;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

import org.example.TypedAstBuilder.Function;

//TODO:(joh): Bessere Fehler!

sealed interface AstNode {
	Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder);
  String toDebugText();
}
;

sealed interface Statement extends AstNode {
}
;

sealed interface Expression extends Statement {
		
};

enum Type implements AstNode {
  String,
  Bool,
  Int;

  public String toString() {
    switch (this) {
      case String:
        return "string";
      case Bool:
        return "bool";
      case Int:
        return "int";
      default:
        return "";
    }
  }

@Override
public Result<TypedAstNode, java.lang.String> getTypedAstNode(TypedAstBuilder builder) {
	// TODO Auto-generated method stub
	throw new UnsupportedOperationException("Unimplemented method 'getTypedAstNode'");
}

@Override
public java.lang.String toDebugText() {
	// TODO Auto-generated method stub
	throw new UnsupportedOperationException("Unimplemented method 'toDebugText'");
}
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
  Le;

  public String toString() {
    String res;
    switch (this) {
      case Mul -> res = "*";
      case Add -> res = "+";
      case Div -> res = "/";
      case Eq -> res = "==";
      case Ge -> res = ">=";
      case Gt -> res = ">";
      case Le -> res = "<=";
      case Lt -> res = "<";
      case Neq -> res = "!=";
      case Sub -> res = "-";
      default -> res = "Unknown";
    }
    return res;
  }
}

record Id(String name) implements Expression {
  public String toDebugText() {
    return String.format("%s", this.name);
  }
	@Override
	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
		Optional<TypedAstBuilder.Symbol> sym = builder.searchVariable(this.name);
		if(sym.isPresent()) {
			return new Ok<>(new TypedId(name,sym.get()));
		} 
		return new Err<>(String.format("Unresolved Type %s", this.name));
	}
}
;

sealed interface Literal extends Expression {};

record BoolLiteral(boolean lit) implements Literal {
  public String toDebugText() {
    return String.format("%s", this.lit == true ? "true" : "false");
  }
	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
		return new Ok<>(new TypedLiteral(this, Type.Bool));
	}
}

record StringLiteral(String literal) implements Literal {
  public String toDebugText() {
    return String.format("%s", this.literal);
  }
	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
		return new Ok<>(new TypedLiteral(this, Type.String));
	}
}
;

record IntLiteral(int literal) implements Literal {
  public String toDebugText() {
    return String.format("%d", this.literal);
  }
	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
		return new Ok<>(new TypedLiteral(this, Type.Int));
	}
}
;

record BinOp(Expression lhs, BinopType op, Expression rhs) implements Expression {
  public String toDebugText() {
    return String.format("%s %s %s", lhs.toDebugText(), op.toString(), rhs.toDebugText());
  }

	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
		var typedLhs = lhs.getTypedAstNode(builder); 

		if(typedLhs instanceof Err message) {
			return new Err<>(String.format("Error in Binary Operation lhs: %s", message.err())); 
		}

		var typedRhs = rhs.getTypedAstNode(builder);
		if(typedRhs instanceof Err message) {
			return new Err<>(String.format("Error in Binary Operation rhs: %s", message.err())); 
		}
		TypedExpression typedLhsData = (TypedExpression)typedLhs.unwrap();
		TypedExpression typedRhsData = (TypedExpression)typedLhs.unwrap();
		Type lhsT = typedLhsData.getType();
		Type rhsT = typedRhsData.getType();

		if(!typedLhsData.getType().equals(typedRhsData.getType())) {
			return new Err<>(String.format("Expected %s %s %s, got %s %s %s", lhsT.toString(), op.toString(), lhsT.toString(), lhsT.toString(), op.toString(), rhsT.toString()));
		}

		return new Ok<>(new TypedBinOP(typedLhsData, op, typedRhsData));
	}
}
;

record FncallArgs(List<Expression> args) implements AstNode {
  public String toDebugText() {
    String str = args.stream().map(e -> e.toDebugText()).collect(Collectors.joining(","));

    return String.format("(%s)", str);
  }

	@Override
	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {

		var result = builder.getExpressionTypes(this.args);
		if(!result.isOk()) {
			return new Err<>(result.getErr());
		}
		return new Ok<>(new TypedFncallArgs(result.unwrap()));
	}
}
;

record Fncall(Id id, Optional<FncallArgs> params) implements Expression {
	public String toDebugText() {
		return String.format("%s%s", id.toDebugText(), params.map(FncallArgs::toDebugText).orElse(""));
	}
	@Override
	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
		Result<TypedAstNode, String> tId = this.id.getTypedAstNode(builder);	
		if(!tId.isOk()) {
			return new Err<>(tId.getErr());
		}

		
	Optional<TypedFncallArgs> tArgs = Optional.empty(); 
	if(this.params.isPresent()) {
		Result<TypedAstNode, String> tempArgs = this.params.get().getTypedAstNode(builder);
		if(!tempArgs.isOk()) {
				return new Err<>(tempArgs.getErr());
		}
		tArgs = Optional.of((TypedFncallArgs)tempArgs.unwrap());
	}

		Optional<Function> funcType = builder.getFunction(id.name()); 
		if(!funcType.isPresent()) {
			return new Err<>(String.format("Function %s not resolved", this.id));
		}
		return new Ok<>(new TypedFncall((TypedId)tId.unwrap(), tArgs, funcType.get().returnType));
	}	
};

record VarDecl(Id id, Type type, Optional<Expression> expr) implements Statement {
  public String toDebugText() {
    return expr.map(
            e -> String.format("%s: %s = %s", id.toDebugText(), type.toString(), e.toDebugText()))
        .orElse(String.format("%s: %s", id.toDebugText(), type.toString()));
  }
	@Override
	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
		Result<TypedAstNode, String> tId = this.id.getTypedAstNode(builder);
		if(!tId.isOk()) {
			return new Err<>(tId.getErr());
		}
		Optional<TypedExpression> typedExpression = Optional.empty();

		if(expr.isPresent()) {
			Result<TypedAstNode, String> node = this.expr.get().getTypedAstNode(builder);
			if(!node.isOk()) {
				return new Err<>(tId.getErr());
			}
			var expr = (TypedExpression)node.unwrap();

			if(!expr.getType().equals(this.type)) {
				return new Err<>(String.format("Cannot initialize Variable %s of type %s with expression of type %s", id.name(), type.toString(), expr.getType()));
			}
			typedExpression = Optional.of(expr);
		}
		return new Ok<>(new TypedVarDecl((TypedId)tId.unwrap(), this.type, typedExpression));	
	}
}

record Assign(Id id, Expression expr) implements Statement {
  public String toDebugText() {
    return String.format("%s = %s", id.toDebugText(), expr.toDebugText());
  }

	@Override
	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
		Result<TypedAstNode, String> tId = this.id.getTypedAstNode(builder);
		if(!tId.isOk()) {
			return new Err<>(tId.getErr());
		}
		Result<TypedAstNode, String> typedExpressionRes = this.id.getTypedAstNode(builder);
		if(!typedExpressionRes.isOk()) {
			return new Err<>(tId.getErr());
		}

		TypedExpression typedExpression = (TypedExpression) typedExpressionRes.unwrap(); 
		TypedId typedId = (TypedId) tId.unwrap();

		if(!typedExpression.getType().equals(typedId.getType())) {
			return new Err<>(String.format("Cannot assign an expression of type %s to variable %s", typedExpression.getType(), typedId.getType()));
		}

		return new Ok<>(new TypedAssign(typedId, typedExpression));
	}
}
;

record Block(List<Statement> statements) implements Statement {
	public String toDebugText() {
		String statementsString = statements
			.stream()
			.map(s -> s.toDebugText())
			.collect(Collectors.joining("  \n  "));	
		return String.format("{\n  %s\n}", statementsString);
	}
	@Override
	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
		StringBuilder errorMessageBuilder = new StringBuilder();
		List<TypedStatement> typedStatements = new ArrayList<>();
		boolean hasErrors = false;
		builder.enterNewScope();
		for(Statement s : statements) {
			Result<TypedAstNode, String> typedResult = s.getTypedAstNode(builder);
			if(!typedResult.isOk()) {
				hasErrors = true;
				errorMessageBuilder.append(typedResult.getErr() + "\n");
			}
				TypedAstNode typedNode = typedResult.unwrap(); 
				typedStatements.add((TypedStatement)typedNode);
			} 
		builder.leaveScope();
		if(hasErrors) {
			return new Err<>(errorMessageBuilder.toString());
		}

		return new Ok<>(new TypedBlock(typedStatements));
	}
};

record Param(Id id, Type type) implements AstNode {
  public String toDebugText() {
    return String.format("%s : %s", id.toDebugText(), type.toString());
  }
	@Override
	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
		Result<TypedAstNode, String> tId = this.id.getTypedAstNode(builder);
		if(!tId.isOk()) {
			return new Err<>(tId.getErr());
		}
		return new Ok<>(new TypedParam((TypedId)tId.unwrap(), this.type));
	}

}

record Params(List<Param> params) implements AstNode {
  public String toDebugText() {
    return String.format(
        "(%s)", params.stream().map(Param::toDebugText).collect(Collectors.joining(",")));
  }

	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
		StringBuilder errorMessageBuilder = new StringBuilder();
		List<TypedParam> typedParams = new ArrayList<>();
		boolean hasErrors = false;
		for(Param p : params) {
			Result<TypedAstNode, String> typedResult = p.getTypedAstNode(builder);
			if(!typedResult.isOk()) {
				hasErrors = true;
				errorMessageBuilder.append(typedResult.getErr() + "\n");
			}
				TypedAstNode typedNode = typedResult.unwrap(); 
				typedParams.add((TypedParam)typedNode);
			}
		if(hasErrors) {
			return new Err<>(errorMessageBuilder.toString());
		}
		return new Ok<>(new TypedParams(typedParams));
	}
}
;


record Fndecl(Id id, Optional<Params> params, Type returnType, Block block) implements Statement {
  public String toDebugText() {
    return String.format(
        "fn %s(%s) -> %s %s",
        id.toDebugText(),
        params.map(Params::toDebugText).orElse(""),
        returnType.toString(),
        block.toDebugText());
  }

	@Override
	public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
		// TODO Auto-generated method stub
		var tId = this.id.getTypedAstNode(builder);
		if(!tId.isOk()) {
			return new Err<>(tId.getErr());
		}
		Optional<TypedParams> typedParams = Optional.empty();  		
		if(params.isPresent()) {
			var typedParamsNode = this.params.get().getTypedAstNode(builder);
			if(!typedParamsNode.isOk()) {
				return new Err<>(typedParamsNode.getErr());
			}
			typedParams = Optional.of((TypedParams)typedParamsNode.unwrap());
		}
		var typedBlockResult = this.block.getTypedAstNode(builder);
		if(!typedBlockResult.isOk()) {
			return new Err<>(typedBlockResult.getErr());
		}
		return new Ok<>(new TypedFndecl((TypedId)tId.unwrap(), typedParams, returnType, (TypedBlock)typedBlockResult.unwrap()));
	}
}
;

record Return(Expression expr) implements Statement {
  public String toDebugText() {
    return String.format("return %s", expr.toDebugText());
  }
}
;

record While(Expression expr, Block block) implements Statement {
  public String toDebugText() {
    return String.format("while(%s) %s", expr.toDebugText(), block.toDebugText());
  }
}

record Cond(Expression cond, Block ifBlock, Optional<Block> elseBlock) implements Statement {
  public String toDebugText() {
    return String.format(
        "if(%s) %s %s",
        cond.toDebugText(), ifBlock.toDebugText(), elseBlock.map(b -> "else %s" + b).orElse(""));
  }
}
;

