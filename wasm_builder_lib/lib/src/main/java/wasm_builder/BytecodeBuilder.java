package wasm_builder;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

public class BytecodeBuilder {
	private final WasmBuilder builder;

	public WasmBuilder getWasmBuilder() {
		return this.builder;
	}

	public BytecodeBuilder() throws IOException {
		this.builder = new WasmBuilder();
	}

	public void build(List<Func> funcs) throws IOException {
		builder.build(funcs);
	}

	public Func createFunction(FuncType funcType) {
		return builder.addFunc(funcType, Optional.empty());
	}

	public Func createFunction(FuncType funcType, List<WasmValueType> locals) {
		return builder.addFunc(funcType, Optional.of(locals));
	}

	public void setGlobals(List<WasmValueType> globals) {
		this.builder.setGlobals(globals);
	}
}
