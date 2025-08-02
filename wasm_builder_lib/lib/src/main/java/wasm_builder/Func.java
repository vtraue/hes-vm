package wasm_builder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Func {
	private final FuncType funcType;
	final private BytecodeWriter body;
	final private Validator validator;
	private ArrayList<Local> locals;
	private int funcIdx;
	private WasmBuilder wasmBuilder;

	public Func(FuncType funcType, List<Local> locals, List<GlobalType> globals, WasmBuilder wasmBuilder) {
		this.wasmBuilder = wasmBuilder;
		this.funcType = funcType;
		this.body = new BytecodeWriter();
		this.locals = new ArrayList<>(locals);
		this.validator = new Validator(globals, locals);
	}

	public Func( FuncType funcType, WasmBuilder wasmBuilder) {
		this( funcType, Collections.emptyList(), Collections.emptyList(), wasmBuilder);
	}

	public ByteArrayOutputStream getBody() {
		return this.body.getOutputStream();
	}

	public ArrayList<Local> getLocals() {
		return this.locals;
	}

	public FuncType getFuncType() {
		return funcType;
	}

	public void addLocal(Local localType) {
		this.locals.add(localType);
	}


	public int addStringData(List<String> strings) {
		return this.wasmBuilder.addStringData(strings);
	}

	public void emitEnd() throws IOException {
		body.writeOpcode(InstructionOpCode.END);
	}

	public void emitCall(int id) throws IOException {
		body.writeOpcode(InstructionOpCode.CALL);
		body.writeU32(id);
	}

	public void emitI32Load() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_LOAD);
		body.writeU32(0); //align
		body.writeU32(0); //offset
	}

	public void emitI32Load(int offset) throws IOException {
		body.writeOpcode(InstructionOpCode.I32_LOAD);
		body.writeU32(0); //align
		body.writeU32(offset); //offset
	}

	public void emitI32Store() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_STORE);
		body.writeU32(0); //align
		body.writeU32(0); //offset
	}

	public void emitI32Store(int offset) throws IOException {
		body.writeOpcode(InstructionOpCode.I32_STORE);
		body.writeU32(0); //align
		body.writeU32(offset); //offset
	}

	public void emitI32Const(int value) throws IOException {
		body.writeOpcode(InstructionOpCode.I32_CONST);
		body.writeI32(value);
	}

	// TODO: encoding i64, f32, f64

	public void emitLocalSet(int id) throws IOException {
		body.writeOpcode(InstructionOpCode.LOCAL_SET);
		body.writeU32(id);
	}

	public void emitLocalGet(int id) throws IOException {
		body.writeOpcode(InstructionOpCode.LOCAL_GET);
		body.writeU32(id);
	}

	public void emitLocalTee(int id) throws IOException {
		body.writeOpcode(InstructionOpCode.LOCAL_TEE);
		body.writeU32(id);
	}

	public void emitGlobalSet(int id) throws IOException {
		body.writeOpcode(InstructionOpCode.GLOBAL_SET);
		body.writeU32(id);
	}

	public void emitGlobalGet(int id) throws IOException {
		body.writeOpcode(InstructionOpCode.GLOBAL_GET);
		body.writeU32(id);
	}

	// binops
	public void emitI32Add() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_ADD);
	}

	public void emitI32Sub() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_SUB);
	}

	public void emitI32Mul() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_MUL);
	}

	public void emitI32Div() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_DIV_S);
	}

	public void emitI32Rem() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_REM_S);
	}

	public void emitI32And() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_AND);
	}

	public void emitI32Or() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_OR);
	}

	public void emitI32Xor() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_XOR);
	}

	public void emitI32Eqz() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_EQZ);
	}

	public void emitI32Eq() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_EQ);
	}

	public void emitI32Ne() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_NE);
	}

	public void emitI32Lt() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_LT_S);
	}

	public void emitI32Gt() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_GT_S);
	}

	public void emitI32Le() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_LE_S);
	}

	public void emitI32Ge() throws IOException {
		body.writeOpcode(InstructionOpCode.I32_GE_S);
	}

	// control
	public void emitIf() throws IOException {
		body.writeOpcode(InstructionOpCode.IF);
	}

	public void emitElse() throws IOException {
		body.writeOpcode(InstructionOpCode.ELSE);
	}

	public void emitBlock() throws IOException {
		body.writeOpcode(InstructionOpCode.BLOCK);
	}

	public void emitLoop() throws IOException {
		body.writeOpcode(InstructionOpCode.LOOP);
	}

	public void emitBlockType() throws IOException {
		body.writeByte((byte) 0x40);
	}

	public void emitBlockType(ValueType valtype) throws IOException {
		body.writeByte((byte) valtype.code);
	}
	public void emitBlockType(int typeidx) throws IOException {
		body.writeI32(typeidx);
	}

	public void emitBr(int jumpIndex) throws IOException {
		body.writeOpcode(InstructionOpCode.BR);
		body.writeU32(jumpIndex);
	}

	public void emitBrIf(int jumpIndex) throws IOException {
		body.writeOpcode(InstructionOpCode.BR_IF);
		body.writeU32(jumpIndex);
	}

	public void emitNop() throws IOException {
		body.writeOpcode(InstructionOpCode.NOP);
	}

	public void emitReturn() throws IOException {
		body.writeOpcode(InstructionOpCode.RETURN);
	}

	public void emitUnreachable() throws IOException {
		body.writeOpcode(InstructionOpCode.UNREACHABLE);
	}
}
