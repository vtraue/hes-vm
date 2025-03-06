// Generated from ../main/antlr/MiniC.g4 by ANTLR 4.13.2
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.misc.*;
import org.antlr.v4.runtime.tree.*;
import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast", "CheckReturnValue", "this-escape"})
public class MiniCParser extends Parser {
	static { RuntimeMetaData.checkVersion("4.13.2", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		T__0=1, T__1=2, T__2=3, T__3=4, T__4=5, T__5=6, T__6=7, T__7=8, T__8=9, 
		T__9=10, T__10=11, T__11=12, T__12=13, T__13=14, T__14=15, T__15=16, T__16=17, 
		T__17=18, T__18=19, T__19=20, T__20=21, T__21=22, ID=23, NUMBER=24, STRING=25, 
		COMMENT=26, WS=27;
	public static final int
		RULE_program = 0, RULE_stmt = 1, RULE_vardecl = 2, RULE_assign = 3, RULE_fndecl = 4, 
		RULE_params = 5, RULE_return = 6, RULE_fncall = 7, RULE_args = 8, RULE_block = 9, 
		RULE_while = 10, RULE_cond = 11, RULE_expr = 12, RULE_type = 13;
	private static String[] makeRuleNames() {
		return new String[] {
			"program", "stmt", "vardecl", "assign", "fndecl", "params", "return", 
			"fncall", "args", "block", "while", "cond", "expr", "type"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, "';'", "'='", "'('", "')'", "','", "'return'", "'{'", "'}'", "'while'", 
			"'if'", "'else'", "'*'", "'/'", "'+'", "'-'", "'=='", "'!='", "'>'", 
			"'<'", "'int'", "'string'", "'bool'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, "ID", 
			"NUMBER", "STRING", "COMMENT", "WS"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}

	@Override
	public String getGrammarFileName() { return "MiniC.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public ATN getATN() { return _ATN; }

	public MiniCParser(TokenStream input) {
		super(input);
		_interp = new ParserATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ProgramContext extends ParserRuleContext {
		public TerminalNode EOF() { return getToken(MiniCParser.EOF, 0); }
		public List<StmtContext> stmt() {
			return getRuleContexts(StmtContext.class);
		}
		public StmtContext stmt(int i) {
			return getRuleContext(StmtContext.class,i);
		}
		public ProgramContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_program; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterProgram(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitProgram(this);
		}
	}

	public final ProgramContext program() throws RecognitionException {
		ProgramContext _localctx = new ProgramContext(_ctx, getState());
		enterRule(_localctx, 0, RULE_program);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(31);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while ((((_la) & ~0x3f) == 0 && ((1L << _la) & 66062024L) != 0)) {
				{
				{
				setState(28);
				stmt();
				}
				}
				setState(33);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(34);
			match(EOF);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class StmtContext extends ParserRuleContext {
		public VardeclContext vardecl() {
			return getRuleContext(VardeclContext.class,0);
		}
		public AssignContext assign() {
			return getRuleContext(AssignContext.class,0);
		}
		public FndeclContext fndecl() {
			return getRuleContext(FndeclContext.class,0);
		}
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public BlockContext block() {
			return getRuleContext(BlockContext.class,0);
		}
		public WhileContext while_() {
			return getRuleContext(WhileContext.class,0);
		}
		public CondContext cond() {
			return getRuleContext(CondContext.class,0);
		}
		public ReturnContext return_() {
			return getRuleContext(ReturnContext.class,0);
		}
		public StmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_stmt; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterStmt(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitStmt(this);
		}
	}

	public final StmtContext stmt() throws RecognitionException {
		StmtContext _localctx = new StmtContext(_ctx, getState());
		enterRule(_localctx, 2, RULE_stmt);
		try {
			setState(46);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,1,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(36);
				vardecl();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(37);
				assign();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(38);
				fndecl();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(39);
				expr(0);
				setState(40);
				match(T__0);
				}
				break;
			case 5:
				enterOuterAlt(_localctx, 5);
				{
				setState(42);
				block();
				}
				break;
			case 6:
				enterOuterAlt(_localctx, 6);
				{
				setState(43);
				while_();
				}
				break;
			case 7:
				enterOuterAlt(_localctx, 7);
				{
				setState(44);
				cond();
				}
				break;
			case 8:
				enterOuterAlt(_localctx, 8);
				{
				setState(45);
				return_();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class VardeclContext extends ParserRuleContext {
		public TypeContext type() {
			return getRuleContext(TypeContext.class,0);
		}
		public TerminalNode ID() { return getToken(MiniCParser.ID, 0); }
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public VardeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_vardecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterVardecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitVardecl(this);
		}
	}

	public final VardeclContext vardecl() throws RecognitionException {
		VardeclContext _localctx = new VardeclContext(_ctx, getState());
		enterRule(_localctx, 4, RULE_vardecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(48);
			type();
			setState(49);
			match(ID);
			setState(52);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__1) {
				{
				setState(50);
				match(T__1);
				setState(51);
				expr(0);
				}
			}

			setState(54);
			match(T__0);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AssignContext extends ParserRuleContext {
		public TerminalNode ID() { return getToken(MiniCParser.ID, 0); }
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public AssignContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_assign; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterAssign(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitAssign(this);
		}
	}

	public final AssignContext assign() throws RecognitionException {
		AssignContext _localctx = new AssignContext(_ctx, getState());
		enterRule(_localctx, 6, RULE_assign);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(56);
			match(ID);
			setState(57);
			match(T__1);
			setState(58);
			expr(0);
			setState(59);
			match(T__0);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class FndeclContext extends ParserRuleContext {
		public TypeContext type() {
			return getRuleContext(TypeContext.class,0);
		}
		public TerminalNode ID() { return getToken(MiniCParser.ID, 0); }
		public BlockContext block() {
			return getRuleContext(BlockContext.class,0);
		}
		public ParamsContext params() {
			return getRuleContext(ParamsContext.class,0);
		}
		public FndeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_fndecl; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterFndecl(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitFndecl(this);
		}
	}

	public final FndeclContext fndecl() throws RecognitionException {
		FndeclContext _localctx = new FndeclContext(_ctx, getState());
		enterRule(_localctx, 8, RULE_fndecl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(61);
			type();
			setState(62);
			match(ID);
			setState(63);
			match(T__2);
			setState(65);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if ((((_la) & ~0x3f) == 0 && ((1L << _la) & 7340032L) != 0)) {
				{
				setState(64);
				params();
				}
			}

			setState(67);
			match(T__3);
			setState(68);
			block();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ParamsContext extends ParserRuleContext {
		public List<TypeContext> type() {
			return getRuleContexts(TypeContext.class);
		}
		public TypeContext type(int i) {
			return getRuleContext(TypeContext.class,i);
		}
		public List<TerminalNode> ID() { return getTokens(MiniCParser.ID); }
		public TerminalNode ID(int i) {
			return getToken(MiniCParser.ID, i);
		}
		public ParamsContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_params; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterParams(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitParams(this);
		}
	}

	public final ParamsContext params() throws RecognitionException {
		ParamsContext _localctx = new ParamsContext(_ctx, getState());
		enterRule(_localctx, 10, RULE_params);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(70);
			type();
			setState(71);
			match(ID);
			setState(78);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__4) {
				{
				{
				setState(72);
				match(T__4);
				setState(73);
				type();
				setState(74);
				match(ID);
				}
				}
				setState(80);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ReturnContext extends ParserRuleContext {
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public ReturnContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_return; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterReturn(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitReturn(this);
		}
	}

	public final ReturnContext return_() throws RecognitionException {
		ReturnContext _localctx = new ReturnContext(_ctx, getState());
		enterRule(_localctx, 12, RULE_return);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(81);
			match(T__5);
			setState(82);
			expr(0);
			setState(83);
			match(T__0);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class FncallContext extends ParserRuleContext {
		public TerminalNode ID() { return getToken(MiniCParser.ID, 0); }
		public ArgsContext args() {
			return getRuleContext(ArgsContext.class,0);
		}
		public FncallContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_fncall; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterFncall(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitFncall(this);
		}
	}

	public final FncallContext fncall() throws RecognitionException {
		FncallContext _localctx = new FncallContext(_ctx, getState());
		enterRule(_localctx, 14, RULE_fncall);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(85);
			match(ID);
			setState(86);
			match(T__2);
			setState(88);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if ((((_la) & ~0x3f) == 0 && ((1L << _la) & 58720264L) != 0)) {
				{
				setState(87);
				args();
				}
			}

			setState(90);
			match(T__3);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ArgsContext extends ParserRuleContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public ArgsContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_args; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterArgs(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitArgs(this);
		}
	}

	public final ArgsContext args() throws RecognitionException {
		ArgsContext _localctx = new ArgsContext(_ctx, getState());
		enterRule(_localctx, 16, RULE_args);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(92);
			expr(0);
			setState(97);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__4) {
				{
				{
				setState(93);
				match(T__4);
				setState(94);
				expr(0);
				}
				}
				setState(99);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class BlockContext extends ParserRuleContext {
		public List<StmtContext> stmt() {
			return getRuleContexts(StmtContext.class);
		}
		public StmtContext stmt(int i) {
			return getRuleContext(StmtContext.class,i);
		}
		public BlockContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_block; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterBlock(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitBlock(this);
		}
	}

	public final BlockContext block() throws RecognitionException {
		BlockContext _localctx = new BlockContext(_ctx, getState());
		enterRule(_localctx, 18, RULE_block);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(100);
			match(T__6);
			setState(104);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while ((((_la) & ~0x3f) == 0 && ((1L << _la) & 66062024L) != 0)) {
				{
				{
				setState(101);
				stmt();
				}
				}
				setState(106);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(107);
			match(T__7);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class WhileContext extends ParserRuleContext {
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public BlockContext block() {
			return getRuleContext(BlockContext.class,0);
		}
		public WhileContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_while; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterWhile(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitWhile(this);
		}
	}

	public final WhileContext while_() throws RecognitionException {
		WhileContext _localctx = new WhileContext(_ctx, getState());
		enterRule(_localctx, 20, RULE_while);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(109);
			match(T__8);
			setState(110);
			match(T__2);
			setState(111);
			expr(0);
			setState(112);
			match(T__3);
			setState(113);
			block();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class CondContext extends ParserRuleContext {
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public List<BlockContext> block() {
			return getRuleContexts(BlockContext.class);
		}
		public BlockContext block(int i) {
			return getRuleContext(BlockContext.class,i);
		}
		public CondContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_cond; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterCond(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitCond(this);
		}
	}

	public final CondContext cond() throws RecognitionException {
		CondContext _localctx = new CondContext(_ctx, getState());
		enterRule(_localctx, 22, RULE_cond);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(115);
			match(T__9);
			setState(116);
			match(T__2);
			setState(117);
			expr(0);
			setState(118);
			match(T__3);
			setState(119);
			block();
			setState(122);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__10) {
				{
				setState(120);
				match(T__10);
				setState(121);
				block();
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExprContext extends ParserRuleContext {
		public FncallContext fncall() {
			return getRuleContext(FncallContext.class,0);
		}
		public TerminalNode ID() { return getToken(MiniCParser.ID, 0); }
		public TerminalNode NUMBER() { return getToken(MiniCParser.NUMBER, 0); }
		public TerminalNode STRING() { return getToken(MiniCParser.STRING, 0); }
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public ExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_expr; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterExpr(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitExpr(this);
		}
	}

	public final ExprContext expr() throws RecognitionException {
		return expr(0);
	}

	private ExprContext expr(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		ExprContext _localctx = new ExprContext(_ctx, _parentState);
		ExprContext _prevctx = _localctx;
		int _startState = 24;
		enterRecursionRule(_localctx, 24, RULE_expr, _p);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(133);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,9,_ctx) ) {
			case 1:
				{
				setState(125);
				fncall();
				}
				break;
			case 2:
				{
				setState(126);
				match(ID);
				}
				break;
			case 3:
				{
				setState(127);
				match(NUMBER);
				}
				break;
			case 4:
				{
				setState(128);
				match(STRING);
				}
				break;
			case 5:
				{
				setState(129);
				match(T__2);
				setState(130);
				expr(0);
				setState(131);
				match(T__3);
				}
				break;
			}
			_ctx.stop = _input.LT(-1);
			setState(161);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,11,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(159);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,10,_ctx) ) {
					case 1:
						{
						_localctx = new ExprContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(135);
						if (!(precpred(_ctx, 12))) throw new FailedPredicateException(this, "precpred(_ctx, 12)");
						setState(136);
						match(T__11);
						setState(137);
						expr(13);
						}
						break;
					case 2:
						{
						_localctx = new ExprContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(138);
						if (!(precpred(_ctx, 11))) throw new FailedPredicateException(this, "precpred(_ctx, 11)");
						setState(139);
						match(T__12);
						setState(140);
						expr(12);
						}
						break;
					case 3:
						{
						_localctx = new ExprContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(141);
						if (!(precpred(_ctx, 10))) throw new FailedPredicateException(this, "precpred(_ctx, 10)");
						setState(142);
						match(T__13);
						setState(143);
						expr(11);
						}
						break;
					case 4:
						{
						_localctx = new ExprContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(144);
						if (!(precpred(_ctx, 9))) throw new FailedPredicateException(this, "precpred(_ctx, 9)");
						setState(145);
						match(T__14);
						setState(146);
						expr(10);
						}
						break;
					case 5:
						{
						_localctx = new ExprContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(147);
						if (!(precpred(_ctx, 8))) throw new FailedPredicateException(this, "precpred(_ctx, 8)");
						setState(148);
						match(T__15);
						setState(149);
						expr(9);
						}
						break;
					case 6:
						{
						_localctx = new ExprContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(150);
						if (!(precpred(_ctx, 7))) throw new FailedPredicateException(this, "precpred(_ctx, 7)");
						setState(151);
						match(T__16);
						setState(152);
						expr(8);
						}
						break;
					case 7:
						{
						_localctx = new ExprContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(153);
						if (!(precpred(_ctx, 6))) throw new FailedPredicateException(this, "precpred(_ctx, 6)");
						setState(154);
						match(T__17);
						setState(155);
						expr(7);
						}
						break;
					case 8:
						{
						_localctx = new ExprContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(156);
						if (!(precpred(_ctx, 5))) throw new FailedPredicateException(this, "precpred(_ctx, 5)");
						setState(157);
						match(T__18);
						setState(158);
						expr(6);
						}
						break;
					}
					} 
				}
				setState(163);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,11,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			unrollRecursionContexts(_parentctx);
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TypeContext extends ParserRuleContext {
		public TypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_type; }
		@Override
		public void enterRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).enterType(this);
		}
		@Override
		public void exitRule(ParseTreeListener listener) {
			if ( listener instanceof MiniCListener ) ((MiniCListener)listener).exitType(this);
		}
	}

	public final TypeContext type() throws RecognitionException {
		TypeContext _localctx = new TypeContext(_ctx, getState());
		enterRule(_localctx, 26, RULE_type);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(164);
			_la = _input.LA(1);
			if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & 7340032L) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public boolean sempred(RuleContext _localctx, int ruleIndex, int predIndex) {
		switch (ruleIndex) {
		case 12:
			return expr_sempred((ExprContext)_localctx, predIndex);
		}
		return true;
	}
	private boolean expr_sempred(ExprContext _localctx, int predIndex) {
		switch (predIndex) {
		case 0:
			return precpred(_ctx, 12);
		case 1:
			return precpred(_ctx, 11);
		case 2:
			return precpred(_ctx, 10);
		case 3:
			return precpred(_ctx, 9);
		case 4:
			return precpred(_ctx, 8);
		case 5:
			return precpred(_ctx, 7);
		case 6:
			return precpred(_ctx, 6);
		case 7:
			return precpred(_ctx, 5);
		}
		return true;
	}

	public static final String _serializedATN =
		"\u0004\u0001\u001b\u00a7\u0002\u0000\u0007\u0000\u0002\u0001\u0007\u0001"+
		"\u0002\u0002\u0007\u0002\u0002\u0003\u0007\u0003\u0002\u0004\u0007\u0004"+
		"\u0002\u0005\u0007\u0005\u0002\u0006\u0007\u0006\u0002\u0007\u0007\u0007"+
		"\u0002\b\u0007\b\u0002\t\u0007\t\u0002\n\u0007\n\u0002\u000b\u0007\u000b"+
		"\u0002\f\u0007\f\u0002\r\u0007\r\u0001\u0000\u0005\u0000\u001e\b\u0000"+
		"\n\u0000\f\u0000!\t\u0000\u0001\u0000\u0001\u0000\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0003\u0001/\b\u0001\u0001\u0002\u0001\u0002"+
		"\u0001\u0002\u0001\u0002\u0003\u00025\b\u0002\u0001\u0002\u0001\u0002"+
		"\u0001\u0003\u0001\u0003\u0001\u0003\u0001\u0003\u0001\u0003\u0001\u0004"+
		"\u0001\u0004\u0001\u0004\u0001\u0004\u0003\u0004B\b\u0004\u0001\u0004"+
		"\u0001\u0004\u0001\u0004\u0001\u0005\u0001\u0005\u0001\u0005\u0001\u0005"+
		"\u0001\u0005\u0001\u0005\u0005\u0005M\b\u0005\n\u0005\f\u0005P\t\u0005"+
		"\u0001\u0006\u0001\u0006\u0001\u0006\u0001\u0006\u0001\u0007\u0001\u0007"+
		"\u0001\u0007\u0003\u0007Y\b\u0007\u0001\u0007\u0001\u0007\u0001\b\u0001"+
		"\b\u0001\b\u0005\b`\b\b\n\b\f\bc\t\b\u0001\t\u0001\t\u0005\tg\b\t\n\t"+
		"\f\tj\t\t\u0001\t\u0001\t\u0001\n\u0001\n\u0001\n\u0001\n\u0001\n\u0001"+
		"\n\u0001\u000b\u0001\u000b\u0001\u000b\u0001\u000b\u0001\u000b\u0001\u000b"+
		"\u0001\u000b\u0003\u000b{\b\u000b\u0001\f\u0001\f\u0001\f\u0001\f\u0001"+
		"\f\u0001\f\u0001\f\u0001\f\u0001\f\u0003\f\u0086\b\f\u0001\f\u0001\f\u0001"+
		"\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001"+
		"\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001"+
		"\f\u0001\f\u0001\f\u0001\f\u0005\f\u00a0\b\f\n\f\f\f\u00a3\t\f\u0001\r"+
		"\u0001\r\u0001\r\u0000\u0001\u0018\u000e\u0000\u0002\u0004\u0006\b\n\f"+
		"\u000e\u0010\u0012\u0014\u0016\u0018\u001a\u0000\u0001\u0001\u0000\u0014"+
		"\u0016\u00b3\u0000\u001f\u0001\u0000\u0000\u0000\u0002.\u0001\u0000\u0000"+
		"\u0000\u00040\u0001\u0000\u0000\u0000\u00068\u0001\u0000\u0000\u0000\b"+
		"=\u0001\u0000\u0000\u0000\nF\u0001\u0000\u0000\u0000\fQ\u0001\u0000\u0000"+
		"\u0000\u000eU\u0001\u0000\u0000\u0000\u0010\\\u0001\u0000\u0000\u0000"+
		"\u0012d\u0001\u0000\u0000\u0000\u0014m\u0001\u0000\u0000\u0000\u0016s"+
		"\u0001\u0000\u0000\u0000\u0018\u0085\u0001\u0000\u0000\u0000\u001a\u00a4"+
		"\u0001\u0000\u0000\u0000\u001c\u001e\u0003\u0002\u0001\u0000\u001d\u001c"+
		"\u0001\u0000\u0000\u0000\u001e!\u0001\u0000\u0000\u0000\u001f\u001d\u0001"+
		"\u0000\u0000\u0000\u001f \u0001\u0000\u0000\u0000 \"\u0001\u0000\u0000"+
		"\u0000!\u001f\u0001\u0000\u0000\u0000\"#\u0005\u0000\u0000\u0001#\u0001"+
		"\u0001\u0000\u0000\u0000$/\u0003\u0004\u0002\u0000%/\u0003\u0006\u0003"+
		"\u0000&/\u0003\b\u0004\u0000\'(\u0003\u0018\f\u0000()\u0005\u0001\u0000"+
		"\u0000)/\u0001\u0000\u0000\u0000*/\u0003\u0012\t\u0000+/\u0003\u0014\n"+
		"\u0000,/\u0003\u0016\u000b\u0000-/\u0003\f\u0006\u0000.$\u0001\u0000\u0000"+
		"\u0000.%\u0001\u0000\u0000\u0000.&\u0001\u0000\u0000\u0000.\'\u0001\u0000"+
		"\u0000\u0000.*\u0001\u0000\u0000\u0000.+\u0001\u0000\u0000\u0000.,\u0001"+
		"\u0000\u0000\u0000.-\u0001\u0000\u0000\u0000/\u0003\u0001\u0000\u0000"+
		"\u000001\u0003\u001a\r\u000014\u0005\u0017\u0000\u000023\u0005\u0002\u0000"+
		"\u000035\u0003\u0018\f\u000042\u0001\u0000\u0000\u000045\u0001\u0000\u0000"+
		"\u000056\u0001\u0000\u0000\u000067\u0005\u0001\u0000\u00007\u0005\u0001"+
		"\u0000\u0000\u000089\u0005\u0017\u0000\u00009:\u0005\u0002\u0000\u0000"+
		":;\u0003\u0018\f\u0000;<\u0005\u0001\u0000\u0000<\u0007\u0001\u0000\u0000"+
		"\u0000=>\u0003\u001a\r\u0000>?\u0005\u0017\u0000\u0000?A\u0005\u0003\u0000"+
		"\u0000@B\u0003\n\u0005\u0000A@\u0001\u0000\u0000\u0000AB\u0001\u0000\u0000"+
		"\u0000BC\u0001\u0000\u0000\u0000CD\u0005\u0004\u0000\u0000DE\u0003\u0012"+
		"\t\u0000E\t\u0001\u0000\u0000\u0000FG\u0003\u001a\r\u0000GN\u0005\u0017"+
		"\u0000\u0000HI\u0005\u0005\u0000\u0000IJ\u0003\u001a\r\u0000JK\u0005\u0017"+
		"\u0000\u0000KM\u0001\u0000\u0000\u0000LH\u0001\u0000\u0000\u0000MP\u0001"+
		"\u0000\u0000\u0000NL\u0001\u0000\u0000\u0000NO\u0001\u0000\u0000\u0000"+
		"O\u000b\u0001\u0000\u0000\u0000PN\u0001\u0000\u0000\u0000QR\u0005\u0006"+
		"\u0000\u0000RS\u0003\u0018\f\u0000ST\u0005\u0001\u0000\u0000T\r\u0001"+
		"\u0000\u0000\u0000UV\u0005\u0017\u0000\u0000VX\u0005\u0003\u0000\u0000"+
		"WY\u0003\u0010\b\u0000XW\u0001\u0000\u0000\u0000XY\u0001\u0000\u0000\u0000"+
		"YZ\u0001\u0000\u0000\u0000Z[\u0005\u0004\u0000\u0000[\u000f\u0001\u0000"+
		"\u0000\u0000\\a\u0003\u0018\f\u0000]^\u0005\u0005\u0000\u0000^`\u0003"+
		"\u0018\f\u0000_]\u0001\u0000\u0000\u0000`c\u0001\u0000\u0000\u0000a_\u0001"+
		"\u0000\u0000\u0000ab\u0001\u0000\u0000\u0000b\u0011\u0001\u0000\u0000"+
		"\u0000ca\u0001\u0000\u0000\u0000dh\u0005\u0007\u0000\u0000eg\u0003\u0002"+
		"\u0001\u0000fe\u0001\u0000\u0000\u0000gj\u0001\u0000\u0000\u0000hf\u0001"+
		"\u0000\u0000\u0000hi\u0001\u0000\u0000\u0000ik\u0001\u0000\u0000\u0000"+
		"jh\u0001\u0000\u0000\u0000kl\u0005\b\u0000\u0000l\u0013\u0001\u0000\u0000"+
		"\u0000mn\u0005\t\u0000\u0000no\u0005\u0003\u0000\u0000op\u0003\u0018\f"+
		"\u0000pq\u0005\u0004\u0000\u0000qr\u0003\u0012\t\u0000r\u0015\u0001\u0000"+
		"\u0000\u0000st\u0005\n\u0000\u0000tu\u0005\u0003\u0000\u0000uv\u0003\u0018"+
		"\f\u0000vw\u0005\u0004\u0000\u0000wz\u0003\u0012\t\u0000xy\u0005\u000b"+
		"\u0000\u0000y{\u0003\u0012\t\u0000zx\u0001\u0000\u0000\u0000z{\u0001\u0000"+
		"\u0000\u0000{\u0017\u0001\u0000\u0000\u0000|}\u0006\f\uffff\uffff\u0000"+
		"}\u0086\u0003\u000e\u0007\u0000~\u0086\u0005\u0017\u0000\u0000\u007f\u0086"+
		"\u0005\u0018\u0000\u0000\u0080\u0086\u0005\u0019\u0000\u0000\u0081\u0082"+
		"\u0005\u0003\u0000\u0000\u0082\u0083\u0003\u0018\f\u0000\u0083\u0084\u0005"+
		"\u0004\u0000\u0000\u0084\u0086\u0001\u0000\u0000\u0000\u0085|\u0001\u0000"+
		"\u0000\u0000\u0085~\u0001\u0000\u0000\u0000\u0085\u007f\u0001\u0000\u0000"+
		"\u0000\u0085\u0080\u0001\u0000\u0000\u0000\u0085\u0081\u0001\u0000\u0000"+
		"\u0000\u0086\u00a1\u0001\u0000\u0000\u0000\u0087\u0088\n\f\u0000\u0000"+
		"\u0088\u0089\u0005\f\u0000\u0000\u0089\u00a0\u0003\u0018\f\r\u008a\u008b"+
		"\n\u000b\u0000\u0000\u008b\u008c\u0005\r\u0000\u0000\u008c\u00a0\u0003"+
		"\u0018\f\f\u008d\u008e\n\n\u0000\u0000\u008e\u008f\u0005\u000e\u0000\u0000"+
		"\u008f\u00a0\u0003\u0018\f\u000b\u0090\u0091\n\t\u0000\u0000\u0091\u0092"+
		"\u0005\u000f\u0000\u0000\u0092\u00a0\u0003\u0018\f\n\u0093\u0094\n\b\u0000"+
		"\u0000\u0094\u0095\u0005\u0010\u0000\u0000\u0095\u00a0\u0003\u0018\f\t"+
		"\u0096\u0097\n\u0007\u0000\u0000\u0097\u0098\u0005\u0011\u0000\u0000\u0098"+
		"\u00a0\u0003\u0018\f\b\u0099\u009a\n\u0006\u0000\u0000\u009a\u009b\u0005"+
		"\u0012\u0000\u0000\u009b\u00a0\u0003\u0018\f\u0007\u009c\u009d\n\u0005"+
		"\u0000\u0000\u009d\u009e\u0005\u0013\u0000\u0000\u009e\u00a0\u0003\u0018"+
		"\f\u0006\u009f\u0087\u0001\u0000\u0000\u0000\u009f\u008a\u0001\u0000\u0000"+
		"\u0000\u009f\u008d\u0001\u0000\u0000\u0000\u009f\u0090\u0001\u0000\u0000"+
		"\u0000\u009f\u0093\u0001\u0000\u0000\u0000\u009f\u0096\u0001\u0000\u0000"+
		"\u0000\u009f\u0099\u0001\u0000\u0000\u0000\u009f\u009c\u0001\u0000\u0000"+
		"\u0000\u00a0\u00a3\u0001\u0000\u0000\u0000\u00a1\u009f\u0001\u0000\u0000"+
		"\u0000\u00a1\u00a2\u0001\u0000\u0000\u0000\u00a2\u0019\u0001\u0000\u0000"+
		"\u0000\u00a3\u00a1\u0001\u0000\u0000\u0000\u00a4\u00a5\u0007\u0000\u0000"+
		"\u0000\u00a5\u001b\u0001\u0000\u0000\u0000\f\u001f.4ANXahz\u0085\u009f"+
		"\u00a1";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}