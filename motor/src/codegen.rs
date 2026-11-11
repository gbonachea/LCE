//! Generación de código: AST -> C++ (Qt6).

use crate::ast::{Declaracion, Expr, Operador, OperadorUnario, Parametro, Programa, Tipo};

pub struct Generador {
    salida: String,
    nivel: usize,
}

impl Generador {
    pub fn new() -> Self {
        Self {
            salida: String::new(),
            nivel: 0,
        }
    }

    fn escribir(&mut self, texto: &str) {
        for _ in 0..self.nivel {
            self.salida.push_str("    ");
        }
        self.salida.push_str(texto);
        self.salida.push('\n');
    }

    /// Mapea un tipo del lenguaje a su equivalente en C++/Qt.
    fn tipo_cpp(tipo: &Tipo) -> &'static str {
        match tipo {
            Tipo::Entero => "int",
            Tipo::Decimal => "double",
            Tipo::Booleano => "bool",
            Tipo::Cadena => "QString",
            Tipo::Ventana => "QMainWindow*",
            Tipo::Boton => "QPushButton*",
            Tipo::Etiqueta => "QLabel*",
            Tipo::CajaTexto => "QLineEdit*",
            Tipo::Nada => "void",
        }
    }

    pub fn generar(&mut self, programa: &Programa) -> String {
        self.cabeceras();
        self.salida.push('\n');
        self.funciones_auxiliares();
        self.salida.push('\n');
        self.funciones_usuario(programa);
        self.generar_main(programa);
        self.salida.clone()
    }

    fn cabeceras(&mut self) {
        self.escribir("// Código generado automáticamente por lce (LCE -> C++/Qt).");
        self.escribir("// No editar manualmente: cualquier cambio se perderá.");
        self.escribir("#include <QApplication>");
        self.escribir("#include <QMainWindow>");
        self.escribir("#include <QWidget>");
        self.escribir("#include <QVBoxLayout>");
        self.escribir("#include <QPushButton>");
        self.escribir("#include <QLabel>");
        self.escribir("#include <QLineEdit>");
        self.escribir("#include <QMessageBox>");
        self.escribir("#include <QObject>");
        self.escribir("#include <QString>");
    }

    /// Funciones nativas del lenguaje, escritas una sola vez en el C++ generado.
    fn funciones_auxiliares(&mut self) {
        self.escribir("static QMainWindow* crear_ventana(const QString& titulo, int ancho, int alto) {");
        self.nivel += 1;
        self.escribir("auto* ventana = new QMainWindow();");
        self.escribir("ventana->setWindowTitle(titulo);");
        self.escribir("ventana->resize(ancho, alto);");
        self.escribir("return ventana;");
        self.nivel -= 1;
        self.escribir("}");
        self.salida.push('\n');

        self.escribir("static QPushButton* crear_boton(const QString& texto) {");
        self.nivel += 1;
        self.escribir("return new QPushButton(texto);");
        self.nivel -= 1;
        self.escribir("}");
        self.salida.push('\n');

        self.escribir("static QLabel* crear_etiqueta(const QString& texto) {");
        self.nivel += 1;
        self.escribir("return new QLabel(texto);");
        self.nivel -= 1;
        self.escribir("}");
        self.salida.push('\n');

        self.escribir("static QLineEdit* crear_caja_texto(const QString& marcador) {");
        self.nivel += 1;
        self.escribir("auto* caja = new QLineEdit();");
        self.escribir("caja->setPlaceholderText(marcador);");
        self.escribir("return caja;");
        self.nivel -= 1;
        self.escribir("}");
        self.salida.push('\n');

        self.escribir("static QString texto(QLineEdit* caja) {");
        self.nivel += 1;
        self.escribir("return caja->text();");
        self.nivel -= 1;
        self.escribir("}");
        self.salida.push('\n');

        self.escribir("static void establecer_texto(QLineEdit* caja, const QString& valor) {");
        self.nivel += 1;
        self.escribir("caja->setText(valor);");
        self.nivel -= 1;
        self.escribir("}");
        self.salida.push('\n');

        self.escribir("static void mostrar_mensaje(const QString& titulo, const QString& texto) {");
        self.nivel += 1;
        self.escribir("QMessageBox::information(nullptr, titulo, texto);");
        self.nivel -= 1;
        self.escribir("}");
        self.salida.push('\n');

        self.escribir("static void agregar(QMainWindow* ventana, QWidget* widget) {");
        self.nivel += 1;
        self.escribir("auto* central = ventana->centralWidget();");
        self.escribir("QVBoxLayout* layout = nullptr;");
        self.escribir("if (central == nullptr) {");
        self.nivel += 1;
        self.escribir("central = new QWidget();");
        self.escribir("layout = new QVBoxLayout(central);");
        self.escribir("ventana->setCentralWidget(central);");
        self.nivel -= 1;
        self.escribir("} else {");
        self.nivel += 1;
        self.escribir("layout = qobject_cast<QVBoxLayout*>(central->layout());");
        self.nivel -= 1;
        self.escribir("}");
        self.escribir("layout->addWidget(widget);");
        self.nivel -= 1;
        self.escribir("}");
        self.salida.push('\n');

        self.escribir("static void mostrar(QWidget* widget) {");
        self.nivel += 1;
        self.escribir("widget->show();");
        self.nivel -= 1;
        self.escribir("}");
        self.salida.push('\n');
    }

    /// Emite las funciones definidas por el usuario: primero los prototipos
    /// (para permitir llamadas entre funciones en cualquier orden) y luego las
    /// definiciones completas.
    fn funciones_usuario(&mut self, programa: &Programa) {
        let funciones: Vec<&Declaracion> = programa
            .declaraciones
            .iter()
            .filter(|d| matches!(d, Declaracion::Funcion { .. }))
            .collect();

        if funciones.is_empty() {
            return;
        }

        for funcion in &funciones {
            self.generar_prototipo(funcion);
        }
        self.salida.push('\n');

        for funcion in &funciones {
            self.generar_funcion(funcion);
            self.salida.push('\n');
        }
    }

    fn generar_prototipo(&mut self, declaracion: &Declaracion) {
        if let Declaracion::Funcion { nombre, parametros, retorno, .. } = declaracion {
            let firma = self.firma_parametros(parametros);
            self.escribir(&format!("{} {}({});", Self::tipo_cpp(retorno), nombre, firma));
        }
    }

    fn generar_funcion(&mut self, declaracion: &Declaracion) {
        if let Declaracion::Funcion { nombre, parametros, retorno, cuerpo } = declaracion {
            let firma = self.firma_parametros(parametros);
            self.escribir(&format!("{} {}({}) {{", Self::tipo_cpp(retorno), nombre, firma));
            self.nivel += 1;
            self.generar_declaraciones(cuerpo);
            self.nivel -= 1;
            self.escribir("}");
        }
    }

    /// Construye la lista de parámetros `tipo nombre, tipo nombre, ...`,
    /// añadiendo `&` cuando el parámetro se pasa por referencia.
    fn firma_parametros(&self, parametros: &[Parametro]) -> String {
        parametros
            .iter()
            .map(|p| {
                let referencia = if p.por_referencia { "&" } else { "" };
                format!("{}{} {}", Self::tipo_cpp(&p.tipo), referencia, p.nombre)
            })
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn generar_main(&mut self, programa: &Programa) {
        self.escribir("int main(int argc, char *argv[]) {");
        self.nivel += 1;
        self.escribir("QApplication app(argc, argv);");
        self.salida.push('\n');
        for declaracion in &programa.declaraciones {
            if !matches!(declaracion, Declaracion::Funcion { .. }) {
                self.generar_declaracion(declaracion);
            }
        }
        self.salida.push('\n');
        self.escribir("return app.exec();");
        self.nivel -= 1;
        self.escribir("}");
    }

    fn generar_declaraciones(&mut self, declaraciones: &[Declaracion]) {
        for declaracion in declaraciones {
            self.generar_declaracion(declaracion);
        }
    }

    /// Genera un `si`/`sino si`/`sino` encadenado.
    /// Cuando la rama `sino` contiene un único `Si`, se emite como `else if`.
    fn generar_si(&mut self, condicion: &Expr, entonces: &[Declaracion], mut sino: &[Declaracion]) {
        let linea_if = format!("if ({}) {{", self.expr_a_cpp(condicion));
        self.escribir(&linea_if);
        self.nivel += 1;
        self.generar_declaraciones(entonces);
        self.nivel -= 1;

        loop {
            match sino {
                [] => {
                    self.escribir("}");
                    return;
                }
                [Declaracion::Si { condicion: c, entonces: t, sino: s }] => {
                    let linea_else_if = format!("}} else if ({}) {{", self.expr_a_cpp(c));
                    self.escribir(&linea_else_if);
                    self.nivel += 1;
                    self.generar_declaraciones(t);
                    self.nivel -= 1;
                    sino = s.as_slice();
                }
                _ => {
                    self.escribir("} else {");
                    self.nivel += 1;
                    self.generar_declaraciones(sino);
                    self.nivel -= 1;
                    self.escribir("}");
                    return;
                }
            }
        }
    }

    fn generar_declaracion(&mut self, declaracion: &Declaracion) {
        match declaracion {
            Declaracion::Variable { nombre, tipo, valor } => {
                let tipo_cpp = Self::tipo_cpp(tipo);
                let valor_cpp = self.expr_a_cpp(valor);
                self.escribir(&format!("{} {} = {};", tipo_cpp, nombre, valor_cpp));
            }
            Declaracion::Asignacion { nombre, valor } => {
                let valor_cpp = self.expr_a_cpp(valor);
                self.escribir(&format!("{} = {};", nombre, valor_cpp));
            }
            Declaracion::Expresion(expr) => {
                let valor_cpp = self.expr_a_cpp(expr);
                self.escribir(&format!("{};", valor_cpp));
            }
            Declaracion::Si { condicion, entonces, sino } => {
                self.generar_si(condicion, entonces, sino);
            }
            Declaracion::Mientras { condicion, cuerpo } => {
                let condicion_cpp = self.expr_a_cpp(condicion);
                self.escribir(&format!("while ({}) {{", condicion_cpp));
                self.nivel += 1;
                self.generar_declaraciones(cuerpo);
                self.nivel -= 1;
                self.escribir("}");
            }
            Declaracion::Para { variable, inicio, fin, cuerpo } => {
                let inicio_cpp = self.expr_a_cpp(inicio);
                let fin_cpp = self.expr_a_cpp(fin);
                self.escribir(&format!(
                    "for (int {} = {}; {} < {}; {}++) {{",
                    variable, inicio_cpp, variable, fin_cpp, variable
                ));
                self.nivel += 1;
                self.generar_declaraciones(cuerpo);
                self.nivel -= 1;
                self.escribir("}");
            }
            Declaracion::Retornar(valor) => match valor {
                Some(expr) => {
                    let valor_cpp = self.expr_a_cpp(expr);
                    self.escribir(&format!("return {};", valor_cpp));
                }
                None => self.escribir("return;"),
            },
            Declaracion::ManejarClic { widget, cuerpo } => {
                self.escribir(&format!(
                    "QObject::connect({}, &QPushButton::clicked, [&]() {{",
                    widget
                ));
                self.nivel += 1;
                self.generar_declaraciones(cuerpo);
                self.nivel -= 1;
                self.escribir("});");
            }
            Declaracion::Funcion { .. } => {
                // Las funciones se emiten antes de main (ver funciones_usuario).
            }
        }
    }

    fn expr_a_cpp(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Numero(n) => n.clone(),
            Expr::Cadena(s) => format!("QString({})", Self::cadena_cpp(s)),
            Expr::Booleano(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Expr::Variable(n) => n.clone(),
            Expr::Binario(izquierda, operador, derecha) => {
                let izquierda_cpp = self.expr_a_cpp(izquierda);
                let derecha_cpp = self.expr_a_cpp(derecha);
                let operador_cpp = match operador {
                    Operador::Suma => "+",
                    Operador::Resta => "-",
                    Operador::Multiplicacion => "*",
                    Operador::Division => "/",
                    Operador::Modulo => "%",
                    Operador::IgualIgual => "==",
                    Operador::Distinto => "!=",
                    Operador::Menor => "<",
                    Operador::Mayor => ">",
                    Operador::MenorIgual => "<=",
                    Operador::MayorIgual => ">=",
                    Operador::Y => "&&",
                    Operador::O => "||",
                };
                format!("({} {} {})", izquierda_cpp, operador_cpp, derecha_cpp)
            }
            Expr::Unario(operador, expr) => {
                let valor_cpp = self.expr_a_cpp(expr);
                match operador {
                    OperadorUnario::Negativo => format!("(-{})", valor_cpp),
                    OperadorUnario::No => format!("(!{})", valor_cpp),
                }
            }
            Expr::Llamada { nombre, argumentos } => {
                let argumentos_cpp: Vec<String> = argumentos
                    .iter()
                    .map(|argumento| self.expr_a_cpp(argumento))
                    .collect();
                format!("{}({})", nombre, argumentos_cpp.join(", "))
            }
        }
    }

    /// Escapa una cadena para usarla como literal de C++.
    fn cadena_cpp(s: &str) -> String {
        let mut resultado = String::from("\"");
        for c in s.chars() {
            match c {
                '"' => resultado.push_str("\\\""),
                '\\' => resultado.push_str("\\\\"),
                '\n' => resultado.push_str("\\n"),
                '\t' => resultado.push_str("\\t"),
                _ => resultado.push(c),
            }
        }
        resultado.push('"');
        resultado
    }
}

