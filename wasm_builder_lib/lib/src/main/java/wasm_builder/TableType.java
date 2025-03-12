package wasm_builder;

public record TableType(int min, int max, boolean refExt) implements Importable{};
