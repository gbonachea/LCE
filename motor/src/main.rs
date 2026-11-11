//! Punto de entrada del transpilador `lce`.
//!
//! Flujo: archivo `.es` -> verificación (léxico/sintaxis/tipos) -> C++ ->
//! compilador nativo con `-O3`.

use std::env;
use std::fs;
use std::process::{exit, Command};

fn main() {
    let argumentos: Vec<String> = env::args().collect();

    if argumentos.len() < 2 {
        imprimir_uso();
        exit(1);
    }

    if argumentos.iter().any(|a| a == "-h" || a == "--help") {
        imprimir_uso();
        return;
    }

    let archivo = &argumentos[1];
    let no_compilar = argumentos.iter().any(|a| a == "--no-compilar");
    let solo_verificar = argumentos.iter().any(|a| a == "--solo-verificar");
    let salida = argumentos
        .iter()
        .position(|a| a == "-o")
        .and_then(|i| argumentos.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "programa".to_string());

    // 1. Leer el archivo fuente.
    let fuente = match fs::read_to_string(archivo) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("No se pudo leer '{}': {}", archivo, e);
            exit(1);
        }
    };

    // 2. Verificación: léxico + sintaxis + semántica.
    let programa = match lce::verificar(&fuente) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {}", e.etiqueta(), e.mensaje);
            exit(1);
        }
    };
    println!("✔ Verificación semántica correcta.");

    if solo_verificar {
        return;
    }

    // 3. Generación de C++.
    let mut generador = lce::codegen::Generador::new();
    let codigo_cpp = generador.generar(&programa);

    if let Err(e) = fs::write("temp.cpp", &codigo_cpp) {
        eprintln!("No se pudo escribir temp.cpp: {}", e);
        exit(1);
    }
    println!("✔ temp.cpp generado correctamente.");

    if no_compilar {
        return;
    }

    // 4. Compilación nativa con optimización -O3.
    let compilador = detectar_compilador();
    println!("Compilando con {} ...", compilador);

    let mut comando = Command::new(compilador);
    comando
        .arg("-std=c++17")
        .arg("-O3")
        .arg("-fPIC")
        .arg("-pipe")
        .arg("temp.cpp")
        .arg("-o")
        .arg(&salida);

    for bandera in banderas_qt() {
        comando.arg(bandera);
    }

    let estado = match comando.status() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("No se pudo ejecutar el compilador '{}': {}", compilador, e);
            exit(1);
        }
    };

    if !estado.success() {
        eprintln!("La compilación de C++ falló.");
        exit(1);
    }

    println!("✔ Binario nativo generado: {}", salida);
}

fn imprimir_uso() {
    println!("Uso: lce <archivo.es> [opciones]");
    println!();
    println!("Opciones:");
    println!("  -o <salida>       Nombre del binario resultante (por defecto: programa)");
    println!("  --no-compilar     Solo genera temp.cpp, sin invocar al compilador");
    println!("  --solo-verificar  Analiza el código (léxico, sintaxis y tipos) sin generar nada");
    println!("  -h, --help        Muestra esta ayuda");
}

/// Prefiere `clang++`; si no está disponible, usa `g++`.
fn detectar_compilador() -> &'static str {
    let clang_disponible = Command::new("clang++")
        .arg("--version")
        .output()
        .map(|salida| salida.status.success())
        .unwrap_or(false);

    if clang_disponible {
        "clang++"
    } else {
        "g++"
    }
}

/// Obtiene las banderas de compilación y enlazado de Qt6 desde `pkg-config`.
fn banderas_qt() -> Vec<String> {
    let salida = Command::new("pkg-config")
        .args(["--cflags", "--libs", "Qt6Widgets"])
        .output();

    match salida {
        Ok(salida) if salida.status.success() => {
            let texto = String::from_utf8_lossy(&salida.stdout);
            texto.split_whitespace().map(|s| s.to_string()).collect()
        }
        _ => {
            eprintln!("Aviso: no se pudieron obtener las banderas de Qt6 via pkg-config.");
            Vec::new()
        }
    }
}
