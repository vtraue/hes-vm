import java.io.IOException;
import java.util.ArrayList;
import java.util.HexFormat;

import wasmBuilder.*;

class Main {

	public static void main(String[] args) {
		HexFormat hex = HexFormat.of();
		WasmBuilder builder = new WasmBuilder();
		int leb128Test = 123456;
		ArrayList<Integer> leb128Out = new ArrayList<Integer>();
		leb128Out = WasmBuilder.encodeI32ToLeb128(leb128Test);

		System.out.println(leb128Out.toString());
		try {

			builder.addLocalSet(123456);
		} catch (IOException e) {
			System.err.println(e.getMessage());
		}
		byte[] out = builder.out.toByteArray();
		System.out.println("out: " + hex.formatHex(out));

	}
}
