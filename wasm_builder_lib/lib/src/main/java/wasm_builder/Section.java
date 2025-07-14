package wasm_builder;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

abstract class Section {
    SectionId id;
    abstract void write(BytecodeWriter bw) throws IOException;
    abstract void validate();
}

class CustomSection extends Section {
   final SectionId id = SectionId.Custom;
    @Override
    void write(BytecodeWriter bw) throws IOException {

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
        bw.writeByte((byte)id.ordinal());
        bw.writeWithSize(functionTypesOS(funcTypeList));
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

    @Override
    void validate() {
        //TODO
    }
}

class ImportSection extends Section {
    final SectionId id = SectionId.Import;
    List<Import> imports;
    List<FuncType> importedFuncTypes;

    ImportSection(List<Import> importList, List<FuncType> importedTypesList) {
        this.imports = importList;
        this.importedFuncTypes = importedTypesList;
    }

    @Override
    void write(BytecodeWriter bw) throws IOException {
        BytecodeWriter importBytes = new BytecodeWriter();
        importBytes.writeU32(imports.size());
        for (Import im : imports) {
            writeImport(im, importBytes);
        }
        bw.writeByte((byte)id.ordinal());
        bw.writeWithSize(importBytes.getOutputStream());
    }

    private void writeImport(Import im, BytecodeWriter bw) throws IOException {
        bw.writeU32(im.getModule().length());
        bw.writeBytes(im.getModule().getBytes(StandardCharsets.UTF_8));
        bw.writeU32(im.getName().length());
        bw.writeBytes(im.getName().getBytes(StandardCharsets.UTF_8));

        switch(im.getDesc()){
            case FuncType f -> {
                bw.writeByte((byte)0x00);
                bw.writeU32(importedFuncTypes.indexOf(f));
            }
            case TableType t -> {
                bw.writeByte((byte)0x01);
                if (t.refExt()){
                    bw.writeByte((byte)0x67); // externref
                } else {
                    bw.writeByte((byte)0x70); // funcref
                }
                bw.writeByte((byte)0x01);
                bw.writeU32(t.min());
                bw.writeU32(t.max());
            }
            case MemType m -> {
                bw.writeByte((byte)0x02);
                bw.writeU32(m.min());
                bw.writeU32(m.max());
            }
            case GlobalType g -> {
                bw.writeByte((byte)0x03);

                bw.writeByte((byte)g.getValtype().code);
                if (g.isMutable()) {
                    bw.writeByte((byte)0x01);
                }else {
                    bw.writeByte((byte)0x00);
                }
            }
        }
    }

    @Override
    void validate() {
        //TODO

    }
}

class FunctionSection extends Section {
    final SectionId id = SectionId.Function;
    List<FuncType> funcTypes;
    int importedFuncsNum;

    FunctionSection(List<FuncType> funcTypeList, int importedFuncsNum) {
        this.funcTypes = funcTypeList;
        this.importedFuncsNum = importedFuncsNum;
    }

    @Override
    void write(BytecodeWriter bw) throws IOException {
        BytecodeWriter funcIds = new BytecodeWriter();
        funcIds.writeU32(funcTypes.size());
        int idx = 0;
        // TODO: NameSection hier füllen? Aber wie?
        for (FuncType f : funcTypes) {
            int funcIdx = idx + importedFuncsNum;
            funcIds.writeByte((byte) funcIdx);
            idx ++;
        }
        bw.writeByte((byte)id.ordinal());
        bw.writeWithSize(funcIds.getOutputStream());
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
    void write(BytecodeWriter bw) throws IOException {
        bw.writeByte((byte) id.ordinal());
        bw.writeU32(3); // Section Size
        bw.writeU32(1); // Num Memories
        bw.writeU32(0); // limits flags
        bw.writeU32(1); // limits min / initial
    }

    @Override
    void validate() {
        //TODO

    }
}

class GlobalSection extends Section {
    final SectionId id = SectionId.Global;
    List<GlobalType> globals;

    GlobalSection(List<GlobalType> globals) {
        this.globals = globals;
    }

