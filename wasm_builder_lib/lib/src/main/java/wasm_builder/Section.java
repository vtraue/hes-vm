package wasm_builder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.List;

abstract class Section {
    SectionId id;
    abstract void write(BytecodeWriter bw) throws IOException;
    abstract void validate();
}

class CustomSection extends Section {
   final SectionId id = SectionId.Custom;
    @Override
    void write(BytecodeWriter bw) {

    }

    @Override
    void validate(){
       //TODO
   }
}

class TypeSection extends Section {
    final SectionId id = SectionId.Type;
    List<FuncType> funcTypeList;

    TypeSection(List<FuncType> funcTypeList) {
        this.funcTypeList = funcTypeList;
    }

    @Override
    void write(BytecodeWriter bw) throws IOException {
        bw.writeU32(id.ordinal());
        bw.writeWithSize(functionTypesOS(funcTypeList));
    }

    @Override
    void validate() {
        //TODO
    }

    private ByteArrayOutputStream functionTypesOS(List<FuncType> funcTypes) throws IOException {
        BytecodeWriter ftbw = new BytecodeWriter();
        ftbw.writeU32(funcTypes.size());
        for (FuncType f : funcTypes) {
            ftbw.writeByte((byte) 0x60);
            ftbw.writeU32(f.getParams().size());
            ftbw.writeBytes(f.getParams());
            ftbw.writeU32(f.getResults().size());
            ftbw.writeBytes(f.getResults());
        }
        return ftbw.getOutputStream();
    }
}

class ImportSection extends Section {
    final SectionId id = SectionId.Import;

    @Override
    void write(BytecodeWriter bw) {
        //TODO

    }

    @Override
    void validate() {
        //TODO

    }
}

class FunctionSection extends Section {
    final SectionId id = SectionId.Function;
    @Override
    void write(BytecodeWriter bw) {
        //TODO

    }

    @Override
    void validate() {
        //TODO

    }

}

class TableSection extends Section {
    final SectionId id = SectionId.Table;

    @Override
    void write(BytecodeWriter bw) {
        //TODO

    }

    @Override
    void validate() {
        //TODO

    }
}

class MemorySection extends Section {
    final SectionId id = SectionId.Memory;

    @Override
    void write(BytecodeWriter bw) {
        //TODO

    }

    @Override
    void validate() {
        //TODO

    }
}

class GlobalSection extends Section {
    final SectionId id = SectionId.Global;

    @Override
    void write(BytecodeWriter bw) {
        //TODO

    }

    @Override
    void validate() {
        //TODO

    }
}

class ExportSection extends Section {
    final SectionId id = SectionId.Export;

    @Override
    void write(BytecodeWriter bw) {
        //TODO

    }

    @Override
    void validate() {
        //TODO

    }
}

class StartSection extends Section {
    final SectionId id = SectionId.Start;

    @Override
    void write(BytecodeWriter bw) {
        //TODO

    }

    @Override
    void validate() {
        //TODO

    }
}

class ElementSection extends Section {
    final SectionId id = SectionId.Element;

    @Override
    void write(BytecodeWriter bw) {
        //TODO

    }

    @Override
    void validate() {
        //TODO

    }
}

class CodeSection extends Section {
    final SectionId id = SectionId.Code;

    @Override
    void write(BytecodeWriter bw) {
        //TODO

    }

    @Override
    void validate() {
        //TODO

    }
}

class DataSection extends Section {
    final SectionId id = SectionId.Data;

    @Override
    void write(BytecodeWriter bw) {
        //TODO

    }

    @Override
    void validate() {
        //TODO

    }
}

class DataCountSection extends Section {
    final SectionId id = SectionId.DataCount;

    @Override
    void write(BytecodeWriter bw) {
        //TODO

    }

    @Override
    void validate() {
        //TODO

    }
}
