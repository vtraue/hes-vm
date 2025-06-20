package wasm_builder;

public class BinOp extends Instruction{
    BinOp(InstructionOpCode op) {
        super(op);
    }

    @Override
    public void validate(Validator validator) {

    }

    @Override
    public void writeBytecode(BytecodeWriter bw) {

    }
}