    @Override
    void write(BytecodeWriter bw) throws IOException {
        BytecodeWriter globalsBytes = new BytecodeWriter();
        globalsBytes.writeU32(globals.size()); // anzahl globals
        for (GlobalType g : globals) {
            globalsBytes.writeByte((byte)g.getValtype().code);
            if (g.isMutable()){
                globalsBytes.writeByte((byte)1);
            } else {
                globalsBytes.writeByte((byte)0);
            }
            writeGlobalExpr(g.getValtype(), g.getInit(), globalsBytes);
        }
        bw.writeByte((byte) id.ordinal());
        bw.writeWithSize(globalsBytes.getOutputStream());
    }

    void writeGlobalExpr(ValueType valType, Number init, BytecodeWriter bw) throws IOException {
        switch(valType) {
            case i32 -> {
                bw.writeOpcode(InstructionOpCode.I32_CONST);
                bw.writeI32((Integer) init);
            }
            case i64 -> {
                bw.writeOpcode(InstructionOpCode.I64_CONST);
                bw.writeI64((Integer) init);
            }
            case f32 -> {
                bw.writeOpcode(InstructionOpCode.F32_CONST);
                bw.writeF32((Double) init);
            }
            case f64 -> {
                bw.writeOpcode(InstructionOpCode.F64_CONST);
                bw.writeF64((Double) init);
            }
            case funcref -> {
                bw.writeOpcode(InstructionOpCode.REF_FUNC);
                bw.writeU32((Integer)init);
            }
            case externref -> {
                bw.writeOpcode(InstructionOpCode.REF_NULL);
                bw.writeByte((byte) 0x67);
            }
        }
        bw.writeOpcode(InstructionOpCode.END);
    }

    @Override
    void validate() {
        //TODO

    }
}

class ExportSection extends Section {
    final SectionId id = SectionId.Export;
    List<Export> exports;

    ExportSection(List<Export> exportList) {
        this.exports = exportList;
    }
    @Override
    void write(BytecodeWriter bw) throws IOException {
        BytecodeWriter exportBytes = new BytecodeWriter();
        exportBytes.writeU32(exports.size() + 1);
        for(Export e : exports) {
            writeExport(e, exportBytes);
        }

        //Exportiere immer Memory ID 0
        String memoryName = "memory";
        exportBytes.writeBytes(memoryName.getBytes(StandardCharsets.UTF_8));
        exportBytes.writeByte((byte)0x02);
        exportBytes.writeByte((byte)0x00);

        bw.writeByte((byte)id.ordinal());
        bw.writeWithSize(exportBytes.getOutputStream());
    }

    private void writeExport(Export ex, BytecodeWriter bw) throws IOException {
        bw.writeU32(ex.name().length());
        bw.writeBytes(ex.name().getBytes(StandardCharsets.UTF_8));
        bw.writeByte((byte)0x00);
        bw.writeU32(ex.funcId());
    }

    @Override
    void validate() {
        //TODO

    }
}

class StartSection extends Section {
    final SectionId id = SectionId.Start;
    int startId;

    StartSection(int startId) {
        this.startId = startId;
    }

