package wasm_builder;

public final class GlobalType implements Importable {
    private ValueType valtype;
    private boolean mutable;
    private Number init;

    public Number getInit() {
        return init;
    }

    public boolean isMutable() {
        return mutable;
    }

    public ValueType getValtype() {
        return valtype;
    }

    public GlobalType(ValueType valtype, boolean mutable, int init) throws WasmBuilderException {
        if (valtype == ValueType.i64 || valtype == ValueType.i32) {
        this.valtype = valtype;
        this.mutable = mutable;
        this.init = init;
        } else {
            throw new WasmBuilderException("Global Constructor: Wrong init Type (int) for ValueType " + valtype.toString());
        }
    }
    public GlobalType(ValueType valtype, boolean mutable, double init) throws WasmBuilderException {
        if (valtype == ValueType.f64 || valtype == ValueType.f32) {
            this.valtype = valtype;
            this.mutable = mutable;
            this.init = init;
        } else {
            throw new WasmBuilderException("Global Constructor: Wrong init Type (double) for ValueType " + valtype.toString());
        }
    }
}

