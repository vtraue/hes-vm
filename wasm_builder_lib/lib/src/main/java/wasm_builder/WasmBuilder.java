package wasm_builder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.List;
import java.util.Optional;

public class WasmBuilder {

	private ByteArrayOutputStream out = new ByteArrayOutputStream();
	private ArrayList<FuncType> funcTypes = new ArrayList<>();
	private ArrayList<WasmValueType> globals = new ArrayList<>();

	public void build(List<Func> funcs) throws IOException {
		writeBinaryMagic(out);
		writeBinaryVersion(out);
		if (!funcTypes.isEmpty()) {
			writeTypeSection(funcTypes, out);
			writeFuncSection(funcTypes, out);
		}
		writeMemSection(out);
		if(!globals.isEmpty()){
			writeGlobalSection(globals, out);
		}
		if (!funcTypes.isEmpty()) {
			writeCodeSection(funcs, out);
		}
	}

	public Func addFunc(FuncType funcType, Optional<List<WasmValueType>> locals) {
		this.funcTypes.add(funcType);
		return new Func(funcType, locals);
	}

	public void setGlobals(List<WasmValueType> globals) {
		this.globals = (ArrayList<WasmValueType>) globals;
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
			write(encodeI32ToLeb128(f.getParams().size()), os);
			write(f.getParams(), os);
			write(encodeI32ToLeb128(f.getResults().size()), os);
			write(f.getResults(), os);
		}
	}

	public void writeTypeSection(List<FuncType> functypes, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream functypesBytes = new ByteArrayOutputStream();
		write(encodeI32ToLeb128(functypes.size()), functypesBytes);
		writeFunctionTypes(functypes, functypesBytes);

		write((byte) SectionId.Type.ordinal(), os);
		write(encodeI32ToLeb128(functypesBytes.size()), os);
		os.write(functypesBytes.toByteArray());
	}

	public void writeFuncSection(List<FuncType> funcTypes, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream funcIdsBytes = new ByteArrayOutputStream();

		write(encodeI32ToLeb128(funcTypes.size()), funcIdsBytes);
		for (FuncType funcType : funcTypes) {
			write((byte) funcTypes.indexOf(funcType), funcIdsBytes);
		}

		write((byte) SectionId.Function.ordinal(), os);
		write(encodeI32ToLeb128(funcIdsBytes.size()), os);
		os.write(funcIdsBytes.toByteArray());
	}

	public void writeMemSection(ByteArrayOutputStream os) throws IOException {
		write((byte) SectionId.Memory.ordinal(), os);
		write(encodeI32ToLeb128(3), os); // Section Size
		write(encodeI32ToLeb128(1), os); // Num Memories
		write(encodeI32ToLeb128(0), os); // limits flags
		write(encodeI32ToLeb128(0), os); // limits min / initial

	}
	public void writeGlobalSection(List<WasmValueType> globals, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream globalsBytes = new ByteArrayOutputStream();
		write(encodeI32ToLeb128(globals.size()), globalsBytes); // Anz Globals
		for (WasmValueType wasmValueType : globals) {
			write((byte)wasmValueType.code, globalsBytes);
			write((byte)1, globalsBytes); // mutable
			Instructions.addI32Const(0, globalsBytes);
			Instructions.addElse(globalsBytes);
		}
		write((byte)SectionId.Global.ordinal(), os);
		write(encodeI32ToLeb128(globalsBytes.size()), os);
		os.write(globalsBytes.toByteArray());
	}
	public void writeCodeSection(List<Func> funcs, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream funcBodiesBytes = new ByteArrayOutputStream();
		// Anzahl der Funktionen
		write(encodeI32ToLeb128(funcs.size()), funcBodiesBytes);
		for (Func func : funcs) {
			writeFuncBody(func, funcBodiesBytes);
		}

		write((byte) SectionId.Code.ordinal(), os);
		// Größe der Code-Section in Byte
		write(encodeI32ToLeb128(funcBodiesBytes.size()), os);
		os.write(funcBodiesBytes.toByteArray());

	}

	public void writeFuncBody(Func f, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream funcBodyBytes = new ByteArrayOutputStream();
		writeFuncLocals(f.getLocals(), funcBodyBytes);
		funcBodyBytes.write(f.getBody().toByteArray());

		// Größe des Bodies in Byte mit local decl und instructions
		write(encodeI32ToLeb128(funcBodyBytes.size()), os);
		os.write(funcBodyBytes.toByteArray());
	}

	public void writeFuncLocals(List<WasmValueType> locals, ByteArrayOutputStream os) throws IOException {
		if (locals.isEmpty()) {
			write(encodeI32ToLeb128(0), os);
		} else if (locals.size() == 1) {
			write(encodeI32ToLeb128(1), os); // Anzahl Deklarationen
			write(encodeI32ToLeb128(1), os); // Anzahl Typ
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
					write(encodeI32ToLeb128(typeCount), declsBytes);
					write((byte) lastType.code, declsBytes);
					typeCount = 1;
					declCount++;
					lastType = wasmValueType;
				}
			}
			if (typeCount > 1) {
				write(encodeI32ToLeb128(typeCount), declsBytes);
				write((byte) lastType.code, declsBytes);
				declCount++;
			}
			write(encodeI32ToLeb128(declCount), os);
			os.write(declsBytes.toByteArray());
		}
	}

	public void writeBinaryMagic(ByteArrayOutputStream os) throws IOException {
		byte[] wasmBinaryMagic = { 0x0, 'a', 's', 'm' };
		os.write(wasmBinaryMagic);
	}

	public void writeBinaryVersion(ByteArrayOutputStream os) throws IOException {
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
