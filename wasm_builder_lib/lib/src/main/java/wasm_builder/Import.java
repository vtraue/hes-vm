package wasm_builder;
sealed interface ImportType {}
record FuncId(int id) implements ImportType{};
record TableType(int min, int max, boolean refExt) implements ImportType{};
record MemType(int min, int max) implements ImportType{};
record GlobalType(WasmValueType valtype, boolean mutable) implements ImportType{};

public class Import {
    private final String module;
    private final String name;
    private ImportType desc;
    public Import(String module, String name, ImportType desc) {
        this.module = module;
        this.name = name;
        this.desc = desc;
    }

    public String getName() {
        return name;
    }

    public String getModule() {
        return module;
    }

    public ImportType getDesc(){
        return desc;
    }
}
