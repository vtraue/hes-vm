import java.io.ByteArrayOutputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.List;
import java.util.Arrays;
import wasm_builder.*;

class TestMain {

	public static void main(String[] args) {
		try {
//			createTestfile1Simple();
			createTestfile1();

//			int i = 424242;
//			ArrayList<Integer> result = WasmBuilder.encodeU32ToLeb128(i);
//			HexFormat hex = HexFormat.ofDelimiter(", ").withPrefix("#");
//			ByteArrayOutputStream leb = new ByteArrayOutputStream();
//			for (Integer e : result) {
//				byte[] byteId = { (byte) e.intValue() };
//				leb.write(byteId);
//			}
//			System.out.println(leb);
//			System.out.println( hex.formatHex(leb.toByteArray()));



			// Unterschiedliche Arten FuncTypes zu initialisieren
			// Liste mit Parametertypen und Reulttypen erstellen
			ArrayList<WasmValueType> params = new ArrayList<>();
			params.add(WasmValueType.i32);
			params.add(WasmValueType.i32);
			ArrayList<WasmValueType> results = new ArrayList<>();
			results.add(WasmValueType.i32);
			// Funktionstyp mit Parametern und Results erstellen
			FuncType funcType = new FuncType(params, results);
			// Listen mit params und results direkt im Konstruktoraufruf erstellen
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

			// testfile1_simple.wat sollte funktionieren (simplifiziert ohne import, call
			// und start)

			// System.out.println("out (ByteCodeBuilder): " +
			// bbuilder.getWasmBuilder().getAsHexString());

		} catch (IOException e) {
			System.err.println(e.getMessage());
		}

	}

	static void createTestfile1() throws IOException {
		BytecodeBuilder bb = new BytecodeBuilder();
		FuncType emptyFuncType = new FuncType();
		Func main = bb.createFunction(emptyFuncType);
		List<Func> funcs = new ArrayList<>();
		funcs.add(main);
		// imports testen

		// globals testen
		List<GlobalType> globals = List.of(new GlobalType(WasmValueType.i32, true), new GlobalType(WasmValueType.i32, true));
		bb.setGlobals(globals);
		bb.build(funcs);

		// zusammengebautes Byte-Array in Datei schreiben
		FileOutputStream out = new FileOutputStream("./out/testfile1.wasm");
		out.write(bb.getWasmBuilder().getByteArray());
		out.close();
	}

	static void createTestfile1Simple() throws IOException {
		/*
		 * SEHR simple Testfile
		 * mit drei Funktionen:
		 * main mit 0 Parametern und 1 i32 lokalen Variable
		 * try1 und try2 mit jeweils 1 i32 Parameter und 2 i32 lokalen Variablen
		 * Logisch ergibt der Code keinen Sinn und dient nur zur Überprüfung
		 * des erstellten Bytecodes
		 */
		BytecodeBuilder bbuilder = new BytecodeBuilder();

		// Funktionstypen mit Parametern und Results erstellen
		FuncType funcType3 = new FuncType(
				new ArrayList<WasmValueType>(
						Arrays.asList(WasmValueType.i32)),
				new ArrayList<WasmValueType>());

		FuncType emptyFuncType = new FuncType();

		// Listen mit locals der Funktionen erstellen
		List<WasmValueType> mainLocals = (List.of(WasmValueType.i32));
		List<WasmValueType> try1Locals = (List.of(WasmValueType.i32, WasmValueType.i32));

		// Funktionen erstellen und Instructions hinzufügen
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

		// ArrayList mit allen Funktionen
		ArrayList<Func> funcs = new ArrayList<>();
		funcs.add(main);
		funcs.add(try1);
		funcs.add(try2);

		// build aufrufen mit der ArrayList von Funktionen
		bbuilder.build(funcs);

		// zusammengebautes Byte-Array in Datei schreiben
		FileOutputStream out = new FileOutputStream("./out/testfile1_simple.wasm");
		out.write(bbuilder.getWasmBuilder().getByteArray());
		out.close();
	}
}
