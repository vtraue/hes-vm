package wasm_builder;

public record GlobalType(ValueType valtype, boolean mutable) implements Importable{};
