// Generated from src/main/antlr/Reflang.g4 by ANTLR 4.13.2
import org.antlr.v4.runtime.tree.ParseTreeVisitor;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link ReflangParser}.
 *
 * @param <T> The return type of the visit operation. Use {@link Void} for
 * operations with no return type.
 */
public interface ReflangVisitor<T> extends ParseTreeVisitor<T> {
	/**
	 * Visit a parse tree produced by {@link ReflangParser#program}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitProgram(ReflangParser.ProgramContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#statement}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitStatement(ReflangParser.StatementContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#stmtExpr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitStmtExpr(ReflangParser.StmtExprContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#vardecl}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitVardecl(ReflangParser.VardeclContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#assign}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitAssign(ReflangParser.AssignContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#varname}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitVarname(ReflangParser.VarnameContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#fndecl}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitFndecl(ReflangParser.FndeclContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#param}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitParam(ReflangParser.ParamContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#params}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitParams(ReflangParser.ParamsContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#return}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitReturn(ReflangParser.ReturnContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#fncall}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitFncall(ReflangParser.FncallContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#args}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitArgs(ReflangParser.ArgsContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#block}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitBlock(ReflangParser.BlockContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#while}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitWhile(ReflangParser.WhileContext ctx);
	/**
	 * Visit a parse tree produced by {@link ReflangParser#cond}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitCond(ReflangParser.CondContext ctx);
	/**
	 * Visit a parse tree produced by the {@code TInt}
	 * labeled alternative in {@link ReflangParser#type}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitTInt(ReflangParser.TIntContext ctx);
	/**
	 * Visit a parse tree produced by the {@code TString}
	 * labeled alternative in {@link ReflangParser#type}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitTString(ReflangParser.TStringContext ctx);
	/**
	 * Visit a parse tree produced by the {@code TBool}
	 * labeled alternative in {@link ReflangParser#type}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitTBool(ReflangParser.TBoolContext ctx);
	/**
	 * Visit a parse tree produced by the {@code Add}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitAdd(ReflangParser.AddContext ctx);
	/**
	 * Visit a parse tree produced by the {@code Sub}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitSub(ReflangParser.SubContext ctx);
	/**
	 * Visit a parse tree produced by the {@code LiteralBool}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitLiteralBool(ReflangParser.LiteralBoolContext ctx);
	/**
	 * Visit a parse tree produced by the {@code Lt}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitLt(ReflangParser.LtContext ctx);
	/**
	 * Visit a parse tree produced by the {@code LiteralStr}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitLiteralStr(ReflangParser.LiteralStrContext ctx);
	/**
	 * Visit a parse tree produced by the {@code Eq}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitEq(ReflangParser.EqContext ctx);
	/**
	 * Visit a parse tree produced by the {@code Gt}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitGt(ReflangParser.GtContext ctx);
	/**
	 * Visit a parse tree produced by the {@code Div}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitDiv(ReflangParser.DivContext ctx);
	/**
	 * Visit a parse tree produced by the {@code Mult}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitMult(ReflangParser.MultContext ctx);
	/**
	 * Visit a parse tree produced by the {@code Le}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitLe(ReflangParser.LeContext ctx);
	/**
	 * Visit a parse tree produced by the {@code fnc}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitFnc(ReflangParser.FncContext ctx);
	/**
	 * Visit a parse tree produced by the {@code LiteralNmb}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitLiteralNmb(ReflangParser.LiteralNmbContext ctx);
	/**
	 * Visit a parse tree produced by the {@code Id}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitId(ReflangParser.IdContext ctx);
	/**
	 * Visit a parse tree produced by the {@code Neq}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitNeq(ReflangParser.NeqContext ctx);
	/**
	 * Visit a parse tree produced by the {@code Ge}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitGe(ReflangParser.GeContext ctx);
	/**
	 * Visit a parse tree produced by the {@code Paren}
	 * labeled alternative in {@link ReflangParser#expr}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitParen(ReflangParser.ParenContext ctx);
	/**
	 * Visit a parse tree produced by the {@code LiteralTrue}
	 * labeled alternative in {@link ReflangParser#bool_literal}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitLiteralTrue(ReflangParser.LiteralTrueContext ctx);
	/**
	 * Visit a parse tree produced by the {@code LiteralFalse}
	 * labeled alternative in {@link ReflangParser#bool_literal}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitLiteralFalse(ReflangParser.LiteralFalseContext ctx);
}