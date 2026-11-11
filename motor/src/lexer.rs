//! Analizador léxico: convierte el código fuente `.es` en una lista de tokens.

use crate::token::{palabra_clave, TipoToken, Token};

pub struct Lexer {
    caracteres: Vec<char>,
    posicion: usize,
    linea: usize,
    columna: usize,
}

impl Lexer {
    pub fn new(fuente: &str) -> Self {
        Self {
            caracteres: fuente.chars().collect(),
            posicion: 0,
            linea: 1,
            columna: 1,
        }
    }

    fn actual(&self) -> Option<char> {
        self.caracteres.get(self.posicion).copied()
    }

    fn siguiente(&self) -> Option<char> {
        self.caracteres.get(self.posicion + 1).copied()
    }

    fn avanzar(&mut self) -> Option<char> {
        let c = self.actual()?;
        self.posicion += 1;
        if c == '\n' {
            self.linea += 1;
            self.columna = 1;
        } else {
            self.columna += 1;
        }
        Some(c)
    }

    /// Produce la lista completa de tokens.
    pub fn tokenizar(mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            self.saltar_trivia();

            let linea = self.linea;
            let columna = self.columna;

            let tipo = match self.actual() {
                None => {
                    tokens.push(Token {
                        tipo: TipoToken::Eof,
                        linea,
                        columna,
                    });
                    break;
                }
                Some('\n') => {
                    self.avanzar();
                    TipoToken::NuevaLinea
                }
                Some(c) if c.is_ascii_digit() => TipoToken::Numero(self.leer_numero()),
                Some(c) if c.is_alphabetic() || c == '_' => self.leer_identificador_o_palabra(),
                Some('"') => TipoToken::Texto(self.leer_cadena()?),
                Some(c) => {
                    self.avanzar();
                    match c {
                        '{' => TipoToken::LlaveIzq,
                        '}' => TipoToken::LlaveDer,
                        '(' => TipoToken::ParenIzq,
                        ')' => TipoToken::ParenDer,
                        '[' => TipoToken::CorcheteIzq,
                        ']' => TipoToken::CorcheteDer,
                        '=' => {
                            if self.actual() == Some('=') {
                                self.avanzar();
                                TipoToken::IgualIgual
                            } else {
                                TipoToken::Igual
                            }
                        }
                        ':' => TipoToken::DosPuntos,
                        ';' => TipoToken::PuntoYComa,
                        ',' => TipoToken::Coma,
                        '&' => TipoToken::Ampersand,
                        '+' => TipoToken::Mas,
                        '-' => {
                            if self.actual() == Some('>') {
                                self.avanzar();
                                TipoToken::Flecha
                            } else {
                                TipoToken::Menos
                            }
                        }
                        '*' => TipoToken::Por,
                        '/' => TipoToken::Entre,
                        '%' => TipoToken::Modulo,
                        '<' => {
                            if self.actual() == Some('=') {
                                self.avanzar();
                                TipoToken::MenorIgual
                            } else {
                                TipoToken::Menor
                            }
                        }
                        '>' => {
                            if self.actual() == Some('=') {
                                self.avanzar();
                                TipoToken::MayorIgual
                            } else {
                                TipoToken::Mayor
                            }
                        }
                        '!' => {
                            if self.actual() == Some('=') {
                                self.avanzar();
                                TipoToken::Distinto
                            } else {
                                TipoToken::No
                            }
                        }
                        _ => {
                            return Err(format!(
                                "Carácter inesperado '{}' en la línea {} columna {}",
                                c, linea, columna
                            ))
                        }
                    }
                }
            };

            tokens.push(Token {
                tipo,
                linea,
                columna,
            });
        }

