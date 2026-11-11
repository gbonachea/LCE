//! Definición de tokens del lenguaje LCE (Qt).

/// Tipos de token reconocidos por el lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum TipoToken {
    // ---- Literales ----
    Identificador(String),
    Numero(String),
    Texto(String),

    // ---- Palabras clave ----
    Programa,
    Funcion,
    Si,
    Sino,
    Mientras,
    Para,
    Retornar,
    // Tipos
    Entero,
    Decimal,
    Booleano,
    Cadena,
    Nada,
    // Widgets / tipos de interfaz
    Ventana,
    Boton,
    Etiqueta,
    CajaTexto,
    // Literales booleanos
    Verdadero,
    Falso,
    // Eventos
    Al,
    HacerClic,
    // Bucles
    Desde,
    Hasta,

    // ---- Símbolos ----
    NuevaLinea,    // salto de línea (termina una sentencia)
    LlaveIzq,      // {
    LlaveDer,      // }
    ParenIzq,      // (
    ParenDer,      // )
    CorcheteIzq,   // [
    CorcheteDer,   // ]
    Igual,         // =
    DosPuntos,     // :
    PuntoYComa,    // ;
    Coma,          // ,
    Flecha,        // ->
    Ampersand,     // &
    Mas,           // +
    Menos,         // -
    Por,           // *
    Entre,         // /
    Modulo,        // %
    Menor,         // <
    Mayor,         // >
    MenorIgual,    // <=
    MayorIgual,    // >=
    IgualIgual,    // ==
    Distinto,      // !=
    Y,             // y  (&&)
    O,             // o  (||)
    No,            // no (!)

    Eof,
}

/// Token con información de posición para mensajes de error.
#[derive(Debug, Clone)]
pub struct Token {
    pub tipo: TipoToken,
    pub linea: usize,
    pub columna: usize,
}

/// Devuelve el token de palabra clave correspondiente a `texto`, o `None`.
pub fn palabra_clave(texto: &str) -> Option<TipoToken> {
    Some(match texto {
        "programa" => TipoToken::Programa,
        "funcion" => TipoToken::Funcion,
        "si" => TipoToken::Si,
        "sino" => TipoToken::Sino,
        "mientras" => TipoToken::Mientras,
        "para" => TipoToken::Para,
        "retornar" => TipoToken::Retornar,
        "entero" => TipoToken::Entero,
        "decimal" => TipoToken::Decimal,
        "booleano" => TipoToken::Booleano,
        "cadena" => TipoToken::Cadena,
        "nada" => TipoToken::Nada,
        "ventana" => TipoToken::Ventana,
        "boton" => TipoToken::Boton,
        "etiqueta" => TipoToken::Etiqueta,
        "caja_texto" => TipoToken::CajaTexto,
        "verdadero" => TipoToken::Verdadero,
        "falso" => TipoToken::Falso,
        "al" => TipoToken::Al,
        "hacer_clic" => TipoToken::HacerClic,
        "desde" => TipoToken::Desde,
        "hasta" => TipoToken::Hasta,
        "y" => TipoToken::Y,
        "o" => TipoToken::O,
        "no" => TipoToken::No,
        _ => return None,
    })
}
