package wasm_builder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.List;
import java.util.Optional;

public class WasmBuilder {

	private BytecodeWriter bw = new BytecodeWriter();
	private ByteArrayOutputStream out = new ByteArrayOutputStream();
	private ArrayList<FuncType> funcTypes = new ArrayList<>();
	private ArrayList<GlobalType> globals = new ArrayList<>();
	private ArrayList<Import> imports = new ArrayList<>();
	private ArrayList<FuncType> importedFuncTypes = new ArrayList<>();
	private ArrayList<GlobalType> importedGlobals = new ArrayList<>();
	private Optional<Integer> startFunctionId = Optional.empty();
	private NameSection nameSection = new NameSection();

	private ArrayList<Export> exportedFuncs = new ArrayList<>();
	private ArrayList<byte[]> stringLiterals = new ArrayList<>();
	private int stringLiteralMemIndex = 0;

	private void fillFuncTypes(List<Func> funcs) {
	  for (Func f : funcs) {
		  this.funcTypes.add(f.getFuncType());
	  }
	}

	private byte[] getStringLiteralBytes(String s) {
		ByteArrayOutputStream litBytes = new ByteArrayOutputStream();
		try {
			var buffer = ByteBuffer.allocate(4);
			buffer.order(ByteOrder.LITTLE_ENDIAN);

			byte[] sizeBytes = buffer.putInt(s.length()).array();

			litBytes.write(sizeBytes);
			litBytes.write(s.getBytes(StandardCharsets.UTF_8));
			litBytes.write(0);
		} catch(Exception e) {
			System.out.println(e.toString());
		}
		return litBytes.toByteArray();
	}

	private void writeBinaryMagic(BytecodeWriter os) throws IOException {
		byte[] wasmBinaryMagic = { 0x0, 'a', 's', 'm' };
		os.writeBytes(wasmBinaryMagic);
	}

	private void writeBinaryVersion(BytecodeWriter os) throws IOException {
		byte[] wasmBinaryVersion = { 0x01, 0x00, 0x00, 0x00 };
		os.writeBytes(wasmBinaryVersion);
	}

	// API
	public void build(List<Func> funcs) throws IOException {
	  	out.reset();
		  bw.reset();
		fillFuncTypes(funcs);
		ArrayList<FuncType> allFuncTypes = new ArrayList<>(importedFuncTypes);
		allFuncTypes.addAll(funcTypes);
		ArrayList<GlobalType> allGlobals = new ArrayList<>(importedGlobals);
		allGlobals.addAll(globals);
		writeBinaryMagic(bw);
		writeBinaryVersion(bw);

		// Type Section
		if (!funcTypes.isEmpty()) {
			TypeSection ts = new TypeSection(funcTypes);
			ts.write(bw);
		}

		// Import Section
		ImportSection is = new ImportSection(imports, importedFuncTypes);
		is.write(bw);

		// Function Section
		if (!funcTypes.isEmpty()) {
			// Add Function Names to NameSection
			nameSection.addFunctionNames(funcTypes, importedFuncTypes.size());
			// Write Function Section
			FunctionSection fs = new FunctionSection(funcTypes, importedFuncTypes.size());
			fs.write(bw);
		}

		// Memory Section
		MemorySection ms = new MemorySection();
		ms.write(bw);

		// Global Section
		if(!globals.isEmpty()){
			GlobalSection globalSection = new GlobalSection(globals);
			globalSection.write(bw);
		}

		// Export Section
		if(!exportedFuncs.isEmpty()) {
			ExportSection exportSection = new ExportSection(exportedFuncs);
			exportSection.write(bw);
		}

		// Start Section
		if(this.startFunctionId.isPresent()) {
			StartSection startSection = new StartSection(startFunctionId.get());
			startSection.write(bw);
		}

		// Data Count Section
		if(!stringLiterals.isEmpty()) {
			DataCountSection dataCountSection = new DataCountSection(stringLiterals);
			dataCountSection.write(bw);
		}

		// Code Section
		if (!funcTypes.isEmpty()) {
			// Add Function Local Names to NameSection
			nameSection.addLocalNames(funcs, importedFuncTypes.size());

			CodeSection codeSection = new CodeSection(funcs, importedFuncTypes.size());
			codeSection.write(bw);
		}

		// Data Section
		if(!stringLiterals.isEmpty()) {
			DataSection dataSection = new DataSection(stringLiterals);
			dataSection.write(bw);
		}

		// Name Section
		nameSection.write(bw);
	}

	public int addStringData(List<String> strings) {
		int currentIndex = this.stringLiteralMemIndex;
		for(String s : strings) {
			var literal = getStringLiteralBytes(s);
			this.stringLiteralMemIndex += literal.length;
			this.stringLiterals.add(literal);
		}
		return currentIndex;
	}

	public void importFunc(String module, String name, FuncType funcType) {
		Import im = new Import(module, name, funcType);
		addImport(im);
	}

	public void addImport(Import im) {
		this.imports.add(im);
		switch (im.getDesc()){
			case FuncType funcType -> {
				this.importedFuncTypes.add(funcType);
			}
			case GlobalType globalType -> {
				this.importedGlobals.add(globalType);
			}
			case MemType ignored -> {
				//TODO
			}
			case TableType ignored -> {
				//TODO
			}
		}
	}

	public void setStartFunction(int id) {
		this.startFunctionId = Optional.of(id);
	}

	public Func createFunction(FuncType funcType, List<Local> locals) {
		ArrayList<GlobalType> allGlobals = new ArrayList<>(importedGlobals);
		allGlobals.addAll(globals);
		return new Func(funcType, locals, allGlobals);
	}

	public Func createFunction(FuncType funcType) {
		return new Func(funcType );
	}

	public void setGlobals(List<GlobalType> globals) {
		this.globals.addAll(globals);
	}

	public void addGlobal(GlobalType global) {
		this.globals.add(global);
	}

	public void addExport(Export export) {
		this.exportedFuncs.add(export);
	}

	public void exportFunction(String name, int id) {
		addExport(new Export(name, id));
	}

	public void setImports(List<Import> imports) {
		for (Import im : imports) {
			addImport(im);
		}
	}

	public void setModuleName(String name) {
	  this.nameSection.setModuleName(name);
	}

	public byte[] getByteArray() {
		return bw.getOutputStream().toByteArray();
	}
}