        Ok(tokens)
    }

    /// Ignora espacios, tabulaciones y comentarios (`//` y `#`).
    /// El salto de línea (`\n`) NO se ignora: se emite como token `NuevaLinea`.
    fn saltar_trivia(&mut self) {
        loop {
            match self.actual() {
                Some(' ') | Some('\t') | Some('\r') => {
                    self.avanzar();
                }
                Some('/') if self.siguiente() == Some('/') => {
                    while let Some(ch) = self.actual() {
                        if ch == '\n' {
                            break;
                        }
                        self.avanzar();
                    }
                }
                Some('#') => {
                    while let Some(ch) = self.actual() {
                        if ch == '\n' {
                            break;
                        }
                        self.avanzar();
                    }
                }
                _ => break,
            }
        }
    }

    fn leer_numero(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.actual() {
            if c.is_ascii_digit() || c == '.' {
                s.push(c);
                self.avanzar();
            } else {
                break;
            }
        }
        s
    }

    fn leer_identificador_o_palabra(&mut self) -> TipoToken {
        let mut s = String::new();
        while let Some(c) = self.actual() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.avanzar();
            } else {
                break;
            }
        }
        // Si coincide con una palabra clave se usa el token reservado;
        // en caso contrario es un identificador (variable o función nativa).
        palabra_clave(&s).unwrap_or(TipoToken::Identificador(s))
    }

    fn leer_cadena(&mut self) -> Result<String, String> {
        self.avanzar(); // comilla de apertura
        let mut s = String::new();
        while let Some(c) = self.actual() {
            match c {
                '"' => {
                    self.avanzar();
                    return Ok(s);
                }
                '\\' => {
                    self.avanzar();
                    match self.actual() {
                        Some('n') => s.push('\n'),
                        Some('t') => s.push('\t'),
                        Some('"') => s.push('"'),
                        Some('\\') => s.push('\\'),
                        Some(otro) => {
                            s.push('\\');
                            s.push(otro);
                        }
                        None => break,
                    }
                    self.avanzar();
                }
                '\n' => return Err("Cadena de texto sin cerrar (salto de línea)".into()),
                c => {
                    s.push(c);
                    self.avanzar();
                }
            }
        }
        Err("Cadena de texto sin cerrar (falta la comilla de cierre)".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Devuelve solo los tipos de token (incluye `Eof`).
    fn tipos(fuente: &str) -> Vec<TipoToken> {
        Lexer::new(fuente)
            .tokenizar()
            .unwrap()
            .into_iter()
            .map(|t| t.tipo)
            .collect()
    }

    #[test]
    fn reconoce_todas_las_palabras_clave() {
        let casos = [
            ("programa", TipoToken::Programa),
            ("funcion", TipoToken::Funcion),
            ("si", TipoToken::Si),
            ("sino", TipoToken::Sino),
            ("mientras", TipoToken::Mientras),
            ("para", TipoToken::Para),
            ("desde", TipoToken::Desde),
            ("hasta", TipoToken::Hasta),
            ("retornar", TipoToken::Retornar),
            ("entero", TipoToken::Entero),
            ("decimal", TipoToken::Decimal),
            ("booleano", TipoToken::Booleano),
            ("cadena", TipoToken::Cadena),
            ("nada", TipoToken::Nada),
            ("ventana", TipoToken::Ventana),
            ("boton", TipoToken::Boton),
            ("etiqueta", TipoToken::Etiqueta),
            ("caja_texto", TipoToken::CajaTexto),
            ("verdadero", TipoToken::Verdadero),
            ("falso", TipoToken::Falso),
            ("al", TipoToken::Al),
            ("hacer_clic", TipoToken::HacerClic),
            ("y", TipoToken::Y),
            ("o", TipoToken::O),
            ("no", TipoToken::No),
        ];
        for (texto, esperado) in casos {
            assert_eq!(tipos(texto), vec![esperado, TipoToken::Eof], "palabra: {}", texto);
        }
    }

    #[test]
    fn reconoce_operadores_y_simbolos() {
        let casos = [
            ("{", TipoToken::LlaveIzq),
            ("}", TipoToken::LlaveDer),
            ("(", TipoToken::ParenIzq),
            (")", TipoToken::ParenDer),
            ("[", TipoToken::CorcheteIzq),
            ("]", TipoToken::CorcheteDer),
            ("=", TipoToken::Igual),
            ("==", TipoToken::IgualIgual),
            ("!=", TipoToken::Distinto),
            ("<", TipoToken::Menor),
            ("<=", TipoToken::MenorIgual),
            (">", TipoToken::Mayor),
            (">=", TipoToken::MayorIgual),
            ("+", TipoToken::Mas),
            ("-", TipoToken::Menos),
            ("*", TipoToken::Por),
            ("/", TipoToken::Entre),
            ("%", TipoToken::Modulo),
            ("->", TipoToken::Flecha),
            ("&", TipoToken::Ampersand),
            (",", TipoToken::Coma),
            (";", TipoToken::PuntoYComa),
            (":", TipoToken::DosPuntos),
        ];
        for (texto, esperado) in casos {
            assert_eq!(tipos(texto), vec![esperado, TipoToken::Eof], "símbolo: {}", texto);
        }
    }

    #[test]
    fn emite_nueva_linea_por_cada_salto() {
        let t = tipos("a\nb\n\nc");
        assert_eq!(
            t,
            vec![
                TipoToken::Identificador("a".into()),
                TipoToken::NuevaLinea,
                TipoToken::Identificador("b".into()),
                TipoToken::NuevaLinea,
                TipoToken::NuevaLinea,
                TipoToken::Identificador("c".into()),
                TipoToken::Eof,
            ]
        );
    }

    #[test]
    fn ignora_comentarios_de_linea() {
        let t = tipos("// comentario\n# otro\na");
        assert_eq!(
            t,
            vec![
                TipoToken::NuevaLinea,
                TipoToken::NuevaLinea,
                TipoToken::Identificador("a".into()),
                TipoToken::Eof,
            ]
        );
    }

    #[test]
    fn reconoce_numeros_enteros_y_decimales() {
        assert_eq!(tipos("42")[0], TipoToken::Numero("42".into()));
        assert_eq!(tipos("3.14")[0], TipoToken::Numero("3.14".into()));
    }

    #[test]
    fn reconoce_cadenas_con_escapes() {
        assert_eq!(tipos("\"hola\"")[0], TipoToken::Texto("hola".into()));
        assert_eq!(tipos("\"a\\nb\"")[0], TipoToken::Texto("a\nb".into()));
        assert_eq!(tipos("\"comillas \\\"\"")[0], TipoToken::Texto("comillas \"".into()));
    }

    #[test]
    fn reconoce_identificadores_acentuados() {
        assert_eq!(tipos("número")[0], TipoToken::Identificador("número".into()));
        assert_eq!(tipos("_oculto")[0], TipoToken::Identificador("_oculto".into()));
    }

    #[test]
    fn registra_linea_y_columna() {
        let tokens = Lexer::new("a\nb").tokenizar().unwrap();
        assert_eq!(tokens[0].linea, 1);
        assert_eq!(tokens[0].columna, 1);
        assert_eq!(tokens[2].tipo, TipoToken::Identificador("b".into()));
        assert_eq!(tokens[2].linea, 2);
        assert_eq!(tokens[2].columna, 1);
    }

    #[test]
    fn falla_con_cadena_sin_cerrar() {
        assert!(Lexer::new("\"abc").tokenizar().is_err());
    }

    #[test]
    fn falla_con_caracter_desconocido() {
        assert!(Lexer::new("@").tokenizar().is_err());
    }
}
