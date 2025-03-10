package wasm_builder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;

public record Instructions() {

	public static void addCall(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.CALL.code, os);
		WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(id), os);
	}

	public static void addEnd(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.END.code, os);
	}

	public static void addLocalSet(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.LOCAL_SET.code, os);
		WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(id), os);
	}

	public static void addLocalGet(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.LOCAL_GET.code, os);
		WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(id), os);
	}

	public static void addGlobalSet(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.GLOBAL_SET.code, os);
		WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(id), os);
	}

	public static void addGlobalGet(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.GLOBAL_GET.code, os);
		WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(id), os);
	}

	public static void addBinOp(WasmInstructionOpCode binop, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) binop.code, os);
	}
}
