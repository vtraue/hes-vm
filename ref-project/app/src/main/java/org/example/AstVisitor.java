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
  private int stringLiteralPointer = 0;
  
  List<Statement> statements = new ArrayList<Statement>();
  

  @Override
  public AstNode visitStatement(ReflangParser.StatementContext ctx) {
    var result = visitChildren(ctx);
    if(result != null && this.depth == 0) {
      this.statements.add((Statement)result);
    }
    return result;  
  }

  @Override
  public AstNode visitStmtExpr(ReflangParser.StmtExprContext ctx) {
    var result = (Expression)this.visit(ctx.e);
    return result;
  }
  @Override
  public AstNode visitAssign(ReflangParser.AssignContext ctx) {
    Id varname = (Id)this.visit(ctx.name);
    Expression initExpr = (Expression)(this.visit(ctx.init_expr));    
    return new Assign(varname, initExpr);
  }

  public AstNode visitBreak(ReflangParser.BreakContext ctx) {
      if(ctx.expr() != null) {
          return new Break(Optional.of((Expression)this.visit(ctx.expr())));
      } else {
        return new Break(Optional.empty());
      }
  }
  
  @Override 
  public AstNode visitDerefAssign(ReflangParser.DerefAssignContext ctx) {
    Id varname = (Id)this.visit(ctx.name); 
    Expression expr = (Expression)(this.visit(ctx.init_expr));
    return new DerefAssign(varname, expr);
  }
  
  
  @Override
  public AstNode visitImport_fndecl(ReflangParser.Import_fndeclContext ctx) {
    Optional<Params> params = Optional.empty(); 
    if(ctx.params() != null) {
      params = Optional.of((Params)this.visit(ctx.params())); 
    }
    Optional<Type> retType = Optional.empty();
    if(ctx.ret_type != null) {
      retType = Optional.of((Type)this.visit(ctx.ret_type));  
    }
    String env = ctx.env_name.getText();
    return new ExternFndecl(ctx.name.getText(), env, params, retType);    
  }
  @Override
  public AstNode visitVardeclt(ReflangParser.VardecltContext ctx) {
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
    Id fnName = (Id)this.visit(ctx.name);
    Optional<Params> params = Optional.empty(); 
    if(ctx.params() != null) {
      params = Optional.of((Params)this.visit(ctx.params())); 
    }

    Optional<Type> retType = Optional.empty();
    if(ctx.ret_type != null) {
      retType = Optional.of((Type)this.visit(ctx.ret_type));  
    }

    Block declBlock = (Block)this.visit(ctx.decl_block);

    return new Fndecl(fnName, params, retType, false, declBlock);
  }
  @Override
  public AstNode visitExport_fndecl(ReflangParser.Export_fndeclContext ctx) {
    Id fnName = (Id)this.visit(ctx.name);
    Optional<Params> params = Optional.empty(); 
    if(ctx.params() != null) {
      params = Optional.of((Params)this.visit(ctx.params())); 
    }

    Optional<Type> retType = Optional.empty();
    if(ctx.ret_type != null) {
      retType = Optional.of((Type)this.visit(ctx.ret_type));  
    }
    Block declBlock = (Block)this.visit(ctx.decl_block);
    return new Fndecl(fnName, params, retType, true, declBlock);

  }

  @Override
  public AstNode visitParams(ReflangParser.ParamsContext ctx) {
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

    Id name = (Id)this.visit(ctx.name); 
    Type t = (Type)this.visit(ctx.t);

    return new Param(name, t);
  }

  @Override
  public AstNode visitReturn(ReflangParser.ReturnContext ctx) {
    if(ctx.expr() != null) {
      return new Return(Optional.of((Expression)this.visit(ctx.expr())));
    } else {
      return new Return(Optional.empty());
    }
  }
  @Override 
  public AstNode visitVarname(ReflangParser.VarnameContext ctx) {
    return new Id(ctx.name.getText());  
  }

  @Override
  public AstNode visitFncall(ReflangParser.FncallContext ctx) {
    Optional<FncallArgs> args = Optional.empty(); 

    if(ctx.args() != null) {
      args = Optional.of((FncallArgs)this.visit(ctx.args())); 
    }
    var result = new Fncall(ctx.name.getText(), args);

    return result; 
  }

  @Override
  public AstNode visitArgs(ReflangParser.ArgsContext ctx) {
    Stream<Expression> rest_stream = ctx.rest
      .stream()
      .map(p -> (Expression)this.visit(p));

    Stream<Expression> first_stream = Stream.of((Expression)this.visit(ctx.first));

    List<Expression> expressions = Stream.concat(first_stream, rest_stream)
      .collect(Collectors.toList());
    
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
    Expression condExpression = (Expression)this.visit(ctx.expr());
    Block ifBlock = (Block)this.visit(ctx.if_block);
    Optional<Block> elseBlock = Optional.empty(); 
    if(ctx.else_block != null) {
      elseBlock = Optional.of((Block)this.visit(ctx.else_block)); 
    }

    return new Cond(condExpression, ifBlock, elseBlock);
  }
  //NOTE: (joh): das geht bestimmt besser
  @Override
  public AstNode visitTInt(ReflangParser.TIntContext ctx) {
    return PrimitiveType.Int;
  }
  @Override
  public AstNode visitTString (ReflangParser.TStringContext ctx) {
    return PrimitiveType.String;
  }

  @Override
  public AstNode visitTBool (ReflangParser.TBoolContext ctx) {
    return PrimitiveType.Bool;
  }
  
  @Override
  public AstNode visitLiteralNmb(ReflangParser.LiteralNmbContext ctx) {
    return new IntLiteral(Integer.parseInt(ctx.NUMBER().getText())); 
  }
  
  @Override
  public AstNode visitLiteralStr(ReflangParser.LiteralStrContext ctx) {
    String str = ctx.STRING().getText();
    String text = str.substring(1, str.length() - 1);
    var literal = new StringLiteral(text, stringLiteralPointer);   
    stringLiteralPointer += text.length() + 5;
    return literal;
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

  @Override
  public AstNode visitPointerType(ReflangParser.PointerTypeContext ctx) {
    int depth = ctx.depth.size();
    PrimitiveType parent = (PrimitiveType)this.visit(ctx.parent); 
    return new PointerType(parent, depth);
  }

  @Override
  public AstNode visitRef(ReflangParser.RefContext ctx) {
    Id target = (Id)this.visit(ctx.name);    
    return new Ref(target);
  }

  @Override
  public AstNode visitDeref(ReflangParser.DerefContext ctx) {
    Id target = (Id)this.visit(ctx.name);    
    return new Deref(target);
  }

  @Override
  public AstNode visitCast(ReflangParser.CastContext ctx) {
    Type t = (Type)this.visit(ctx.t);
    Expression expr = (Expression)this.visit(ctx.src_expr);    
    return new Cast(t, expr);
  }
}