    @Override
    void write(BytecodeWriter bw) throws IOException {
        BytecodeWriter s = new BytecodeWriter();
        s.writeU32(startId);

        bw.writeByte((byte) id.ordinal());
        bw.writeWithSize(s.getOutputStream());
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
    List<Func> funcs;
    int firstIdx; // Anzahl importierter Funktionen (Indizes der Funktionen fangen danach an)

    CodeSection(List<Func> funcs, int firstIdx) {
        this.funcs = funcs;
    }

    @Override
    void write(BytecodeWriter bw) throws IOException {
        BytecodeWriter funcBodiesBytes = new BytecodeWriter();
        //Anzahl der Funktionen
        funcBodiesBytes.writeU32(funcs.size());
        int funcIdx = firstIdx;
        for (Func f : funcs) {
            writeFuncBody(funcIdx, f, funcBodiesBytes);
            funcIdx ++;
        }

        bw.writeByte((byte)id.ordinal());
        bw.writeWithSize(funcBodiesBytes.getOutputStream());
    }

    private void writeFuncBody(int funcIdx, Func f, BytecodeWriter bw) throws IOException {
        BytecodeWriter funcBodyBytes = new BytecodeWriter();
        writeFuncLocals(funcIdx, f.getLocals(), funcBodyBytes);
        if(f.getBody().size() > 0){
            funcBodyBytes.writeBytes(f.getBody().toByteArray());
        } else {
            funcBodyBytes.writeOpcode(InstructionOpCode.END);
        }

        bw.writeWithSize(funcBodyBytes.getOutputStream());
    }

    private void writeFuncLocals(int funcIdx, ArrayList<Local> locals, BytecodeWriter bw) throws IOException {
        if (locals.isEmpty()) {
            bw.writeU32(0);
        } else if (locals.size() == 1) {
            bw.writeU32(1); // Anzahl Deklarationen
            bw.writeU32(1); // Anzahl Typ
            bw.writeByte((byte) locals.getFirst().type().code);
        } else {
            int declCount = 0, typeCount = 0;
            ValueType lastType = locals.getFirst().type();
            BytecodeWriter declsBytes = new BytecodeWriter();

            // i32 i32 i64 i32 i32 -> 2 i32 1 i64 2 i32
            int localIdx = 0;
            for (Local l : locals) {
                if (l.type() == lastType) {
                    typeCount++;
                } else {
                    declsBytes.writeU32(typeCount);
                    declsBytes.writeByte((byte) lastType.code);
                    typeCount = 1;
                    declCount++;
                    lastType = l.type();
                }
                localIdx ++;
            }
            if (typeCount > 1) {
                declsBytes.writeU32(typeCount);
                declsBytes.writeByte((byte) lastType.code);
                declCount++;
            }
            bw.writeU32(declCount);
            bw.writeBytes(declsBytes.toByteArray());
        }
    }

    @Override
    void validate() {
        //TODO

    }
}

class DataSection extends Section {
    final SectionId id = SectionId.Data;
    List<byte[]> stringLiterals;

    DataSection(List<byte[]> stringLiterals) {
        this.stringLiterals = stringLiterals;
    }

    @Override
    void write(BytecodeWriter bw) throws IOException {
        BytecodeWriter dataBytes = new BytecodeWriter();
        dataBytes.writeU32(stringLiterals.size());
        int offset = 0;
        for(var literal: stringLiterals) {
            offset += writeStringData(literal, offset, dataBytes);
        }

        bw.writeByte((byte)id.ordinal());
        bw.writeWithSize(dataBytes.getOutputStream());
    }

    private int writeStringData(byte[] literal, int offset, BytecodeWriter bw) throws IOException {
        writeActiveDataMode(offset, bw);
        bw.writeU32(literal.length);
        bw.writeBytes(literal);
        return literal.length;
    }

    private void writeActiveDataMode(int offset, BytecodeWriter bw) throws IOException {
        bw.writeByte((byte) 0);
        bw.writeOpcode(InstructionOpCode.I32_CONST);
        bw.writeOpcode(InstructionOpCode.END);
    }

    @Override
    void validate() {
        //TODO

    }
}

class DataCountSection extends Section {
    final SectionId id = SectionId.DataCount;
    List<byte[]> stringLiterals;

    DataCountSection(List<byte[]> stringLiterals) {
        this.stringLiterals = stringLiterals;
    }

    @Override
    void write(BytecodeWriter bw) throws IOException {
        BytecodeWriter dcBytes = new BytecodeWriter();
        dcBytes.writeU32(stringLiterals.size());

        bw.writeByte((byte)id.ordinal());
        bw.writeWithSize(dcBytes.getOutputStream());
    }

    @Override
    void validate() {
        //TODO

    }
}

class NameSection extends CustomSection{
    final SectionId id = super.id;
    private String moduleName = "";
    private final ArrayList<NameAssoc> functionNames = new ArrayList<>();
    private final ArrayList<IndirectNameAssoc> localNames = new ArrayList<>();

    void setModuleName(String moduleName) {
        this.moduleName = moduleName;
    }

