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
			ArrayList<Integer> params = new ArrayList<>();
			params.add((int) WasmValueType.i32.code);
			ArrayList<Integer> results = new ArrayList<>();
			results.add((int) WasmValueType.i32.code);
			ArrayList<wasmBuilder.FuncType> functypes = new ArrayList<>();
			functypes.add(new FuncType(params, results));

			BytecodeBuilder bbuilder = new BytecodeBuilder(functypes);
			bbuilder.emitLe();
			System.out.println("out (ByteCodeBuilder): " + bbuilder.getWasmBuilder().getAsHexString());

			FileOutputStream out = new FileOutputStream("tesfile.wasm");
			out.write(bbuilder.getWasmBuilder().getByteArray());
			out.close();
		} catch (IOException e) {
			System.err.println(e.getMessage());
		}

	}
}
