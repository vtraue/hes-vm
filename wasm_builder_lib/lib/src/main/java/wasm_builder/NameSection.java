package wasm_builder;

import java.util.ArrayList;
public class NameSection {
    private String moduleName = "";
    private final ArrayList<NameAssoc> functionNames = new ArrayList<>();
    private final ArrayList<NameAssoc> localNames = new ArrayList<>();

    public void setModuleName(String moduleName) {
        this.moduleName = moduleName;
    }

    public String getModuleName() {
        return moduleName;
    }

    public void addFunctionName (int idx, String functionName) {
       this.functionNames.add(new NameAssoc(idx, functionName));
    }

    public void addLocalName (int idx, String localName) {
       this.localNames.add(new NameAssoc(idx, localName));
    }

    public ArrayList<NameAssoc> getFunctionNames() {
        return functionNames;
    }

    public ArrayList<NameAssoc> getLocalNames() {
        return localNames;
    }
}
