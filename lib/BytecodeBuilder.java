import java.io.IOException;
import java.util.ArrayList;
import wasmBuilder.*;

public class BytecodeBuilder {
	private ArrayList<FunctionType> functiontypes = new ArrayList<FunctionType>();
	private WasmBuilder builder;

	public WasmBuilder getWasmBuilder() {
		return this.builder;
	}

	public BytecodeBuilder(ArrayList<FunctionType> functypes) throws IOException {
		this.builder = new WasmBuilder();
		this.functiontypes = functypes;
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
