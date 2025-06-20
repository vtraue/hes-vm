
package wasm_builder;

public enum ValueType implements Type, InstructionParam{
	i32((short) 0x7F),
	i64((short) 0x7E),
	f32((short) 0x7D),
	f64((short) 0x7C),
	v128((short) 0x7B),
	funcref((short) 0x70),
	externref((short) 0x67),
	;

	public short code;

	private ValueType(short code) {
		this.code = code;
	}

	public void writeBytecode(BytecodeWriter bw) {
		// TODO
	}
}
