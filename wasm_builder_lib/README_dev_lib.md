
#### Erstellen einer jar-Datei
- `./gradlew build` -> `.java`-Dateien im Ordner `lib/src/main/java/` werden kompiliert und es wir eine `.jar`-Datei im Ordner `lib/build/libs/` generiert.

#### Kompilieren OHNE Erstellen einer jar-Datei
- `./gradlew build -x jar` -> `.java`-Dateien im Ordner `lib/arc/main/java/` werden kompiliert ohne eine `.jar`-Datei zu erstellen.

#### Ausführen
- `./gradlew run` -> Dateien in `app/src/main/java` werden kompiliert und ausgeführt
    - erzeugte `.wasm`-Dateien sollten im Ordner `out` erstellt werden ("../out/<filename>.wasm" im Java-Code)
