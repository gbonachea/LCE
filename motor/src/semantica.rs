//! Análisis semántico: comprobación de declaraciones y tipos.
//!
//! Se ejecuta después del parser y antes de la generación de código.
//! Detecta, con mensajes en español, errores como variables no declaradas,
//! tipos incompatibles, aridad incorrecta en llamadas, etc.

use std::collections::HashMap;

use crate::ast::{Declaracion, Expr, Operador, OperadorUnario, Parametro, Programa, Tipo};

#[derive(Clone)]
struct Firma {
    parametros: Vec<Parametro>,
    retorno: Tipo,
}

pub struct Analizador {
    funciones: HashMap<String, Firma>,
    ambitos: Vec<HashMap<String, Tipo>>,
    retorno_actual: Tipo,
    dentro_de_funcion: bool,
}

impl Analizador {
    pub fn new() -> Self {
        Self {
            funciones: HashMap::new(),
            ambitos: vec![HashMap::new()],
            retorno_actual: Tipo::Entero,
            dentro_de_funcion: false,
        }
    }

    /// Punto de entrada: analiza el programa completo.
    pub fn analizar(programa: &Programa) -> Result<(), String> {
        let mut analizador = Analizador::new();
        analizador.registrar_funciones(programa)?;
        analizador.analizar_programa(programa)
    }

    fn registrar_funciones(&mut self, programa: &Programa) -> Result<(), String> {
        for declaracion in &programa.declaraciones {
            if let Declaracion::Funcion { nombre, parametros, retorno, .. } = declaracion {
                if self.funciones.contains_key(nombre) {
                    return Err(format!("La función '{}' ya está definida", nombre));
                }
                self.funciones.insert(
                    nombre.clone(),
                    Firma {
                        parametros: parametros.clone(),
                        retorno: retorno.clone(),
                    },
                );
            }
        }
        Ok(())
    }

    fn analizar_programa(&mut self, programa: &Programa) -> Result<(), String> {
        // 1. Cada función se analiza en un ámbito aislado (no ve las variables
        //    locales de main, igual que en C++).
        for declaracion in &programa.declaraciones {
            if let Declaracion::Funcion { nombre, parametros, retorno, cuerpo } = declaracion {
                self.analizar_funcion(nombre, parametros, retorno, cuerpo)?;
            }
        }

        // 2. Cuerpo principal de main.
        self.ambitos = vec![HashMap::new()];
        self.dentro_de_funcion = false;
        for declaracion in &programa.declaraciones {
            if !matches!(declaracion, Declaracion::Funcion { .. }) {
                self.analizar_declaracion(declaracion)?;
            }
        }
        Ok(())
    }

    fn analizar_funcion(
        &mut self,
        nombre: &str,
        parametros: &[Parametro],
        retorno: &Tipo,
        cuerpo: &[Declaracion],
    ) -> Result<(), String> {
        self.ambitos = vec![HashMap::new()];
        for parametro in parametros {
            self.declarar(&parametro.nombre, parametro.tipo.clone())?;
        }

        self.retorno_actual = retorno.clone();
        self.dentro_de_funcion = true;

        if *retorno != Tipo::Nada && !contiene_retornar(cuerpo) {
            return Err(format!(
                "La función '{}' debe retornar un valor de tipo '{}'",
                nombre,
                nombre_tipo(retorno)
            ));
        }

        self.analizar_declaraciones(cuerpo)?;

        self.dentro_de_funcion = false;
        self.retorno_actual = Tipo::Entero;
        Ok(())
    }

    // ---- Utilidades de ámbitos ----

    fn declarar(&mut self, nombre: &str, tipo: Tipo) -> Result<(), String> {
        let ambito = self.ambitos.last_mut().expect("siempre hay un ámbito");
        if ambito.contains_key(nombre) {
            return Err(format!("La variable '{}' ya está declarada en este ámbito", nombre));
        }
        ambito.insert(nombre.to_string(), tipo);
        Ok(())
    }

    fn buscar(&self, nombre: &str) -> Option<&Tipo> {
        for ambito in self.ambitos.iter().rev() {
            if let Some(tipo) = ambito.get(nombre) {
                return Some(tipo);
            }
        }
        None
    }

