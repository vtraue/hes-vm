// Generated from src/main/antlr/Reflang.g4 by ANTLR 4.13.2
import org.antlr.v4.runtime.tree.ParseTreeListener;

/**
 * This interface defines a complete listener for a parse tree produced by {@link ReflangParser}.
 */
public interface ReflangListener extends ParseTreeListener {
<<<<<<< HEAD
	/**
	 * Enter a parse tree produced by {@link ReflangParser#program}.
	 * @param ctx the parse tree
	 */
	void enterProgram(ReflangParser.ProgramContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#program}.
	 * @param ctx the parse tree
	 */
	void exitProgram(ReflangParser.ProgramContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#statement}.
	 * @param ctx the parse tree
	 */
	void enterStatement(ReflangParser.StatementContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#statement}.
	 * @param ctx the parse tree
	 */
	void exitStatement(ReflangParser.StatementContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#stmtExpr}.
	 * @param ctx the parse tree
	 */
	void enterStmtExpr(ReflangParser.StmtExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#stmtExpr}.
	 * @param ctx the parse tree
	 */
	void exitStmtExpr(ReflangParser.StmtExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#vardecl}.
	 * @param ctx the parse tree
	 */
	void enterVardecl(ReflangParser.VardeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#vardecl}.
	 * @param ctx the parse tree
	 */
	void exitVardecl(ReflangParser.VardeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#assign}.
	 * @param ctx the parse tree
	 */
	void enterAssign(ReflangParser.AssignContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#assign}.
	 * @param ctx the parse tree
	 */
	void exitAssign(ReflangParser.AssignContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#varname}.
	 * @param ctx the parse tree
	 */
	void enterVarname(ReflangParser.VarnameContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#varname}.
	 * @param ctx the parse tree
	 */
	void exitVarname(ReflangParser.VarnameContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#fndecl}.
	 * @param ctx the parse tree
	 */
	void enterFndecl(ReflangParser.FndeclContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#fndecl}.
	 * @param ctx the parse tree
	 */
	void exitFndecl(ReflangParser.FndeclContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#param}.
	 * @param ctx the parse tree
	 */
	void enterParam(ReflangParser.ParamContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#param}.
	 * @param ctx the parse tree
	 */
	void exitParam(ReflangParser.ParamContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#params}.
	 * @param ctx the parse tree
	 */
	void enterParams(ReflangParser.ParamsContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#params}.
	 * @param ctx the parse tree
	 */
	void exitParams(ReflangParser.ParamsContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#return}.
	 * @param ctx the parse tree
	 */
	void enterReturn(ReflangParser.ReturnContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#return}.
	 * @param ctx the parse tree
	 */
	void exitReturn(ReflangParser.ReturnContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#fncall}.
	 * @param ctx the parse tree
	 */
	void enterFncall(ReflangParser.FncallContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#fncall}.
	 * @param ctx the parse tree
	 */
	void exitFncall(ReflangParser.FncallContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#args}.
	 * @param ctx the parse tree
	 */
	void enterArgs(ReflangParser.ArgsContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#args}.
	 * @param ctx the parse tree
	 */
	void exitArgs(ReflangParser.ArgsContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#block}.
	 * @param ctx the parse tree
	 */
	void enterBlock(ReflangParser.BlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#block}.
	 * @param ctx the parse tree
	 */
	void exitBlock(ReflangParser.BlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#while}.
	 * @param ctx the parse tree
	 */
	void enterWhile(ReflangParser.WhileContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#while}.
	 * @param ctx the parse tree
	 */
	void exitWhile(ReflangParser.WhileContext ctx);
	/**
	 * Enter a parse tree produced by {@link ReflangParser#cond}.
	 * @param ctx the parse tree
	 */
	void enterCond(ReflangParser.CondContext ctx);
	/**
	 * Exit a parse tree produced by {@link ReflangParser#cond}.
	 * @param ctx the parse tree
	 */
	void exitCond(ReflangParser.CondContext ctx);
	/**
	 * Enter a parse tree produced by the {@code TInt}
	 * labeled alternative in {@link ReflangParser#type}.
	 * @param ctx the parse tree
	 */
	void enterTInt(ReflangParser.TIntContext ctx);
	/**
	 * Exit a parse tree produced by the {@code TInt}
	 * labeled alternative in {@link ReflangParser#type}.
	 * @param ctx the parse tree
	 */
	void exitTInt(ReflangParser.TIntContext ctx);
	/**
	 * Enter a parse tree produced by the {@code TString}
	 * labeled alternative in {@link ReflangParser#type}.
	 * @param ctx the parse tree
	 */
	void enterTString(ReflangParser.TStringContext ctx);
	/**
	 * Exit a parse tree produced by the {@code TString}
	 * labeled alternative in {@link ReflangParser#type}.
	 * @param ctx the parse tree
	 */
	void exitTString(ReflangParser.TStringContext ctx);
	/**
	 * Enter a parse tree produced by the {@code TBool}
	 * labeled alternative in {@link ReflangParser#type}.
	 * @param ctx the parse tree
	 */
	void enterTBool(ReflangParser.TBoolContext ctx);
	/**
	 * Exit a parse tree produced by the {@code TBool}
	 * labeled alternative in {@link ReflangParser#type}.
	 * @param ctx the parse tree
	 */
	void exitTBool(ReflangParser.TBoolContext ctx);
	/**
	 * Enter a parse tree produced by the {@code Add}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterAdd(ReflangParser.AddContext ctx);
	/**
	 * Exit a parse tree produced by the {@code Add}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitAdd(ReflangParser.AddContext ctx);
	/**
	 * Enter a parse tree produced by the {@code Sub}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterSub(ReflangParser.SubContext ctx);
	/**
	 * Exit a parse tree produced by the {@code Sub}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitSub(ReflangParser.SubContext ctx);
	/**
	 * Enter a parse tree produced by the {@code LiteralBool}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterLiteralBool(ReflangParser.LiteralBoolContext ctx);
	/**
	 * Exit a parse tree produced by the {@code LiteralBool}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitLiteralBool(ReflangParser.LiteralBoolContext ctx);
	/**
	 * Enter a parse tree produced by the {@code Lt}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterLt(ReflangParser.LtContext ctx);
	/**
	 * Exit a parse tree produced by the {@code Lt}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitLt(ReflangParser.LtContext ctx);
	/**
	 * Enter a parse tree produced by the {@code LiteralStr}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterLiteralStr(ReflangParser.LiteralStrContext ctx);
	/**
	 * Exit a parse tree produced by the {@code LiteralStr}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitLiteralStr(ReflangParser.LiteralStrContext ctx);
	/**
	 * Enter a parse tree produced by the {@code Eq}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterEq(ReflangParser.EqContext ctx);
	/**
	 * Exit a parse tree produced by the {@code Eq}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitEq(ReflangParser.EqContext ctx);
	/**
	 * Enter a parse tree produced by the {@code Gt}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterGt(ReflangParser.GtContext ctx);
	/**
	 * Exit a parse tree produced by the {@code Gt}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitGt(ReflangParser.GtContext ctx);
	/**
	 * Enter a parse tree produced by the {@code Div}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterDiv(ReflangParser.DivContext ctx);
	/**
	 * Exit a parse tree produced by the {@code Div}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitDiv(ReflangParser.DivContext ctx);
	/**
	 * Enter a parse tree produced by the {@code Mult}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterMult(ReflangParser.MultContext ctx);
	/**
	 * Exit a parse tree produced by the {@code Mult}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitMult(ReflangParser.MultContext ctx);
	/**
	 * Enter a parse tree produced by the {@code Le}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterLe(ReflangParser.LeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code Le}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitLe(ReflangParser.LeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code fnc}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterFnc(ReflangParser.FncContext ctx);
	/**
	 * Exit a parse tree produced by the {@code fnc}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitFnc(ReflangParser.FncContext ctx);
	/**
	 * Enter a parse tree produced by the {@code LiteralNmb}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterLiteralNmb(ReflangParser.LiteralNmbContext ctx);
	/**
	 * Exit a parse tree produced by the {@code LiteralNmb}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitLiteralNmb(ReflangParser.LiteralNmbContext ctx);
	/**
	 * Enter a parse tree produced by the {@code Id}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterId(ReflangParser.IdContext ctx);
	/**
	 * Exit a parse tree produced by the {@code Id}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitId(ReflangParser.IdContext ctx);
	/**
	 * Enter a parse tree produced by the {@code Neq}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterNeq(ReflangParser.NeqContext ctx);
	/**
	 * Exit a parse tree produced by the {@code Neq}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitNeq(ReflangParser.NeqContext ctx);
	/**
	 * Enter a parse tree produced by the {@code Ge}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterGe(ReflangParser.GeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code Ge}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitGe(ReflangParser.GeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code Paren}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterParen(ReflangParser.ParenContext ctx);
	/**
	 * Exit a parse tree produced by the {@code Paren}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitParen(ReflangParser.ParenContext ctx);
	/**
	 * Enter a parse tree produced by the {@code LiteralTrue}
	 * labeled alternative in {@link ReflangParser#bool_literal}.
	 * @param ctx the parse tree
	 */
	void enterLiteralTrue(ReflangParser.LiteralTrueContext ctx);
	/**
	 * Exit a parse tree produced by the {@code LiteralTrue}
	 * labeled alternative in {@link ReflangParser#bool_literal}.
	 * @param ctx the parse tree
	 */
	void exitLiteralTrue(ReflangParser.LiteralTrueContext ctx);
	/**
	 * Enter a parse tree produced by the {@code LiteralFalse}
	 * labeled alternative in {@link ReflangParser#bool_literal}.
	 * @param ctx the parse tree
	 */
	void enterLiteralFalse(ReflangParser.LiteralFalseContext ctx);
	/**
	 * Exit a parse tree produced by the {@code LiteralFalse}
	 * labeled alternative in {@link ReflangParser#bool_literal}.
	 * @param ctx the parse tree
	 */
	void exitLiteralFalse(ReflangParser.LiteralFalseContext ctx);
}
=======
  /**
   * Enter a parse tree produced by {@link ReflangParser#program}.
   *
   * @param ctx the parse tree
   */
  void enterProgram(ReflangParser.ProgramContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#program}.
   *
   * @param ctx the parse tree
   */
  void exitProgram(ReflangParser.ProgramContext ctx);

  /**
   * Enter a parse tree produced by {@link ReflangParser#statement}.
   *
   * @param ctx the parse tree
   */
  void enterStatement(ReflangParser.StatementContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#statement}.
   *
   * @param ctx the parse tree
   */
  void exitStatement(ReflangParser.StatementContext ctx);

  /**
   * Enter a parse tree produced by {@link ReflangParser#vardecl}.
   *
   * @param ctx the parse tree
   */
  void enterVardecl(ReflangParser.VardeclContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#vardecl}.
   *
   * @param ctx the parse tree
   */
  void exitVardecl(ReflangParser.VardeclContext ctx);

  /**
   * Enter a parse tree produced by {@link ReflangParser#assign}.
   *
   * @param ctx the parse tree
   */
  void enterAssign(ReflangParser.AssignContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#assign}.
   *
   * @param ctx the parse tree
   */
  void exitAssign(ReflangParser.AssignContext ctx);

  /**
   * Enter a parse tree produced by {@link ReflangParser#fndecl}.
   *
   * @param ctx the parse tree
   */
  void enterFndecl(ReflangParser.FndeclContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#fndecl}.
   *
   * @param ctx the parse tree
   */
  void exitFndecl(ReflangParser.FndeclContext ctx);

  /**
   * Enter a parse tree produced by {@link ReflangParser#param}.
   *
   * @param ctx the parse tree
   */
  void enterParam(ReflangParser.ParamContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#param}.
   *
   * @param ctx the parse tree
   */
  void exitParam(ReflangParser.ParamContext ctx);

  /**
   * Enter a parse tree produced by {@link ReflangParser#params}.
   *
   * @param ctx the parse tree
   */
  void enterParams(ReflangParser.ParamsContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#params}.
   *
   * @param ctx the parse tree
   */
  void exitParams(ReflangParser.ParamsContext ctx);

  /**
   * Enter a parse tree produced by {@link ReflangParser#return}.
   *
   * @param ctx the parse tree
   */
  void enterReturn(ReflangParser.ReturnContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#return}.
   *
   * @param ctx the parse tree
   */
  void exitReturn(ReflangParser.ReturnContext ctx);

  /**
   * Enter a parse tree produced by {@link ReflangParser#fncall}.
   *
   * @param ctx the parse tree
   */
  void enterFncall(ReflangParser.FncallContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#fncall}.
   *
   * @param ctx the parse tree
   */
  void exitFncall(ReflangParser.FncallContext ctx);

  /**
   * Enter a parse tree produced by {@link ReflangParser#args}.
   *
   * @param ctx the parse tree
   */
  void enterArgs(ReflangParser.ArgsContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#args}.
   *
   * @param ctx the parse tree
   */
  void exitArgs(ReflangParser.ArgsContext ctx);

  /**
   * Enter a parse tree produced by {@link ReflangParser#block}.
   *
   * @param ctx the parse tree
   */
  void enterBlock(ReflangParser.BlockContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#block}.
   *
   * @param ctx the parse tree
   */
  void exitBlock(ReflangParser.BlockContext ctx);

  /**
   * Enter a parse tree produced by {@link ReflangParser#while}.
   *
   * @param ctx the parse tree
   */
  void enterWhile(ReflangParser.WhileContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#while}.
   *
   * @param ctx the parse tree
   */
  void exitWhile(ReflangParser.WhileContext ctx);

  /**
   * Enter a parse tree produced by {@link ReflangParser#cond}.
   *
   * @param ctx the parse tree
   */
  void enterCond(ReflangParser.CondContext ctx);

  /**
   * Exit a parse tree produced by {@link ReflangParser#cond}.
   *
   * @param ctx the parse tree
   */
  void exitCond(ReflangParser.CondContext ctx);

  /**
   * Enter a parse tree produced by the {@code Int} labeled alternative in {@link
   * ReflangParser#type}.
   *
   * @param ctx the parse tree
   */
  void enterInt(ReflangParser.IntContext ctx);

  /**
   * Exit a parse tree produced by the {@code Int} labeled alternative in {@link
   * ReflangParser#type}.
   *
   * @param ctx the parse tree
   */
  void exitInt(ReflangParser.IntContext ctx);

  /**
   * Enter a parse tree produced by the {@code String} labeled alternative in {@link
   * ReflangParser#type}.
   *
   * @param ctx the parse tree
   */
  void enterString(ReflangParser.StringContext ctx);

  /**
   * Exit a parse tree produced by the {@code String} labeled alternative in {@link
   * ReflangParser#type}.
   *
   * @param ctx the parse tree
   */
  void exitString(ReflangParser.StringContext ctx);

  /**
   * Enter a parse tree produced by the {@code Bool} labeled alternative in {@link
   * ReflangParser#type}.
   *
   * @param ctx the parse tree
   */
  void enterBool(ReflangParser.BoolContext ctx);

  /**
   * Exit a parse tree produced by the {@code Bool} labeled alternative in {@link
   * ReflangParser#type}.
   *
   * @param ctx the parse tree
   */
  void exitBool(ReflangParser.BoolContext ctx);

  /**
   * Enter a parse tree produced by the {@code Add} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterAdd(ReflangParser.AddContext ctx);

  /**
   * Exit a parse tree produced by the {@code Add} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitAdd(ReflangParser.AddContext ctx);

  /**
   * Enter a parse tree produced by the {@code Sub} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterSub(ReflangParser.SubContext ctx);

  /**
   * Exit a parse tree produced by the {@code Sub} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitSub(ReflangParser.SubContext ctx);

  /**
   * Enter a parse tree produced by the {@code Nmb} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterNmb(ReflangParser.NmbContext ctx);

  /**
   * Exit a parse tree produced by the {@code Nmb} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitNmb(ReflangParser.NmbContext ctx);

  /**
   * Enter a parse tree produced by the {@code Lt} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterLt(ReflangParser.LtContext ctx);

  /**
   * Exit a parse tree produced by the {@code Lt} labeled alternative in {@link ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitLt(ReflangParser.LtContext ctx);

  /**
   * Enter a parse tree produced by the {@code Blit} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterBlit(ReflangParser.BlitContext ctx);

  /**
   * Exit a parse tree produced by the {@code Blit} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitBlit(ReflangParser.BlitContext ctx);

  /**
   * Enter a parse tree produced by the {@code Eq} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterEq(ReflangParser.EqContext ctx);

  /**
   * Exit a parse tree produced by the {@code Eq} labeled alternative in {@link ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitEq(ReflangParser.EqContext ctx);

  /**
   * Enter a parse tree produced by the {@code Gt} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterGt(ReflangParser.GtContext ctx);

  /**
   * Exit a parse tree produced by the {@code Gt} labeled alternative in {@link ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitGt(ReflangParser.GtContext ctx);

  /**
   * Enter a parse tree produced by the {@code Str} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterStr(ReflangParser.StrContext ctx);

  /**
   * Exit a parse tree produced by the {@code Str} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitStr(ReflangParser.StrContext ctx);

  /**
   * Enter a parse tree produced by the {@code Div} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterDiv(ReflangParser.DivContext ctx);

  /**
   * Exit a parse tree produced by the {@code Div} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitDiv(ReflangParser.DivContext ctx);

  /**
   * Enter a parse tree produced by the {@code Mult} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterMult(ReflangParser.MultContext ctx);

  /**
   * Exit a parse tree produced by the {@code Mult} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitMult(ReflangParser.MultContext ctx);

  /**
   * Enter a parse tree produced by the {@code Le} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterLe(ReflangParser.LeContext ctx);

  /**
   * Exit a parse tree produced by the {@code Le} labeled alternative in {@link ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitLe(ReflangParser.LeContext ctx);

  /**
   * Enter a parse tree produced by the {@code fnc} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterFnc(ReflangParser.FncContext ctx);

  /**
   * Exit a parse tree produced by the {@code fnc} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitFnc(ReflangParser.FncContext ctx);

  /**
   * Enter a parse tree produced by the {@code Id} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterId(ReflangParser.IdContext ctx);

  /**
   * Exit a parse tree produced by the {@code Id} labeled alternative in {@link ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitId(ReflangParser.IdContext ctx);

  /**
   * Enter a parse tree produced by the {@code Neq} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterNeq(ReflangParser.NeqContext ctx);

  /**
   * Exit a parse tree produced by the {@code Neq} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitNeq(ReflangParser.NeqContext ctx);

  /**
   * Enter a parse tree produced by the {@code Ge} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterGe(ReflangParser.GeContext ctx);

  /**
   * Exit a parse tree produced by the {@code Ge} labeled alternative in {@link ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitGe(ReflangParser.GeContext ctx);

  /**
   * Enter a parse tree produced by the {@code Paren} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterParen(ReflangParser.ParenContext ctx);

  /**
   * Exit a parse tree produced by the {@code Paren} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitParen(ReflangParser.ParenContext ctx);

  /**
   * Enter a parse tree produced by the {@code True} labeled alternative in {@link
   * ReflangParser#bool_literal}.
   *
   * @param ctx the parse tree
   */
  void enterTrue(ReflangParser.TrueContext ctx);

  /**
   * Exit a parse tree produced by the {@code True} labeled alternative in {@link
   * ReflangParser#bool_literal}.
   *
   * @param ctx the parse tree
   */
  void exitTrue(ReflangParser.TrueContext ctx);

  /**
   * Enter a parse tree produced by the {@code False} labeled alternative in {@link
   * ReflangParser#bool_literal}.
   *
   * @param ctx the parse tree
   */
  void enterFalse(ReflangParser.FalseContext ctx);

  /**
   * Exit a parse tree produced by the {@code False} labeled alternative in {@link
   * ReflangParser#bool_literal}.
   *
   * @param ctx the parse tree
   */
  void exitFalse(ReflangParser.FalseContext ctx);
}
>>>>>>> c0560b1153c7bec5fb02b136ee3a127fbb144945
