//! Motor del lenguaje **LCE (Qt)**.
//!
//! Expone las fases del compilador (léxica, sintáctica, semántica y generación
//! de código) para poder reutilizarlas desde el binario `lce`, desde tests de
//! integración y desde herramientas externas.

pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod semantica;
pub mod token;

use ast::Programa;

/// Fase del compilador en la que se produjo un error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fase {
    Lexica,
    Sintactica,
    Semantica,
}

/// Error de compilación con la fase en la que ocurrió.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorCompilacion {
    pub fase: Fase,
    pub mensaje: String,
}

impl ErrorCompilacion {
    fn nueva(fase: Fase, mensaje: impl Into<String>) -> Self {
        Self {
            fase,
            mensaje: mensaje.into(),
        }
    }

    /// Etiqueta legible en español para mostrar al usuario.
    pub fn etiqueta(&self) -> &'static str {
        match self.fase {
            Fase::Lexica => "Error léxico",
            Fase::Sintactica => "Error de sintaxis",
            Fase::Semantica => "Error semántico",
        }
    }
}

/// Ejecuta las fases léxica, sintáctica y semántica y devuelve el AST validado.
pub fn verificar(fuente: &str) -> Result<Programa, ErrorCompilacion> {
    let tokens = lexer::Lexer::new(fuente)
        .tokenizar()
        .map_err(|e| ErrorCompilacion::nueva(Fase::Lexica, e))?;

    let mut parser = parser::Parser::new(tokens);
    let programa = parser
        .parsear()
        .map_err(|e| ErrorCompilacion::nueva(Fase::Sintactica, e))?;

    semantica::Analizador::analizar(&programa)
        .map_err(|e| ErrorCompilacion::nueva(Fase::Semantica, e))?;

    Ok(programa)
}

/// Compila código fuente `.es` a C++/Qt (sin invocar al compilador C++).
pub fn compilar_a_cpp(fuente: &str) -> Result<String, ErrorCompilacion> {
    let programa = verificar(fuente)?;
    let mut generador = codegen::Generador::new();
    Ok(generador.generar(&programa))
}
