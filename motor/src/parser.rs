//! Analizador sintáctico (descenso recursivo): tokens -> AST.

use crate::ast::{Declaracion, Expr, Operador, OperadorUnario, Parametro, Programa, Tipo};
use crate::token::{TipoToken, Token};

pub struct Parser {
    tokens: Vec<Token>,
    posicion: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, posicion: 0 }
    }

    fn actual(&self) -> &TipoToken {
        &self.tokens[self.posicion].tipo
    }

    fn avanzar(&mut self) -> Token {
        let token = self.tokens[self.posicion].clone();
        if self.posicion < self.tokens.len() - 1 {
            self.posicion += 1;
        }
        token
    }

    fn esperar(&mut self, esperado: &TipoToken, mensaje: &str) -> Result<Token, String> {
        if self.actual() == esperado {
            Ok(self.avanzar())
        } else {
            Err(format!("{} (se encontró {:?})", mensaje, self.actual()))
        }
    }

    /// Omite los tokens de salto de línea pendientes (líneas en blanco).
    fn omitir_nuevas_lineas(&mut self) {
        while *self.actual() == TipoToken::NuevaLinea {
            self.avanzar();
        }
    }

    /// Consume el terminador de sentencia: `;` o un salto de línea.
    /// También acepta `}` o fin de archivo (última sentencia del bloque).
    fn consumir_terminador(&mut self) -> Result<(), String> {
        match self.actual().clone() {
            TipoToken::PuntoYComa => {
                self.avanzar();
            }
            TipoToken::NuevaLinea | TipoToken::LlaveDer | TipoToken::Eof => {}
            _ => {
                return Err(format!(
                    "Se esperaba ';' o fin de línea (se encontró {:?})",
                    self.actual()
                ))
            }
        }
        self.omitir_nuevas_lineas();
        Ok(())
    }

    /// Punto de entrada: `programa Nombre { ... }`.
    pub fn parsear(&mut self) -> Result<Programa, String> {
        self.omitir_nuevas_lineas();
        self.esperar(&TipoToken::Programa, "Se esperaba 'programa'")?;

        let nombre = match self.avanzar().tipo {
            TipoToken::Identificador(n) => n,
            _ => return Err("Se esperaba el nombre del programa".into()),
        };

        self.esperar(&TipoToken::LlaveIzq, "Se esperaba '{'")?;

        let mut declaraciones = Vec::new();
        loop {
            self.omitir_nuevas_lineas();
            if *self.actual() == TipoToken::LlaveDer {
                break;
            }
            declaraciones.push(self.declaracion()?);
        }

        self.esperar(&TipoToken::LlaveDer, "Se esperaba '}'")?;
        Ok(Programa { nombre, declaraciones })
    }

    fn declaracion(&mut self) -> Result<Declaracion, String> {
        match self.actual().clone() {
            TipoToken::Entero
            | TipoToken::Decimal
            | TipoToken::Booleano
            | TipoToken::Cadena
            | TipoToken::Ventana
            | TipoToken::Boton
            | TipoToken::Etiqueta
            | TipoToken::CajaTexto => self.declaracion_variable(),
            TipoToken::Si => self.si(),
            TipoToken::Mientras => self.mientras(),
            TipoToken::Para => self.para(),
            TipoToken::Retornar => self.retornar(),
            TipoToken::Al => self.manejador_clic(),
            TipoToken::Funcion => self.funcion(),
            TipoToken::Identificador(_) => self.asignacion_o_expresion(),
            _ => Err(format!("Declaración inesperada: {:?}", self.actual())),
        }
    }

    fn tipo_actual(&self) -> Option<Tipo> {
        Some(match self.actual() {
            TipoToken::Entero => Tipo::Entero,
            TipoToken::Decimal => Tipo::Decimal,
            TipoToken::Booleano => Tipo::Booleano,
            TipoToken::Cadena => Tipo::Cadena,
            TipoToken::Ventana => Tipo::Ventana,
            TipoToken::Boton => Tipo::Boton,
            TipoToken::Etiqueta => Tipo::Etiqueta,
            TipoToken::CajaTexto => Tipo::CajaTexto,
            _ => return None,
        })
    }

    fn declaracion_variable(&mut self) -> Result<Declaracion, String> {
        let tipo = self.tipo_actual().expect("tipo válido");
        self.avanzar();

        let nombre = match self.avanzar().tipo {
            TipoToken::Identificador(n) => n,
            _ => return Err("Se esperaba un nombre de variable".into()),
        };

        self.esperar(&TipoToken::Igual, "Se esperaba '='")?;
        let valor = self.expresion()?;
        self.consumir_terminador()?;

        Ok(Declaracion::Variable { nombre, tipo, valor })
    }

    fn si(&mut self) -> Result<Declaracion, String> {
        self.avanzar(); // 'si'
        self.si_resto()
    }

    /// Parsea `(condición) bloque [ sino bloque | sino si ... ]`
    /// asumiendo que la palabra `si` ya fue consumida.
    fn si_resto(&mut self) -> Result<Declaracion, String> {
        self.esperar(&TipoToken::ParenIzq, "Se esperaba '(' después de 'si'")?;
        let condicion = self.expresion()?;
        self.esperar(&TipoToken::ParenDer, "Se esperaba ')'")?;

        let entonces = self.bloque()?;
        let mut sino = Vec::new();

        if *self.actual() == TipoToken::Sino {
            self.avanzar();
            if *self.actual() == TipoToken::Si {
                self.avanzar(); // consume 'si'
                // 'sino si (...)': se anida como una rama Si dentro de 'sino'.
                sino = vec![self.si_resto()?];
            } else {
                sino = self.bloque()?;
            }
        }

        Ok(Declaracion::Si { condicion, entonces, sino })
    }

    /// `para i desde inicio hasta fin { ... }`
    fn para(&mut self) -> Result<Declaracion, String> {
        self.avanzar(); // 'para'

        let variable = match self.avanzar().tipo {
            TipoToken::Identificador(n) => n,
            _ => return Err("Se esperaba el nombre de la variable del bucle".into()),
        };

        self.esperar(&TipoToken::Desde, "Se esperaba 'desde'")?;
        let inicio = self.expresion()?;
        self.esperar(&TipoToken::Hasta, "Se esperaba 'hasta'")?;
        let fin = self.expresion()?;

        let cuerpo = self.bloque()?;
        Ok(Declaracion::Para { variable, inicio, fin, cuerpo })
    }

    fn mientras(&mut self) -> Result<Declaracion, String> {
        self.avanzar(); // 'mientras'
        self.esperar(&TipoToken::ParenIzq, "Se esperaba '(' después de 'mientras'")?;
        let condicion = self.expresion()?;
        self.esperar(&TipoToken::ParenDer, "Se esperaba ')'")?;

        let cuerpo = self.bloque()?;
        Ok(Declaracion::Mientras { condicion, cuerpo })
    }

    fn retornar(&mut self) -> Result<Declaracion, String> {
        self.avanzar(); // 'retornar'
        let valor = match self.actual() {
            TipoToken::PuntoYComa | TipoToken::NuevaLinea | TipoToken::LlaveDer => None,
            _ => Some(self.expresion()?),
        };
        self.consumir_terminador()?;
        Ok(Declaracion::Retornar(valor))
    }

    fn manejador_clic(&mut self) -> Result<Declaracion, String> {
        self.avanzar(); // 'al'
        self.esperar(&TipoToken::HacerClic, "Se esperaba 'hacer_clic'")?;
        self.esperar(&TipoToken::ParenIzq, "Se esperaba '('")?;

        let widget = match self.avanzar().tipo {
            TipoToken::Identificador(n) => n,
            _ => return Err("Se esperaba el nombre del widget".into()),
        };

        self.esperar(&TipoToken::ParenDer, "Se esperaba ')'")?;
        let cuerpo = self.bloque()?;

        Ok(Declaracion::ManejarClic { widget, cuerpo })
    }

    /// `funcion nombre(tipo param, ...) [-> tipo] { ... }`
    fn funcion(&mut self) -> Result<Declaracion, String> {
        self.avanzar(); // 'funcion'

        let nombre = match self.avanzar().tipo {
            TipoToken::Identificador(n) => n,
            _ => return Err("Se esperaba el nombre de la función".into()),
        };

        self.esperar(&TipoToken::ParenIzq, "Se esperaba '(' después del nombre de la función")?;

        let mut parametros = Vec::new();
        if *self.actual() != TipoToken::ParenDer {
            loop {
                let tipo = self.tipo_actual().ok_or_else(|| {
                    format!("Se esperaba el tipo del parámetro (se encontró {:?})", self.actual())
                })?;
                self.avanzar();

                // Paso por referencia opcional: `entero &x`
                let por_referencia = if *self.actual() == TipoToken::Ampersand {
                    self.avanzar();
                    true
                } else {
                    false
                };

                let nombre_parametro = match self.avanzar().tipo {
                    TipoToken::Identificador(n) => n,
                    _ => return Err("Se esperaba el nombre del parámetro".into()),
                };

                parametros.push(Parametro {
                    nombre: nombre_parametro,
                    tipo,
                    por_referencia,
                });

                if *self.actual() != TipoToken::Coma {
                    break;
                }
                self.avanzar();
            }
        }
        self.esperar(&TipoToken::ParenDer, "Se esperaba ')'")?;

        // Tipo de retorno opcional: por defecto 'nada' (void).
        let retorno = if *self.actual() == TipoToken::Flecha {
            self.avanzar();
            self.tipo_o_nada()?
        } else {
            Tipo::Nada
        };

        let cuerpo = self.bloque()?;
        Ok(Declaracion::Funcion { nombre, parametros, retorno, cuerpo })
    }

    /// Lee un tipo de retorno, incluyendo `nada`.
    fn tipo_o_nada(&mut self) -> Result<Tipo, String> {
        if *self.actual() == TipoToken::Nada {
            self.avanzar();
            return Ok(Tipo::Nada);
        }
        match self.tipo_actual() {
            Some(tipo) => {
                self.avanzar();
                Ok(tipo)
            }
            None => Err(format!("Se esperaba un tipo de retorno (se encontró {:?})", self.actual())),
        }
    }

    fn bloque(&mut self) -> Result<Vec<Declaracion>, String> {
        self.omitir_nuevas_lineas();
        self.esperar(&TipoToken::LlaveIzq, "Se esperaba '{'")?;
        let mut declaraciones = Vec::new();
        loop {
            self.omitir_nuevas_lineas();
            if *self.actual() == TipoToken::LlaveDer {
                break;
            }
            declaraciones.push(self.declaracion()?);
        }
        self.esperar(&TipoToken::LlaveDer, "Se esperaba '}'")?;
        Ok(declaraciones)
    }

    fn asignacion_o_expresion(&mut self) -> Result<Declaracion, String> {
        let nombre = match self.avanzar().tipo {
            TipoToken::Identificador(n) => n,
            _ => unreachable!("ya validado como Identificador"),
        };

        if *self.actual() == TipoToken::Igual {
            self.avanzar();
            let valor = self.expresion()?;
            self.consumir_terminador()?;
            Ok(Declaracion::Asignacion { nombre, valor })
        } else {
            let expr = self.expresion_desde(nombre)?;
            self.consumir_terminador()?;
            Ok(Declaracion::Expresion(expr))
        }
    }

    /// Continúa parseando una expresión que ya empezó con el identificador `nombre`.
    fn expresion_desde(&mut self, nombre: String) -> Result<Expr, String> {
        if *self.actual() == TipoToken::ParenIzq {
            self.avanzar();
            let mut argumentos = Vec::new();
            if *self.actual() != TipoToken::ParenDer {
                loop {
                    argumentos.push(self.expresion()?);
                    if *self.actual() != TipoToken::Coma {
                        break;
                    }
                    self.avanzar();
                }
            }
            self.esperar(&TipoToken::ParenDer, "Se esperaba ')'")?;
            Ok(Expr::Llamada { nombre, argumentos })
        } else {
            Ok(Expr::Variable(nombre))
        }
    }

    fn expresion(&mut self) -> Result<Expr, String> {
        self.logico_o()
    }

    fn logico_o(&mut self) -> Result<Expr, String> {
        let mut izquierda = self.logico_y()?;
        while *self.actual() == TipoToken::O {
            self.avanzar();
            let derecha = self.logico_y()?;
            izquierda = Expr::Binario(Box::new(izquierda), Operador::O, Box::new(derecha));
        }
        Ok(izquierda)
    }

    fn logico_y(&mut self) -> Result<Expr, String> {
        let mut izquierda = self.igualdad()?;
        while *self.actual() == TipoToken::Y {
            self.avanzar();
            let derecha = self.igualdad()?;
            izquierda = Expr::Binario(Box::new(izquierda), Operador::Y, Box::new(derecha));
        }
        Ok(izquierda)
    }

    fn igualdad(&mut self) -> Result<Expr, String> {
        let mut izquierda = self.comparacion()?;
        loop {
            let operador = match self.actual() {
                TipoToken::IgualIgual => Operador::IgualIgual,
                TipoToken::Distinto => Operador::Distinto,
                _ => break,
            };
            self.avanzar();
            let derecha = self.comparacion()?;
            izquierda = Expr::Binario(Box::new(izquierda), operador, Box::new(derecha));
        }
        Ok(izquierda)
    }

    fn comparacion(&mut self) -> Result<Expr, String> {
        let mut izquierda = self.termino()?;
        loop {
            let operador = match self.actual() {
                TipoToken::Menor => Operador::Menor,
                TipoToken::Mayor => Operador::Mayor,
                TipoToken::MenorIgual => Operador::MenorIgual,
                TipoToken::MayorIgual => Operador::MayorIgual,
                _ => break,
            };
            self.avanzar();
            let derecha = self.termino()?;
            izquierda = Expr::Binario(Box::new(izquierda), operador, Box::new(derecha));
        }
        Ok(izquierda)
    }

    fn termino(&mut self) -> Result<Expr, String> {
        let mut izquierda = self.factor()?;
        loop {
            let operador = match self.actual() {
                TipoToken::Mas => Operador::Suma,
                TipoToken::Menos => Operador::Resta,
                _ => break,
            };
            self.avanzar();
            let derecha = self.factor()?;
            izquierda = Expr::Binario(Box::new(izquierda), operador, Box::new(derecha));
        }
        Ok(izquierda)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut izquierda = self.unario()?;
        loop {
            let operador = match self.actual() {
                TipoToken::Por => Operador::Multiplicacion,
                TipoToken::Entre => Operador::Division,
                TipoToken::Modulo => Operador::Modulo,
                _ => break,
            };
            self.avanzar();
            let derecha = self.unario()?;
            izquierda = Expr::Binario(Box::new(izquierda), operador, Box::new(derecha));
        }
        Ok(izquierda)
    }

    fn unario(&mut self) -> Result<Expr, String> {
        match self.actual().clone() {
            TipoToken::Menos => {
                self.avanzar();
                Ok(Expr::Unario(OperadorUnario::Negativo, Box::new(self.unario()?)))
            }
            TipoToken::No => {
                self.avanzar();
                Ok(Expr::Unario(OperadorUnario::No, Box::new(self.unario()?)))
            }
            _ => self.primario(),
        }
    }

    fn primario(&mut self) -> Result<Expr, String> {
        match self.actual().clone() {
            TipoToken::Numero(n) => {
                self.avanzar();
                Ok(Expr::Numero(n))
            }
            TipoToken::Texto(s) => {
                self.avanzar();
                Ok(Expr::Cadena(s))
            }
            TipoToken::Verdadero => {
                self.avanzar();
                Ok(Expr::Booleano(true))
            }
            TipoToken::Falso => {
                self.avanzar();
                Ok(Expr::Booleano(false))
            }
            TipoToken::Identificador(n) => {
                self.avanzar();
                self.expresion_desde(n)
            }
            TipoToken::ParenIzq => {
                self.avanzar();
                let expr = self.expresion()?;
                self.esperar(&TipoToken::ParenDer, "Se esperaba ')'")?;
                Ok(expr)
            }
            _ => Err(format!("Expresión inesperada: {:?}", self.actual())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parsear(fuente: &str) -> Programa {
        let tokens = Lexer::new(fuente).tokenizar().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parsear().unwrap()
    }

    #[test]
    fn parsea_programa_vacio() {
        let programa = parsear("programa Vacio { }");
        assert_eq!(programa.nombre, "Vacio");
        assert!(programa.declaraciones.is_empty());
    }

    #[test]
    fn parsea_declaraciones_de_todos_los_tipos() {
        let programa = parsear(
            "programa P {\n\
             entero a = 1\n\
             decimal b = 2.5\n\
             booleano c = verdadero\n\
             cadena d = \"x\"\n\
             ventana e = crear_ventana(\"v\", 1, 1)\n\
             boton f = crear_boton(\"b\")\n\
             }",
        );
        assert_eq!(programa.declaraciones.len(), 6);
        let tipos: Vec<Tipo> = programa
            .declaraciones
            .iter()
            .map(|d| match d {
                Declaracion::Variable { tipo, .. } => tipo.clone(),
                _ => panic!("se esperaba una variable"),
            })
            .collect();
        assert_eq!(
            tipos,
            vec![
                Tipo::Entero,
                Tipo::Decimal,
                Tipo::Booleano,
                Tipo::Cadena,
                Tipo::Ventana,
                Tipo::Boton,
            ]
        );
    }

    #[test]
    fn parsea_asignacion() {
        let programa = parsear("programa P { entero x = 1\n x = x + 1 }");
        assert!(matches!(
            &programa.declaraciones[1],
            Declaracion::Asignacion { nombre, .. } if nombre == "x"
        ));
    }

    #[test]
    fn respeta_precedencia_de_operadores() {
        // 1 + 2 * 3 -> Binario(1, +, Binario(2, *, 3))
        let programa = parsear("programa P { entero x = 1 + 2 * 3 }");
        match &programa.declaraciones[0] {
            Declaracion::Variable { valor, .. } => match valor {
                Expr::Binario(_, Operador::Suma, derecha) => {
                    assert!(matches!(
                        **derecha,
                        Expr::Binario(_, Operador::Multiplicacion, _)
                    ));
                }
                _ => panic!("se esperaba una suma en la raíz"),
            },
            _ => panic!("se esperaba una variable"),
        }
    }

    #[test]
    fn parsea_operadores_logicos() {
        let programa = parsear("programa P { booleano b = verdadero y no falso }");
        match &programa.declaraciones[0] {
            Declaracion::Variable {
                valor: Expr::Binario(_, Operador::Y, derecha),
                ..
            } => {
                assert!(matches!(**derecha, Expr::Unario(OperadorUnario::No, _)));
            }
            _ => panic!("se esperaba un operador 'y'"),
        }
    }

    #[test]
    fn parsea_operador_o() {
        let programa = parsear("programa P { booleano b = falso o verdadero }");
        assert!(matches!(
            &programa.declaraciones[0],
            Declaracion::Variable { valor: Expr::Binario(_, Operador::O, _), .. }
        ));
    }

    #[test]
    fn parsea_operadores_unarios_y_parentesis() {
        let programa = parsear("programa P { booleano b = no (x == 1) }");
        match &programa.declaraciones[0] {
            Declaracion::Variable {
                valor: Expr::Unario(OperadorUnario::No, _),
                ..
            } => {}
            _ => panic!("se esperaba un 'no' unario"),
        }
    }

    #[test]
    fn parsea_si_sino() {
        let programa = parsear("programa P { si (a) { b = 1 } sino { b = 2 } }");
        match &programa.declaraciones[0] {
            Declaracion::Si { entonces, sino, .. } => {
                assert_eq!(entonces.len(), 1);
                assert_eq!(sino.len(), 1);
            }
            _ => panic!("se esperaba un 'si'"),
        }
    }

    #[test]
    fn parsea_mientras() {
        let programa = parsear("programa P { mientras (a) { b = 1 } }");
        assert!(matches!(&programa.declaraciones[0], Declaracion::Mientras { .. }));
    }

    #[test]
    fn parsea_para_con_expresiones() {
        let programa = parsear("programa P { para i desde 0 hasta x + 1 { i = i + 1 } }");
        match &programa.declaraciones[0] {
            Declaracion::Para { variable, fin, .. } => {
                assert_eq!(variable, "i");
                assert!(matches!(fin, Expr::Binario(_, Operador::Suma, _)));
            }
            _ => panic!("se esperaba un 'para'"),
        }
    }

    #[test]
    fn parsea_funcion_completa() {
        let programa = parsear(
            "programa P {\n\
             funcion suma(entero a, entero &b) -> entero {\n\
                 retornar a + b\n\
             }\n\
             }",
        );
        match &programa.declaraciones[0] {
            Declaracion::Funcion { nombre, parametros, retorno, cuerpo } => {
                assert_eq!(nombre, "suma");
                assert_eq!(*retorno, Tipo::Entero);
                assert_eq!(parametros.len(), 2);
                assert_eq!(parametros[0].nombre, "a");
                assert!(!parametros[0].por_referencia);
                assert!(parametros[1].por_referencia);
                assert_eq!(cuerpo.len(), 1);
            }
            _ => panic!("se esperaba una función"),
        }
    }

    #[test]
    fn parsea_funcion_sin_retorno() {
        let programa = parsear("programa P { funcion hola() { retornar } }");
        match &programa.declaraciones[0] {
            Declaracion::Funcion { retorno, parametros, .. } => {
                assert_eq!(*retorno, Tipo::Nada);
                assert!(parametros.is_empty());
            }
            _ => panic!("se esperaba una función"),
        }
    }

    #[test]
    fn parsea_manejador_de_clic() {
        let programa =
            parsear("programa P { al hacer_clic(b) { mostrar_mensaje(\"a\", \"b\") } }");
        match &programa.declaraciones[0] {
            Declaracion::ManejarClic { widget, cuerpo } => {
                assert_eq!(widget, "b");
                assert_eq!(cuerpo.len(), 1);
            }
            _ => panic!("se esperaba un manejador de clic"),
        }
    }

    #[test]
    fn punto_y_coma_es_opcional() {
        let programa = parsear("programa P { entero x = 1; entero z = 2 }");
        assert_eq!(programa.declaraciones.len(), 2);
    }

    #[test]
    fn falla_si_falta_llave() {
        let tokens = Lexer::new("programa P { entero x = 1").tokenizar().unwrap();
        assert!(Parser::new(tokens).parsear().is_err());
    }

    #[test]
    fn falla_si_falta_parentesis_en_si() {
        let tokens = Lexer::new("programa P { si a { } }").tokenizar().unwrap();
        assert!(Parser::new(tokens).parsear().is_err());
    }
}
