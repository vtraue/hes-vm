import java.io.IOException;
import java.util.ArrayList;

import wasmBuilder.*;

class Main {

	public static void main(String[] args) {
		try {

			WasmBuilder builder = new WasmBuilder();
			builder.addLocalSet(1);
			System.out.println("out: " + builder.getAsHexString());
		} catch (IOException e) {
			System.err.println(e.getMessage());
		}

	}
}