impl Default for Generador {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn generar(fuente: &str) -> String {
        let tokens = Lexer::new(fuente).tokenizar().unwrap();
        let mut parser = Parser::new(tokens);
        let programa = parser.parsear().unwrap();
        Generador::new().generar(&programa)
    }

    #[test]
    fn escapa_cadena() {
        assert_eq!(Generador::cadena_cpp("hola"), "\"hola\"");
        assert_eq!(Generador::cadena_cpp("a\"b"), "\"a\\\"b\"");
        assert_eq!(Generador::cadena_cpp("a\nb"), "\"a\\nb\"");
        assert_eq!(Generador::cadena_cpp("a\\b"), "\"a\\\\b\"");
    }

    #[test]
    fn genera_estructura_basica() {
        let cpp = generar("programa P { }");
        assert!(cpp.contains("#include <QApplication>"));
        assert!(cpp.contains("int main(int argc, char *argv[])"));
        assert!(cpp.contains("QApplication app(argc, argv);"));
        assert!(cpp.contains("return app.exec();"));
    }

    #[test]
    fn mapea_tipos_escalares() {
        let cpp = generar(
            "programa P {\n\
             entero a = 1\n\
             decimal b = 2.5\n\
             booleano c = verdadero\n\
             cadena d = \"x\"\n\
             }",
        );
        assert!(cpp.contains("int a = 1;"));
        assert!(cpp.contains("double b = 2.5;"));
        assert!(cpp.contains("bool c = true;"));
        assert!(cpp.contains("QString d = QString(\"x\");"));
    }

