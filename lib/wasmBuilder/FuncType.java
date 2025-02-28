package wasmBuilder;

import java.util.ArrayList;

public class FuncType {

	private ArrayList<Integer> params = new ArrayList<>();
	private ArrayList<Integer> results = new ArrayList<>();

	public FuncType(ArrayList<WasmValueType> params, ArrayList<WasmValueType> results) {
		for (WasmValueType wasmValueType : params) {
			this.params.add((int) wasmValueType.code);
		}
		for (WasmValueType wasmValueType : results) {
			this.results.add((int) wasmValueType.code);
		}
	}

	public ArrayList<Integer> getParams() {
		return this.params;
	}

	public ArrayList<Integer> getResults() {
		return this.results;
	}
}