    fn entrar_ambito(&mut self) {
        self.ambitos.push(HashMap::new());
    }

    fn salir_ambito(&mut self) {
        self.ambitos.pop();
    }

    // ---- Declaraciones ----

    fn analizar_declaraciones(&mut self, declaraciones: &[Declaracion]) -> Result<(), String> {
        for declaracion in declaraciones {
            self.analizar_declaracion(declaracion)?;
        }
        Ok(())
    }

    fn analizar_declaracion(&mut self, declaracion: &Declaracion) -> Result<(), String> {
        match declaracion {
            Declaracion::Funcion { .. } => Err(
                "Las funciones solo pueden declararse en el nivel superior del programa".into(),
            ),
            Declaracion::Variable { nombre, tipo, valor } => {
                let tipo_valor = self.tipo_de(valor)?;
                if !compatible(tipo, &tipo_valor) {
                    return Err(format!(
                        "No se puede asignar un valor de tipo '{}' a la variable '{}' de tipo '{}'",
                        nombre_tipo(&tipo_valor),
                        nombre,
                        nombre_tipo(tipo)
                    ));
                }
                self.declarar(nombre, tipo.clone())
            }
            Declaracion::Asignacion { nombre, valor } => {
                let tipo_variable = self
                    .buscar(nombre)
                    .cloned()
                    .ok_or_else(|| format!("La variable '{}' no está declarada", nombre))?;
                let tipo_valor = self.tipo_de(valor)?;
                if !compatible(&tipo_variable, &tipo_valor) {
                    return Err(format!(
                        "No se puede asignar un valor de tipo '{}' a '{}' de tipo '{}'",
                        nombre_tipo(&tipo_valor),
                        nombre,
                        nombre_tipo(&tipo_variable)
                    ));
                }
                Ok(())
            }
            Declaracion::Expresion(expr) => {
                self.tipo_de(expr)?;
                Ok(())
            }
            Declaracion::Si { condicion, entonces, sino } => {
                self.requerir_booleano(condicion, "la condición de 'si'")?;
                self.entrar_ambito();
                self.analizar_declaraciones(entonces)?;
                self.salir_ambito();
                if !sino.is_empty() {
                    self.entrar_ambito();
                    self.analizar_declaraciones(sino)?;
                    self.salir_ambito();
                }
                Ok(())
            }
            Declaracion::Mientras { condicion, cuerpo } => {
                self.requerir_booleano(condicion, "la condición de 'mientras'")?;
                self.entrar_ambito();
                self.analizar_declaraciones(cuerpo)?;
                self.salir_ambito();
                Ok(())
            }
            Declaracion::Para { variable, inicio, fin, cuerpo } => {
                let tipo_inicio = self.tipo_de(inicio)?;
                let tipo_fin = self.tipo_de(fin)?;
                if tipo_inicio != Tipo::Entero || tipo_fin != Tipo::Entero {
                    return Err("Los límites de 'para' deben ser de tipo 'entero'".into());
                }
                self.entrar_ambito();
                self.declarar(variable, Tipo::Entero)?;
                self.analizar_declaraciones(cuerpo)?;
                self.salir_ambito();
                Ok(())
            }
            Declaracion::Retornar(valor) => self.analizar_retornar(valor),
            Declaracion::ManejarClic { widget, cuerpo } => {
                let tipo_widget = self
                    .buscar(widget)
                    .cloned()
                    .ok_or_else(|| format!("El widget '{}' no está declarado", widget))?;
                if tipo_widget != Tipo::Boton {
                    return Err(format!(
                        "'{}' no es un botón (es de tipo '{}')",
                        widget,
                        nombre_tipo(&tipo_widget)
                    ));
                }
                self.entrar_ambito();
                self.analizar_declaraciones(cuerpo)?;
                self.salir_ambito();
                Ok(())
            }
        }
    }

