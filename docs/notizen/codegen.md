# Codegen

## Vorraussetzungen
- Jede Variable hat eine Scope-unabhägige, eindeutige, funktionsweite ID 
    - Sollte relativ trivial zu implementieren sein. 
    - Entweder vor der semantischen Analyse im Visitor durch Zählen der Deklarationen oder während Sema
- Anzahl aller lokalen Variablen in einer Funktion ist bekannt
- Code ist oragnisiert in Funktionen. Es existiert kein Code außerhalb von Funkionen
- Parameter- und Rückgabetypen von Funktionen sind bekannt
- Namen von Funktionen sind einzigartig
- Garantien durch semantische Analyse:
    - Korrektheit von Scopes, Variablen...

## Assembler-API 
- 1: API exposed nur rohe WASM Instruktionen mit Helfer-Abstraktionen
    - Pro: 
        - Scope der Java-Bibliothek sehr überschaubar
        - Erlertnes Wissen über WASM-Bytecode ist "allgemingültig". 
        - Resourcen aus dem Internet sind uneigeschränkt gültig
    - Contra:
        - Deutlich mehr Dokumentation nötig
        - Wohlmöglich schwerer für Studierende
- 2: API exposed rohes WASM - VM exposed viele Funktionen zum Manipulieren der VM  <
    - Pro:
        - Alles bei 1
        - Generierter Code ist nicht mehr Runtime unanbhängig
    - Contra:
        - Konzepte die die VM übernimmt müssten später nachgelernt werden
- 3: API exposed pseudo-Instruktionen die von Assember zu WASM umgewandelt werden <
    - Pro: 
        - Simplifiziert kompliziertere, sprachabhängigere Konzepte (Pointer, Refs, Structs ...)
    - Contra:
        - Studierende lernen "Dialekt" statt "echtem" Bytecode
- 4: Studierende generieren Zwischencode der von VM zu WASM umgewandelt wird. 
    - Contra: 
        - Mag ich nicht mehr

Ich denke irgendwas zwischen 2 und 3 ist am Besten. Für sehr simple Sprachen ist 1 völlig okay, WASM ist ja bereits sehr high level
verglichem mit anderem Bytecode aber wenn später Konzepte wie Klassen, Referenzen oder sogar Vtables behandelt werden sollen ist
das vielleicht etwas viel.
So oder so brauchen die Studierende *sehr* viele Beispiele und gute Dokumentation um mit den Konzepten umgehen zu können.
Eine offene Frage für uns ist wie gut Studierende im 3. Semester mit dem neuen Lehrplan sich mit Ideen wie dem Stack oder Pointern überhaupt auskennen.
All das baut jetzt mehr oder weniger darauf auf, dass die Studierenden schon mal C geschrieben haben und die Frage beantworten können
was ein Pointer ist und wie der im Speicher aussehen könnte.

## Codegen-Aufbau
- Ziel: Direktes Übersetzen von AST-Nodes ohne Backtracking oder mehreren Durchläufen
- Studierende sollten für jeden AST Node eine einfache Funktion schreiben können die unabhängig vom "Kontext" Code generieren kann
- Jede Expression legt min. einen Wert auf den Stack ab
- Jede Expression muss konsumiert werden
- Statements legen keinen Wert auf den Stack ab

Wir wollen den Assembler mit einer Referenzsprache testen und schauen was umsetzbar und halbwegs ergonomisch ist.
Gute Lernmaterialen sollten m.E den einfachsten Weg zeigen wie man Codeschnipsel zu WASM konvertieren kann.
Die Codeschnipsel sollten dem Aufabu von AST Nodes folgen. z.B
```c
    int a = 5;
```
Lässt sich in Deklaration `c int a;` und Expression "5" aufteilen.
Die Expression in dem Beispiel sollte in jeder Situation den gleichen WASM Code erzeugen

### Lokale Variablen
- Min. 3 möglichkeiten Variablen zu verwalten
    - 1: WASM-Locals enthalten alle Werte  
        - get/set Instruktionen erfordern ids die zur Kompilierzeit bekannt sein müssen 
    - 2: Werte werden in selbstverwalteten Stack im Speicher verwaltet. WASM-Locals enthalten Referenzen
    - 3: Hybrid: Werte werden solange in Locals gespeichert bis sie als Pointer / Referenzen gebraucht werden
        - Ich glaube das ist nicht einfach zu implementieren
### Fall 1:
- Sprache kann keine Referenzen / Pointer haben
- Structs / Klasen nur seperat über Pointer oder garnicht

- C:
```c
    int a = 5;
```
- Wasm:
```wat
    ;; 5
    i32.const 5
    ;; int a = 
    local.set $a
```
- C
```c
    a = a + 10;
```
- Wasm

```wat
    ;; a 
    local.get $a
    ;; 10
    i32.const 10
    ;; + 
    i32.add 
    ;; a =
    local.set $a
    
```
C
```c
    a = b;
```
Wasm
```wat
    ;; b
    local.get $b
    ;; a =  
    local.set $a
    
```
### Fall 2:
- Implementiert von Studierenden oder VM muss Stack handeln
- Mögliche Implementation:

- C:
```c
    int a = 5;
```
- Wasm:
```wat
    ;; int a;
    global.get $sp
    local.set $a
    global.get $sp
    i32.add 4
    global.set $sp
        
    ;; 5
    i32.const 5
    ;; a = 
    local.get $a
    i32.store 0 1
     
```
- C
```c
    a = a + 10;
```
- Wasm

```wat
    ;; a 
    local.get $a
    i32.load 0 1

    ;; 10
    i32.const 10
    ;; + 
    i32.add 
    ;; a =
    local.get $a
    i32.store 0 1
    
```
```
C
```c
    a = b;
```

Wasm
```wat
    ;; b
    local.get $b
    i32.load 0 1
    ;; a =  
    local.load $a
    i32.store 0 1 
```

```
C
```c
    int *c = &b; 
```
Wasm
```wat
    ;; int* c
    global.get $sp
    local.set $c
    global.get $sp
    i32.add 4
    global.set $sp

    ;; &b
    local.get $b
    
    ;; c =  
    local.get $c
    i32.store 0 1 

```
C
```c
    b = *c + 5; 
```
Wasm
```wat
    ;; *c
    local.get $c
    i32.load 0 1
    
    ;; 5
    i32.const 5
    
    ;; + 
    i32.add 

    ;; b =
    local.get $b
    i32.store 0 1


