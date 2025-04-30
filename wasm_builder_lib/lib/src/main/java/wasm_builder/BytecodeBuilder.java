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

	public BytecodeBuilder() {
		this.builder = new WasmBuilder();
	}

	public void build(List<Func> funcs) throws IOException {
		builder.build(funcs);
	}

	public Func createFunction(FuncType funcType) {
		return builder.addFunc(funcType);
	}

	public Func createFunction(FuncType funcType, List<Local> locals) {
		return builder.addFunc(funcType, locals);
	}

	public void setGlobals(List<GlobalType> globals) {
		this.builder.setGlobals(globals);
	}

	public void setGlobals(GlobalType global) {
		this.builder.addGlobal(global);
	}

	public void setImports(List<Import> imports){
		builder.setImports(imports);
	}

	public void addImport(Import im) {
		builder.addImport(im);
	}

	public void importFunc(String module, String name, FuncType funcType) {
		Import im = new Import(module, name, funcType);
		builder.addImport(im);
	}

	public void setStartFunction(int id) {
		builder.setStartFunction(id);
	}

	public void exportFunction(String name, int id) {
		builder.addExport(new Export(name, id));
	}

  public int addStringData(List<String> strings) {
    return builder.addStringData(strings);
  }
}
