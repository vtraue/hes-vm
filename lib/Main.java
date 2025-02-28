import java.io.File;
import java.io.FileOutputStream;
import java.io.FileWriter;
import java.io.IOException;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.List;

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

			bbuilder.enterFunction(funcType);
			bbuilder.build();

			System.out.println("out (ByteCodeBuilder): " + bbuilder.getWasmBuilder().getAsHexString());

			FileOutputStream out = new FileOutputStream("testfile.wasm");
			out.write(bbuilder.getWasmBuilder().getByteArray());
			out.close();
		} catch (IOException e) {
			System.err.println(e.getMessage());
		}

	}
}
