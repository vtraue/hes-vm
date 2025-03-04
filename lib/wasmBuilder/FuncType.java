package wasmBuilder;

import java.util.ArrayList;
import java.util.List;

public class FuncType {

	private ArrayList<Integer> params;
	private ArrayList<Integer> results;

	public FuncType(List<WasmValueType> params, List<WasmValueType> results) {
		this();
		for (WasmValueType wasmValueType : params) {
			this.params.add((int) wasmValueType.code);
		}
		for (WasmValueType wasmValueType : results) {
			this.results.add((int) wasmValueType.code);
		}
	}

	public FuncType() {
		this.params = new ArrayList<>();
		this.results = new ArrayList<>();
	}

	public ArrayList<Integer> getParams() {
		return this.params;
	}

	public ArrayList<Integer> getResults() {
		return this.results;
	}
}
