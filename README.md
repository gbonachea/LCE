# LCE — Lenguaje de programación en español para apps nativas Qt

Un lenguaje de programación **100 % en español** orientado al desarrollo de
aplicaciones de escritorio de alto rendimiento usando **Qt (C++)**. El código
`.es` / `.lce` se transpila a C++17/Qt6 y se compila a un binario nativo ultra
rápido.

```
                    ┌──────────────────────────────────────────────┐
  archivo.es  ───►  │  motor (Rust)                                │
                    │  lexer → parser → semántica → codegen        │
                    │  → temp.cpp                                  │
                    └───────────────┬──────────────────────────────┘
                                    │  g++/clang++ -O3
                                    ▼
                              binario nativo
```

## Estructura del repositorio

```
Lenguaje/
├── README.md                        ← esta guía
├── docs/
│   ├── 01-sintaxis-gramatica.md     ← Módulo 1
│   ├── 02-motor-transpilador.md     ← Módulo 2
│   ├── 03-extension-vscode.md       ← Módulo 3
│   ├── 04-empaquetado-deb.md        ← Módulo 4
│   └── 05-opencode-skill.md         ← Módulo 5 (skill de OpenCode)
├── ejemplos/
│   ├── ventana_boton.es             ← app Qt completa con botón
│   ├── hola_mundo.es                ← variables y control de flujo
│   ├── funciones.es                 ← funciones de usuario (funcion)
│   ├── bucles.es                    ← bucle 'para' y 'sino si'
│   ├── formulario.es                ← formulario Qt con caja de texto
│   └── referencias.es               ← parámetros por referencia (&)
├── motor/                           ← transpilador en Rust
│   ├── Cargo.toml
│   ├── src/{main,lib,token,lexer,ast,parser,semantica,codegen}.rs
│   └── tests/integracion.rs         ← tests de integración del pipeline
├── Makefile                         ← build / test / ejemplos / deb
├── docs/manual/generar_manual.py    ← genera el manual PDF
├── dist/                            ← paquetes generados (deb/vsix/zip/pdf)
├── .opencode/skills/lce/            ← skill de OpenCode (SKILL.md + referencias)
├── extension/                       ← extensión VS Code / OpenCode
│   ├── package.json
│   ├── language-configuration.json
│   ├── syntaxes/lce.tmLanguage.json
│   ├── snippets/lce.json
│   └── .vscode/{tasks,launch}.json
└── empaquetado/                     ← distribución .deb
    ├── build-deb.sh
    └── deb/DEBIAN/control
```

## Inicio rápido

```bash
# 1. Compilar el motor (requiere Rust)
cd motor && cargo build --release

# 2. Compilar un programa .es (requiere g++ y Qt6)
./target/release/lce ../ejemplos/ventana_boton.es -o mi_app
./mi_app

# 3. Instalar la extensión en VS Code (ver docs/03)
# 4. Empaquetar como .deb (ver docs/04)
./empaquetado/build-deb.sh
```

## Paquetes y manual

Los entregables listos para usar están en `dist/`:

| Archivo | Qué es | Instalación |
| --- | --- | --- |
| `lce_0.1.0_amd64.deb` | Compilador `lce` para Linux | `sudo apt install ./dist/lce_0.1.0_amd64.deb` |
| `lce-0.1.0.vsix` | Extensión de VS Code | `code --install-extension dist/lce-0.1.0.vsix` |
| `lce-0.1.0.zip` | Extensión para instalación manual (VS Code) | descomprimir en `~/.vscode/extensions/lce/` |
| `LCE-Manual-v0.1.0.pdf` | Manual completo en español | abrir con cualquier lector de PDF |

> El manual explica desde cero cómo programar en LCE: tipos, control de flujo,
> funciones, interfaz gráfica con Qt, palabras clave, funciones nativas, atajos
> de VS Code y solución de problemas.

## Tests

El motor incluye una batería exhaustiva con **90 tests** (79 unitarios + 11 de
integración). Se ejecutan con `cargo`:

```bash
cd motor
cargo test                 # toda la batería
cargo test -- --nocapture  # mostrando la salida de cada test
cargo test semantica       # solo los tests de un módulo
cargo clippy --all-targets # linter (sin warnings)
```

