import java.io.FileOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;

import wasmBuilder.*;

class Main {

	public static void main(String[] args) {
		try {

			// Liste mit Parametertypen und Reulttypen erstellen
			ArrayList<WasmValueType> params = new ArrayList<>();
			params.add(WasmValueType.i32);
			params.add(WasmValueType.i32);
			ArrayList<WasmValueType> results = new ArrayList<>();
			results.add(WasmValueType.i32);
			// Funktionstyp mit Parametern und Results erstellen
			FuncType funcType = new FuncType(params, results);
			FuncType funcType2 = new FuncType(
					new ArrayList<WasmValueType>() {
						{
							add(WasmValueType.i32);
							add(WasmValueType.i32);
						}
					},
					new ArrayList<WasmValueType>() {
						{
							add(WasmValueType.i32);
						}
					});
			FuncType funcType3 = new FuncType(
					new ArrayList<WasmValueType>(
							Arrays.asList(WasmValueType.i32, WasmValueType.i32)),
					new ArrayList<WasmValueType>(
							Arrays.asList(WasmValueType.i32)));

			BytecodeBuilder bbuilder = new BytecodeBuilder();
			ArrayList<Func> funcs = new ArrayList<>();

			// Funktion erstellen und Instructions hinzufügen
			Func func1 = bbuilder.createFunction(funcType);
			func1.emitLocalGet(0);
			func1.emitLocalGet(1);
			func1.emitAdd();
			func1.emitEnd();

			// Funktion(en) der ArrayList mit Funktionen hinzufügen
			funcs.add(func1);

			// build aufrufen mit der ArrayList von Funktionen
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
