//! Árbol de sintaxis abstracta (AST).

/// Programa raíz: un bloque `programa Nombre { ... }`.
#[derive(Debug, Clone)]
pub struct Programa {
    pub nombre: String,
    pub declaraciones: Vec<Declaracion>,
}

/// Declaración o sentencia dentro de un bloque.
#[derive(Debug, Clone)]
pub enum Declaracion {
    Variable {
        nombre: String,
        tipo: Tipo,
        valor: Expr,
    },
    Asignacion {
        nombre: String,
        valor: Expr,
    },
    /// Llamada a función usada como sentencia (p. ej. `agregar(v, b)`).
    Expresion(Expr),
    Si {
        condicion: Expr,
        entonces: Vec<Declaracion>,
        sino: Vec<Declaracion>,
    },
    Mientras {
        condicion: Expr,
        cuerpo: Vec<Declaracion>,
    },
    Retornar(Option<Expr>),
    /// `al hacer_clic(widget) { ... }`
    ManejarClic {
        widget: String,
        cuerpo: Vec<Declaracion>,
    },
    /// `funcion nombre(tipo param, ...) -> tipo { ... }`
    Funcion {
        nombre: String,
        parametros: Vec<Parametro>,
        retorno: Tipo,
        cuerpo: Vec<Declaracion>,
    },
    /// `para i desde inicio hasta fin { ... }`
    Para {
        variable: String,
        inicio: Expr,
        fin: Expr,
        cuerpo: Vec<Declaracion>,
    },
}

/// Parámetro de una función: nombre + tipo (+ paso por referencia).
#[derive(Debug, Clone)]
pub struct Parametro {
    pub nombre: String,
    pub tipo: Tipo,
    pub por_referencia: bool,
}

/// Tipos soportados por el lenguaje.
#[derive(Debug, Clone, PartialEq)]
pub enum Tipo {
    Entero,
    Decimal,
    Booleano,
    Cadena,
    Ventana,
    Boton,
    Etiqueta,
    CajaTexto,
    Nada,
}

/// Expresión.
#[derive(Debug, Clone)]
pub enum Expr {
    Numero(String),
    Cadena(String),
    Booleano(bool),
    Variable(String),
    Binario(Box<Expr>, Operador, Box<Expr>),
    Unario(OperadorUnario, Box<Expr>),
    Llamada {
        nombre: String,
        argumentos: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operador {
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Modulo,
    IgualIgual,
    Distinto,
    Menor,
    Mayor,
    MenorIgual,
    MayorIgual,
    Y,
    O,
}

#[derive(Debug, Clone)]
pub enum OperadorUnario {
    Negativo,
    No,
}
