package wasmBuilder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HexFormat;

public class WasmBuilder {

	private ByteArrayOutputStream out = new ByteArrayOutputStream();
	private ArrayList<FuncType> funcTypes = new ArrayList<>();
	private ArrayList<Func> funcs = new ArrayList<>();
	private int currentFunction;

	public void build(ArrayList<Func> funcs) throws IOException {

		writeBinaryMagic(out);
		writeBinaryVersion(out);
		if (!funcTypes.isEmpty()) {
			writeTypeSection(funcTypes, out);
			writeFuncSection(funcTypes, out);
		}
	}

	public Func addFunc(FuncType funcType) {
		this.funcTypes.add(funcType);
		return new Func(funcType);

	}

	public Func getCurrentFunction() {
		return this.funcs.get(currentFunction);
	}

	public byte[] getByteArray() {
		return out.toByteArray();
	}

	public String getAsHexString() {
		HexFormat hex = HexFormat.of();
		return hex.formatHex(out.toByteArray());
	}

	private static void write(byte code, ByteArrayOutputStream os) throws IOException {
		byte[] b = { code };
		os.write(b);
	}

	private static void write(ArrayList<Integer> al, ByteArrayOutputStream os) throws IOException {
		for (Integer e : al) {
			byte[] byteId = { (byte) e.intValue() };
			os.write(byteId);
		}
	}

	private void writeFunctionTypes(ArrayList<FuncType> functypes, ByteArrayOutputStream os) throws IOException {
		for (FuncType f : functypes) {
			write((byte) 0x60, os);
			write((byte) f.getParams().size(), os);
			write(f.getParams(), os);
			write((byte) f.getResults().size(), os);
			write(f.getResults(), os);
		}
	}

	public static void addLocalSet(int id, ByteArrayOutputStream os) throws IOException {
		write((byte) WasmInstructionOpCode.LOCAL_SET.code, os);
		write(encodeI32ToLeb128(id), os);
	}

	public static void addLocalGet(int id, ByteArrayOutputStream os) throws IOException {
		write((byte) WasmInstructionOpCode.LOCAL_GET.code, os);
		write(encodeI32ToLeb128(id), os);
	}

	public static void addGlobalSet(int id, ByteArrayOutputStream os) throws IOException {
		write((byte) WasmInstructionOpCode.GLOBAL_SET.code, os);
		write(encodeI32ToLeb128(id), os);
	}

	public static void addGlobalGet(int id, ByteArrayOutputStream os) throws IOException {
		write((byte) WasmInstructionOpCode.GLOBAL_GET.code, os);
		write(encodeI32ToLeb128(id), os);
	}

	public static void addBinOp(WasmInstructionOpCode binop, ByteArrayOutputStream os) throws IOException {
		write((byte) binop.code, os);
	}

	public void writeTypeSection(ArrayList<FuncType> functypes, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream functypesBytes = new ByteArrayOutputStream();
		write(encodeI32ToLeb128(functypes.size()), functypesBytes);
		writeFunctionTypes(functypes, functypesBytes);

		write((byte) SectionId.Type.ordinal(), os);
		write((byte) functypesBytes.size(), os);
		os.write(functypesBytes.toByteArray());
	}

	public void writeFuncSection(ArrayList<FuncType> funcTypes, ByteArrayOutputStream os) throws IOException {
		ByteArrayOutputStream funcIdsBytes = new ByteArrayOutputStream();

		write(encodeI32ToLeb128(funcTypes.size()), funcIdsBytes);
		for (FuncType funcType : funcTypes) {
			write((byte) funcTypes.indexOf(funcType), funcIdsBytes);
		}

		write((byte) SectionId.Function.ordinal(), os);
		write((byte) funcIdsBytes.size(), os);
		os.write(funcIdsBytes.toByteArray());
	}

	public void writeCodeSection(ArrayList<Func> funcs, ByteArrayOutputStream os) {

	}

	public void writeBinaryMagic(ByteArrayOutputStream os) throws IOException {
		byte[] wasmBinaryMagic = { 0x0, 'a', 's', 'm' };
		os.write(wasmBinaryMagic);
	}

	public void writeBinaryVersion(ByteArrayOutputStream os) throws IOException {
		byte[] wasmBinaryVersion = { 0x01, 0x00, 0x00, 0x00 };
		os.write(wasmBinaryVersion);
	}

	public static ArrayList<Integer> encodeI32ToLeb128(int value) {
		value |= 0;
		ArrayList<Integer> result = new ArrayList<Integer>();
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
