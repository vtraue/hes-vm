package wasm_builder;

import java.util.List;

public class CtrlFrame {
    private InstructionOpCode opcode; // OpCode mit dem wir in den Block gegangen sind
    private List<ValueType> startTypes; // "Parameter"
    private List<ValueType> endTypes; // "Returntypes"
    private int height; // Höhe des valStack zum Start des Blocks
    private Boolean unreachable; // Marker für unreachable (nach unconditional "br" oder "unreachable")

    public void setOpcode(InstructionOpCode opcode) {
        this.opcode = opcode;
    }

    public InstructionOpCode getOpcode() {
        return opcode;
    }

    public List<ValueType> getStartTypes() {
        return startTypes;
    }

    public void setStartTypes(List<ValueType> startTypes) {
        this.startTypes = startTypes;
    }

    public List<ValueType> getEndTypes() {
        return endTypes;
    }

    public void setEndTypes(List<ValueType> endTypes) {
        this.endTypes = endTypes;
    }

    public int getHeight() {
        return height;
    }

    public void setHeight(int height) {
        this.height = height;
    }

    public Boolean getUnreachable() {
        return unreachable;
    }

    public void setUnreachable(Boolean unreachable) {
        this.unreachable = unreachable;
    }
}
