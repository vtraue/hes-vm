import java.io.IOException;
import java.util.ArrayList;
import wasmBuilder.*;

public class BytecodeBuilder {
	private WasmBuilder builder;

	public WasmBuilder getWasmBuilder() {
		return this.builder;
	}

	public BytecodeBuilder() throws IOException {
		this.builder = new WasmBuilder();
	}

	public void build() throws IOException {
		builder.build();
	}

	public void enterFunction(FuncType funcType) {
		builder.addFuncType(funcType);
	}

	public void exitFunction() {
		builder.setInFunction(false);
	}

	public void emitLocalSet(int id) throws IOException {
		builder.addLocalSet(id);
	}

	public void emitLocalGet(int id) throws IOException {
		builder.addLocalGet(id);
	}

	public void emitGlobalSet(int id) throws IOException {
		builder.addGlobalSet(id);
	}

	public void emitGlobalGet(int id) throws IOException {
		builder.addGlobalGet(id);
	}

	public void emitAdd() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_ADD);
	}

	public void emitSub() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_SUB);
	}

	public void emitMul() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_MUL);
	}

	public void emitDiv() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_DIV_S);
	}

	public void emitRem() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_REM_S);
	}

	public void emitAnd() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_AND);
	}

	public void emitOr() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_OR);
	}

	public void emitXor() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_XOR);
	}

	public void emitEqz() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_EQZ);
	}

	public void emitEq() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_EQ);
	}

	public void emitNe() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_NE);
	}

	public void emitLt() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_LT_S);
	}

	public void emitGt() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_GT_S);
	}

	public void emitLe() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_LE_S);
	}

	public void emitGe() throws IOException {
		builder.addBinOp(WasmInstructionOpCode.I32_GE_S);
	}
}
