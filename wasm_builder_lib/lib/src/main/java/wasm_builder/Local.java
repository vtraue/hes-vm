package wasm_builder;

public record Local(WasmValueType type, String name) {
    public Local (WasmValueType type) {
        this(type, "");
    }
}
