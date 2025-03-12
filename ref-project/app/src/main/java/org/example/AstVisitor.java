package org.example;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;
import java.util.stream.Stream;
public class AstVisitor extends ReflangBaseVisitor<AstNode>{
	int depth = 0;
	int currentVarDeclId = 0;
	List<Statement> statements = new ArrayList<Statement>();
	

	@Override
	public AstNode visitStatement(ReflangParser.StatementContext ctx) {
		System.out.println("stmt");
		var result = visitChildren(ctx);
		if(result != null && this.depth == 0) {
			this.statements.add((Statement)result);
		}
		return result;	
	}

	@Override
	public AstNode visitStmtExpr(ReflangParser.StmtExprContext ctx) {
		System.out.println("stmt");
		var result = (Expression)this.visit(ctx.e);
		System.out.println(result.toDebugText());
		return result;
	}
	@Override
	public AstNode visitAssign(ReflangParser.AssignContext ctx) {
		System.out.println("assign");
		Id varname = (Id)this.visit(ctx.name);
		Expression initExpr = (Expression)(this.visit(ctx.init_expr));		
		return new Assign(varname, initExpr);
	}

	@Override
	public AstNode visitImport_fndecl(ReflangParser.Import_fndeclContext ctx) {
		Id fnName = (Id)this.visit(ctx.name);
		Optional<Params> params = Optional.empty(); 
		if(ctx.params() != null) {
			params = Optional.of((Params)this.visit(ctx.params()));	
		}
		Type ret_type = (Type)this.visit(ctx.ret_type);

		return new ExternFndecl(fnName, params, ret_type);		

	}
	@Override
	public AstNode visitVardeclt(ReflangParser.VardecltContext ctx) {
		System.out.println("vardecl");
		Id varname = new Id(ctx.name.getText());
		Type t = (Type)this.visit(ctx.t);

		Optional<Expression> initExpr = Optional.empty(); 
		if(ctx.init_expr != null) {
			initExpr = Optional.of((Expression)this.visit(ctx.init_expr));	
		}
		
		return new VarDecl(varname, Optional.of(t), initExpr);
	}
	@Override
	public AstNode visitVardecl(ReflangParser.VardeclContext ctx) {
		Id varname = new Id(ctx.name.getText());
		return new VarDecl(varname, Optional.empty(), Optional.of((Expression)this.visit(ctx.init_expr)));
		
	}
	/*
	@Override
	public AstNode visitId(ReflangParser.IdContext ctx) {
		System.out.println("id");
		return new Id(ctx.ID().getText());
	}
	*/

	@Override
	public AstNode visitBlock(ReflangParser.BlockContext ctx) {
		System.out.println("block");
		this.depth += 1;
		List<Statement> statements = ctx.statements
			.stream()
			.map(statement -> (Statement)this.visit(statement))
			.filter(s -> s != null)
			.collect(Collectors.toList());
		this.depth -= 1;
		//System.out.println(statements.stream().map(Statement::toDebugText).collect(Collectors.joining("\n")));
		return new Block(statements);
	}

	@Override
	public AstNode visitFndecl(ReflangParser.FndeclContext ctx) {
		System.out.println("fndecl");
		Id fnName = (Id)this.visit(ctx.name);
		Optional<Params> params = Optional.empty(); 
		if(ctx.params() != null) {
			params = Optional.of((Params)this.visit(ctx.params()));	
		}

		Type ret_type = (Type)this.visit(ctx.ret_type);
		Block declBlock = (Block)this.visit(ctx.decl_block);

		return new Fndecl(fnName, params, ret_type, declBlock);
	}
	@Override
	public AstNode visitParams(ReflangParser.ParamsContext ctx) {
		System.out.println("Params");
		Stream<Param> rest_stream = ctx.rest
			.stream()
			.map(p -> (Param)this.visit(p));

		Stream<Param> first_stream = Stream.of((Param)this.visit(ctx.first));

		List<Param> params = Stream.concat(first_stream, rest_stream)
			.collect(Collectors.toList());

		return new Params(params);
	}

	@Override
	public AstNode visitParam(ReflangParser.ParamContext ctx) {
		System.out.println("Param");

		Id name = (Id)this.visit(ctx.name);	
		Type t = (Type)this.visit(ctx.t);

		return new Param(name, t);
	}

	@Override
	public AstNode visitReturn(ReflangParser.ReturnContext ctx) {
		return new Return((Expression)this.visit(ctx.expr()));
	}
	@Override 
	public AstNode visitVarname(ReflangParser.VarnameContext ctx) {
		System.out.println("Varname");
		return new Id(ctx.name.getText());	
	}

	@Override
	public AstNode visitFncall(ReflangParser.FncallContext ctx) {
		System.out.println("Fncall");
		Id name = (Id)this.visit(ctx.name); 
		System.out.println(name.toDebugText());
		Optional<FncallArgs> args = Optional.empty(); 

		if(ctx.args() != null) {
			args = Optional.of((FncallArgs)this.visit(ctx.args()));	
		}
		var result = new Fncall(name, args);
		System.out.println(result.toDebugText());

		return result; 
	}

	@Override
	public AstNode visitArgs(ReflangParser.ArgsContext ctx) {
		System.out.println("Args");
		Stream<Expression> rest_stream = ctx.rest
			.stream()
			.map(p -> (Expression)this.visit(p));

		Stream<Expression> first_stream = Stream.of((Expression)this.visit(ctx.first));

		List<Expression> expressions = Stream.concat(first_stream, rest_stream)
			.collect(Collectors.toList());
		
		System.out.println("Args done");
		return new FncallArgs(expressions);
	}

