package org.example;
import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.tree.ParseTree;
import org.example.TypedAstBuilder.Function;

import wasm_builder.*;

public class App {
  public String getGreeting() {
    return "Hello World!";
  }

  public static void main(String... args) throws IOException {
    File folder = new File("test");
		BytecodeBuilder bytecodeBuilder = new BytecodeBuilder();	
		for(File entry : folder.listFiles()) {
			String content = new String(Files.readAllBytes(entry.toPath()));
			ReflangLexer lexer = new ReflangLexer(CharStreams.fromString(content));
			CommonTokenStream tokens = new CommonTokenStream(lexer);
			ReflangParser parser = new ReflangParser(tokens);
			
			ParseTree tree = parser.program();
			AstVisitor visitor = new AstVisitor();
			visitor.visit(tree);
			List<TypedStatement> typedNodes = new ArrayList<>();
			TypedAstBuilder builder = new TypedAstBuilder();			
			builder.enterNewFunction("main", Type.Int, Optional.empty()).unwrap();			

			for(Statement s : visitor.statements) {
				var typedResult = s.getTypedAstNode(builder);
				if(!typedResult.isOk()) {
					System.out.println(typedResult.getErr());
				} else {
					typedNodes.add((TypedStatement)typedResult.unwrap());		
					System.out.println(s.toDebugText());
				}
			}
			builder.leaveFunction();
			bytecodeBuilder.setGlobals(Arrays.asList(WasmValueType.i32));
			ArrayList<wasm_builder.Func> wasmFuncs = new ArrayList<>();
				
			for(TypedStatement s : typedNodes) {
				if(s instanceof TypedFndecl decl) {
					Function funcType = builder.getFunction(decl.id()).get();
					wasm_builder.Func wasmFunc = bytecodeBuilder.createFunction(funcType.toWasmFuncType());
					funcType.getLocalValueTypes().stream().forEach(l -> wasmFunc.addLocal(l));
					for(TypedStatement is : decl.block()) {
						is.toWasmCode(wasmFunc);
					}

					wasmFunc.emitEnd();	
					wasmFuncs.add(wasmFunc);

				}
			}
			bytecodeBuilder.build(wasmFuncs);
			FileOutputStream out = new FileOutputStream("gen.wasm");
			out.write(bytecodeBuilder.getWasmBuilder().getByteArray());
			out.close();
  	}
	}
}
