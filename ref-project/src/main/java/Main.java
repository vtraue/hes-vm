import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.tree.ParseTree;
import wasm_builder.*;

public class Main {
  public String getGreeting() {
    return "Hello World!";
  }

  public static void main(String... args) throws IOException {
		File folder = new File("test");

		for(File entry : folder.listFiles()) {
			String content = new String(Files.readAllBytes(entry.toPath()));
			ReflangLexer lexer = new ReflangLexer(CharStreams.fromString(content));
			CommonTokenStream tokens = new CommonTokenStream(lexer);
			ReflangParser parser = new ReflangParser(tokens);
			
			ParseTree tree = parser.program();
			AstVisitor visitor = new AstVisitor();
			System.out.println("Huh?");
			visitor.visit(tree);
		}
  }
}
