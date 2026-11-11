# Módulo 2 — El motor transpilador (Rust)

## 1. Arquitectura

El motor `lce` es un programa en **Rust sin dependencias externas** (solo la
biblioteca estándar) que implementa un pipeline de compilación clásico:

```
archivo.es
    │
    ▼
┌─────────┐   ┌─────────┐   ┌────────────┐   ┌──────────┐   ┌──────────────┐
│ Lexer   │──►│ Parser  │──►│ Semántica  │──►│ Codegen  │──►│ g++/clang++  │
│ (tokens)│   │ (AST)   │   │ (tipos)    │   │(temp.cpp)│   │ binario nativo│
└─────────┘   └─────────┘   └────────────┘   └──────────┘   └──────────────┘
```

| Fase        | Archivo            | Entrada → Salida                          |
| ----------- | ------------------ | ----------------------------------------- |
| Léxica      | `src/lexer.rs`     | texto → `Vec<Token>`                      |
| Sintáctica  | `src/parser.rs`    | tokens → `AST`                            |
| Semántica   | `src/semantica.rs` | AST → `AST` validado (o error en español) |
| Código      | `src/codegen.rs`   | AST → `temp.cpp`                          |
| Driver      | `src/main.rs`      | orquesta todo + invoca al compilador C++  |

## 2. Compilar el motor

```bash
cd motor
cargo build --release          # binario en target/release/lce
cargo test                     # tests unitarios del lexer y codegen
```

## 3. Uso

```bash
lce archivo.es -o mi_app           # genera temp.cpp y compila el binario
lce archivo.es --no-compilar       # solo genera temp.cpp (para inspeccionar)
lce archivo.es -o /tmp/mi_app
```

## 4. Generación de C++ y compilación optimizada

`codegen.rs` produce un `temp.cpp` autocontenido que:

1. Incluye las cabeceras de Qt6 (`QApplication`, `QMainWindow`,
   `QPushButton`, `QMessageBox`, etc.).
2. Emite una vez las funciones nativas (`crear_ventana`, `crear_boton`,
   `mostrar_mensaje`, `agregar`, `mostrar`, `texto`, `establecer_texto`, …).
3. Emite las **funciones de usuario** (`funcion`): primero los prototipos
   (para permitir llamadas en cualquier orden) y luego las definiciones.
   Los parámetros por referencia (`entero &x`) se emiten como `int& x`.
4. Traduce el bloque `programa` a `int main(...)`, instanciando `QApplication`
   y devolviendo `app.exec()`.

### Verificación semántica (antes del codegen)

`semantica.rs` recorre el AST con una tabla de símbolos por ámbitos y valida:

- Que las variables estén declaradas antes de usarse (y sin duplicados).
- Que las asignaciones y los argumentos respeten los tipos, con conversión
  implícita `entero → decimal`.
- Que las llamadas tengan la aridad correcta.
- Que las condiciones de `si`/`mientras` sean booleanas.
- Que `retornar` coincida con el tipo de retorno de la función.
- Que los parámetros por referencia reciban una variable (no un literal).

Los errores se reportan en español y detienen la compilación antes de generar
C++, evitando mensajes crípticos de `g++`.

La compilación se automatiza con `std::process::Command`:

```rust
comando
    .arg("-std=c++17")   // C++ moderno
    .arg("-O3")          // optimización agresiva
    .arg("-fPIC")        // código independiente de posición
    .arg("-pipe")        // menos archivos temporales
    .arg("temp.cpp")
    .arg("-o").arg(&salida);
```

Las banderas de Qt se obtienen dinámicamente de `pkg-config`:

```rust
pkg-config --cflags --libs Qt6Widgets
```

Esto devuelve algo como `-I/usr/include/x86_64-linux-gnu/qt6 ... -lQt6Widgets`,
que se inyecta directamente en la línea de compilación.

### Bandera `-O3` y rendimiento

- `-O3` habilita vectorización automática (SIMD), inlining agresivo y
  eliminación de código muerto.
- Al compilar a binario nativo (no JIT ni interpretado), el programa resultante
  corre directamente sobre la CPU sin capa intermedia.

### Selección de compilador

`detectar_compilador()` prefiere `clang++` (mejores mensajes de error y
optimizaciones) y cae automáticamente a `g++` si no está instalado.

## 5. Estructura del código Rust

```
motor/
├── Cargo.toml
├── src/
│   ├── main.rs      # CLI, orquestación, llamada a g++/clang++
│   ├── lib.rs       # API pública: verificar() y compilar_a_cpp()
│   ├── token.rs     # tipos de token + tabla de palabras clave
│   ├── lexer.rs     # análisis léxico (con tests)
│   ├── ast.rs       # definición del AST
│   ├── parser.rs    # parser de descenso recursivo (con tests)
│   ├── semantica.rs # verificación de tipos y declaraciones (con tests)
│   └── codegen.rs   # emisión de C++/Qt (con tests)
└── tests/
    └── integracion.rs  # pipeline completo + compilación real
```

## 6. API de librería y tests

El crate expone una API pública reutilizable por herramientas y por los tests
de integración:

```rust
use lce::{verificar, compilar_a_cpp, Fase};

// Solo verificación (léxico + sintaxis + tipos)
let ast = verificar(fuente)?;

// Verificación + generación de C++
match compilar_a_cpp(fuente) {
    Ok(cpp) => { /* ... */ }
    Err(e) => eprintln!("{}: {}", e.etiqueta(), e.mensaje), // "Error semántico: ..."
}
```

### Batería de tests

`cargo test` ejecuta **90 tests**: 79 unitarios repartidos entre los módulos y 11
de integración. Los de integración:

- Compilan los 6 ejemplos de `ejemplos/` a C++ y comprueban fragmentos clave.
- Verifican que cada fase reporta su `Fase` correcta (léxica/sintáctica/semántica).
- Si `g++` y Qt6 están disponibles, **compilan de verdad** el C++ generado y
  comprueban que produce un binario; si no, el test se omite sin fallar.

Comandos útiles:

```bash
cd motor
cargo test
cargo test -- --nocapture
cargo test semantica
cargo clippy --all-targets
```

En la raíz, el `Makefile` ofrece `make test`, `make test-verbose`,
`make ejemplos` y `make todo`.
