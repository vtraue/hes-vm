// Generated from src/main/antlr/Reflang.g4 by ANTLR 4.13.2
import org.antlr.v4.runtime.tree.ParseTreeVisitor;

/**
 * This interface defines a complete generic visitor for a parse tree produced by {@link
 * ReflangParser}.
 *
 * @param <T> The return type of the visit operation. Use {@link Void} for operations with no return
 *     type.
 */
public interface ReflangVisitor<T> extends ParseTreeVisitor<T> {
  /**
   * Visit a parse tree produced by {@link ReflangParser#program}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitProgram(ReflangParser.ProgramContext ctx);

  /**
   * Visit a parse tree produced by {@link ReflangParser#statement}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitStatement(ReflangParser.StatementContext ctx);

  /**
   * Visit a parse tree produced by {@link ReflangParser#vardecl}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitVardecl(ReflangParser.VardeclContext ctx);

  /**
   * Visit a parse tree produced by {@link ReflangParser#assign}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitAssign(ReflangParser.AssignContext ctx);

  /**
   * Visit a parse tree produced by {@link ReflangParser#fndecl}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitFndecl(ReflangParser.FndeclContext ctx);

  /**
   * Visit a parse tree produced by {@link ReflangParser#param}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitParam(ReflangParser.ParamContext ctx);

  /**
   * Visit a parse tree produced by {@link ReflangParser#params}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitParams(ReflangParser.ParamsContext ctx);

  /**
   * Visit a parse tree produced by {@link ReflangParser#return}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitReturn(ReflangParser.ReturnContext ctx);

  /**
   * Visit a parse tree produced by {@link ReflangParser#fncall}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitFncall(ReflangParser.FncallContext ctx);

  /**
   * Visit a parse tree produced by {@link ReflangParser#args}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitArgs(ReflangParser.ArgsContext ctx);

  /**
   * Visit a parse tree produced by {@link ReflangParser#block}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitBlock(ReflangParser.BlockContext ctx);

  /**
   * Visit a parse tree produced by {@link ReflangParser#while}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitWhile(ReflangParser.WhileContext ctx);

  /**
   * Visit a parse tree produced by {@link ReflangParser#cond}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitCond(ReflangParser.CondContext ctx);

  /**
   * Visit a parse tree produced by the {@code Int} labeled alternative in {@link
   * ReflangParser#type}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitInt(ReflangParser.IntContext ctx);

  /**
   * Visit a parse tree produced by the {@code String} labeled alternative in {@link
   * ReflangParser#type}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitString(ReflangParser.StringContext ctx);

  /**
   * Visit a parse tree produced by the {@code Bool} labeled alternative in {@link
   * ReflangParser#type}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitBool(ReflangParser.BoolContext ctx);

  /**
   * Visit a parse tree produced by the {@code Add} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitAdd(ReflangParser.AddContext ctx);

  /**
   * Visit a parse tree produced by the {@code Sub} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitSub(ReflangParser.SubContext ctx);

  /**
   * Visit a parse tree produced by the {@code Nmb} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitNmb(ReflangParser.NmbContext ctx);

  /**
   * Visit a parse tree produced by the {@code Lt} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitLt(ReflangParser.LtContext ctx);

  /**
   * Visit a parse tree produced by the {@code Blit} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitBlit(ReflangParser.BlitContext ctx);

  /**
   * Visit a parse tree produced by the {@code Eq} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitEq(ReflangParser.EqContext ctx);

  /**
   * Visit a parse tree produced by the {@code Gt} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitGt(ReflangParser.GtContext ctx);

  /**
   * Visit a parse tree produced by the {@code Str} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitStr(ReflangParser.StrContext ctx);

  /**
   * Visit a parse tree produced by the {@code Div} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitDiv(ReflangParser.DivContext ctx);

  /**
   * Visit a parse tree produced by the {@code Mult} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitMult(ReflangParser.MultContext ctx);

  /**
   * Visit a parse tree produced by the {@code Le} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitLe(ReflangParser.LeContext ctx);

  /**
   * Visit a parse tree produced by the {@code fnc} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitFnc(ReflangParser.FncContext ctx);

  /**
   * Visit a parse tree produced by the {@code Id} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitId(ReflangParser.IdContext ctx);

  /**
   * Visit a parse tree produced by the {@code Neq} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitNeq(ReflangParser.NeqContext ctx);

  /**
   * Visit a parse tree produced by the {@code Ge} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitGe(ReflangParser.GeContext ctx);

  /**
   * Visit a parse tree produced by the {@code Paren} labeled alternative in {@link
   * ReflangParser#expr}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitParen(ReflangParser.ParenContext ctx);

  /**
   * Visit a parse tree produced by the {@code True} labeled alternative in {@link
   * ReflangParser#bool_literal}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitTrue(ReflangParser.TrueContext ctx);

  /**
   * Visit a parse tree produced by the {@code False} labeled alternative in {@link
   * ReflangParser#bool_literal}.
   *
   * @param ctx the parse tree
   * @return the visitor result
   */
  T visitFalse(ReflangParser.FalseContext ctx);
}