	@Override
	public AstNode visitWhile(ReflangParser.WhileContext ctx) {
		Expression expr = (Expression)this.visit(ctx.expr());
		Block block = (Block)this.visit(ctx.block());
		return new While(expr, block);
	}

	@Override
	public AstNode visitCond(ReflangParser.CondContext ctx) {
		System.out.println("cond");
		Expression condExpression = (Expression)this.visit(ctx.expr());
		Block ifBlock = (Block)this.visit(ctx.if_block);
		System.out.printf("If block: %s", ifBlock.toDebugText());
		Optional<Block> elseBlock = Optional.empty(); 
		if(ctx.else_block != null) {
			elseBlock = Optional.of((Block)this.visit(ctx.else_block));	
		}

		return new Cond(condExpression, ifBlock, elseBlock);
	}
	//NOTE: (joh): das geht bestimmt besser
	@Override
	public AstNode visitTInt(ReflangParser.TIntContext ctx) {
		System.out.println("tint");
		return Type.Int;
	}
	@Override
	public AstNode visitTString (ReflangParser.TStringContext ctx) {
		return Type.String;
	}

	@Override
	public AstNode visitTBool (ReflangParser.TBoolContext ctx) {
		return Type.Bool;
	}
	
	@Override
	public AstNode visitLiteralNmb(ReflangParser.LiteralNmbContext ctx) {
		return new IntLiteral(Integer.parseInt(ctx.NUMBER().getText())); 
	}
	
	@Override
	public AstNode visitLiteralStr(ReflangParser.LiteralStrContext ctx) {
		System.out.println("String literal");
		return new StringLiteral(ctx.STRING().getText());
	}
	@Override
	public AstNode visitLiteralTrue(ReflangParser.LiteralTrueContext ctx) {
		return new BoolLiteral(true);	
	}
	@Override
	public AstNode visitLiteralFalse(ReflangParser.LiteralFalseContext ctx) {
		return new BoolLiteral(false);
	}

	@Override
	public AstNode visitMult(ReflangParser.MultContext ctx) {
		BinopType op = BinopType.Mul;
		Expression lhs = (Expression)this.visit(ctx.lhs);
		Expression rhs = (Expression)this.visit(ctx.rhs);
		return new BinOp(lhs, op, rhs); 
	}

	@Override
	public AstNode visitAdd(ReflangParser.AddContext ctx) {
		BinopType op = BinopType.Add;
		Expression lhs = (Expression)this.visit(ctx.lhs);
		Expression rhs = (Expression)this.visit(ctx.rhs);
		return new BinOp(lhs, op, rhs); 
	}

	@Override
	public AstNode visitDiv(ReflangParser.DivContext ctx) {
		BinopType op = BinopType.Div;
		Expression lhs = (Expression)this.visit(ctx.lhs);
		Expression rhs = (Expression)this.visit(ctx.rhs);
		return new BinOp(lhs, op, rhs); 
	}

	@Override
	public AstNode visitSub(ReflangParser.SubContext ctx) {
		BinopType op = BinopType.Sub;
		Expression lhs = (Expression)this.visit(ctx.lhs);
		Expression rhs = (Expression)this.visit(ctx.rhs);
		return new BinOp(lhs, op, rhs); 
	}

	@Override
	public AstNode visitEq(ReflangParser.EqContext ctx) {
		BinopType op = BinopType.Eq;
		Expression lhs = (Expression)this.visit(ctx.lhs);
		Expression rhs = (Expression)this.visit(ctx.rhs);
		return new BinOp(lhs, op, rhs); 
	}

	@Override
	public AstNode visitNeq(ReflangParser.NeqContext ctx) {
		BinopType op = BinopType.Neq;
		Expression lhs = (Expression)this.visit(ctx.lhs);
		Expression rhs = (Expression)this.visit(ctx.rhs);
		return new BinOp(lhs, op, rhs); 
	}

	@Override
	public AstNode visitGt(ReflangParser.GtContext ctx) {
		BinopType op = BinopType.Gt;
		Expression lhs = (Expression)this.visit(ctx.lhs);
		Expression rhs = (Expression)this.visit(ctx.rhs);
		return new BinOp(lhs, op, rhs); 
	}

	@Override
	public AstNode visitLt(ReflangParser.LtContext ctx) {
		BinopType op = BinopType.Lt;
		Expression lhs = (Expression)this.visit(ctx.lhs);
		Expression rhs = (Expression)this.visit(ctx.rhs);
		return new BinOp(lhs, op, rhs); 
	}

	@Override
	public AstNode visitLe(ReflangParser.LeContext ctx) {
		BinopType op = BinopType.Le;
		Expression lhs = (Expression)this.visit(ctx.lhs);
		Expression rhs = (Expression)this.visit(ctx.rhs);
		return new BinOp(lhs, op, rhs); 
	}

	@Override
	public AstNode visitGe(ReflangParser.GeContext ctx) {
		BinopType op = BinopType.Ge;
		Expression lhs = (Expression)this.visit(ctx.lhs);
		Expression rhs = (Expression)this.visit(ctx.rhs);
		return new BinOp(lhs, op, rhs); 
	}
}


