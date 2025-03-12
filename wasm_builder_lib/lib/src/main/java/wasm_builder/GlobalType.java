package wasm_builder;

public record GlobalType(WasmValueType valtype, boolean mutable) implements Importable{};
