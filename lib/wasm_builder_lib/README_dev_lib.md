
# Erstellen einer jar-Datei
- `./gradlew build` -> `.java`-Dateien im Ordner `lib/arc/main/java/` werden compiliert und es wir eine `.jar`-Datei im Ordner `lib/build/libs/` generiert.

# Kompilieren OHNE erstellen einer jar-Datei
- `./gradlew build -x jar` -> `.java`-Dateien im Ordner `lib/arc/main/java/` werden compiliert ohne eine `.jar`-Datei zu erstellen.
