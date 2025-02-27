package wasmBuilder;

import java.util.ArrayList;

public class FuncType {

	private ArrayList<Integer> params = new ArrayList<>();
	private ArrayList<Integer> results = new ArrayList<>();

	public FuncType(ArrayList<Integer> params, ArrayList<Integer> results) {
		this.params = params;
		this.results = results;
	}

	public ArrayList<Integer> getParams() {
		return this.params;
	}

	public ArrayList<Integer> getResults() {
		return this.results;
	}
}
