# Extensión: LCE (Qt) para VS Code

Soporte de lenguaje para **LCE (Qt)**, un lenguaje de programación en español
que se compila a aplicaciones nativas Qt6/C++.

## Características

- Resaltado de sintaxis para archivos `.es`.
- Comentarios (`//`, `#`, `/* */`), autocompletado de llaves y plegado de código.
- Fragmentos (snippets) para `programa`, `ventana`, `boton`, `si`, `mientras`, etc.
- Tarea de compilación integrada (`Ctrl+Shift+B`) que invoca al compilador `lce`.

## Instalación

### Opción A: paquete `.vsix` (VS Code)

El paquete ya está generado en `dist/lce-0.1.0.vsix`:

```bash
code --install-extension dist/lce-0.1.0.vsix
```

Si quieres regenerarlo tú mismo:

```bash
cd extension
npx --yes @vscode/vsce package --out ../dist/lce-0.1.0.vsix
```

### Opción B: `.zip` / instalación manual (OpenCode / VS Code)

```bash
mkdir -p ~/.vscode/extensions/lce
unzip -o dist/lce-0.1.0.zip -d ~/.vscode/extensions/lce/
```

## Requisitos

- El compilador `lce` instalado y en el `PATH` (ver módulo 4 de empaquetado).
- Qt6 (`qt6-base-dev`) y `g++` instalados para compilar el binario.

## Atajos

| Acción                              | Atajo        |
| ----------------------------------- | ------------ |
| Compilar archivo `.es` actual       | `Ctrl+Shift+B` |
| Solo generar `temp.cpp`             | Tarea "lce: solo generar temp.cpp" |
| Depurar con gdb                     | `F5`         |
