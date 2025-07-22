package wasm_builder;

public interface InstructionParam extends BytecodeWriteable{}

record I32(int arg) implements InstructionParam{
    public void writeBytecode(BytecodeWriter bw) {
        //TODO
    }
};
record I64(long arg) implements InstructionParam{
    public void writeBytecode(BytecodeWriter bw) {

        //TODO
    }
};
record F32(float arg) implements InstructionParam{
    public void writeBytecode(BytecodeWriter bw) {

        //TODO
    }
};
record F64(double arg) implements InstructionParam{
    public void writeBytecode(BytecodeWriter bw){

        //TODO
    }
};
record MemArg(int align, int offset) implements InstructionParam{
    public void writeBytecode(BytecodeWriter bw){

        //TODO
    }
};