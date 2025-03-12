package wasm_builder;

public sealed interface Importable permits FuncType, TableType, MemType, GlobalType {}
