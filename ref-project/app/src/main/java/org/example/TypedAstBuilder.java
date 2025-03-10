package org.example;
import java.util.Map;
import java.util.Optional;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

public class TypedAstBuilder {
	public record Symbol (
		Type type
	) {};

	public record Function (
		Type returnType,
		Optional<Params> argTypes
	) {};

	public class Enviroment {
		public Optional<Enviroment> parent;
		private Map<String, Symbol> variables; 

		public Enviroment(Optional<Enviroment> parent) {
			this.parent = parent;
			this.variables = new HashMap<>();
		}

		Optional<Symbol> addSymbol(String name, Symbol sym) {
			Optional<Symbol> found = Optional.ofNullable(this.variables.get(name));
			if(found.isPresent()) {
				return found;
			} 
			this.variables.put(name, sym);
			return Optional.empty();
		}

		Optional<Symbol> getSymbol(String name) {
			return Optional.ofNullable(this.variables.get(name));
		}

		Optional<Symbol> searchVariable(String name) {
				Optional<Enviroment> current = Optional.of(this);	

				while(current.isPresent()) {
					var sym = current.get().getSymbol(name);
					if(sym.isPresent()) {
						return sym;
					}	
					current = current.get().parent;
				}
				return Optional.empty();
		}
	}

	public Enviroment currentEnv = new Enviroment(Optional.empty());
	private Map<String, Function> functions = new HashMap<>();
	private Optional<String> currentFunction = Optional.empty();		

	Result<Function, Function> enterNewFunction(String name, Type returnType, Optional<Params> args) {
		Function f = new Function(returnType, args);	

		Optional<Function> found = Optional.ofNullable(this.functions.get(name));
		if(found.isPresent()) {
			return new Err<>(found.get());
		}
		this.functions.put(name, f);

		this.enterNewScope();
		if(args.isPresent()) {
			System.out.println("Blubbi");
			args
				.get()
				.params()
				.stream()
				.forEach(a -> this.addVariable(a.id().name(), a.type()));
		}
		this.currentFunction = Optional.of(name);

		return new Ok<>(f); 
	}
	
	Optional<Function> getCurrentFunction() {
		if(this.currentFunction.isEmpty()) {
			return Optional.empty();
		}
		return Optional.of(this.functions.get(this.currentFunction.get()));
	}

	void leaveFunction() {
		this.leaveScope();
		this.currentFunction = Optional.empty();

	}
	Result<Symbol, Symbol> addVariable(String name, Type t) {
		Symbol new_sym = new Symbol(t);
		var res = this.currentEnv.addSymbol(name, new Symbol(t));

		if(res.isPresent()) {
			return new Err<>(res.get());
		}
		return new Ok<>(new_sym);
	}
	void enterNewScope() {
		Enviroment nextEnv = new Enviroment(Optional.of(this.currentEnv));
		currentEnv = nextEnv;
	}

	Optional<Function> getFunction(String name) {
		return Optional.ofNullable(this.functions.get(name));
	}

	Result<Enviroment, String> leaveScope() {
		if(!this.currentEnv.parent.isPresent()) {
			return new Err<Enviroment, String>("Cannot leave bottom most scope");
		}
		var nextEnv = this.currentEnv.parent.get();
		this.currentEnv = nextEnv;
		return new Ok<>(nextEnv);
	}

	Optional<Symbol> searchVariable(String name) {
		return this.currentEnv.searchVariable(name);
	}

	Result<List<TypedExpression>, String> getExpressionTypes(List<Expression> nodes) {
		StringBuilder errorMessageBuilder = new StringBuilder();
		List<TypedExpression> typedStatements = new ArrayList<>();
		boolean hasErrors = false;
		for(Expression e : nodes) {
			Result<TypedAstNode, String> typedResult = e.getTypedAstNode(this);
			if(!typedResult.isOk()) {
				hasErrors = true;
				errorMessageBuilder.append(typedResult.getErr() + "\n");
			}
				TypedAstNode typedNode = typedResult.unwrap(); 
				typedStatements.add((TypedExpression)typedNode);
			} 
		if(hasErrors) {
			return new Err<>(errorMessageBuilder.toString());
		}
		return new Ok<>(typedStatements);
	}
} 
