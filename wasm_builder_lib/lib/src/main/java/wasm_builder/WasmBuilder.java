package wasm_builder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.List;
import java.util.Optional;

public class WasmBuilder {

	private ByteArrayOutputStream out = new ByteArrayOutputStream();
	private ArrayList<FuncType> funcTypes = new ArrayList<>();
	private ArrayList<GlobalType> globals = new ArrayList<>();
	private ArrayList<Import> imports = new ArrayList<>();
	private ArrayList<FuncType> importedFuncTypes = new ArrayList<>();
	private ArrayList<GlobalType> importedGlobals = new ArrayList<>();
	private Optional<Integer> startFunctionId = Optional.empty();	

	private ArrayList<Export> exportedFuncs = new ArrayList<>(); 
  //private StringPool stringPool = new StringPool();
  private ArrayList<byte[]> stringLiterals = new ArrayList<>();
  private int stringLiteralMemIndex = 0;

	public void build(List<Func> funcs) throws IOException {
		ArrayList<FuncType> allFuncTypes = new ArrayList<>(importedFuncTypes);
		allFuncTypes.addAll(funcTypes);
		ArrayList<GlobalType> allGlobals = new ArrayList<>(importedGlobals);
		allGlobals.addAll(globals);
		writeBinaryMagic(out);
		writeBinaryVersion(out);

		if (!funcTypes.isEmpty()) {
			writeTypeSection(allFuncTypes, out);
		}
		writeImportSection(imports, out);
		if (!funcTypes.isEmpty()) {
			writeFuncSection(funcTypes, out);
		}
		writeMemSection(out);
		if(!globals.isEmpty()){
			writeGlobalSection(globals, out);
		}
		if(!exportedFuncs.isEmpty()) {
			writeExportSection(this.exportedFuncs, out);
		}
		if(this.startFunctionId.isPresent()) {
			writeStartSection(this.startFunctionId.get(), out);	
		}

    if(!stringLiterals.isEmpty()) {
      writeDataCountSection(out);
    }
    
		if (!funcTypes.isEmpty()) {
			writeCodeSection(funcs, out);
		}

    if(!stringLiterals.isEmpty()) {
      writeDataSection(out);
    }
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

	private void writeStartSection(int id, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream s = new ByteArrayOutputStream();
		write(encodeU32ToLeb128(id), s);

		write((byte) SectionId.Start.ordinal(), os);
		write(encodeU32ToLeb128(s.size()), os);
		
		os.write(s.toByteArray());
	}	
	public void setStartFunction(int id) {
		this.startFunctionId = Optional.of(id);	
	}

	public Func addFunc(FuncType funcType, Optional<List<WasmValueType>> locals) {
		this.funcTypes.add(funcType);
		return new Func(this,funcType, locals);
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

	public void setImports(List<Import> imports) {
		for (Import im : imports) {
			addImport(im);
		}
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

	public byte[] getByteArray() {
		return out.toByteArray();
	}

	public String getAsHexString() {
		HexFormat hex = HexFormat.of();
		return hex.formatHex(out.toByteArray());
	}

	static void write(byte code, ByteArrayOutputStream os) throws IOException {
		byte[] b = { code };
		os.write(b);
	}

	static void write(List<Integer> al, ByteArrayOutputStream os) throws IOException {
		for (Integer e : al) {
			byte[] byteId = { (byte) e.intValue() };
			os.write(byteId);
		}
	}


	private void writeFunctionTypes(List<FuncType> functypes, ByteArrayOutputStream os) throws IOException {
		for (FuncType f : functypes) {
			write((byte) 0x60, os);
			write(encodeU32ToLeb128(f.getParams().size()), os);
			write(f.getParams(), os);
			write(encodeU32ToLeb128(f.getResults().size()), os);
			write(f.getResults(), os);
		}
	}

	private void writeTypeSection(List<FuncType> functypes, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream functypesBytes = new ByteArrayOutputStream();
		write(encodeU32ToLeb128(functypes.size()), functypesBytes);
		writeFunctionTypes(functypes, functypesBytes);

		write((byte) SectionId.Type.ordinal(), os);
		write(encodeU32ToLeb128(functypesBytes.size()), os);
		os.write(functypesBytes.toByteArray());
	}

	private void writeImportSection(List<Import> imports, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream importBytes = new ByteArrayOutputStream();
		write(encodeU32ToLeb128(imports.size()), importBytes);
		for (Import im : imports) {
			writeImport(im, importBytes);
		}

		write((byte) SectionId.Import.ordinal(), os);
		write(encodeU32ToLeb128(importBytes.size()), os);
		os.write(importBytes.toByteArray());
	}

	private void writeImport(Import im, ByteArrayOutputStream os) throws IOException {
		write(encodeU32ToLeb128(im.getModule().length()), os);
		os.write(im.getModule().getBytes(StandardCharsets.UTF_8));
		write(encodeU32ToLeb128(im.getName().length()), os);
		os.write(im.getName().getBytes(StandardCharsets.UTF_8));

		switch(im.getDesc()){
			case FuncType f -> {
				write((byte)0x00, os);
				write(encodeU32ToLeb128(importedFuncTypes.indexOf(f)), os);
			}
			case TableType t -> {
				write((byte)0x01, os);
				if (t.refExt()){
					write((byte)0x67, os); // externref
				} else {

					write((byte)0x70, os); // funcref
				}
				write((byte)0x01, os);
				write(encodeU32ToLeb128(t.min()), os);
				write(encodeU32ToLeb128(t.max()), os);
			}
			case MemType m -> {
				write((byte)0x02, os);
				write(encodeU32ToLeb128(m.min()), os);
				write(encodeU32ToLeb128(m.max()), os);
			}
			case GlobalType g -> {
				write((byte)0x03, os);
				write((byte)g.valtype().code, os);
				if (g.mutable()) {
					write((byte)0x01, os);
				}else {
					write((byte)0x00, os);
				}
			}
		}
	}
	
	private void writeExportSection(List<Export> exports, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream exportBytes = new ByteArrayOutputStream();
		System.out.println("Writing export section");
		write(encodeU32ToLeb128(exports.size() + 1), exportBytes);
		for(Export export : exports) {
			writeExport(export, exportBytes);
		}
    //Exportiere immer Memory ID 0 
    String memoryName = "memory";
		write(encodeU32ToLeb128(memoryName.length()), exportBytes);
		exportBytes.write(memoryName.getBytes(StandardCharsets.UTF_8));
		write((byte)0x02, exportBytes);
		write((byte)0x00, exportBytes);

		write((byte) SectionId.Export.ordinal(), os);
		write(encodeU32ToLeb128(exportBytes.size()), os);
		os.write(exportBytes.toByteArray());
	}

	private void writeExport(Export export, ByteArrayOutputStream os) throws IOException {
		write(encodeU32ToLeb128(export.name().length()), os);
		os.write(export.name().getBytes(StandardCharsets.UTF_8));
		write((byte)0x00, os);
		write(encodeU32ToLeb128(export.funcId()), os);
	}

	private void writeFuncSection(List<FuncType> funcTypes, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream funcIdsBytes = new ByteArrayOutputStream();

		write(encodeU32ToLeb128(funcTypes.size()), funcIdsBytes);
		for (FuncType funcType : funcTypes) {
			write((byte) (funcTypes.indexOf(funcType) + importedFuncTypes.size()), funcIdsBytes);
		}

		write((byte) SectionId.Function.ordinal(), os);
		write(encodeU32ToLeb128(funcIdsBytes.size()), os);
		os.write(funcIdsBytes.toByteArray());
	}

	private void writeMemSection(ByteArrayOutputStream os) throws IOException {
		write((byte) SectionId.Memory.ordinal(), os);
		write(encodeU32ToLeb128(3), os); // Section Size
		write(encodeU32ToLeb128(1), os); // Num Memories
		write(encodeU32ToLeb128(0), os); // limits flags
		write(encodeU32ToLeb128(1), os); // limits min / initial
	}

  private byte[] getStringLiteralBytes(String s) {
    
    ByteArrayOutputStream litBytes = new ByteArrayOutputStream();
    try {
      byte[] sizeBytes = ByteBuffer.allocate(4).putInt(s.length()).array();

      litBytes.write(sizeBytes);
      litBytes.write(s.getBytes(StandardCharsets.UTF_8));
      litBytes.write(0);
    } catch(Exception e) {
      System.out.println(e.toString());
    }
    return litBytes.toByteArray();
  }

  private void writeActiveDataMode(int offset, ByteArrayOutputStream os) throws IOException{
    os.write(0);   
    Instructions.addI32Const(offset, os);
    Instructions.addEnd(os);
  }

  private int writeStringData(byte[] data, int offset, ByteArrayOutputStream os) throws IOException {
    writeActiveDataMode(offset, os);  
    write(encodeU32ToLeb128(data.length), os);
    os.write(data);
    return data.length;
  }

  private void writeDataCountSection(ByteArrayOutputStream os) throws IOException {
    ByteArrayOutputStream section = new ByteArrayOutputStream();
    write(encodeU32ToLeb128(this.stringLiterals.size()), section);

    write((byte)SectionId.DataCount.ordinal(), os);
    write(encodeU32ToLeb128(section.size()), os);
    section.writeTo(os); 
    
  }

  private void writeDataSection(ByteArrayOutputStream os) throws IOException {
    ByteArrayOutputStream section = new ByteArrayOutputStream();
    write(encodeU32ToLeb128(this.stringLiterals.size()), section);
    int offset = 0;
    for(var literal: this.stringLiterals) {
      offset += writeStringData(literal, offset, section); 
    }
		write((byte)SectionId.Data.ordinal(), os);
    write(encodeU32ToLeb128(section.size()), os);
    section.writeTo(os);
  }

	private void writeGlobalSection(List<GlobalType> globals, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream globalsBytes = new ByteArrayOutputStream();
		write(encodeU32ToLeb128(globals.size()), globalsBytes); // Anz Globals
		for (GlobalType globalType : globals) {
			write((byte)globalType.valtype().code, globalsBytes);
			if (globalType.mutable()) {

				write((byte)1, globalsBytes); // mutable
			} else {

				write((byte)0, globalsBytes); // immutable
			}
			Instructions.addI32Const(0, globalsBytes);
			Instructions.addEnd(globalsBytes);
		}
		write((byte)SectionId.Global.ordinal(), os);
		write(encodeU32ToLeb128(globalsBytes.size()), os);
		os.write(globalsBytes.toByteArray());
	}
	private void writeCodeSection(List<Func> funcs, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream funcBodiesBytes = new ByteArrayOutputStream();
		// Anzahl der Funktionen
		write(encodeU32ToLeb128(funcs.size()), funcBodiesBytes);
		for (Func func : funcs) {
			writeFuncBody(func, funcBodiesBytes);
		}

		write((byte) SectionId.Code.ordinal(), os);
		// Größe der Code-Section in Byte
		write(encodeU32ToLeb128(funcBodiesBytes.size()), os);
		os.write(funcBodiesBytes.toByteArray());

	}

	private void writeFuncBody(Func f, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream funcBodyBytes = new ByteArrayOutputStream();
		writeFuncLocals(f.getLocals(), funcBodyBytes);
		if(f.getBody().size() > 0){
			funcBodyBytes.write(f.getBody().toByteArray());
		} else {
			Instructions.addEnd(funcBodyBytes);
		}

		// Größe des Bodies in Byte mit local decl und instructions
		write(encodeU32ToLeb128(funcBodyBytes.size()), os);
		os.write(funcBodyBytes.toByteArray());
	}

	private void writeFuncLocals(List<WasmValueType> locals, ByteArrayOutputStream os) throws IOException {
		if (locals.isEmpty()) {
			write(encodeU32ToLeb128(0), os);
		} else if (locals.size() == 1) {
			write(encodeU32ToLeb128(1), os); // Anzahl Deklarationen
			write(encodeU32ToLeb128(1), os); // Anzahl Typ
			write((byte) locals.get(0).code, os);
		} else {
			int declCount = 0, typeCount = 0;
			WasmValueType lastType = locals.get(0);
			ByteArrayOutputStream declsBytes = new ByteArrayOutputStream();
			// i32 i32 i64 i32 i32 -> 2 i32 1 i64 2 i32

			for (WasmValueType wasmValueType : locals) {
				if (wasmValueType == lastType) {
					typeCount++;
				} else {
					write(encodeU32ToLeb128(typeCount), declsBytes);
					write((byte) lastType.code, declsBytes);
					typeCount = 1;
					declCount++;
					lastType = wasmValueType;
				}
			}
			if (typeCount > 1) {
				write(encodeU32ToLeb128(typeCount), declsBytes);
				write((byte) lastType.code, declsBytes);
				declCount++;
			}
			write(encodeU32ToLeb128(declCount), os);
			os.write(declsBytes.toByteArray());
		}
	}

	private void writeBinaryMagic(ByteArrayOutputStream os) throws IOException {
		byte[] wasmBinaryMagic = { 0x0, 'a', 's', 'm' };
		os.write(wasmBinaryMagic);
	}

	private void writeBinaryVersion(ByteArrayOutputStream os) throws IOException {
		byte[] wasmBinaryVersion = { 0x01, 0x00, 0x00, 0x00 };
		os.write(wasmBinaryVersion);
	}

	public static ArrayList<Integer> encodeU32ToLeb128(int value) {
		value |= 0;
		ArrayList<Integer> result = new ArrayList<>();
		while (true) {
			int byte_ = value & 0x7f;
			value >>= 7;
			if (value == 0) {
				result.add(byte_);
				return result;
			}
			result.add(byte_ | 0x80);
		}
	}

	public static ArrayList<Integer> encodeI32ToLeb128(int value) {
		value |= 0;
		ArrayList<Integer> result = new ArrayList<>();
		while (true) {

			int byte_ = value & 0x7f;
			value >>= 7;
			if ((value == 0 && (byte_ & 0x40) == 0) || (value == -1 && (byte_ & 0x40) != 0)) {
				result.add(byte_);
				return result;
			}
			result.add(byte_ | 0x80);
		}

	}

}
