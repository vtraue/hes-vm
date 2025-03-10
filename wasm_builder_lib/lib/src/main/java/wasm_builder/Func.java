package wasm_builder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

public class Func {
	private FuncType funcType;
	private int Id;
	private ByteArrayOutputStream body;
	private ArrayList<WasmValueType> locals;

	public Func(FuncType funcType, Optional<List<WasmValueType>> locals) {
		this.funcType = funcType;
		this.body = new ByteArrayOutputStream();
		this.locals = locals.isPresent() ? new ArrayList<>(locals.get()) : new ArrayList<>();
	}

	public ByteArrayOutputStream getBody() {
		return this.body;
	}

	public ArrayList<WasmValueType> getLocals() {
		return this.locals;
	}

	public FuncType getFuncType() {
		return funcType;
	}

	public ByteArrayOutputStream getFuncCode() {
		ByteArrayOutputStream locals = new ByteArrayOutputStream();
		ByteArrayOutputStream funcCode = new ByteArrayOutputStream();
		return funcCode;
	}

	public void emitEnd() throws IOException {
		Instructions.addEnd(body);
	}

	public void emitCall(int id) throws IOException {
		Instructions.addCall(id, body);
	}

	public void emitLocalSet(int id) throws IOException {
		Instructions.addLocalSet(id, body);
	}

	public void emitLocalGet(int id) throws IOException {
		Instructions.addLocalGet(id, body);
	}

	public void emitGlobalSet(int id) throws IOException {
		Instructions.addGlobalSet(id, body);
	}

	public void emitGlobalGet(int id) throws IOException {
		Instructions.addGlobalGet(id, body);
	}

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
}
