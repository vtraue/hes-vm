import java.io.FileOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
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
							Arrays.asList(WasmValueType.i32)),
					new ArrayList<WasmValueType>());

			FuncType emptyFuncType = new FuncType();
			// testile1.wat sollte funktionieren

			List<WasmValueType> mainLocals = (List.of(WasmValueType.i32));
			List<WasmValueType> try1Locals = (List.of(WasmValueType.i32, WasmValueType.i32));

			BytecodeBuilder bbuilder = new BytecodeBuilder();
			ArrayList<Func> funcs = new ArrayList<>();

			// Funktion erstellen und Instructions hinzufügen
			Func main = bbuilder.createFunction(emptyFuncType, mainLocals);
			main.emitLocalGet(0);
			main.emitLocalSet(0);
			main.emitEnd();

			Func try1 = bbuilder.createFunction(funcType3, try1Locals);
			try1.emitLocalGet(0);
			try1.emitLocalSet(1);
			try1.emitEnd();

			Func try2 = bbuilder.createFunction(funcType3, try1Locals);
			try2.emitEnd();

			// Funktion(en) der ArrayList mit Funktionen hinzufügen
			funcs.add(main);
			funcs.add(try1);
			funcs.add(try2);

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
