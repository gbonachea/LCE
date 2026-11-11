# Módulo 3 — Extensión para VS Code / OpenCode

## 1. Estructura exacta de archivos

```
extension/
├── package.json                    # manifiesto de la extensión
├── language-configuration.json     # comentarios, pares, plegado
├── syntaxes/
│   └── lce.tmLanguage.json # reglas de resaltado (TextMate)
├── snippets/
│   └── lce.json                # fragmentos reutilizables
├── .vscode/
│   ├── tasks.json                  # atajo Ctrl+Shift+B para compilar
│   └── launch.json                 # depuración con gdb
└── README.md
```

## 2. Cómo funciona cada archivo

### `package.json`

- Registra el **language id** `lce` y la extensión `.es`.
- Apunta al `language-configuration.json` (comportamiento de edición) y al
  grammar TextMate `source.lce` (resaltado).
- Declara los snippets del lenguaje.

### `language-configuration.json`

Define comentarios (`//`, `#`, `/* */`), paréntesis/llaves/corchetes para el
autocompletado de cierre y marcadores de plegado (`// #region`).

### `syntaxes/lce.tmLanguage.json`

Gramática TextMate con `scopeName` `source.lce` que asigna colores mediante
expresiones regulares a:

- Comentarios → `comment.line.double-slash.es`, `comment.block.es`.
- Cadenas → `string.quoted.double.es`.
- Números → `constant.numeric.es`.
- Funciones nativas → `support.function.es`.
- Palabras clave de control → `keyword.control.es`.
- Tipos → `storage.type.es`.
- Literales `verdadero`/`falso` → `constant.language.es`.
- Operadores `y`/`o`/`no` → `keyword.operator.logical.es`.
- Identificadores → `variable.other.es`.

## 3. Compilación con atajo (`tasks.json`)

La tarea por defecto (`group.kind = "build"`, `isDefault = true`) se dispara con
`Ctrl+Shift+B` e invoca:

```
lce ${file} -o ${fileDirname}/${fileBasenameNoExtension}
```

- `${file}` → archivo `.es` abierto actualmente.
- `${fileDirname}/${fileBasenameNoExtension}` → binario con el mismo nombre que
  el fuente, en la misma carpeta.

## 4. Instalación

### Formato `.vsix` (recomendado para VS Code)

El paquete ya está generado en `dist/lce-0.1.0.vsix`:

```bash
code --install-extension dist/lce-0.1.0.vsix
```

O desde la interfaz: **Extensiones → ⋯ → Instalar desde VSIX**.

Si quieres regenerarlo tú mismo:

```bash
cd extension
npx --yes @vscode/vsce package --out ../dist/lce-0.1.0.vsix
```

### Formato `.zip` (instalación manual / OpenCode)

El paquete `dist/lce-0.1.0.zip` contiene la extensión completa. Descomprímelo
en la carpeta de extensiones del editor:

```bash
mkdir -p ~/.vscode/extensions/lce
unzip -o dist/lce-0.1.0.zip -d ~/.vscode/extensions/lce/
```

Reinicia el editor y los archivos `.es` / `.lce` se reconocerán automáticamente.

## 5. Para que el atajo funcione

El binario `lce` debe estar en el `PATH` del usuario. Si aún no está
instalado globalmente, se puede crear un enlace temporal:

```bash
sudo ln -s "$(pwd)/motor/target/release/lce" /usr/local/bin/lce
```

O instalar el paquete `.deb` del Módulo 4, que lo coloca en `/usr/bin/lce`.
