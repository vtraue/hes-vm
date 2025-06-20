package wasm_builder;

import java.util.List;

class Instruction implements BytecodeWriteable {
    private InstructionOpCode opcode;
    private List<InstructionParam> params;

    InstructionOpCode getOpcode() {
        return opcode;
    }

    Instruction(InstructionOpCode op) {
        this.opcode = op;
    }

    void setOpcode(InstructionOpCode opcode) {
        this.opcode = opcode;
    }

    void validate (Validator validator) {
        //TODO
    };

    @Override
    public void writeBytecode(BytecodeWriter bw) {
        //TODO
    }

}