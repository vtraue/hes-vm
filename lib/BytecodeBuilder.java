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

	public void build(ArrayList<Func> funcs) throws IOException {
		builder.build(funcs);
	}

	public Func createFunction(FuncType funcType) {
		return builder.addFunc(funcType);
	}
}