    String getModuleName() {
        return moduleName;
    }

    void addFunctionNames(List<FuncType> funcTypes, int importedFuncsNum) {
        int idx = 0;
        for(FuncType f : funcTypes) {
            int funcIdx = idx + importedFuncsNum;
            addFunctionName(funcIdx, f.getName());
            idx ++;
        }
    }
    void addFunctionName (int idx, String functionName) {
        this.functionNames.add(new NameAssoc(idx, functionName));
    }

    void addLocalNames(List<Func> funcs, int importedFuncsNum) {
        int idx = 0;
        for (Func f : funcs) {
            int funcIdx = idx + importedFuncsNum;
            addFuncLocalNames(funcIdx, f.getLocals());
            idx ++;
        }
    }

    private void addFuncLocalNames(int funcIdx, ArrayList<Local> locals) {
        int localIdx = 0;
        for (Local l : locals) {
            addLocalName(funcIdx, localIdx, l.name());
            localIdx ++;
        }
    }

    void addLocalName (int funcIdx, int localIdx, String localName) {
        Optional<IndirectNameAssoc> res = this.localNames.stream().filter(n -> n.funcIdx() == funcIdx).findAny();
        if (res.isPresent()) {
            res.get().locals().add(new NameAssoc(localIdx, localName));
        } else {
            List<NameAssoc> names = new ArrayList<>();
            names.add(new NameAssoc(localIdx, localName));
            this.localNames.add(new IndirectNameAssoc(funcIdx, names));
        }
    }

    ArrayList<NameAssoc> getFunctionNames() {
        return functionNames;
    }

    ArrayList<IndirectNameAssoc> getLocalNames() {
        return localNames;
    }

    void write(BytecodeWriter bw) throws IOException {
        enum subsectionIds {
            ModuleName,
            FunctionNames,
            LocalNames,
        }

        // Module Name Subsection
        BytecodeWriter moduleNameBytes = new BytecodeWriter();
        moduleNameBytes.writeU32(this.moduleName.length());
        moduleNameBytes.writeBytes(this.moduleName.getBytes(StandardCharsets.UTF_8));

        // Function Names Subsection
        BytecodeWriter functionNamesBytes = new BytecodeWriter();
        functionNamesBytes.writeU32(this.functionNames.size());
        for (NameAssoc n : this.functionNames) {
            functionNamesBytes.writeU32(n.idx());
            functionNamesBytes.writeU32(n.name().length());
            functionNamesBytes.writeBytes(n.name().getBytes(StandardCharsets.UTF_8));
        }

        // Local Names Subsection
        // vec(funcIdx, vec(localIdx, name))
        // local indices with names grouped by function indices

        BytecodeWriter localNamesBytes = new BytecodeWriter();
        localNamesBytes.writeU32(this.localNames.size()); // count indirectnameassocs
        for (IndirectNameAssoc in : this.localNames) {
            localNamesBytes.writeU32(in.funcIdx());
            localNamesBytes.writeU32(in.locals().size());
            for (NameAssoc n : in.locals()) {
                localNamesBytes.writeU32(n.idx());
                localNamesBytes.writeU32(n.name().length());
                localNamesBytes.writeBytes(n.name().getBytes(StandardCharsets.UTF_8));
            }
        }

        // Name Section aus Subsections zusammenbasteln

        BytecodeWriter nameSection = new BytecodeWriter();
        String name = "name";
        nameSection.writeU32(name.length());
        nameSection.writeBytes(name.getBytes(StandardCharsets.UTF_8));

        nameSection.writeByte((byte)subsectionIds.ModuleName.ordinal());
        nameSection.writeWithSize(moduleNameBytes.getOutputStream());

        nameSection.writeByte((byte)subsectionIds.FunctionNames.ordinal());
        nameSection.writeWithSize(functionNamesBytes.getOutputStream());

        nameSection.writeByte((byte)subsectionIds.LocalNames.ordinal());
        nameSection.writeWithSize(localNamesBytes.getOutputStream());

        bw.writeByte((byte)id.ordinal());
        bw.writeWithSize(nameSection.getOutputStream());
    }
}
