
public enum WasmTypeOpCode {
	i32((short) 0x7F),
	i64((short) 0x7E),
	f32((short) 0x7D),
	f64((short) 0x7C),
	;

	private WasmTypeOpCode(short code) {

	}
}
