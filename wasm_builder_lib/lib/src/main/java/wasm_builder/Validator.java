package wasm_builder;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

public class Validator {
    private Stack<Type> valStack;
    private Stack<CtrlFrame> ctrlStack;
    private List<GlobalType> globals;
    private List<Local> locals;
    public enum Unknown implements Type {
        Unknown(),
        ;
    }

    public Validator( List<GlobalType> globals, List<Local> locals) {
        this.valStack = new Stack<>();
        this.ctrlStack = new Stack<>();
        this.globals = globals;
        this.locals = locals;
    }

    private void pushVal(Type type) {
        valStack.push(type);
    }

   private Type popVal() throws Exception{
        // Block enthält keine bekannten Typen und wurde als unreachable markiert -> return Unknown; nichts wird vom valStack gepoppt
       if (valStack.size() == ctrlStack.peek().getHeight() && ctrlStack.peek().getUnreachable()) {
           return Unknown.Unknown;
       }
       // Block enthält keine bekannten Typen. Etwas poppen würde zum Underflow des valStacks führen (Es wird versucht ein Wert zu poppen, der nicht zum Block gehört) -> Error
       if (valStack.size() == ctrlStack.peek().getHeight()) {
           throw new Exception();
       }
       return this.valStack.pop();
   }

   private Type popVal(Type expected) throws Exception{
        Type actual = popVal();
        if (actual != expected) {
            throw new Exception();
        }
        return actual;
   }

   private void pushVals(List<Type> types) {
        for (Type t : types) {
            pushVal(t);
        }
   }

   private List<Type> popVals(List<Type> vals) throws Exception {
        List<Type> popped = new ArrayList<>();
        List<Type> valsRev = vals.reversed();
        for (Type t : valsRev) {
            popped.add(popVal(t));
        }
        return popped.reversed();
   }

   private void validateUnop(Type t) throws Exception {
        popVal(t);
        pushVal(t);
   }

   private void validateBinop(Type t) throws Exception {
        popVal(t);
        popVal(t);
        pushVal(t);
   }

   private void validateTestop(Type t) throws Exception {
        popVal(t);
        pushVal(ValueType.i32);
   }

   private void validateRelop(Type t) throws Exception {
        popVal(t);
        popVal(t);
        pushVal(ValueType.i32);
   }

   public void validate(InstructionOpCode opCode) throws Exception {
        switch(opCode) {
            // t.const c
            case I32_CONST -> pushVal(ValueType.i32);
            case I64_CONST -> pushVal(ValueType.i64);
            case F32_CONST -> pushVal(ValueType.f32);
            case F64_CONST -> pushVal(ValueType.f64);
            // t.unop
            // TODO: add neg, abs, floor, trunc, etc. ?
            // t.binop
            case I32_ADD -> validateBinop(ValueType.i32);
            case I32_SUB -> validateBinop(ValueType.i32);
            case I32_MUL -> validateBinop(ValueType.i32);
            case I32_DIV_S -> validateBinop(ValueType.i32);
            case I32_REM_S -> validateBinop(ValueType.i32);
            case I32_AND -> validateBinop(ValueType.i32);
            case I32_OR -> validateBinop(ValueType.i32);
            case I32_XOR -> validateBinop(ValueType.i32);
            // t.testop
            case I32_EQZ -> validateTestop(ValueType.i32);
            // t.relop
            case I32_EQ -> validateRelop(ValueType.i32);
            case I32_NE -> validateRelop(ValueType.i32);
            case I32_LT_S -> validateRelop(ValueType.i32);
            case I32_GT_S -> validateRelop(ValueType.i32);
            case I32_LE_S -> validateRelop(ValueType.i32);
            case I32_GE_S -> validateRelop(ValueType.i32);
            // Parametric Instructions
            case DROP -> popVal();
        }
   }

    public void validate(InstructionOpCode opCode, int n) throws Exception {

        // TODO ???
    }
}