    #[test]
    fn mapea_tipos_de_widget() {
        let cpp = generar(
            "programa P {\n\
             ventana v = crear_ventana(\"v\", 10, 20)\n\
             boton b = crear_boton(\"b\")\n\
             caja_texto c = crear_caja_texto(\"c\")\n\
             etiqueta e = crear_etiqueta(\"e\")\n\
             }",
        );
        assert!(cpp.contains("QMainWindow* v = crear_ventana(QString(\"v\"), 10, 20);"));
        assert!(cpp.contains("QPushButton* b = crear_boton(QString(\"b\"));"));
        assert!(cpp.contains("QLineEdit* c = crear_caja_texto(QString(\"c\"));"));
        assert!(cpp.contains("QLabel* e = crear_etiqueta(QString(\"e\"));"));
    }

    #[test]
    fn genera_operadores_y_concatenacion() {
        let cpp = generar(
            "programa P {\n\
             cadena s = \"a\" + \"b\"\n\
             entero x = 1 + 2 * 3\n\
             booleano b = 1 < 2 y verdadero\n\
             }",
        );
        assert!(cpp.contains("(QString(\"a\") + QString(\"b\"))"));
        assert!(cpp.contains("(1 + (2 * 3))"));
        assert!(cpp.contains("((1 < 2) && true)"));
    }