    fn analizar_retornar(&mut self, valor: &Option<Expr>) -> Result<(), String> {
        // En main un 'retornar' equivale a salir del programa; no se comprueba.
        if !self.dentro_de_funcion {
            return Ok(());
        }

        match valor {
            Some(expr) => {
                let tipo = self.tipo_de(expr)?;
                if self.retorno_actual == Tipo::Nada {
                    return Err(
                        "Esta función fue declarada como 'nada' y no debe retornar un valor".into(),
                    );
                }
                if !compatible(&self.retorno_actual, &tipo) {
                    return Err(format!(
                        "La función debe retornar '{}' pero se retorna '{}'",
                        nombre_tipo(&self.retorno_actual),
                        nombre_tipo(&tipo)
                    ));
                }
            }
            None => {
                if self.retorno_actual != Tipo::Nada {
                    return Err(format!(
                        "La función debe retornar un valor de tipo '{}'",
                        nombre_tipo(&self.retorno_actual)
                    ));
                }
            }
        }
        Ok(())
    }

    // ---- Expresiones ----

    fn requerir_booleano(&mut self, expr: &Expr, contexto: &str) -> Result<(), String> {
        let tipo = self.tipo_de(expr)?;
        if tipo != Tipo::Booleano {
            return Err(format!(
                "{} debe ser de tipo 'booleano' (se encontró '{}')",
                contexto,
                nombre_tipo(&tipo)
            ));
        }
        Ok(())
    }

    fn tipo_de(&mut self, expr: &Expr) -> Result<Tipo, String> {
        match expr {
            Expr::Numero(n) => Ok(if n.contains('.') {
                Tipo::Decimal
            } else {
                Tipo::Entero
            }),
            Expr::Cadena(_) => Ok(Tipo::Cadena),
            Expr::Booleano(_) => Ok(Tipo::Booleano),
            Expr::Variable(nombre) => self
                .buscar(nombre)
                .cloned()
                .ok_or_else(|| format!("La variable '{}' no está declarada", nombre)),
            Expr::Unario(operador, expr) => {
                let tipo = self.tipo_de(expr)?;
                match operador {
                    OperadorUnario::Negativo => {
                        if !es_numerico(&tipo) {
                            return Err(format!(
                                "El operador '-' requiere un valor numérico (se encontró '{}')",
                                nombre_tipo(&tipo)
                            ));
                        }
                        Ok(tipo)
                    }
                    OperadorUnario::No => {
                        if tipo != Tipo::Booleano {
                            return Err(format!(
                                "El operador 'no' requiere un valor booleano (se encontró '{}')",
                                nombre_tipo(&tipo)
                            ));
                        }
                        Ok(Tipo::Booleano)
                    }
                }
            }
            Expr::Binario(izquierda, operador, derecha) => {
                let tipo_izq = self.tipo_de(izquierda)?;
                let tipo_der = self.tipo_de(derecha)?;
                self.tipo_binario(operador, &tipo_izq, &tipo_der)
            }
            Expr::Llamada { nombre, argumentos } => self.tipo_llamada(nombre, argumentos),
        }
    }

    fn tipo_binario(
        &self,
        operador: &Operador,
        tipo_izq: &Tipo,
        tipo_der: &Tipo,
    ) -> Result<Tipo, String> {
        match operador {
            Operador::Suma | Operador::Resta | Operador::Multiplicacion | Operador::Division => {
                // Concatenación de cadenas con '+'.
                if matches!(operador, Operador::Suma)
                    && tipo_izq == &Tipo::Cadena
                    && tipo_der == &Tipo::Cadena
                {
                    return Ok(Tipo::Cadena);
                }
                if !es_numerico(tipo_izq) || !es_numerico(tipo_der) {
                    return Err(format!(
                        "El operador requiere operandos numéricos (se encontró '{}' y '{}')",
                        nombre_tipo(tipo_izq),
                        nombre_tipo(tipo_der)
                    ));
                }
                if tipo_izq == &Tipo::Decimal || tipo_der == &Tipo::Decimal {
                    Ok(Tipo::Decimal)
                } else {
                    Ok(Tipo::Entero)
                }
            }
            Operador::Modulo => {
                if tipo_izq != &Tipo::Entero || tipo_der != &Tipo::Entero {
                    return Err("El operador '%' requiere operandos de tipo 'entero'".into());
                }
                Ok(Tipo::Entero)
            }
            Operador::Menor | Operador::Mayor | Operador::MenorIgual | Operador::MayorIgual => {
                let ambos_numericos = es_numerico(tipo_izq) && es_numerico(tipo_der);
                let ambas_cadenas = tipo_izq == &Tipo::Cadena && tipo_der == &Tipo::Cadena;
                if !ambos_numericos && !ambas_cadenas {
                    return Err(format!(
                        "No se pueden comparar '{}' y '{}'",
                        nombre_tipo(tipo_izq),
                        nombre_tipo(tipo_der)
                    ));
                }
                Ok(Tipo::Booleano)
            }
            Operador::IgualIgual | Operador::Distinto => {
                let comparables = (es_numerico(tipo_izq) && es_numerico(tipo_der))
                    || compatible(tipo_izq, tipo_der)
                    || compatible(tipo_der, tipo_izq);
                if !comparables {
                    return Err(format!(
                        "No se pueden comparar '{}' y '{}'",
                        nombre_tipo(tipo_izq),
                        nombre_tipo(tipo_der)
                    ));
                }
                Ok(Tipo::Booleano)
            }
            Operador::Y | Operador::O => {
                if tipo_izq != &Tipo::Booleano || tipo_der != &Tipo::Booleano {
                    return Err(
                        "Los operadores 'y'/'o' requieren operandos de tipo 'booleano'".into(),
                    );
                }
                Ok(Tipo::Booleano)
            }
        }
    }

