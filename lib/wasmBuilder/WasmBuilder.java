package wasmBuilder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.List;

public class WasmBuilder {

	private ByteArrayOutputStream out = new ByteArrayOutputStream();

	public WasmBuilder(ArrayList<FuncType> functypes) throws IOException {
		addBinaryMagic();
		addBinaryVersion();
		addEnterTypeSection(functypes);
	}

	public byte[] getByteArray() {
		return out.toByteArray();
	}

	public String getAsHexString() {
		HexFormat hex = HexFormat.of();
		return hex.formatHex(out.toByteArray());
	}

	private void write(byte code, ByteArrayOutputStream os) throws IOException {
		byte[] b = { code };
		os.write(b);
	}

	private void write(ArrayList<Integer> al, ByteArrayOutputStream os) throws IOException {
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

	public void addLocalSet(int id) throws IOException {
		write((byte) WasmInstructionOpCode.LOCAL_SET.code, out);
		write(encodeI32ToLeb128(id), out);
	}

	public void addLocalGet(int id) throws IOException {
		write((byte) WasmInstructionOpCode.LOCAL_GET.code, out);
		write(encodeI32ToLeb128(id), out);
	}

	public void addGlobalSet(int id) throws IOException {
		write((byte) WasmInstructionOpCode.GLOBAL_SET.code, out);
		write(encodeI32ToLeb128(id), out);
	}

	public void addGlobalGet(int id) throws IOException {
		write((byte) WasmInstructionOpCode.GLOBAL_GET.code, out);
		write(encodeI32ToLeb128(id), out);
	}

	public void addBinOp(WasmInstructionOpCode binop) throws IOException {
		write((byte) binop.code, out);
	}

	public void addEnterTypeSection(ArrayList<FuncType> functypes) throws IOException {
		ByteArrayOutputStream functypesBytes = new ByteArrayOutputStream();
		write(encodeI32ToLeb128(functypes.size()), functypesBytes);
		writeFunctionTypes(functypes, functypesBytes);
		write((byte) SectionId.Type.ordinal(), out);
		write((byte) functypesBytes.size(), out);
		out.write(functypesBytes.toByteArray());
	}

	public void addBinaryMagic() throws IOException {
		byte[] wasmBinaryMagic = { 0x0, 'a', 's', 'm' };
		out.write(wasmBinaryMagic);
	}

	public void addBinaryVersion() throws IOException {
		byte[] wasmBinaryVersion = { 0x01, 0x00, 0x00, 0x00 };
		out.write(wasmBinaryVersion);
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