    #[test]
    fn genera_unario() {
        let cpp = generar("programa P { booleano b = no verdadero\n entero x = -5 }");
        assert!(cpp.contains("(!true)"));
        assert!(cpp.contains("(-5)"));
    }

    #[test]
    fn genera_sino_si_como_else_if() {
        let cpp = generar("programa P { si (a) { b = 1 } sino si (c) { b = 2 } sino { b = 3 } }");
        assert_eq!(cpp.matches("} else if (").count(), 1);
        assert!(cpp.contains("} else {"));
    }

    #[test]
    fn genera_bucle_mientras() {
        let cpp = generar("programa P { mientras (a < 10) { a = a + 1 } }");
        assert!(cpp.contains("while ((a < 10))"));
    }

    #[test]
    fn genera_bucle_para() {
        let cpp = generar("programa P { para i desde 0 hasta 10 { i = i + 1 } }");
        assert!(cpp.contains("for (int i = 0; i < 10; i++)"));
    }

    #[test]
    fn genera_funcion_con_prototipo_y_referencia() {
        let cpp = generar(
            "programa P {\n\
             funcion suma(entero a, entero &b) -> entero {\n\
                 retornar a + b\n\
             }\n\
             }",
        );
        assert!(cpp.contains("int suma(int a, int& b);"));
        assert!(cpp.contains("int suma(int a, int& b) {"));
        assert!(cpp.contains("return (a + b);"));
    }

