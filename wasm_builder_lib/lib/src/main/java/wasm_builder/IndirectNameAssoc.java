package wasm_builder;

import java.util.List;

public record IndirectNameAssoc(int funcIdx, List<NameAssoc> locals) {
}
