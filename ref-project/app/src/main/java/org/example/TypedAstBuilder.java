package org.example;
import java.util.Map;
import java.util.Optional;
import java.lang.Math;

import wasm_builder.BytecodeBuilder;
import wasm_builder.Func;
import wasm_builder.FuncType;
import wasm_builder.Local;
import wasm_builder.WasmValueType;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;

public class TypedAstBuilder {
  public record Symbol (
    int id,
    Type type,
    boolean local 
  ) {
    WasmValueType toValueType() {
      return this.type.toWasmValueType();
    }
  };

  interface Function {
    int getId();
    Optional<Params> getArgs();
    Optional<Type> getReturnType();
    Optional<List<WasmValueType>> getArgValueTypes();
    FuncType toWasmFuncType();  
  }

  public record InternalFunction (
    int id,
    Optional<Type> returnType,
    Optional<Params> argTypes,
    boolean export,
    List<Symbol> locals
  ) implements Function {

    void addLocal(Symbol sym) {
      this.locals.add(sym);
    }

    List<Local> getWasmLocals() {
      return this.locals.stream().map(sym -> new Local(sym.toValueType())).toList();
    }

    public Optional<List<WasmValueType>> getArgValueTypes() {
      return this.argTypes.map(Params::toWasmValueTypes);
    }
    
    public FuncType toWasmFuncType() {
      var params = this.getArgValueTypes().orElse(new ArrayList<>());
      var result = this.returnType.map(t -> Arrays.asList(t.toWasmValueType())).orElse(new ArrayList<>());

      return new wasm_builder.FuncType(params, result); 
    }

    @Override
    public int getId() {
      return this.id; 
    }

    @Override
    public Optional<Params> getArgs() {
      return this.argTypes;
    }

    @Override
    public Optional<Type> getReturnType() {
      return this.returnType;
    }
  };

  public record ExternalFunction (
    int id,
    String env,
    String name,
    Optional<Type> returnType,
    Optional<Params> argTypes
   ) implements Function {

    public Optional<List<WasmValueType>> getArgValueTypes() {
      return this.argTypes.map(Params::toWasmValueTypes);
    }

    public FuncType toWasmFuncType() {
      var params = this.getArgValueTypes().orElse(new ArrayList<>());
      var result = this.returnType.map(t -> Arrays.asList(t.toWasmValueType())).orElse(new ArrayList<>());

      return new wasm_builder.FuncType(params, result); 
    }

    @Override
    public int getId() {
      return this.id;
    }

    @Override
    public Optional<Params> getArgs() {
      return this.argTypes;
    }

    @Override
    public Optional<Type> getReturnType() {
      return this.returnType; 
    }

    public void importFunction(BytecodeBuilder builder) {
      builder.importFunc(this.env, this.name, this.toWasmFuncType());
    }
  }


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

  private Map<String, ExternalFunction> externalFunctions = new HashMap<>(); 

  private int functionVariableId = 0;
  private int functionId = 0;
  private int externalFuncId = 0;


  Result<Function, Function> enterNewFunction(String name, Optional<Type> returnType, Optional<Params> args, boolean export) {
    if(this.currentFunction.isPresent()) {
      this.leaveFunction();
    }

    InternalFunction f = new InternalFunction(functionId, returnType, args, export, new ArrayList<>()); 
    functionId += 1;  

    Optional<Function> found = Optional.ofNullable(this.functions.get(name));
    if(found.isPresent()) {
      return new Err<>(found.get());
    }
    this.functions.put(name, f);
    this.enterNewScope();
    this.currentFunction = Optional.of(name);

    if(args.isPresent()) {
      args
        .get()
        .params()
        .stream()
        .forEach(a -> this.addVariable(a.id().name(), a.type()));
    }

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
    this.functionVariableId = 0;
  }

  Result<Symbol, Symbol> addVariable(String name, Type t) {
    Symbol new_sym = new Symbol(functionVariableId, t, true);
      
    var res = this.currentEnv.addSymbol(name, new_sym);
    
    if(res.isPresent()) {
      return new Err<>(res.get());
    }

    if(!this.currentFunction.isPresent()) {
    }

    var currentFunction = (InternalFunction)this.getCurrentFunction().get();
    currentFunction.addLocal(new_sym);
    this.functionVariableId += 1;
    return new Ok<>(new_sym);
  }
  void enterNewScope() {
    Enviroment nextEnv = new Enviroment(Optional.of(this.currentEnv));
    currentEnv = nextEnv;
  }

  Optional<Function> getFunction(String name) {
    var inner = this.functions.get(name);
    if(inner == null) {
      return Optional.ofNullable(this.externalFunctions.get(name));
    }
    return Optional.of(inner); 
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

  Result<ExternalFunction, ExternalFunction> addExternalFunction(String name, String env, Optional<Params> params, Optional<Type> returnType) {
    var prevExtFunction = this.externalFunctions.get(name);
    if(prevExtFunction != null) {
      return new Err<>(prevExtFunction);
    }
    ExternalFunction func = new ExternalFunction(this.externalFuncId, env, name, returnType, params);
    this.externalFunctions.put(name, func);
    this.externalFuncId += 1; 

    return new Ok<>(func);
  }

  int getGlobalFunctionId(Function fn) {
    if(fn instanceof InternalFunction) {
      return fn.getId() + Math.clamp(this.externalFunctions.size(), 0, 999);
    }
    return fn.getId();
  }

  public void importFunctions(BytecodeBuilder builder) {
    var funcs = new ExternalFunction[this.externalFunctions.size()];
    for(var extFunc : this.externalFunctions.values()) {
      funcs[extFunc.id] = extFunc;
    }
    for(var f : funcs) {
      builder.importFunc(f.env, f.name, f.toWasmFuncType());
    }
  }

} 
