package wasm_builder;

public class Import {
    private final String module;
    private final String name;
    Importable desc;
    public Import(String module, String name, Importable desc) {
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

    public Importable getDesc() { return desc; }
}
