// Generated from ../main/antlr/MiniC.g4 by ANTLR 4.13.2
import org.antlr.v4.runtime.tree.ParseTreeListener;

/** This interface defines a complete listener for a parse tree produced by {@link MiniCParser}. */
public interface MiniCListener extends ParseTreeListener {
  /**
   * Enter a parse tree produced by {@link MiniCParser#program}.
   *
   * @param ctx the parse tree
   */
  void enterProgram(MiniCParser.ProgramContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#program}.
   *
   * @param ctx the parse tree
   */
  void exitProgram(MiniCParser.ProgramContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#stmt}.
   *
   * @param ctx the parse tree
   */
  void enterStmt(MiniCParser.StmtContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#stmt}.
   *
   * @param ctx the parse tree
   */
  void exitStmt(MiniCParser.StmtContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#vardecl}.
   *
   * @param ctx the parse tree
   */
  void enterVardecl(MiniCParser.VardeclContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#vardecl}.
   *
   * @param ctx the parse tree
   */
  void exitVardecl(MiniCParser.VardeclContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#assign}.
   *
   * @param ctx the parse tree
   */
  void enterAssign(MiniCParser.AssignContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#assign}.
   *
   * @param ctx the parse tree
   */
  void exitAssign(MiniCParser.AssignContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#fndecl}.
   *
   * @param ctx the parse tree
   */
  void enterFndecl(MiniCParser.FndeclContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#fndecl}.
   *
   * @param ctx the parse tree
   */
  void exitFndecl(MiniCParser.FndeclContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#params}.
   *
   * @param ctx the parse tree
   */
  void enterParams(MiniCParser.ParamsContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#params}.
   *
   * @param ctx the parse tree
   */
  void exitParams(MiniCParser.ParamsContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#return}.
   *
   * @param ctx the parse tree
   */
  void enterReturn(MiniCParser.ReturnContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#return}.
   *
   * @param ctx the parse tree
   */
  void exitReturn(MiniCParser.ReturnContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#fncall}.
   *
   * @param ctx the parse tree
   */
  void enterFncall(MiniCParser.FncallContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#fncall}.
   *
   * @param ctx the parse tree
   */
  void exitFncall(MiniCParser.FncallContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#args}.
   *
   * @param ctx the parse tree
   */
  void enterArgs(MiniCParser.ArgsContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#args}.
   *
   * @param ctx the parse tree
   */
  void exitArgs(MiniCParser.ArgsContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#block}.
   *
   * @param ctx the parse tree
   */
  void enterBlock(MiniCParser.BlockContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#block}.
   *
   * @param ctx the parse tree
   */
  void exitBlock(MiniCParser.BlockContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#while}.
   *
   * @param ctx the parse tree
   */
  void enterWhile(MiniCParser.WhileContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#while}.
   *
   * @param ctx the parse tree
   */
  void exitWhile(MiniCParser.WhileContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#cond}.
   *
   * @param ctx the parse tree
   */
  void enterCond(MiniCParser.CondContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#cond}.
   *
   * @param ctx the parse tree
   */
  void exitCond(MiniCParser.CondContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#expr}.
   *
   * @param ctx the parse tree
   */
  void enterExpr(MiniCParser.ExprContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#expr}.
   *
   * @param ctx the parse tree
   */
  void exitExpr(MiniCParser.ExprContext ctx);

  /**
   * Enter a parse tree produced by {@link MiniCParser#type}.
   *
   * @param ctx the parse tree
   */
  void enterType(MiniCParser.TypeContext ctx);

  /**
   * Exit a parse tree produced by {@link MiniCParser#type}.
   *
   * @param ctx the parse tree
   */
  void exitType(MiniCParser.TypeContext ctx);
}