    fn tipo_llamada(&mut self, nombre: &str, argumentos: &[Expr]) -> Result<Tipo, String> {
        // Funciones definidas por el usuario (clonamos la firma para liberar el préstamo).
        let firma = self.funciones.get(nombre).cloned();

        if let Some(firma) = firma {
            if argumentos.len() != firma.parametros.len() {
                return Err(format!(
                    "La función '{}' espera {} argumento(s) pero recibió {}",
                    nombre,
                    firma.parametros.len(),
                    argumentos.len()
                ));
            }

            for (indice, (argumento, parametro)) in
                argumentos.iter().zip(firma.parametros.iter()).enumerate()
            {
                let tipo_arg = self.tipo_de(argumento)?;
                if parametro.por_referencia {
                    if !matches!(argumento, Expr::Variable(_)) {
                        return Err(format!(
                            "El argumento {} de '{}' se pasa por referencia y debe ser una variable",
                            indice + 1,
                            nombre
                        ));
                    }
                    if tipo_arg != parametro.tipo {
                        return Err(format!(
                            "El argumento {} de '{}' se pasa por referencia y debe ser de tipo '{}' (se recibió '{}')",
                            indice + 1,
                            nombre,
                            nombre_tipo(&parametro.tipo),
                            nombre_tipo(&tipo_arg)
                        ));
                    }
                } else if !compatible(&parametro.tipo, &tipo_arg) {
                    return Err(format!(
                        "Argumento {} de '{}': se esperaba '{}' y se recibió '{}'",
                        indice + 1,
                        nombre,
                        nombre_tipo(&parametro.tipo),
                        nombre_tipo(&tipo_arg)
                    ));
                }
            }

            return Ok(firma.retorno);
        }

        self.tipo_nativa(nombre, argumentos)
    }

