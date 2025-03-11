package wasm_builder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;

public record Instructions() {

	public static void addCall(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.CALL.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(id), os);
	}

	public static void addEnd(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.END.code, os);
	}

	public static void addLocalSet(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.LOCAL_SET.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(id), os);
	}

	public static void addLocalGet(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.LOCAL_GET.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(id), os);
	}

	public static void addLocalTee(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.LOCAL_TEE.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(id), os);
	}

	public static void addGlobalSet(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.GLOBAL_SET.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(id), os);
	}

	public static void addGlobalGet(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.GLOBAL_GET.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(id), os);
	}

	public static void addBinOp(WasmInstructionOpCode binop, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) binop.code, os);
	}

	public static void addI32Load(int align, int offset,  ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.I32_LOAD.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(align), os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(offset), os);
	}

	public static void addI32Store(int align, int offset, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.I32_STORE.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(align), os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(offset), os);
	}

	public static void addI32Const(int n, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.I32_CONST.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(n), os);
	}

	public static void addNop(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.NOP.code, os);
	}

	public static void addReturn(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.RETURN.code, os);
	}

	public static void addUnreachable(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.UNREACHABLE.code, os);
	}

	// control
	public static void addIf(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.IF.code, os);

	}

	public static void addElse(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.ELSE.code, os);
	}

	public static void addBlock(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.BLOCK.code, os);
	}

	public static void addLoop(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) WasmInstructionOpCode.LOOP.code, os);
	}

	public static void addBlockType(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) 0x40, os);
	}

	public static void addBlockType(WasmValueType valtype, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) valtype.code, os);
	}

	public static void addBlockType(int typeidx, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(typeidx), os);
	}
}
