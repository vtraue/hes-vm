package wasmBuilder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.List;

/*
*
* class FunctionType {
* 	List<Type> params;
* 	List<Type> results;
* 	List<Variable> locals;
*
* }
* class Variable {
* 	private int id;
* 	String name;
* 	public Variable(String name) {
* 		this.name = name;
*		this.id = var_id;
*		var_id ++;
* 	}
*
* }
* class CodeBuilder {
* 	private int var_id = 0;
* 	public CodeBuilder(List<FunctionType> functiontypes) (?)
* 	private List<FunctionTypes> functiontypes;
* 	public void setFunctionTypes(List<FunctionType> functiontypes)
* 	public void emitAdd
* 	public void enterFunction(String name, FunctionType f)
* 	public void callFunction(String name)
* 	public void getLocal(String name, FunctionType funtype) {
*
* 	}
* }
* CodeBuilder builder = new CodeBuilder()
* ...
* Function func = builder.enterFunction();
* builder.emitStore(...);
* builder.emitLoad(...);
* builder.leaveFunction();
* builder.callFunction(func);
* ...
* builder.build()
* 
* builder.enterFunction(returntype)
* builder.leaveFunction
* builder.callFunction(id)
*
* void test_fn(int a, int b) {
	int c = 5;
	{ 
		int c = 4;
		{ 
			int c = 9;
			printf(c)
		}
	}
	*
* }
* int a;
* int b;
* a = 12;
* b = 30;
*
* int c = a + b; 
*
* builder.emitLocalSet(a_id, 12);
* builder.emitLocalSet(b_id, 30);
*
* builder.emitLocalGet(a_id)
* builder.emitLocalGet(b_id)
* builder.emitAdd();
* int c_id = builder.declareLocal();
* builder.emitLocalSet(c_id);
*
*/

public class WasmBuilder {

	private ByteArrayOutputStream out = new ByteArrayOutputStream();

	public String getAsHexString() {
		HexFormat hex = HexFormat.of();
		return hex.formatHex(out.toByteArray());
	}

	public WasmBuilder() throws IOException {
		addBinaryMagic();
		addBinaryVersion();
	}

	private void write(byte code) throws IOException {
		byte[] b = { code };
		out.write(b);
	}

	private void write(ArrayList<Integer> al) throws IOException {
		for (Integer e : al) {
			byte[] byteId = { (byte) e.intValue() };
			out.write(byteId);
		}
	}

	public void addLocalSet(int id) throws IOException {
		write((byte) WasmInstructionOpCode.LOCAL_SET.code);
		write(encodeI32ToLeb128(id));
	}

	public void addLocalGet(int id) throws IOException {
		write((byte) WasmInstructionOpCode.LOCAL_GET.code);
		write(encodeI32ToLeb128(id));
	}

	public void addGlobalSet(int id) throws IOException {
		write((byte) WasmInstructionOpCode.GLOBAL_SET.code);
		write(encodeI32ToLeb128(id));
	}

	public void addGlobalGet(int id) throws IOException {
		write((byte) WasmInstructionOpCode.GLOBAL_GET.code);
		write(encodeI32ToLeb128(id));
	}

	public void addBinOp(WasmInstructionOpCode binop) throws IOException {
		write((byte) binop.code);
	}

	public void addEnterTypeSection(List<FunctionType> functypes) {
		// byte typeId = Byte.toUnsignedInt;
		byte typeId = (byte) (SectionId.Type.ordinal());
		ByteArrayOutputStream functypesBytes = new ByteArrayOutputStream();
		for (FunctionType f : functypes) {

		}
		byte[] b = { typeId };
	}

	public void addBinaryMagic() throws IOException {
		byte[] wasmBinaryMagic = { 0x0, 'a', 's', 'm' };
		out.write(wasmBinaryMagic);
	}

	public void addBinaryVersion() throws IOException {
		byte[] wasmBinaryVersion = { 0x01, 0x00, 0x00, 0x00 };
		out.write(wasmBinaryVersion);
	}

	public static ArrayList<Integer> encodeI32ToLeb128(int value) {
		value |= 0;
		ArrayList<Integer> result = new ArrayList<Integer>();
		while (true) {

			int byte_ = value & 0x7f;
			value >>= 7;
			if ((value == 0 && (byte_ & 0x40) == 0) || (value == -1 && (byte_ & 0x40) != 0)) {
				result.add(byte_);
				return result;
			}
			result.add(byte_ | 0x80);
		}

	}

}
