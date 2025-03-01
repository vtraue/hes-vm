import java.io.FileOutputStream;
import java.io.IOException;
import java.util.ArrayList;

import wasmBuilder.*;

class Main {

	public static void main(String[] args) {
		try {
			ArrayList<WasmValueType> params = new ArrayList<>();
			params.add(WasmValueType.i32);
			ArrayList<WasmValueType> results = new ArrayList<>();
			// results.add(WasmValueType.i32);
			FuncType funcType = new FuncType(params, results);

			BytecodeBuilder bbuilder = new BytecodeBuilder();
			ArrayList<Func> funcs = new ArrayList<>();

			funcs.add(bbuilder.createFunction(funcType));
			bbuilder.build(funcs);

			System.out.println("out (ByteCodeBuilder): " + bbuilder.getWasmBuilder().getAsHexString());

			FileOutputStream out = new FileOutputStream("testfile.wasm");
			out.write(bbuilder.getWasmBuilder().getByteArray());
			out.close();
		} catch (IOException e) {
			System.err.println(e.getMessage());
		}

	}
}
