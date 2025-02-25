import java.util.ArrayList;

class Main {

	public static void main(String[] args) {

		int leb128Test = 123456;
		ArrayList<Integer> leb128Out = new ArrayList<Integer>();
		leb128Out = WasmBuilder.encodeI32ToLeb128(leb128Test);

		System.out.println(leb128Out.toString());

	}
}
