/* (C)2025 */
package org.example;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.tree.ParseTree;
import org.example.TypedAstBuilder.InternalFunction;
import wasm_builder.*;

public class App {
    public String getGreeting() {
        return "Hello World!";
    }

    public static void main(String... args) throws IOException {
        File f = new File(args[0]);
        WasmBuilder bytecodeBuilder = new WasmBuilder();
        String content = new String(Files.readAllBytes(f.toPath()));
        ReflangLexer lexer = new ReflangLexer(CharStreams.fromString(content));
        CommonTokenStream tokens = new CommonTokenStream(lexer);
        ReflangParser parser = new ReflangParser(tokens);

        ParseTree tree = parser.program();
        AstVisitor visitor = new AstVisitor();
        visitor.visit(tree);
        List<TypedStatement> typedNodes = new ArrayList<>();
        TypedAstBuilder builder = new TypedAstBuilder();
        // builder.enterNewFunction("main", Type.Int, Optional.empty()).unwrap();

        for (Statement s : visitor.statements) {
            var typedResult = s.getTypedAstNode(builder);
            if (!typedResult.isOk()) {
                System.out.println(typedResult.getErr());
            } else {
                typedNodes.add((TypedStatement) typedResult.unwrap());
            }
        }
        builder.leaveFunction();
        try {
            bytecodeBuilder.setGlobals(Arrays.asList(new GlobalType(ValueType.i32, true, 0)));
        } catch (WasmBuilderException e) {
            throw new RuntimeException(e);
        }
        builder.importFunctions(bytecodeBuilder);

        ArrayList<wasm_builder.Func> wasmFuncs = new ArrayList<>();
        for (TypedStatement s : typedNodes) {
            if (s instanceof TypedExternFndecl extDecl) {}
            if (s instanceof TypedLiteral l && l.lit() instanceof StringLiteral str) {
                bytecodeBuilder.addStringData(Arrays.asList(str.literal()));
            }
            if (s instanceof TypedFndecl decl) {
                InternalFunction funcType = (InternalFunction) builder.getFunction(decl.id()).get();
                wasm_builder.Func wasmFunc =
                        bytecodeBuilder.createFunction(funcType.toWasmFuncType());

                funcType.getWasmLocals().stream().forEach(l -> wasmFunc.addLocal(l));
                for (TypedStatement is : decl.block()) {
                    is.toWasmCode(wasmFunc, builder);
                }
                if (funcType.export()) {
                    bytecodeBuilder.exportFunction(
                            decl.id(), builder.getGlobalFunctionId(funcType));
                }
                wasmFuncs.add(wasmFunc);
            }
        }

        builder.getFunction("main")
                .ifPresent(
                        func ->
                                bytecodeBuilder.setStartFunction(
                                        builder.getGlobalFunctionId(func)));

        bytecodeBuilder.build(wasmFuncs);
        FileOutputStream out = new FileOutputStream("gen.wasm");
        out.write(bytecodeBuilder.getByteArray());
        out.close();
    }
}
