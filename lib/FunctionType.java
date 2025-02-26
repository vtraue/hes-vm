import java.util.ArrayList;
import wasmBuilder.*;

public class FunctionType {
	ArrayList<WasmValueType> locals;
	ArrayList<WasmValueType> results;
	ArrayList<WasmValueType> params;

	public FunctionType(ArrayList<WasmValueType> params, ArrayList<WasmValueType> results,
			ArrayList<WasmValueType> locals) {
		this.params = params;
		this.results = results;
		this.locals = locals;
	}
}
