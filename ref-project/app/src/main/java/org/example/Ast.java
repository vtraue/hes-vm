package org.example;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

import org.example.TypedAstBuilder.Function;
import org.example.TypedAstBuilder.Symbol;

import wasm_builder.WasmValueType;

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
  Int,
  Void;

  public String toString() {
    switch (this) {
      case String:
        return "string";
      case Bool:
        return "bool";
      case Int:
        return "int";
      case Void:
        return "void";
      default:
        return "";
    }
  }
  public WasmValueType toWasmValueType() {
    return WasmValueType.i32;
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
  public void toWasmCode(wasm_builder.Func func) throws IOException {
    switch (this){
      case Mul -> func.emitMul();
      case Add -> func.emitAdd(); 
      case Div -> func.emitDiv();
      case Eq -> func.emitEq();
      case Ge -> func.emitGe();
      case Gt -> func.emitGt();
      case Le -> func.emitLe();
      case Lt -> func.emitLt();
      case Neq -> func.emitNe();
      case Sub -> func.emitSub();
    }
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

record StringLiteral(String literal, int pointer) implements Literal {
  public String toDebugText() {
    return String.format("%s", this.literal);
  }

  public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
    return new Ok<>(new TypedLiteral(this, Type.String));
  }
}

record IntLiteral(int literal) implements Literal {
  public String toDebugText() {
    return String.format("%d", this.literal);
  }
  public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
    return new Ok<>(new TypedLiteral(this, Type.Int));
  }
}


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
    TypedExpression typedRhsData = (TypedExpression)typedRhs.unwrap();
    Type lhsT = typedLhsData.getType();
    Type rhsT = typedRhsData.getType();

    if(!typedLhsData.getType().equals(typedRhsData.getType())) {
      return new Err<>(String.format("Expected %s %s %s, got %s %s %s", lhsT.toString(), op.toString(), lhsT.toString(), lhsT.toString(), op.toString(), rhsT.toString()));
    }

    return new Ok<>(new TypedBinOP(typedLhsData, op, typedRhsData));
  }
}


record FncallArgs(List<Expression> args) implements AstNode {
  public String toDebugText() {
    String str = args.stream().map(AstNode::toDebugText).collect(Collectors.joining(","));

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


record Fncall(String name, Optional<FncallArgs> params) implements Expression {
  public String toDebugText() {
    return String.format("%s%s", name, params.map(FncallArgs::toDebugText).orElse(""));
  }

  @Override
  public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {

  String fnName = this.name;    
  Optional<Function> functionType = builder.getFunction(fnName);  

  if(functionType.isEmpty()) {
      return new Err<>(String.format("Function %s doesnt exists", fnName));
  }
  Optional<TypedFncallArgs> tArgs = Optional.empty(); 
  
  if(this.params.isPresent()) {
    if(functionType.get().getArgs().isEmpty()) {
      return new Err<>(String.format("Expected no arguments, got %d", this.params.get().args().size()));
    }
    var expectedArgs = functionType
      .get()
      .getArgs()
      .get()
      .toTypes(); 

    var paramTypes = this.params.get().args();
    if(expectedArgs.size() != paramTypes.size()) {
      return new Err<>(String.format("Expected %d arguments, got %d", expectedArgs.size(), paramTypes.size()));
    }
    Result<TypedAstNode, String> tempArgs = this.params.get().getTypedAstNode(builder);
    if(!tempArgs.isOk()) {
        return new Err<>(tempArgs.getErr());
    }
    tArgs = Optional.of((TypedFncallArgs)tempArgs.unwrap());
  }
  else {
    if(functionType.get().getArgs().isPresent()) {
      return new Err<>(String.format("Expected %d arguments, got none", functionType.get().getArgs().get().params().size())); 
    }
  }
    return new Ok<>(new TypedFncall(fnName, functionType.get(), tArgs));
  } 
}

record VarDecl(Id id, Optional<Type> type, Optional<Expression> expr) implements Statement {
  public String toDebugText() {
    return expr.map(
            e -> String.format("%s :%s= %s", id.toDebugText(), type.isPresent() ? " " + type.get().toString() : "", e.toDebugText()))
        .orElse(String.format("%s: %s", id.toDebugText(), type.toString()));
  }
  @Override
  public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
    Optional<TypedExpression> typedExpression = Optional.empty();

    if(expr.isPresent()) {
      Result<TypedAstNode, String> node = this.expr.get().getTypedAstNode(builder);
      if(!node.isOk()) {
        return new Err<>(node.getErr());
      }
      var expr = (TypedExpression)node.unwrap();

      typedExpression = Optional.of(expr);
    }
    Type t;
    if(type.isPresent()) {
      if(typedExpression.isPresent()) {
        if(!typedExpression.get().getType().equals(this.type.get())) {
          return new Err<>(String.format("Cannot initialize Variable %s of type %s with expression of type %s", id.name(), type.toString(), typedExpression.get().getType()));
        }
      }
      t = type.get();
    } else {
      t = typedExpression.get().getType();
    }

    Result<Symbol, Symbol> symResult = builder.addVariable(this.id.name(), t);
    if(!symResult.isOk()) {
      return new Err<>(String.format("Variable %s already exists in scope", id.name()));
    }

    return new Ok<>(new TypedVarDecl(new TypedId(id.name(), symResult.unwrap()), t, typedExpression));  
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
    Result<TypedAstNode, String> typedExpressionRes = this.expr.getTypedAstNode(builder);
    if(!typedExpressionRes.isOk()) {
      return new Err<>(tId.getErr());
    }

    TypedExpression typedExpression = (TypedExpression) typedExpressionRes.unwrap(); 
    TypedId typedId = (TypedId) tId.unwrap();

    System.out.printf("TypedExpression: %s\n", typedExpression.getType().toString()); 
    System.out.printf("TypedID: %s\n", typedId.getType().toString()); 
    if(!typedExpression.getType().equals(typedId.getType())) {
      return new Err<>(String.format("Cannot assign an expression of type %s to variable %s", typedExpression.getType(), typedId.getType()));
    }

    return new Ok<>(new TypedAssign(typedId, typedExpression));
  }
}

record Break(Optional<Expression> expr) implements Expression {
  public String toDebugText() {
    return String.format("break %s", expr.map(AstNode::toDebugText).orElse(""));
  }
  @Override
  public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
    Type resType = Type.Void;
    Optional<TypedExpression> tExpr = Optional.empty();
    if(expr.isPresent()) {
      var br_expr = expr.get();
      Result<TypedAstNode, String> typedExpressionRes = br_expr.getTypedAstNode(builder);
      if(!typedExpressionRes.isOk()) {
        return new Err<>(typedExpressionRes.getErr());
      }
      var expr = (TypedExpression)typedExpressionRes.unwrap();
      var exprT = expr.getType();
      tExpr = Optional.of((TypedExpression)typedExpressionRes.unwrap());
    }

