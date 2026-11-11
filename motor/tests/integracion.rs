//! Tests de integración del motor `lce`: recorren el pipeline completo
//! (léxico → sintaxis → semántica → generación de C++) usando la API pública.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use lce::{compilar_a_cpp, verificar, Fase};

/// Ruta absoluta a un ejemplo dentro de `ejemplos/`.
fn ejemplo(nombre: &str) -> String {
    let ruta = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("ejemplos")
        .join(format!("{}.es", nombre));
    fs::read_to_string(&ruta).unwrap_or_else(|_| panic!("no se pudo leer {:?}", ruta))
}

#[test]
fn compila_todos_los_ejemplos() {
    let nombres = [
        "ventana_boton",
        "hola_mundo",
        "funciones",
        "bucles",
        "formulario",
        "referencias",
    ];

    for nombre in nombres {
        let fuente = ejemplo(nombre);
        let cpp = compilar_a_cpp(&fuente).unwrap_or_else(|e| {
            panic!("el ejemplo '{}' falló ({}): {}", nombre, e.etiqueta(), e.mensaje)
        });
        assert!(cpp.contains("int main("), "ejemplo '{}' sin main", nombre);
        assert!(
            cpp.contains("QApplication"),
            "ejemplo '{}' sin QApplication",
            nombre
        );
        assert!(
            cpp.contains("return app.exec();"),
            "ejemplo '{}' sin bucle de eventos",
            nombre
        );
    }
}

#[test]
fn ventana_boton_genera_evento() {
    let cpp = compilar_a_cpp(&ejemplo("ventana_boton")).unwrap();
    assert!(cpp.contains("crear_ventana("));
    assert!(cpp.contains("crear_boton("));
    assert!(cpp.contains("QObject::connect("));
    assert!(cpp.contains("mostrar_mensaje("));
}

#[test]
fn formulario_lee_texto_de_la_caja() {
    let cpp = compilar_a_cpp(&ejemplo("formulario")).unwrap();
    assert!(cpp.contains("texto(nombre)"));
    assert!(cpp.contains("QString(\"Hola, \")"));
}

#[test]
fn referencias_genera_parametro_por_referencia() {
    let cpp = compilar_a_cpp(&ejemplo("referencias")).unwrap();
    assert!(cpp.contains("int& valor"));
}

#[test]
fn funciones_genera_prototipos_y_definiciones() {
    let cpp = compilar_a_cpp(&ejemplo("funciones")).unwrap();
    assert!(cpp.contains("int suma(int a, int b);"));
    assert!(cpp.contains("int suma(int a, int b) {"));
    assert!(cpp.contains("void saludar(QString nombre)"));
}

#[test]
fn bucles_genera_for_y_else_if() {
    let cpp = compilar_a_cpp(&ejemplo("bucles")).unwrap();
    assert!(cpp.contains("for (int i = 1; i < 6; i++)"));
    assert!(cpp.contains("} else if ("));
}

// ---------- Diagnóstico por fase ----------

#[test]
fn error_lexico_reporta_fase() {
    let error = compilar_a_cpp("programa P { \"sin cerrar }").unwrap_err();
    assert_eq!(error.fase, Fase::Lexica);
    assert_eq!(error.etiqueta(), "Error léxico");
}

#[test]
fn error_sintactico_reporta_fase() {
    let error = compilar_a_cpp("programa P { si a { } }").unwrap_err();
    assert_eq!(error.fase, Fase::Sintactica);
    assert_eq!(error.etiqueta(), "Error de sintaxis");
}

#[test]
fn error_semantico_reporta_fase() {
    let error = compilar_a_cpp("programa P { entero x = \"a\" }").unwrap_err();
    assert_eq!(error.fase, Fase::Semantica);
    assert_eq!(error.etiqueta(), "Error semántico");
    assert!(error.mensaje.contains("No se puede asignar"));
}

#[test]
fn verificar_devuelve_ast_sin_generar_codigo() {
    let programa = verificar(&ejemplo("hola_mundo")).unwrap();
    assert_eq!(programa.nombre, "HolaMundo");
    assert!(!programa.declaraciones.is_empty());
}

// ---------- Compilación real con g++/Qt (se omite si no están) ----------

#[test]
fn compila_cpp_real_si_hay_gpp_y_qt() {
    let tiene_gpp = Command::new("g++")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let tiene_qt = Command::new("pkg-config")
        .args(["--exists", "Qt6Widgets"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !(tiene_gpp && tiene_qt) {
        eprintln!("omitiendo compilación real: g++ o Qt6 no disponibles");
        return;
    }

    let cpp = compilar_a_cpp(&ejemplo("ventana_boton")).unwrap();

    let dir = std::env::temp_dir().join(format!("lce_integracion_{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let cpp_path = dir.join("temp.cpp");
    let exe_path = dir.join("app");
    fs::write(&cpp_path, &cpp).unwrap();

    let banderas = Command::new("pkg-config")
        .args(["--cflags", "--libs", "Qt6Widgets"])
        .output()
        .unwrap();
    let banderas = String::from_utf8_lossy(&banderas.stdout);

    let mut comando = Command::new("g++");
    comando
        .arg("-std=c++17")
        .arg("-O2")
        .arg(&cpp_path)
        .arg("-o")
        .arg(&exe_path);
    for bandera in banderas.split_whitespace() {
        comando.arg(bandera);
    }

    let salida = comando.output().unwrap();
    assert!(
        salida.status.success(),
        "g++ falló:\n{}",
        String::from_utf8_lossy(&salida.stderr)
    );
    assert!(exe_path.exists());

    let _ = fs::remove_dir_all(&dir);
}
