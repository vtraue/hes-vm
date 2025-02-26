import java.io.IOException;
import java.util.ArrayList;

import wasmBuilder.*;

class Main {

	public static void main(String[] args) {
		try {
			ArrayList<FunctionType> functypes = new ArrayList<>();
			BytecodeBuilder bbuilder = new BytecodeBuilder(functypes);
			WasmBuilder builder = new WasmBuilder();
			builder.addLocalSet(1);
			bbuilder.emitLe();
			System.out.println("out (WasmBuilder): " + builder.getAsHexString());
			System.out.println("out (ByteCodeBuilder): " + bbuilder.getWasmBuilder().getAsHexString());
		} catch (IOException e) {
			System.err.println(e.getMessage());
		}

	}
}