    return new Ok<>(new TypedBreak(tExpr, resType));
  }
}

record Block(List<Statement> statements) implements Expression {
  public String toDebugText() {
    String statementsString = statements
      .stream()
      .map(AstNode::toDebugText)
      .collect(Collectors.joining("  \n  ")); 
    return String.format("{\n  %s\n}", statementsString);
  }
  @Override
  public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
    StringBuilder errorMessageBuilder = new StringBuilder();
    List<TypedStatement> typedStatements = new ArrayList<>();
    Optional<Type> resultType = Optional.empty(); 
    boolean hasErrors = false;
    builder.enterNewScope();

    for(Statement s : statements) {
      Result<TypedAstNode, String> typedResult = s.getTypedAstNode(builder);
      if(!typedResult.isOk()) {
        hasErrors = true;
        errorMessageBuilder.append(typedResult.getErr() + "\n");
      } else {
        TypedAstNode typedNode = typedResult.unwrap(); 
        typedStatements.add((TypedStatement)typedNode);

        if(typedNode instanceof TypedBreak b) {
          if(resultType.isPresent() && !resultType.get().equals(b.getType())) {
            return new Err<>("Cannot break from block with differing types");
          }
          resultType = Optional.of(b.getType());
        }
      }
    }
    builder.leaveScope();

    if(hasErrors) {
      return new Err<>(errorMessageBuilder.toString());
    }

    return new Ok<>(new TypedBlock(typedStatements, resultType.orElse(Type.Void)));
  }
}

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

  public WasmValueType toWasmValueType() {
    return this.type.toWasmValueType();
  }
}

record Params(List<Param> params) implements AstNode {
  public String toDebugText() {
    return String.format(
        "(%s)", params.stream().map(Param::toDebugText).collect(Collectors.joining(",")));
  }

  public List<WasmValueType> toWasmValueTypes() {
    return this.params
      .stream()
      .map(Param::toWasmValueType)
      .toList();
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
      } else {
        TypedAstNode typedNode = typedResult.unwrap(); 
        typedParams.add((TypedParam)typedNode);
      }
    }
    if(hasErrors) {
      return new Err<>(errorMessageBuilder.toString());
    }
    return new Ok<>(new TypedParams(typedParams));
  }

  public List<Type> toTypes() {
    return this.params.stream().map(Param::type).toList();
  }
}


record ExportDecl(String env) {};
record Fndecl(Id id, Optional<Params> params, Optional<Type> returnType, boolean export, Block block) implements Statement {
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
    //Optional<TypedParams> typedParams = Optional.empty();     
    /*
    if(params.isPresent()) {
      var typedParamsNode = this.params.get().getTypedAstNode(builder);
      if(!typedParamsNode.isOk()) {
        return new Err<>(typedParamsNode.getErr());
      }
      typedParams = Optional.of((TypedParams)typedParamsNode.unwrap());
    }
    */

