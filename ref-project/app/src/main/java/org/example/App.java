package org.example;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.tree.ParseTree;
import wasm_builder.*;

public class App {
  public String getGreeting() {
    return "Hello World!";
  }

  public static void main(String... args) throws IOException {
    File folder = new File("test");
		BytecodeBuilder bb = new BytecodeBuilder();	
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
			builder.enterNewFunction("main", Type.Int, Optional.empty());			

			for(Statement s : visitor.statements) {
				var typedResult = s.getTypedAstNode(builder);
				if(!typedResult.isOk()) {
					System.out.println(typedResult.getErr());
				} else {
					typedNodes.add((TypedStatement)typedResult.unwrap());		
					System.out.println(s.toDebugText());
				}
			}
		}
  }
}
