// Generated from src/main/antlr/Reflang.g4 by ANTLR 4.13.2

import org.antlr.v4.runtime.ParserRuleContext;
import org.antlr.v4.runtime.tree.ErrorNode;
import org.antlr.v4.runtime.tree.TerminalNode;

/**
 * This class provides an empty implementation of {@link ReflangListener}, which can be extended to
 * create a listener which only needs to handle a subset of the available methods.
 */
@SuppressWarnings("CheckReturnValue")
public class ReflangBaseListener implements ReflangListener {
<<<<<<< HEAD
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterProgram(ReflangParser.ProgramContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitProgram(ReflangParser.ProgramContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterStatement(ReflangParser.StatementContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitStatement(ReflangParser.StatementContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterStmtExpr(ReflangParser.StmtExprContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitStmtExpr(ReflangParser.StmtExprContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterVardecl(ReflangParser.VardeclContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitVardecl(ReflangParser.VardeclContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterAssign(ReflangParser.AssignContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitAssign(ReflangParser.AssignContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterVarname(ReflangParser.VarnameContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitVarname(ReflangParser.VarnameContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterFndecl(ReflangParser.FndeclContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitFndecl(ReflangParser.FndeclContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterParam(ReflangParser.ParamContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitParam(ReflangParser.ParamContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterParams(ReflangParser.ParamsContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitParams(ReflangParser.ParamsContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterReturn(ReflangParser.ReturnContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitReturn(ReflangParser.ReturnContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterFncall(ReflangParser.FncallContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitFncall(ReflangParser.FncallContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterArgs(ReflangParser.ArgsContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitArgs(ReflangParser.ArgsContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterBlock(ReflangParser.BlockContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitBlock(ReflangParser.BlockContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterWhile(ReflangParser.WhileContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitWhile(ReflangParser.WhileContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterCond(ReflangParser.CondContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitCond(ReflangParser.CondContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterTInt(ReflangParser.TIntContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitTInt(ReflangParser.TIntContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterTString(ReflangParser.TStringContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitTString(ReflangParser.TStringContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterTBool(ReflangParser.TBoolContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitTBool(ReflangParser.TBoolContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterAdd(ReflangParser.AddContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitAdd(ReflangParser.AddContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterSub(ReflangParser.SubContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitSub(ReflangParser.SubContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterLiteralBool(ReflangParser.LiteralBoolContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitLiteralBool(ReflangParser.LiteralBoolContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterLt(ReflangParser.LtContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitLt(ReflangParser.LtContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterLiteralStr(ReflangParser.LiteralStrContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitLiteralStr(ReflangParser.LiteralStrContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterEq(ReflangParser.EqContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitEq(ReflangParser.EqContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterGt(ReflangParser.GtContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitGt(ReflangParser.GtContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterDiv(ReflangParser.DivContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitDiv(ReflangParser.DivContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterMult(ReflangParser.MultContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitMult(ReflangParser.MultContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterLe(ReflangParser.LeContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitLe(ReflangParser.LeContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterFnc(ReflangParser.FncContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitFnc(ReflangParser.FncContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterLiteralNmb(ReflangParser.LiteralNmbContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitLiteralNmb(ReflangParser.LiteralNmbContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterId(ReflangParser.IdContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitId(ReflangParser.IdContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterNeq(ReflangParser.NeqContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitNeq(ReflangParser.NeqContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterGe(ReflangParser.GeContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitGe(ReflangParser.GeContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterParen(ReflangParser.ParenContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitParen(ReflangParser.ParenContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterLiteralTrue(ReflangParser.LiteralTrueContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitLiteralTrue(ReflangParser.LiteralTrueContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void enterLiteralFalse(ReflangParser.LiteralFalseContext ctx) { }
	/**
	 * {@inheritDoc}
	 *
	 * <p>The default implementation does nothing.</p>
	 */
	@Override public void exitLiteralFalse(ReflangParser.LiteralFalseContext ctx) { }
=======
  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterProgram(ReflangParser.ProgramContext ctx) {}
>>>>>>> c0560b1153c7bec5fb02b136ee3a127fbb144945

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitProgram(ReflangParser.ProgramContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterStatement(ReflangParser.StatementContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitStatement(ReflangParser.StatementContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterVardecl(ReflangParser.VardeclContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitVardecl(ReflangParser.VardeclContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterAssign(ReflangParser.AssignContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitAssign(ReflangParser.AssignContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterFndecl(ReflangParser.FndeclContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitFndecl(ReflangParser.FndeclContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterParam(ReflangParser.ParamContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitParam(ReflangParser.ParamContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterParams(ReflangParser.ParamsContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitParams(ReflangParser.ParamsContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterReturn(ReflangParser.ReturnContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitReturn(ReflangParser.ReturnContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterFncall(ReflangParser.FncallContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitFncall(ReflangParser.FncallContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterArgs(ReflangParser.ArgsContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitArgs(ReflangParser.ArgsContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterBlock(ReflangParser.BlockContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitBlock(ReflangParser.BlockContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterWhile(ReflangParser.WhileContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitWhile(ReflangParser.WhileContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterCond(ReflangParser.CondContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitCond(ReflangParser.CondContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterInt(ReflangParser.IntContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitInt(ReflangParser.IntContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterString(ReflangParser.StringContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitString(ReflangParser.StringContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterBool(ReflangParser.BoolContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitBool(ReflangParser.BoolContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterAdd(ReflangParser.AddContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitAdd(ReflangParser.AddContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterSub(ReflangParser.SubContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitSub(ReflangParser.SubContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterNmb(ReflangParser.NmbContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitNmb(ReflangParser.NmbContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterLt(ReflangParser.LtContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitLt(ReflangParser.LtContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterBlit(ReflangParser.BlitContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitBlit(ReflangParser.BlitContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterEq(ReflangParser.EqContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitEq(ReflangParser.EqContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterGt(ReflangParser.GtContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitGt(ReflangParser.GtContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterStr(ReflangParser.StrContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitStr(ReflangParser.StrContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterDiv(ReflangParser.DivContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitDiv(ReflangParser.DivContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterMult(ReflangParser.MultContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitMult(ReflangParser.MultContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterLe(ReflangParser.LeContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitLe(ReflangParser.LeContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterFnc(ReflangParser.FncContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitFnc(ReflangParser.FncContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterId(ReflangParser.IdContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitId(ReflangParser.IdContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterNeq(ReflangParser.NeqContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitNeq(ReflangParser.NeqContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterGe(ReflangParser.GeContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitGe(ReflangParser.GeContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterParen(ReflangParser.ParenContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitParen(ReflangParser.ParenContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterTrue(ReflangParser.TrueContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitTrue(ReflangParser.TrueContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterFalse(ReflangParser.FalseContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitFalse(ReflangParser.FalseContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void enterEveryRule(ParserRuleContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void exitEveryRule(ParserRuleContext ctx) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void visitTerminal(TerminalNode node) {}

  /**
   * {@inheritDoc}
   *
   * <p>The default implementation does nothing.
   */
  @Override
  public void visitErrorNode(ErrorNode node) {}
}
