package wasm_builder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;

public record Instructions() {

	public static void addCall(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.CALL.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(id), os);
	}

	public static void addEnd(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.END.code, os);
	}

	public static void addLocalSet(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.LOCAL_SET.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(id), os);
	}

	public static void addLocalGet(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.LOCAL_GET.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(id), os);
	}

	public static void addLocalTee(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.LOCAL_TEE.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(id), os);
	}

	public static void addGlobalSet(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.GLOBAL_SET.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(id), os);
	}

	public static void addGlobalGet(int id, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.GLOBAL_GET.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(id), os);
	}

	public static void addBinOp(InstructionOpCode binop, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) binop.code, os);
	}

	public static void addI32Load(int align, int offset,  ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.I32_LOAD.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(align), os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(offset), os);
	}

	public static void addI32Store(int align, int offset, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.I32_STORE.code, os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(align), os);
		WasmBuilder.write(WasmBuilder.encodeU32ToLeb128(offset), os);
	}

	public static void addI32Const(int n, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.I32_CONST.code, os);
		WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(n), os);
	}
	// TODO: encoding i64, f32, f64
//	public static void addI64Const(int n, ByteArrayOutputStream os) throws IOException {
//		WasmBuilder.write((byte) InstructionOpCode.I64_CONST.code, os);
//		WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(n), os);
//	}
//	public static void addF32Const(int n, ByteArrayOutputStream os) throws IOException {
//		WasmBuilder.write((byte) InstructionOpCode.F32_CONST.code, os);
//		WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(n), os);
//	}
//	public static void addF64Const(int n, ByteArrayOutputStream os) throws IOException {
//		WasmBuilder.write((byte) InstructionOpCode.F64_CONST.code, os);
//		WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(n), os);
//	}

	public static void addNop(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.NOP.code, os);
	}

	public static void addReturn(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.RETURN.code, os);
	}

	public static void addUnreachable(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.UNREACHABLE.code, os);
	}

	// control
	public static void addIf(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.IF.code, os);

	}

	public static void addElse(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.ELSE.code, os);
	}

	public static void addBlock(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.BLOCK.code, os);
	}

	public static void addLoop(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) InstructionOpCode.LOOP.code, os);
	}

	public static void addBlockType(ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) 0x40, os);
	}

	public static void addBlockType(ValueType valtype, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write((byte) valtype.code, os);
	}

	public static void addBlockType(int typeidx, ByteArrayOutputStream os) throws IOException {
		WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(typeidx), os);
	}
  public static void addBr(int jumpIndex, ByteArrayOutputStream os) throws IOException {
    WasmBuilder.write((byte) InstructionOpCode.BR.code, os);
    WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(jumpIndex), os);
  }

  public static void addBrIf(int jumpIndex, ByteArrayOutputStream os) throws IOException {
    WasmBuilder.write((byte) InstructionOpCode.BR_IF.code, os);
    WasmBuilder.write(WasmBuilder.encodeI32ToLeb128(jumpIndex), os);
  }
}
