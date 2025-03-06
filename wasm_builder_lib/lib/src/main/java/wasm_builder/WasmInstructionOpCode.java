
package wasm_builder;

public enum WasmInstructionOpCode {
	UNREACHABLE((short) 0x00),
	NOP((short) 0x01),
	END((short) 0x0b),
	BLOCK((short) 0x02),
	LOOP((short) 0x03),
	IF((short) 0x04),
	BR((short) 0x0C),
	BR_IF((short) 0x0D),
	RETURN((short) 0x0F),
	CALL((short) 0x10),
	DROP((short) 0x1A),
	LOCAL_GET((short) 0x20),
	LOCAL_SET((short) 0x21),
	LOCAL_TEE((short) 0x22),
	GLOBAL_GET((short) 0x23),
	GLOBAL_SET((short) 0x24),
	I32_LOAD((short) 0x28),
	I32_STORE((short) 0x36),
	I32_CONST((short) 0x41),
	I32_EQZ((short) 0x45),
	I32_EQ((short) 0x46),
	I32_NE((short) 0x47),
	I32_LT_S((short) 0x48),
	I32_GT_S((short) 0x4A),
	I32_LE_S((short) 0x4C),
	I32_GE_S((short) 0x4E),
	I32_ADD((short) 0x6A),
	I32_SUB((short) 0x6B),
	I32_MUL((short) 0x6C),
	I32_DIV_S((short) 0x6D),
	I32_REM_S((short) 0x6F),
	I32_AND((short) 0x71),
	I32_OR((short) 0x72),
	I32_XOR((short) 0x73),
	;

	public short code;

	private WasmInstructionOpCode(short code) {
		this.code = code;
	}
}
