import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

sealed interface AstNode {
	String toDebugText(); 
};
sealed interface Statement extends AstNode {};
sealed interface Expression extends Statement {};

enum Type implements AstNode {
	String,
	Bool,
	Int;

	public String toDebugText() {
		switch(this) {
			case String: return "string"; 
			case Bool: return "bool"; 
			case Int: return "int"; 
			default: return "";
		}
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
		switch(this) {
			case Mul -> res = "*";
			case Add -> res = "+";
			case Div -> res = "/";
			case Eq -> res = "==";
			case Ge -> res = ">=";
			case Gt -> res = ">";
			case Le -> res = "<=";
			case Lt -> res = "<";
			case Neq -> res =  "!=";
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
};

record BoolLiteral(boolean lit) implements Expression  {
	public String toDebugText() {
		return String.format("%s", this.lit == true ? "true" : "false");
	}
}
record StringLiteral(String literal) implements Expression {
	public String toDebugText() {
		return String.format("%s", this.literal);
	}
};
record IntLiteral(int literal) implements Expression {
	public String toDebugText() {
		return String.format("%d", this.literal);
	}
};

record BinOp(Expression lhs, BinopType op, Expression rhs) implements Expression {
	public String toDebugText() {
		return String.format("%s %s %s", lhs.toDebugText(), op.toString(), rhs.toDebugText());
	}
};

record FncallArgs(List<Expression> args) implements AstNode {
	public String toDebugText() {
		String str = args
			.stream()
			.map(e -> e.toDebugText())
			.collect(Collectors.joining(","));
		return String.format("(%s)", str);
	}
};

record Fncall(Id id, Optional<FncallArgs> params) implements Expression {
	public String toDebugText() {
		return String.format("%s(%s)", id, params.map(FncallArgs::toDebugText).orElse(""));
	}
};

record VarDecl(Id id, Type type, Optional<Expression> expr) implements Statement {
	public String toDebugText() {
		return expr.map(
			e -> String.format("%s: %s = %s", id.toDebugText(), type.toDebugText(), e.toDebugText()))
												 .orElse(String.format("%s: %s", id.toDebugText(), type.toDebugText()));
	}
}
record Assign(Id id, Expression expr) implements Statement {
	public String toDebugText() {
		return String.format("%s = %s", id.toDebugText(), expr.toDebugText());
	}
};
record Block(List<Statement> statements) implements Statement {
	public String toDebugText() {
		String statementsString = statements
			.stream()
			.map(s -> s.toDebugText())
			.collect(Collectors.joining("\n"));	
		return String.format("{\n%s\n}", statementsString);
	}
};

record Params(List<Param> params) implements AstNode {
	public String toDebugText() {
		return String.format("(%s)", params.stream().map(Param::toDebugText).collect(Collectors.joining(",")));
	}
};
record Param(Id id, Type type) implements AstNode {
	public String toDebugText() {
		return String.format("%s : %s", id.toDebugText(), type.toDebugText());
	}
} 
record Fndecl(Id id, Optional<Params> params, Type returnType, Block block) implements Statement {
	public String toDebugText() {
		return String.format("fn %s(%s) -> %s %s", id.toDebugText(), params.map(Params::toDebugText).orElse(""), returnType.toDebugText(), block.toDebugText());
	}
}; 
record Return(Expression expr) implements Statement {
	public String toDebugText() {
		return String.format("return %s", expr.toDebugText());
	}
};
record While(Expression expr, Block block) implements Statement {
	public String toDebugText() {
		return String.format("while(%s) %s", expr.toDebugText(), block.toDebugText());
	}
}

record Cond(Expression cond, Block ifBlock, Optional<Block> elseBlock) implements Statement {
	public String toDebugText() {
		return String.format("if(%s) %s %s", cond.toDebugText(), ifBlock.toDebugText(), elseBlock.map(b -> "else %s" + b).orElse(""));
	}

};

