package wasm_builder;

public record Local(ValueType type, String name) {
    public Local (ValueType type) {
        this(type, "");
    }
}
