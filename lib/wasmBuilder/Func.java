package wasmBuilder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.ArrayList;

public class Func {
	private FuncType funcType;
	private int Id;
	private ByteArrayOutputStream body;
	private ArrayList<WasmValueType> locals = new ArrayList<>();

	public Func(FuncType funcType) {
		this.funcType = funcType;

	}

	public ByteArrayOutputStream getBody() {
		return this.body;

	}

	public FuncType getFuncType() {
		return funcType;
	}

	public ByteArrayOutputStream getfuncCode() {
		ByteArrayOutputStream locals = new ByteArrayOutputStream();
		ByteArrayOutputStream funcCode = new ByteArrayOutputStream();
		return funcCode;
	}

	public void emitLocalSet(int id) throws IOException {
		WasmBuilder.addLocalSet(id, body);
	}

	public void emitLocalGet(int id) throws IOException {
		WasmBuilder.addLocalGet(id, body);
	}

	public void emitGlobalSet(int id) throws IOException {
		WasmBuilder.addGlobalSet(id, body);
	}

	public void emitGlobalGet(int id) throws IOException {
		WasmBuilder.addGlobalGet(id, body);
	}

	public void emitAdd() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_ADD, body);
	}

	public void emitSub() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_SUB, body);
	}

	public void emitMul() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_MUL, body);
	}

	public void emitDiv() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_DIV_S, body);
	}

	public void emitRem() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_REM_S, body);
	}

	public void emitAnd() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_AND, body);
	}

	public void emitOr() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_OR, body);
	}

	public void emitXor() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_XOR, body);
	}

	public void emitEqz() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_EQZ, body);
	}

	public void emitEq() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_EQ, body);
	}

	public void emitNe() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_NE, body);
	}

	public void emitLt() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_LT_S, body);
	}

	public void emitGt() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_GT_S, body);
	}

	public void emitLe() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_LE_S, body);
	}

	public void emitGe() throws IOException {
		WasmBuilder.addBinOp(WasmInstructionOpCode.I32_GE_S, body);
	}
}
