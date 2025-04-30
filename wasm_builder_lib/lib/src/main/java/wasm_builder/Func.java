package wasm_builder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Optional;

public class Func {
	private final FuncType funcType;
  public WasmBuilder builder;
	final private ByteArrayOutputStream body;
	private ArrayList<Local> locals;

	public Func(WasmBuilder builder, FuncType funcType, List<Local> locals) {
		this.builder = builder;
		this.funcType = funcType;
		this.body = new ByteArrayOutputStream();
		this.locals = new ArrayList<>(locals);
	}

	public Func(WasmBuilder builder, FuncType funcType) {
		this(builder, funcType, Collections.emptyList());
	}

	public ByteArrayOutputStream getBody() {
		return this.body;
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

	public void emitEnd() throws IOException {
		Instructions.addEnd(body);
	}

	public void emitCall(int id) throws IOException {
		Instructions.addCall(id, body);
	}

	public void emitLoad() throws IOException {
		Instructions.addI32Load(0, 0, body);
	}

	public void emitStore() throws IOException {
		Instructions.addI32Store(0, 0, body);
	}

	public void emitConst(int value) throws IOException {
		Instructions.addI32Const(value, body);
	}

	public void emitLocalSet(int id) throws IOException {
		Instructions.addLocalSet(id, body);
	}

	public void emitLocalGet(int id) throws IOException {
		Instructions.addLocalGet(id, body);
	}

	public void emitLocalTee(int id) throws IOException {
		Instructions.addLocalTee(id, body);
	}

	public void emitGlobalSet(int id) throws IOException {
		Instructions.addGlobalSet(id, body);
	}

	public void emitGlobalGet(int id) throws IOException {
		Instructions.addGlobalGet(id, body);
	}

	// binops
	public void emitAdd() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_ADD, body);
	}

	public void emitSub() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_SUB, body);
	}

	public void emitMul() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_MUL, body);
	}

	public void emitDiv() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_DIV_S, body);
	}

	public void emitRem() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_REM_S, body);
	}

	public void emitAnd() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_AND, body);
	}

	public void emitOr() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_OR, body);
	}

	public void emitXor() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_XOR, body);
	}

	public void emitEqz() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_EQZ, body);
	}

	public void emitEq() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_EQ, body);
	}

	public void emitNe() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_NE, body);
	}

	public void emitLt() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_LT_S, body);
	}

	public void emitGt() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_GT_S, body);
	}

	public void emitLe() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_LE_S, body);
	}

	public void emitGe() throws IOException {
		Instructions.addBinOp(WasmInstructionOpCode.I32_GE_S, body);
	}

	// control
	public void emitIf() throws IOException {
		Instructions.addIf(body);
	}

	public void emitElse() throws IOException {
		Instructions.addElse(body);
	}

	public void emitBlock() throws IOException {
		Instructions.addBlock(body);
	}

	public void emitLoop() throws IOException {
		Instructions.addLoop(body);
	}

	public void emitBlockType() throws IOException {
		Instructions.addBlockType(body);
	}

	public void emitBlockType(WasmValueType valtype) throws IOException {
		Instructions.addBlockType(valtype, body);

	}

	public void emitBlockType(int typeidx) throws IOException {
		Instructions.addBlockType(typeidx, body);

	}
}