O con el `Makefile` desde la raíz:

```bash
make test          # cargo test
make test-verbose  # cargo test -- --nocapture
make ejemplos      # compila los 6 ejemplos con g++/Qt
make todo          # build + test + ejemplos + .deb
```

| Módulo        | Tests | Qué cubre                                                        |
| ------------- | ----- | ---------------------------------------------------------------- |
| `lexer`       | 10    | Todas las palabras clave, operadores, comentarios, cadenas, posiciones y errores. |
| `parser`      | 16    | Declaraciones, precedencia, lógicos, control de flujo, funciones, referencias y errores. |
| `semantica`   | 40    | Tipos, ámbitos, aridad, retornos, referencias, widgets y todas las reglas de error. |
| `codegen`     | 13    | Mapeo de tipos, operadores, `else if`, `for`, prototipos, `&`, eventos y ayudas Qt. |
| integración   | 11    | Pipeline completo sobre los 6 ejemplos, fases de error y **compilación real con g++/Qt**. |

## Integración con OpenCode (skill)

La extensión `.vsix` es para el **editor**; OpenCode usa **skills**:
instrucciones reutilizables en un `SKILL.md`. El repositorio incluye la skill
`lce` en `.opencode/skills/lce/` (sintaxis, CLI, widgets y errores), con
referencias de apoyo. Instálala globalmente con:

```bash
make skill
```

Luego el agente la carga cuando trabajes con archivos `.es`/`.lce`, o puedes
invocarla con `@lce`. Detalle completo en
[`docs/05-opencode-skill.md`](docs/05-opencode-skill.md).

## Los módulos

1. **Sintaxis y gramática** — palabras clave en español (`si`, `mientras`,
   `ventana`, `boton`, …), gramática EBNF y ejemplo `.es` completo.
   → [`docs/01-sintaxis-gramatica.md`](docs/01-sintaxis-gramatica.md)

2. **Motor transpilador en Rust** — lexer, parser de descenso recursivo,
   **verificador semántico** de tipos y declaraciones, codegen a C++/Qt y
   compilación automática con `-O3`.
   → [`docs/02-motor-transpilador.md`](docs/02-motor-transpilador.md)

3. **Extensión para VS Code** — `package.json`,
   `language-configuration.json`, gramática TextMate y `tasks.json` para
   compilar con `Ctrl+Shift+B`.
   → [`docs/03-extension-vscode.md`](docs/03-extension-vscode.md)

4. **Empaquetado `.deb`** — estructura de carpetas, `DEBIAN/control` y script
   para instalar `lce` globalmente en `/usr/bin`.
   → [`docs/04-empaquetado-deb.md`](docs/04-empaquetado-deb.md)

5. **Integración con OpenCode** — skill `lce` (`SKILL.md` + referencias) para
   que el agente sepa programar en LCE. El `.vsix` es para el editor; la skill
   es para OpenCode.
   → [`docs/05-opencode-skill.md`](docs/05-opencode-skill.md)

## Estado del lenguaje (completo y verificado)

- ✅ Tipos `entero`, `decimal`, `booleano`, `cadena`, `nada` + widgets.
- ✅ Variables, asignación, operadores aritméticos/lógicos/comparación,
  concatenación de cadenas con `+`.
- ✅ Control de flujo: `si`/`sino`/`sino si`, `mientras`, `para` (rango).
- ✅ Funciones de usuario con parámetros tipados, **paso por referencia (`&`)**
  y retorno opcional, emitidas como prototipos + definiciones.
- ✅ Widgets Qt: `ventana`, `boton`, `etiqueta`, `caja_texto`, eventos
  `al hacer_clic`, y lectura/escritura de texto (`texto`, `establecer_texto`).
- ✅ **Verificación semántica** con errores en español (tipos, aridad,
  declaraciones, retornos).
- ✅ El C++ generado compila contra **Qt6Widgets** con `g++ -O3`
  (validado en este entorno con los 6 ejemplos).
- ✅ Extensión VS Code y paquete `.deb` funcionales.

### Extensiones futuras (fuera del alcance actual)

Colecciones (`lista`/`arreglo`), `sino si` con patrones, cierres de usuario y
módulos. La base está preparada para añadirlas sin reescribir el motor.