    builder.enterNewFunction(id.name(), returnType, params, export);
    StringBuilder errorMessageBuilder = new StringBuilder();
    List<TypedStatement> typedStatements = new ArrayList<>();
    boolean hasErrors = false;

    for(Statement s : block.statements()) {
      Result<TypedAstNode, String> typedResult = s.getTypedAstNode(builder);
      if(!typedResult.isOk()) {
        hasErrors = true;
        errorMessageBuilder.append(typedResult.getErr() + "\n");
      } else {
        TypedAstNode typedNode = typedResult.unwrap(); 
        typedStatements.add((TypedStatement)typedNode);
      }
    }
    if(hasErrors) {
      return new Err<>(errorMessageBuilder.toString());
    }
    builder.leaveFunction();  

    return new Ok<>(new TypedFndecl(id.name(), params, returnType, export, typedStatements));
  }
}
;

record ExternFndecl(String id, String env, Optional<Params> params, Optional<Type> returnType) implements Statement {
  public String toDebugText() {
    return String.format(
        "import fn %s(%s) -> %s",
        id,
        params.map(Params::toDebugText).orElse(""),
        returnType.toString());
  }
  @Override
  public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
    var func = builder.getFunction(id);
    if(func.isPresent()) {
      return new Err<>(String.format("Cannot import function %s, name is already taken", id));
    }
    builder.addExternalFunction(id, env, params, returnType);
    return new Ok<>(new TypedExternFndecl(this)); 
  }
}
record Return(Optional<Expression> expr) implements Statement {
  public String toDebugText() {
    return String.format("return %s", expr.map(AstNode::toDebugText).orElse(""));
  }

  @Override
  public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
    Optional<Function> currentFunction = builder.getCurrentFunction();
    if(currentFunction.isEmpty()) {
      return new Err<>("Return used outside of function");
    }

    if(this.expr.isPresent()) {
      var typedExpressionResult = this.expr.get().getTypedAstNode(builder);
      if(!typedExpressionResult.isOk()) {
        return typedExpressionResult;
      }
      var typedExpression = (TypedExpression)typedExpressionResult.unwrap();

      var returnType = currentFunction.get().getReturnType();
      if(returnType.isPresent()) {
        if(!returnType.get().equals(typedExpression.getType())) {
          return new Err<>(String.format("Function expects return type of %s, got %s", returnType.get(), typedExpression.getType())); 
        }
      } 
      return new Ok<>(new TypedReturn(Optional.of(typedExpression)));
    }

    else if(currentFunction.get().getReturnType().isPresent()) {
      return new Err<>("Functions expects at least one return type");
    }

    return new Ok<>(new TypedReturn(Optional.empty()));
  }
}



record While(Expression expr, Block block) implements Statement {
  public String toDebugText() {
    return String.format("while(%s) %s", expr.toDebugText(), block.toDebugText());
  }
  
  public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
    var typedCond = this.expr.getTypedAstNode(builder);
    if(!typedCond.isOk()) {
      return new Err<>(typedCond.getErr());
    }
    var typedBlock = this.block.getTypedAstNode(builder);
    if(!typedBlock.isOk()) {
      return typedBlock;
    }

    var condExpr = (TypedExpression)typedCond.unwrap();
    if(!condExpr.getType().equals(Type.Bool)) {
      return new Err<>(String.format("Expected bool type in while, got %s", condExpr.getType().toDebugText()));
    }
    return new Ok<>(new TypedWhile(condExpr, (TypedBlock)typedBlock.unwrap()));
  }
}

record Cond(Expression cond, Block ifBlock, Optional<Block> elseBlock) implements Statement {
  public String toDebugText() {
    return String.format(
        "if(%s) %s %s",
        cond.toDebugText(), ifBlock.toDebugText(), elseBlock.map(b -> "else %s" + b).orElse(""));
  }


@Override
public Result<TypedAstNode, String> getTypedAstNode(TypedAstBuilder builder) {
  var typedCond = this.cond.getTypedAstNode(builder);
    if(!typedCond.isOk()) {
      return new Err<>(typedCond.getErr());
    }
    var condExpr = (TypedExpression)typedCond.unwrap();
    if(!condExpr.getType().equals(Type.Bool)) {
      return new Err<>(String.format("Expected bool type in if, got %s", condExpr.getType().toDebugText()));
    }

    var typedBlock = this.ifBlock.getTypedAstNode(builder);
    if(!typedBlock.isOk()) {
      return typedBlock;
    }
        
    Optional<TypedBlock> typedElseBlock = Optional.empty();
    if(elseBlock.isPresent()) {
      var typedElseResult = this.elseBlock.get().getTypedAstNode(builder);
      if(!typedElseResult.isOk()) {
        return typedElseResult;
      }
      typedElseBlock = Optional.of((TypedBlock)typedElseResult.unwrap());
    }
    var ifBlock = (TypedBlock)typedBlock.unwrap();
    return new Ok<>(new TypedCond(condExpr, ifBlock, typedElseBlock));
  }
}




