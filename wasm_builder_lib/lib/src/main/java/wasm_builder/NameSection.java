package wasm_builder;

import javax.naming.Name;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

class NameSection extends CustomSection{
    private String moduleName = "";
    private final ArrayList<NameAssoc> functionNames = new ArrayList<>();
    private final ArrayList<IndirectNameAssoc> localNames = new ArrayList<>();

    void setModuleName(String moduleName) {
        this.moduleName = moduleName;
    }

    String getModuleName() {
        return moduleName;
    }

    void addFunctionName (int idx, String functionName) {
       this.functionNames.add(new NameAssoc(idx, functionName));
    }

    void addLocalName (int funcIdx, int localIdx, String localName) {
        Optional<IndirectNameAssoc> res = this.localNames.stream().filter(n -> n.funcIdx() == funcIdx).findAny();
        if (res.isPresent()) {
                res.get().locals().add(new NameAssoc(localIdx, localName));
        } else {
            List<NameAssoc> names = new ArrayList<>();
            names.add(new NameAssoc(localIdx, localName));
            this.localNames.add(new IndirectNameAssoc(funcIdx, names));
        }
    }

    ArrayList<NameAssoc> getFunctionNames() {
        return functionNames;
    }

    ArrayList<IndirectNameAssoc> getLocalNames() {
        return localNames;
    }
}
