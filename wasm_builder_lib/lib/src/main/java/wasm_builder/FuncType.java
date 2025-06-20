package wasm_builder;

import java.util.ArrayList;
import java.util.List;

public final class FuncType implements Importable {

	private ArrayList<Integer> params;
	private ArrayList<Integer> results;
	private String name = "";

	public FuncType(List<ValueType> params, List<ValueType> results) {
		this();
		for (ValueType valueType : params) {
			this.params.add((int) valueType.code);
		}
		for (ValueType valueType : results) {
			this.results.add((int) valueType.code);
		}
	}

	public FuncType(List<ValueType> params, List<ValueType> results, String name) {
		this(params, results);
		this.name = name;
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

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }
}