    #[test]
    fn genera_funcion_void() {
        let cpp = generar("programa P { funcion hola() { retornar } }");
        assert!(cpp.contains("void hola();"));
        assert!(cpp.contains("void hola() {"));
        assert!(cpp.contains("return;"));
    }

    #[test]
    fn genera_evento_de_clic() {
        let cpp = generar(
            "programa P {\n\
             ventana v = crear_ventana(\"v\", 1, 1)\n\
             boton b = crear_boton(\"b\")\n\
             al hacer_clic(b) { mostrar_mensaje(\"a\", \"b\") }\n\
             }",
        );
        assert!(cpp.contains("QObject::connect(b, &QPushButton::clicked, [&]() {"));
        assert!(cpp.contains("mostrar_mensaje(QString(\"a\"), QString(\"b\"));"));
    }

    #[test]
    fn incluye_ayudas_nativas() {
        let cpp = generar("programa P { }");
        assert!(cpp.contains("static QMainWindow* crear_ventana("));
        assert!(cpp.contains("static QPushButton* crear_boton("));
        assert!(cpp.contains("static QLabel* crear_etiqueta("));
        assert!(cpp.contains("static QLineEdit* crear_caja_texto("));
        assert!(cpp.contains("static QString texto(QLineEdit* caja)"));
        assert!(cpp.contains("static void establecer_texto(QLineEdit* caja"));
        assert!(cpp.contains("static void mostrar_mensaje("));
        assert!(cpp.contains("static void agregar("));
        assert!(cpp.contains("static void mostrar("));
    }
}