    fn tipo_nativa(&mut self, nombre: &str, argumentos: &[Expr]) -> Result<Tipo, String> {
        match nombre {
            "crear_ventana" => {
                self.exigir_aridad(nombre, argumentos, 3)?;
                self.exigir_tipo(argumentos, 0, &Tipo::Cadena, nombre)?;
                self.exigir_tipo(argumentos, 1, &Tipo::Entero, nombre)?;
                self.exigir_tipo(argumentos, 2, &Tipo::Entero, nombre)?;
                Ok(Tipo::Ventana)
            }
            "crear_boton" => {
                self.exigir_aridad(nombre, argumentos, 1)?;
                self.exigir_tipo(argumentos, 0, &Tipo::Cadena, nombre)?;
                Ok(Tipo::Boton)
            }
            "crear_etiqueta" => {
                self.exigir_aridad(nombre, argumentos, 1)?;
                self.exigir_tipo(argumentos, 0, &Tipo::Cadena, nombre)?;
                Ok(Tipo::Etiqueta)
            }
            "crear_caja_texto" => {
                self.exigir_aridad(nombre, argumentos, 1)?;
                self.exigir_tipo(argumentos, 0, &Tipo::Cadena, nombre)?;
                Ok(Tipo::CajaTexto)
            }
            "mostrar_mensaje" => {
                self.exigir_aridad(nombre, argumentos, 2)?;
                self.exigir_tipo(argumentos, 0, &Tipo::Cadena, nombre)?;
                self.exigir_tipo(argumentos, 1, &Tipo::Cadena, nombre)?;
                Ok(Tipo::Nada)
            }
            "agregar" => {
                self.exigir_aridad(nombre, argumentos, 2)?;
                self.exigir_tipo(argumentos, 0, &Tipo::Ventana, nombre)?;
                let tipo_widget = self.tipo_de(&argumentos[1])?;
                if !es_widget(&tipo_widget) {
                    return Err(format!(
                        "El segundo argumento de 'agregar' debe ser un widget (se recibió '{}')",
                        nombre_tipo(&tipo_widget)
                    ));
                }
                Ok(Tipo::Nada)
            }
            "mostrar" => {
                self.exigir_aridad(nombre, argumentos, 1)?;
                let tipo_widget = self.tipo_de(&argumentos[0])?;
                if !es_widget(&tipo_widget) {
                    return Err(format!(
                        "'mostrar' requiere un widget (se recibió '{}')",
                        nombre_tipo(&tipo_widget)
                    ));
                }
                Ok(Tipo::Nada)
            }
            "texto" => {
                self.exigir_aridad(nombre, argumentos, 1)?;
                self.exigir_tipo(argumentos, 0, &Tipo::CajaTexto, nombre)?;
                Ok(Tipo::Cadena)
            }
            "establecer_texto" => {
                self.exigir_aridad(nombre, argumentos, 2)?;
                self.exigir_tipo(argumentos, 0, &Tipo::CajaTexto, nombre)?;
                self.exigir_tipo(argumentos, 1, &Tipo::Cadena, nombre)?;
                Ok(Tipo::Nada)
            }
            _ => Err(format!("La función '{}' no está declarada", nombre)),
        }
    }

    fn exigir_aridad(
        &self,
        nombre: &str,
        argumentos: &[Expr],
        esperada: usize,
    ) -> Result<(), String> {
        if argumentos.len() != esperada {
            return Err(format!(
                "La función '{}' espera {} argumento(s) pero recibió {}",
                nombre,
                esperada,
                argumentos.len()
            ));
        }
        Ok(())
    }

    fn exigir_tipo(
        &mut self,
        argumentos: &[Expr],
        indice: usize,
        esperado: &Tipo,
        nombre: &str,
    ) -> Result<(), String> {
        let tipo = self.tipo_de(&argumentos[indice])?;
        if !compatible(esperado, &tipo) {
            return Err(format!(
                "Argumento {} de '{}': se esperaba '{}' y se recibió '{}'",
                indice + 1,
                nombre,
                nombre_tipo(esperado),
                nombre_tipo(&tipo)
            ));
        }
        Ok(())
    }
}

impl Default for Analizador {
    fn default() -> Self {
        Self::new()
    }
}

// ---- Funciones auxiliares ----

fn nombre_tipo(tipo: &Tipo) -> &'static str {
    match tipo {
        Tipo::Entero => "entero",
        Tipo::Decimal => "decimal",
        Tipo::Booleano => "booleano",
        Tipo::Cadena => "cadena",
        Tipo::Ventana => "ventana",
        Tipo::Boton => "boton",
        Tipo::Etiqueta => "etiqueta",
        Tipo::CajaTexto => "caja_texto",
        Tipo::Nada => "nada",
    }
}

fn compatible(esperado: &Tipo, recibido: &Tipo) -> bool {
    esperado == recibido || (esperado == &Tipo::Decimal && recibido == &Tipo::Entero)
}

fn es_numerico(tipo: &Tipo) -> bool {
    matches!(tipo, Tipo::Entero | Tipo::Decimal)
}

fn es_widget(tipo: &Tipo) -> bool {
    matches!(tipo, Tipo::Ventana | Tipo::Boton | Tipo::Etiqueta | Tipo::CajaTexto)
}

