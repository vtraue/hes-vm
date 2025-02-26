import java.util.ArrayList;
import wasmBuilder.*;

public class BytecodeBuilder {
	private ArrayList<FunctionType> functiontypes = new ArrayList<FunctionType>();
	private WasmBuilder builder = new WasmBuilder();

	public BytecodeBuilder(ArrayList<FunctionType> functypes) {
		this.functiontypes = functypes;
	}

	public void emitLocalSet(int id) {
		builder.addLocalSet(id);
	}
}