fn contiene_retornar(declaraciones: &[Declaracion]) -> bool {
    declaraciones.iter().any(|declaracion| match declaracion {
        Declaracion::Retornar(_) => true,
        Declaracion::Si { entonces, sino, .. } => {
            contiene_retornar(entonces) || contiene_retornar(sino)
        }
        Declaracion::Mientras { cuerpo, .. } | Declaracion::Para { cuerpo, .. } => {
            contiene_retornar(cuerpo)
        }
        // Los 'retornar' dentro de un manejador de clic pertenecen al lambda,
        // no a la función; por eso no cuentan.
        Declaracion::ManejarClic { .. } => false,
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn resultado(fuente: &str) -> Result<(), String> {
        let tokens = Lexer::new(fuente).tokenizar().unwrap();
        let mut parser = Parser::new(tokens);
        let programa = parser.parsear().unwrap();
        Analizador::analizar(&programa)
    }

    fn error(fuente: &str) -> String {
        resultado(fuente).unwrap_err()
    }

    fn acepta(fuente: &str) {
        if let Err(e) = resultado(fuente) {
            panic!("se esperaba que compilara, pero dio: {}", e);
        }
    }

    // ---------- Casos válidos ----------

    #[test]
    fn acepta_programa_vacio() {
        acepta("programa P { }");
    }

    #[test]
    fn acepta_aritmetica() {
        acepta("programa P { entero x = 1 + 2 * 3 - 4 / 2 }");
    }

    #[test]
    fn acepta_concatenacion_de_cadenas() {
        acepta("programa P { cadena s = \"a\" + \"b\" }");
    }

    #[test]
    fn acepta_widening_entero_a_decimal() {
        acepta("programa P { decimal d = 5 }");
    }

    #[test]
    fn acepta_comparaciones_numericas() {
        acepta("programa P { booleano b = 1 < 2.5 }");
    }

    #[test]
    fn acepta_comparaciones_de_cadenas() {
        acepta("programa P { booleano b = \"a\" < \"b\" }");
    }

    #[test]
    fn acepta_operadores_logicos() {
        acepta("programa P { booleano b = verdadero y no falso }");
    }

    #[test]
    fn acepta_ambitos_anidados() {
        acepta("programa P { entero x = 1\n si (x > 0) { entero z = 2\n z = z + 1 } }");
    }

    #[test]
    fn acepta_recursion() {
        acepta("programa P { funcion f(entero n) -> entero { retornar f(n - 1) } }");
    }

    #[test]
    fn acepta_funcion_usada_antes_de_definirse() {
        acepta("programa P { entero x = f(1)\n funcion f(entero a) -> entero { retornar a } }");
    }

    #[test]
    fn acepta_referencias() {
        acepta("programa P { funcion inc(entero &v) { v = v + 1 }\n entero x = 0\n inc(x) }");
    }

    #[test]
    fn acepta_widgets_y_eventos() {
        acepta(
            "programa P {\n\
             ventana v = crear_ventana(\"v\", 1, 1)\n\
             boton b = crear_boton(\"b\")\n\
             al hacer_clic(b) { mostrar_mensaje(\"a\", \"b\") }\n\
             agregar(v, b)\n\
             mostrar(v)\n\
             }",
        );
    }

    #[test]
    fn acepta_texto_de_caja() {
        acepta(
            "programa P {\n\
             caja_texto c = crear_caja_texto(\"x\")\n\
             cadena s = texto(c)\n\
             establecer_texto(c, s)\n\
             }",
        );
    }

    #[test]
    fn acepta_condicion_negada() {
        acepta("programa P { booleano b = no verdadero }");
    }

    // ---------- Casos de error ----------

    #[test]
    fn rechaza_variable_no_declarada() {
        assert!(error("programa P { x = 1 }").contains("no está declarada"));
    }

    #[test]
    fn rechaza_variable_duplicada() {
        assert!(error("programa P { entero x = 1\n entero x = 2 }").contains("ya está declarada"));
    }

    #[test]
    fn rechaza_tipo_en_declaracion() {
        assert!(error("programa P { entero x = \"a\" }").contains("No se puede asignar"));
    }

    #[test]
    fn rechaza_tipo_en_asignacion() {
        assert!(error("programa P { entero x = 1\n x = \"a\" }").contains("No se puede asignar"));
    }

    #[test]
    fn rechaza_decimal_a_entero() {
        assert!(error("programa P { entero x = 1.5 }").contains("No se puede asignar"));
    }

    #[test]
    fn rechaza_aritmetica_con_cadena() {
        assert!(error("programa P { entero x = \"a\" - \"b\" }").contains("numéricos"));
    }

    #[test]
    fn rechaza_modulo_decimal() {
        assert!(error("programa P { decimal d = 1.5 % 2 }").contains("entero"));
    }

    #[test]
    fn rechaza_condicion_no_booleana() {
        assert!(error("programa P { si (1) { } }").contains("booleano"));
    }

    #[test]
    fn rechaza_condicion_de_mientras_no_booleana() {
        assert!(error("programa P { mientras (1) { } }").contains("booleano"));
    }

    #[test]
    fn rechaza_limites_de_para_no_enteros() {
        assert!(error("programa P { para i desde 0.5 hasta 2 { } }").contains("entero"));
    }

    #[test]
    fn rechaza_logico_con_entero() {
        assert!(error("programa P { booleano b = 1 y 2 }").contains("booleano"));
    }

    #[test]
    fn rechaza_aridad_de_funcion_usuario() {
        assert!(error("programa P { funcion f(entero a) { }\n f(1, 2) }").contains("argumento"));
    }

    #[test]
    fn rechaza_aridad_de_funcion_nativa() {
        assert!(error("programa P { ventana v = crear_ventana(\"v\", 1) }").contains("argumento"));
    }

    #[test]
    fn rechaza_tipo_de_argumento_nativo() {
        assert!(error("programa P { mostrar_mensaje(1, 2) }").contains("Argumento"));
    }

    #[test]
    fn rechaza_retorno_incorrecto() {
        assert!(error("programa P { funcion f() -> entero { retornar \"a\" } }").contains("retornar"));
    }

    #[test]
    fn rechaza_funcion_sin_retorno() {
        assert!(error("programa P { funcion f() -> entero { } }").contains("debe retornar"));
    }

    #[test]
    fn rechaza_retorno_en_funcion_nada() {
        assert!(error("programa P { funcion f() { retornar 1 } }").contains("nada"));
    }

    #[test]
    fn rechaza_referencia_a_literal() {
        assert!(
            error("programa P { funcion inc(entero &v) { }\n inc(5) }").contains("referencia")
        );
    }

    #[test]
    fn rechaza_referencia_con_tipo_incorrecto() {
        assert!(error(
            "programa P { funcion inc(entero &v) { }\n decimal d = 1.5\n inc(d) }"
        )
        .contains("referencia"));
    }

    #[test]
    fn rechaza_clic_sobre_no_boton() {
        assert!(error(
            "programa P { ventana v = crear_ventana(\"v\", 1, 1)\n al hacer_clic(v) { } }"
        )
        .contains("no es un botón"));
    }

    #[test]
    fn rechaza_widget_no_declarado() {
        assert!(error("programa P { al hacer_clic(b) { } }").contains("no está declarado"));
    }

    #[test]
    fn rechaza_funcion_duplicada() {
        assert!(error("programa P { funcion f() { }\n funcion f() { } }").contains("ya está definida"));
    }

    #[test]
    fn rechaza_funcion_anidada() {
        assert!(
            error("programa P { si (verdadero) { funcion f() { } } }").contains("nivel superior")
        );
    }

    #[test]
    fn rechaza_texto_sobre_no_caja() {
        let mensaje = error("programa P { cadena s = texto(1) }");
        assert!(mensaje.contains("caja_texto") || mensaje.contains("Argumento"));
    }

    #[test]
    fn rechaza_agregar_no_widget() {
        assert!(error(
            "programa P { ventana v = crear_ventana(\"v\", 1, 1)\n entero x = 1\n agregar(v, x) }"
        )
        .contains("widget"));
    }

    #[test]
    fn rechaza_mostrar_no_widget() {
        assert!(error("programa P { entero x = 1\n mostrar(x) }").contains("widget"));
    }
}
