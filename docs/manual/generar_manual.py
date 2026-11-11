#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Genera el manual PDF del lenguaje LCE con ReportLab."""

import os
import re
import textwrap

from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER, TA_JUSTIFY, TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib.units import mm
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    BaseDocTemplate, Frame, Image, KeepTogether, PageBreak, PageTemplate,
    Paragraph, Preformatted, Spacer, Table, TableStyle,
)
from reportlab.platypus.tableofcontents import TableOfContents

# ----------------------------------------------------------------------------
# Rutas y fuentes
# ----------------------------------------------------------------------------
RAIZ = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
FUENTES = "/usr/share/fonts/truetype/dejavu"
ICONO = os.path.join(RAIZ, "extension", "icon.png")
SALIDA = os.path.join(RAIZ, "dist", "LCE-Manual-v0.1.0.pdf")

pdfmetrics.registerFont(TTFont("DejaVu", f"{FUENTES}/DejaVuSans.ttf"))
pdfmetrics.registerFont(TTFont("DejaVu-Bold", f"{FUENTES}/DejaVuSans-Bold.ttf"))
pdfmetrics.registerFont(TTFont("DejaVu-Oblique", f"{FUENTES}/DejaVuSans-Oblique.ttf"))
pdfmetrics.registerFont(TTFont("Mono", f"{FUENTES}/DejaVuSansMono.ttf"))
pdfmetrics.registerFont(TTFont("Mono-Bold", f"{FUENTES}/DejaVuSansMono-Bold.ttf"))
pdfmetrics.registerFontFamily("DejaVu", normal="DejaVu", bold="DejaVu-Bold",
                              italic="DejaVu-Oblique", boldItalic="DejaVu-Bold")
pdfmetrics.registerFontFamily("Mono", normal="Mono", bold="Mono-Bold",
                              italic="Mono", boldItalic="Mono-Bold")

# ----------------------------------------------------------------------------
# Medidas y colores
# ----------------------------------------------------------------------------
AZUL = colors.HexColor("#1e3a8a")
AZUL_CLARO = colors.HexColor("#3b82f6")
GRIS = colors.HexColor("#64748b")
GRIS_CLARO = colors.HexColor("#f1f5f9")
BORDE = colors.HexColor("#cbd5e1")
FONDO_OSCURO = colors.HexColor("#0f172a")

MARGEN_X = 20 * mm
MARGEN_SUP = 26 * mm
MARGEN_INF = 18 * mm
ANCHO = A4[0] - 2 * MARGEN_X

# ----------------------------------------------------------------------------
# Estilos
# ----------------------------------------------------------------------------
e_cuerpo = ParagraphStyle("Body", fontName="DejaVu", fontSize=10.2, leading=15,
                          alignment=TA_JUSTIFY, spaceAfter=6)
e_h1 = ParagraphStyle("H1", fontName="DejaVu-Bold", fontSize=19, leading=23,
                      textColor=AZUL, spaceBefore=6, spaceAfter=10, keepWithNext=1)
e_h2 = ParagraphStyle("H2", fontName="DejaVu-Bold", fontSize=13.5, leading=17,
                      textColor=AZUL, spaceBefore=12, spaceAfter=6, keepWithNext=1)
e_h3 = ParagraphStyle("H3", fontName="DejaVu-Bold", fontSize=11, leading=14,
                      textColor=colors.HexColor("#334155"), spaceBefore=8, spaceAfter=4,
                      keepWithNext=1)
e_codigo = ParagraphStyle("Code", fontName="Mono", fontSize=8.4, leading=11.4,
                          textColor=FONDO_OSCURO)
e_bullet = ParagraphStyle("Bullet", parent=e_cuerpo, leftIndent=16,
                          bulletIndent=4, spaceAfter=3)
e_celda = ParagraphStyle("Cell", fontName="DejaVu", fontSize=8.4, leading=11)
e_celda_cab = ParagraphStyle("CellHead", fontName="DejaVu-Bold", fontSize=8.6,
                             leading=11, textColor=colors.white)
e_aviso_t = ParagraphStyle("AvisoT", fontName="DejaVu-Bold", fontSize=9.6,
                           leading=13, textColor=AZUL, spaceAfter=3)
e_portada_t = ParagraphStyle("CoverT", fontName="DejaVu-Bold", fontSize=60,
                             leading=66, textColor=colors.white, alignment=TA_CENTER)
e_portada_sub = ParagraphStyle("CoverSub", fontName="DejaVu", fontSize=15,
                               leading=21, textColor=colors.HexColor("#93c5fd"),
                               alignment=TA_CENTER)
e_portada_info = ParagraphStyle("CoverInfo", fontName="DejaVu", fontSize=10.5,
                                leading=16, textColor=colors.HexColor("#e2e8f0"),
                                alignment=TA_CENTER)
e_indice = ParagraphStyle("Indice", parent=e_h1)


# ----------------------------------------------------------------------------
# Utilidades de marcado
# ----------------------------------------------------------------------------
def esc(t: str) -> str:
    return t.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def fmt(t: str) -> str:
    t = esc(t)
    t = re.sub(r"\*\*(.+?)\*\*", r"<b>\1</b>", t)
    t = re.sub(r"`(.+?)`", r'<font name="Mono">\1</font>', t)
    return t


def H1(t): return Paragraph(fmt(t), e_h1)
def H2(t): return Paragraph(fmt(t), e_h2)
def H3(t): return Paragraph(fmt(t), e_h3)
def P(t): return Paragraph(fmt(t), e_cuerpo)
def B(t): return Paragraph(fmt(t), e_bullet, bulletText="•")


def COD(t: str):
    texto = textwrap.dedent(t).strip("\n")
    pre = Preformatted(texto, e_codigo)
    tabla = Table([[pre]], colWidths=[ANCHO])
    tabla.setStyle(TableStyle([
        ("BACKGROUND", (0, 0), (-1, -1), GRIS_CLARO),
        ("BOX", (0, 0), (-1, -1), 0.6, BORDE),
        ("LEFTPADDING", (0, 0), (-1, -1), 9),
        ("RIGHTPADDING", (0, 0), (-1, -1), 9),
        ("TOPPADDING", (0, 0), (-1, -1), 7),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 7),
    ]))
    return tabla


def AVISO(titulo, texto, fondo="#dbeafe", borde=AZUL_CLARO):
    contenido = [Paragraph(fmt(titulo), e_aviso_t), Paragraph(fmt(texto), e_cuerpo)]
    tabla = Table([[contenido]], colWidths=[ANCHO])
    tabla.setStyle(TableStyle([
        ("BACKGROUND", (0, 0), (-1, -1), colors.HexColor(fondo)),
        ("BOX", (0, 0), (-1, -1), 0.8, borde),
        ("LEFTPADDING", (0, 0), (-1, -1), 10),
        ("RIGHTPADDING", (0, 0), (-1, -1), 10),
        ("TOPPADDING", (0, 0), (-1, -1), 8),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 8),
    ]))
    return tabla


def REGLA(color=AZUL_CLARO, ancho=40 * mm, alto=1.2 * mm):
    t = Table([[""]], colWidths=[ancho], rowHeights=[alto])
    t.setStyle(TableStyle([("BACKGROUND", (0, 0), (-1, -1), color)]))
    t.hAlign = "CENTER"
    return t


def TABLA(cabeceras, filas, anchos=None):
    datos = [[Paragraph(fmt(c), e_celda_cab) for c in cabeceras]]
    for fila in filas:
        datos.append([Paragraph(fmt(c), e_celda) for c in fila])
    if anchos is None:
        anchos = [ANCHO / len(cabeceras)] * len(cabeceras)
    else:
        total = sum(anchos)
        anchos = [ANCHO * a / total for a in anchos]
    tabla = Table(datos, colWidths=anchos, repeatRows=1)
    estilo = [
        ("BACKGROUND", (0, 0), (-1, 0), AZUL),
        ("TEXTCOLOR", (0, 0), (-1, 0), colors.white),
        ("GRID", (0, 0), (-1, -1), 0.4, BORDE),
        ("VALIGN", (0, 0), (-1, -1), "MIDDLE"),
        ("LEFTPADDING", (0, 0), (-1, -1), 6),
        ("RIGHTPADDING", (0, 0), (-1, -1), 6),
        ("TOPPADDING", (0, 0), (-1, -1), 4),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 4),
    ]
    for i in range(1, len(datos)):
        if i % 2 == 0:
            estilo.append(("BACKGROUND", (0, i), (-1, i), colors.HexColor("#f8fafc")))
    tabla.setStyle(TableStyle(estilo))
    return tabla


# ----------------------------------------------------------------------------
# Documento con índice y cabecera/pie
# ----------------------------------------------------------------------------
class ManualDoc(BaseDocTemplate):
    def __init__(self, filename, **kw):
        super().__init__(filename, **kw)
        marco = Frame(MARGEN_X, MARGEN_INF, ANCHO,
                      A4[1] - MARGEN_SUP - MARGEN_INF, id="normal")
        self.addPageTemplates([PageTemplate(id="todo", frames=[marco],
                                            onPage=self.decorar)])
        self.toc = TableOfContents()
        self.toc.levelStyles = [
            ParagraphStyle("TOC1", fontName="DejaVu-Bold", fontSize=10.5,
                           leading=16, spaceBefore=7, leftIndent=0),
            ParagraphStyle("TOC2", fontName="DejaVu", fontSize=9.2,
                           leading=13, leftIndent=18, spaceBefore=2),
        ]
        self.toc.dotsMinLevel = 0

    def decorar(self, canvas, doc):
        canvas.saveState()
        if doc.page == 1:
            canvas.setFillColor(FONDO_OSCURO)
            canvas.rect(0, 0, A4[0], A4[1], fill=1, stroke=0)
            canvas.setFillColor(AZUL)
            canvas.rect(0, A4[1] - 13 * mm, A4[0], 13 * mm, fill=1, stroke=0)
            canvas.rect(0, 0, A4[0], 8 * mm, fill=1, stroke=0)
            canvas.setFillColor(AZUL_CLARO)
            canvas.rect(0, A4[1] - 13 * mm, A4[0], 1.2 * mm, fill=1, stroke=0)
        else:
            canvas.setFillColor(AZUL)
            canvas.rect(0, A4[1] - 17 * mm, A4[0], 17 * mm, fill=1, stroke=0)
            canvas.setFillColor(colors.white)
            canvas.setFont("DejaVu-Bold", 8.8)
            canvas.drawString(MARGEN_X, A4[1] - 11.5 * mm, "LCE · Manual del lenguaje")
            canvas.setFont("DejaVu", 8)
            canvas.drawRightString(A4[0] - MARGEN_X, A4[1] - 11.5 * mm, "v0.1.0")
            canvas.setFillColor(GRIS)
            canvas.setFont("DejaVu", 8)
            canvas.drawCentredString(A4[0] / 2, 9 * mm, f"Página {doc.page}")
        canvas.restoreState()

    def afterFlowable(self, flowable):
        if isinstance(flowable, Paragraph):
            nombre = flowable.style.name
            texto = flowable.getPlainText()
            if nombre == "H1":
                self.notify("TOCEntry", (0, texto, self.page))
            elif nombre == "H2":
                self.notify("TOCEntry", (1, texto, self.page))


# ----------------------------------------------------------------------------
# Portada e índice
# ----------------------------------------------------------------------------
historia = []

historia += [
    Spacer(1, 58 * mm),
    Paragraph("LCE", e_portada_t),
    Spacer(1, 5 * mm),
    Paragraph("Manual del lenguaje y guía de referencia", e_portada_sub),
    Spacer(1, 6 * mm),
    REGLA(AZUL_CLARO, 42 * mm, 1.4 * mm),
    Spacer(1, 10 * mm),
    Paragraph("Aprende a crear aplicaciones nativas con Qt<br/>desde cero y en español", e_portada_info),
    Spacer(1, 74 * mm),
    Paragraph("Versión 0.1.0 · Noviembre de 2026", e_portada_info),
    Paragraph("Sintaxis en español · Compilación a C++/Qt6", e_portada_info),
    PageBreak(),
    Paragraph(fmt("Índice"), e_indice),
    Spacer(1, 4 * mm),
]

# Marcador que se sustituye por el índice real antes de construir.
historia.append(Spacer(1, 1))
marcador_toc = len(historia) - 1
historia.append(PageBreak())

# ----------------------------------------------------------------------------
# 1. Introducción
# ----------------------------------------------------------------------------
historia += [
    H1("1. ¿Qué es LCE?"),
    P("**LCE** es un lenguaje de programación con **sintaxis 100 % en español** "
      "diseñado para crear aplicaciones de escritorio nativas con el framework "
      "**Qt (C++)**. El código se escribe en archivos `.es` (y también `.lce`), se "
      "traduce a C++17/Qt6 y se compila a un **binario nativo de alto rendimiento**."),
    P("No es un intérprete ni una máquina virtual: LCE es un **transpilador**. El "
      "compilador `lce` lee tu programa, comprueba que los tipos y las declaraciones "
      "sean correctos y genera C++ que `g++` (o `clang++`) convierte en un ejecutable "
      "que corre directamente sobre el sistema operativo."),
    H2("1.1. ¿Para quién es este manual?"),
    P("Para **programadores** que quieran una sintaxis clara en español y la potencia "
      "de Qt, y para **personas que nunca han programado** que quieran aprender desde "
      "cero con ejemplos guiados. No necesitas saber C++ para empezar, aunque ayuda."),
    H2("1.2. Cómo funciona"),
    COD("""
        archivo.es
            │
            ▼
        ┌─────────┐   ┌─────────┐   ┌────────────┐   ┌──────────┐   ┌──────────────┐
        │ Léxico  │──►│ Sintaxis│──►│ Semántica  │──►│ Código   │──►│ g++ / clang++│
        │ tokens  │   │  (AST)  │   │  (tipos)   │   │ temp.cpp │   │ binario nativo│
        └─────────┘   └─────────┘   └────────────┘   └──────────┘   └──────────────┘
    """),
    P("Cada fase produce errores claros **en español**. Si algo falla, recibirás un "
      "mensaje del tipo `Error semántico: La variable 'x' no está declarada` antes de "
      "llegar al compilador de C++."),
    AVISO("Requisitos",
          "Linux (probado en Ubuntu/Debian), `g++` o `clang++`, `pkg-config` y las "
          "bibliotecas de desarrollo de Qt6 (`qt6-base-dev`). El compilador `lce` y la "
          "extensión de VS Code se instalan en los siguientes apartados."),
    PageBreak(),
]

# ----------------------------------------------------------------------------
# 2. Instalación
# ----------------------------------------------------------------------------
historia += [
    H1("2. Instalación"),
    H2("2.1. Instalar el compilador (.deb)"),
    P("Descarga `lce_0.1.0_amd64.deb` y ejecuta:"),
    COD("""
        sudo apt install ./lce_0.1.0_amd64.deb
        # alternativa:
        sudo dpkg -i lce_0.1.0_amd64.deb
    """),
    P("El binario queda en `/usr/bin/lce`. Comprueba la instalación:"),
    COD("""
        which lce          # -> /usr/bin/lce
        lce --help
    """),
    H2("2.2. Instalar la extensión de VS Code"),
    P("Instala el paquete `.vsix` desde la terminal:"),
    COD("""
        code --install-extension lce-0.1.0.vsix
    """),
    P("O desde la interfaz: **Extensiones → menú ⋯ → Instalar desde VSIX**. "
      "La extensión aporta resaltado de sintaxis, fragmentos (snippets), comentarios, "
      "autocompletado de llaves y una tarea para compilar con **Ctrl+Shift+B**."),
    P("Si usas OpenCode o prefieres una instalación manual, descomprime "
      "`lce-0.1.0.zip` en `~/.vscode/extensions/lce/`."),
    H2("2.3. Compilar el motor desde el código fuente (opcional)"),
    P("Si prefieres compilar tú mismo el compilador, necesitas Rust:"),
    COD("""
        cd motor
        cargo build --release      # binario en target/release/lce
        cargo test                 # 90 tests
    """),
    AVISO("Consejo",
          "Para que la tarea de VS Code encuentre el compilador, `lce` debe estar en el "
          "`PATH`. El paquete `.deb` ya lo deja en `/usr/bin/lce`.", "#dcfce7", "#22c55e"),
    PageBreak(),
]

# ----------------------------------------------------------------------------
# 3. Primer programa
# ----------------------------------------------------------------------------
historia += [
    H1("3. Tu primer programa"),
    P("Crea un archivo llamado `hola.es` con este contenido:"),
    COD("""
        programa Hola {

            ventana v = crear_ventana("Mi primera app", 400, 300)
            boton b = crear_boton("Haz clic aquí")

            al hacer_clic(b) {
                mostrar_mensaje("Información", "¡Hola desde LCE!")
            }

            agregar(v, b)
            mostrar(v)
        }
    """),
    P("Compila y ejecuta:"),
    COD("""
        lce hola.es -o hola
        ./hola
    """),
    P("Se abrirá una ventana con un botón; al pulsarlo aparece un cuadro de diálogo. "
      "Enhorabuena: acabas de crear una aplicación nativa de Qt."),
    H2("3.1. ¿Qué hace cada línea?"),
    B("`programa Hola { ... }` define el programa y su nombre. Todo lo que está dentro "
      "se ejecuta al arrancar."),
    B("`ventana v = crear_ventana(...)` crea una ventana principal (`QMainWindow`) con "
      "título y tamaño."),
    B("`boton b = crear_boton(...)` crea un botón (`QPushButton`)."),
    B("`al hacer_clic(b) { ... }` registra el código que se ejecuta al pulsar el botón."),
    B("`agregar(v, b)` coloca el botón dentro de la ventana; `mostrar(v)` muestra la ventana."),
    AVISO("Sin punto y coma",
          "En LCE cada instrucción termina con un **salto de línea**. El punto y coma "
          "`;` es opcional. Las líneas en blanco se ignoran.", "#fef9c3", "#eab308"),
    PageBreak(),
]

# ----------------------------------------------------------------------------
# 4. Fundamentos
# ----------------------------------------------------------------------------
historia += [
    H1("4. Fundamentos del lenguaje"),
    H2("4.1. Estructura de un programa"),
    P("Todo programa empieza con la palabra `programa`, un nombre y un bloque entre "
      "llaves. Dentro del bloque pueden aparecer variables, control de flujo, funciones "
      "y llamadas a funciones nativas de Qt."),
    H2("4.2. Comentarios"),
    P("Los comentarios se escriben con `//` o con `#` y llegan hasta el final de la línea:"),
    COD("""
        // Esto es un comentario
        # Esto también
        entero edad = 25   // comentario al final de la línea
    """),
    H2("4.3. Variables y tipos"),
    P("Una variable se declara con su **tipo**, un nombre y un valor inicial:"),
    COD("""
        entero   edad     = 25
        decimal  precio   = 19.99
        booleano activo   = verdadero
        cadena   nombre   = "Ana"
    """),
    TABLA(["Tipo", "Equivalente en C++", "Qué guarda"], [
        ["entero", "int", "Números enteros (1, -7, 42)"],
        ["decimal", "double", "Números con decimales (3.14)"],
        ["booleano", "bool", "verdadero o falso"],
        ["cadena", "QString", "Texto entre comillas"],
        ["nada", "void", "Ausencia de valor (solo en funciones)"],
        ["ventana", "QMainWindow*", "Una ventana principal"],
        ["boton", "QPushButton*", "Un botón"],
        ["etiqueta", "QLabel*", "Una etiqueta de texto"],
        ["caja_texto", "QLineEdit*", "Un campo de texto"],
    ], [18, 22, 60]),
    H2("4.4. Asignación"),
    P("Para cambiar el valor de una variable ya declarada se usa `=`:"),
    COD("""
        entero contador = 0
        contador = contador + 1
    """),
    H2("4.5. Operadores"),
    TABLA(["Categoría", "Operadores", "Ejemplo"], [
        ["Aritméticos", "+  -  *  /  %", "a + b * 2"],
        ["Comparación", "==  !=  <  >  <=  >=", "edad >= 18"],
        ["Lógicos", "y   o   no", "activo y no bloqueado"],
        ["Concatenación", "+", '"Hola, " + nombre'],
        ["Negación", "-   no", "-5   no verdadero"],
    ], [20, 40, 40]),
    P("Los operadores tienen la precedencia habitual: primero `*`, `/`, `%`; después "
      "`+`, `-`; luego las comparaciones; y por último `y` / `o`. Usa paréntesis para "
      "dejar clara la intención:"),
    COD("""
        booleano mayor = (edad >= 18) y (saldo > 0)
    """),
    AVISO("Concatenar texto",
          "El operador `+` suma números y también **une cadenas**: "
          '`"Hola, " + nombre` produce un único texto.', "#dbeafe", AZUL_CLARO),
    PageBreak(),
]

# ----------------------------------------------------------------------------
# 5. Control de flujo
# ----------------------------------------------------------------------------
historia += [
    H1("5. Control de flujo"),
    H2("5.1. Condicional si / sino / sino si"),
    P("La condición debe ser de tipo `booleano`:"),
    COD("""
        si (edad >= 18) {
            mostrar_mensaje("Acceso", "Bienvenido")
        } sino si (edad >= 13) {
            mostrar_mensaje("Acceso", "Acceso juvenil")
        } sino {
            mostrar_mensaje("Acceso", "Acceso denegado")
        }
    """),
    P("`sino si` permite encadenar condiciones; se traduce a `else if` y admite tantas "
      "ramas como necesites."),
    H2("5.2. Bucle mientras"),
    P("Repite un bloque mientras la condición sea verdadera:"),
    COD("""
        entero contador = 0
        mientras (contador < 3) {
            contador = contador + 1
        }
    """),
    H2("5.3. Bucle para"),
    P("Recorre un rango de enteros. La variable se crea automáticamente y el límite "
      "`hasta` es **exclusivo**:"),
    COD("""
        entero suma = 0
        para i desde 1 hasta 6 {
            suma = suma + i     // i = 1, 2, 3, 4, 5
        }
    """),
    P("`para i desde 1 hasta 6` se traduce a `for (int i = 1; i < 6; i++)`."),
    PageBreak(),
]

# ----------------------------------------------------------------------------
# 6. Funciones
# ----------------------------------------------------------------------------
historia += [
    H1("6. Funciones"),
    P("Una función agrupa código reutilizable. Se declara con `funcion`, un nombre, "
      "parámetros con tipo y, opcionalmente, un tipo de retorno con `->`:"),
    COD("""
        funcion suma(entero a, entero b) -> entero {
            retornar a + b
        }

        entero total = suma(3, 4)     // total = 7
    """),
    H2("6.1. Funciones sin retorno"),
    P("Si se omite el tipo de retorno, la función devuelve `nada` (void):"),
    COD("""
        funcion saludar(cadena nombre) {
            mostrar_mensaje("Bienvenida", nombre)
        }
    """),
    H2("6.2. Parámetros por referencia"),
    P("Con `&` el parámetro se pasa por referencia y los cambios afectan a la variable "
      "original del llamador:"),
    COD("""
        funcion incrementar(entero &valor) {
            valor = valor + 1
        }

        entero contador = 0
        incrementar(contador)     // contador = 1
    """),
    AVISO("Referencias",
          "El argumento de un parámetro `&` debe ser una **variable** (no un literal) y "
          "del mismo tipo. `incrementar(5)` produce un error semántico.", "#fef9c3", "#eab308"),
    H2("6.3. Prototipos y orden"),
    P("Puedes llamar a una función definida más abajo en el archivo: el compilador emite "
      "primero los prototipos de todas las funciones y luego sus definiciones. Esto "
      "permite también la **recursión**:"),
    COD("""
        funcion factorial(entero n) -> entero {
            si (n <= 1) {
                retornar 1
            }
            retornar n * factorial(n - 1)
        }
    """),
    PageBreak(),
]

# ----------------------------------------------------------------------------
# 7. Interfaz gráfica
# ----------------------------------------------------------------------------
historia += [
    H1("7. Interfaz gráfica con Qt"),
    P("LCE incluye funciones nativas para construir interfaces. Cada widget se guarda en "
      "una variable de su tipo y luego se coloca en la ventana con `agregar`."),
    H2("7.1. Crear ventana y widgets"),
    COD("""
        ventana v = crear_ventana("Título", 500, 350)
        boton b = crear_boton("Aceptar")
        etiqueta e = crear_etiqueta("Hola")
        caja_texto c = crear_caja_texto("Escribe aquí")
    """),
    H2("7.2. Colocar y mostrar"),
    COD("""
        agregar(v, e)      // añade la etiqueta al layout vertical
        agregar(v, c)      // añade la caja de texto
        agregar(v, b)      // añade el botón
        mostrar(v)         // muestra la ventana
    """),
    P("Los widgets se apilan en un layout vertical en el orden en que se agregan."),
    H2("7.3. Eventos"),
    COD("""
        al hacer_clic(b) {
            mostrar_mensaje("Título", "Texto del mensaje")
        }
    """),
    H2("7.4. Leer y escribir en una caja de texto"),
    COD("""
        cadena texto_usuario = texto(c)        // lee el contenido
        establecer_texto(c, "Nuevo valor")     // cambia el contenido
    """),
    H2("7.5. Ejemplo completo: formulario"),
    COD("""
        programa Formulario {

            ventana v = crear_ventana("Saludo", 420, 220)
            caja_texto nombre = crear_caja_texto("Escribe tu nombre")
            boton b = crear_boton("Saludar")

            al hacer_clic(b) {
                cadena n = texto(nombre)
                mostrar_mensaje("Saludo", "Hola, " + n)
            }

            agregar(v, nombre)
            agregar(v, b)
            mostrar(v)
        }
    """),
    PageBreak(),
]

# ----------------------------------------------------------------------------
# 8. Tablas de referencia
# ----------------------------------------------------------------------------
historia += [
    H1("8. Referencia de palabras clave"),
    TABLA(["Palabra", "Equivalente", "Uso"], [
        ["programa", "main", "Inicia el programa"],
        ["funcion", "function", "Define una función"],
        ["si", "if", "Condición"],
        ["sino", "else", "Alternativa"],
        ["mientras", "while", "Bucle condicional"],
        ["para", "for", "Bucle con rango"],
        ["desde / hasta", "inicio / fin", "Límites del bucle para"],
        ["retornar", "return", "Devuelve un valor"],
        ["entero", "int", "Tipo entero"],
        ["decimal", "double", "Tipo decimal"],
        ["booleano", "bool", "Tipo lógico"],
        ["cadena", "QString", "Tipo texto"],
        ["nada", "void", "Sin valor"],
        ["ventana", "QMainWindow*", "Ventana principal"],
        ["boton", "QPushButton*", "Botón"],
        ["etiqueta", "QLabel*", "Etiqueta de texto"],
        ["caja_texto", "QLineEdit*", "Campo de texto"],
        ["verdadero / falso", "true / false", "Literales booleanos"],
        ["al hacer_clic", "connect(clicked)", "Evento de clic"],
        ["y / o / no", "&& / || / !", "Operadores lógicos"],
    ], [24, 30, 46]),
    H1("9. Referencia de funciones nativas"),
    TABLA(["Función", "Firma", "Descripción"], [
        ["crear_ventana", "(cadena, entero, entero) -> ventana", "Crea la ventana principal"],
        ["crear_boton", "(cadena) -> boton", "Crea un botón"],
        ["crear_etiqueta", "(cadena) -> etiqueta", "Crea una etiqueta"],
        ["crear_caja_texto", "(cadena) -> caja_texto", "Crea un campo de texto"],
        ["mostrar_mensaje", "(cadena, cadena) -> nada", "Muestra un cuadro de diálogo"],
        ["agregar", "(ventana, widget) -> nada", "Añade un widget a la ventana"],
        ["mostrar", "(widget) -> nada", "Muestra una ventana o widget"],
        ["texto", "(caja_texto) -> cadena", "Lee el texto de la caja"],
        ["establecer_texto", "(caja_texto, cadena) -> nada", "Cambia el texto de la caja"],
    ], [24, 40, 36]),
    PageBreak(),
]

# ----------------------------------------------------------------------------
# 10. Línea de comandos
# ----------------------------------------------------------------------------
historia += [
    H1("10. El compilador lce"),
    P("Uso general:"),
    COD("""
        lce <archivo.es> [opciones]
    """),
    TABLA(["Opción", "Descripción"], [
        ["-o <salida>", "Nombre del binario resultante (por defecto: programa)"],
        ["--no-compilar", "Solo genera temp.cpp, sin invocar a g++/clang++"],
        ["--solo-verificar", "Analiza léxico, sintaxis y tipos sin generar código"],
        ["-h, --help", "Muestra la ayuda"],
    ], [26, 74]),
    H2("10.1. Flujo típico"),
    COD("""
        lce programa.es -o programa     # verifica, genera C++ y compila
        ./programa                      # ejecuta el binario nativo
    """),
    H2("10.2. Depurar la generación"),
    COD("""
        lce programa.es --solo-verificar   # solo comprueba tipos
        lce programa.es --no-compilar      # genera temp.cpp para inspeccionarlo
    """),
    H2("10.3. Mensajes de error"),
    P("Los errores indican la fase en la que se produjeron:"),
    COD("""
        Error léxico: Cadena de texto sin cerrar
        Error de sintaxis: Se esperaba ')' (se encontró LlaveDer)
        Error semántico: La variable 'z' no está declarada
        Error semántico: No se puede asignar un valor de tipo 'cadena'
                         a la variable 'x' de tipo 'entero'
    """),
    PageBreak(),
]

# ----------------------------------------------------------------------------
# 11. VS Code
# ----------------------------------------------------------------------------
historia += [
    H1("11. VS Code: atajos y productividad"),
    P("La extensión de LCE integra el lenguaje en VS Code con resaltado, fragmentos y "
      "compilación. Estos son los atajos más útiles:"),
    TABLA(["Acción", "Atajo"], [
        ["Compilar el archivo actual", "Ctrl+Shift+B"],
        ["Ejecutar / depurar", "F5"],
        ["Comentar / descomentar línea", "Ctrl+/"],
        ["Insertar fragmento (snippet)", "prefijo + Tab"],
        ["Paleta de comandos", "Ctrl+Shift+P"],
        ["Buscar en el archivo", "Ctrl+F"],
        ["Abrir terminal integrada", "Ctrl+ñ  (o Ctrl+`)"],
    ], [60, 40]),
    H2("11.1. Fragmentos disponibles"),
    TABLA(["Prefijo", "Qué inserta"], [
        ["programa", "Bloque principal del programa"],
        ["ventana", "Ventana principal"],
        ["boton", "Botón"],
        ["clic", "Manejador al hacer_clic"],
        ["si", "Condicional si / sino"],
        ["mientras", "Bucle mientras"],
        ["para", "Bucle para con rango"],
        ["mensaje", "Cuadro de mensaje"],
    ], [30, 70]),
    H2("11.2. Tarea de compilación"),
    P("El archivo `.vscode/tasks.json` de la extensión define la tarea por defecto "
      "(Ctrl+Shift+B):"),
    COD("""
        lce ${file} -o ${fileDirname}/${fileBasenameNoExtension}
    """),
    P("También incluye una tarea para generar solo `temp.cpp` y otra para depurar con "
      "`gdb` (F5)."),
    PageBreak(),
]

# ----------------------------------------------------------------------------
# 12. Solución de problemas
# ----------------------------------------------------------------------------
historia += [
    H1("12. Solución de problemas"),
    TABLA(["Síntoma", "Causa probable", "Solución"], [
        ["lce: command not found", "No está instalado", "sudo apt install ./lce_0.1.0_amd64.deb"],
        ["No se pudieron obtener las banderas de Qt6", "Falta Qt6", "sudo apt install qt6-base-dev"],
        ["g++: command not found", "Falta compilador", "sudo apt install build-essential"],
        ["Error semántico: variable no declarada", "Nombre mal escrito", "Revisa la ortografía y el ámbito"],
        ["Error de sintaxis: se esperaba ')'", "Paréntesis sin cerrar", "Revisa las llaves y paréntesis"],
        ["Error: la función espera N argumentos", "Aridad incorrecta", "Pasa el número correcto de argumentos"],
        ["El botón no responde", "Falta al hacer_clic", "Asocia el evento al botón"],
    ], [34, 30, 36]),
    AVISO("Consejo de depuración",
          "Usa `lce archivo.es --solo-verificar` para detectar errores de tipo antes de "
          "compilar. Los mensajes indican exactamente qué variable o argumento falla.",
          "#dcfce7", "#22c55e"),
    PageBreak(),
]

# ----------------------------------------------------------------------------
# 13. Glosario
# ----------------------------------------------------------------------------
historia += [
    H1("13. Glosario"),
    TABLA(["Término", "Significado"], [
        ["Transpilador", "Programa que traduce de un lenguaje a otro (LCE → C++)."],
        ["Token", "Unidad mínima del código (palabra, número, símbolo)."],
        ["AST", "Árbol de sintaxis abstracta: representación del programa."],
        ["Semántica", "Fase que comprueba tipos, declaraciones y ámbitos."],
        ["Widget", "Elemento de interfaz gráfica (botón, caja, ventana)."],
        ["Layout", "Disposición que organiza los widgets dentro de la ventana."],
        ["Binario nativo", "Ejecutable que corre directamente en la CPU."],
    ], [24, 76]),
    PageBreak(),
]

# ----------------------------------------------------------------------------
# Apéndices
# ----------------------------------------------------------------------------
historia += [
    H1("Apéndice A. Gramática (EBNF)"),
    COD("""
        programa      := "programa" IDENT "{" { declaracion } "}"

        declaracion   := declaracion_variable
                       | asignacion
                       | "si" "(" expr ")" bloque
                           { "sino" ( bloque | "si" "(" expr ")" bloque ) }
                       | "mientras" "(" expr ")" bloque
                       | "para" IDENT "desde" expr "hasta" expr bloque
                       | "retornar" [ expr ] term
                       | "al" "hacer_clic" "(" IDENT ")" bloque
                       | llamada term
                       | "funcion" IDENT "(" [ parametro { "," parametro } ] ")"
                           [ "->" tipo_retorno ] bloque

        parametro     := tipo [ "&" ] IDENT
        tipo_retorno  := tipo | "nada"
        tipo          := "entero" | "decimal" | "booleano" | "cadena"
                       | "ventana" | "boton" | "etiqueta" | "caja_texto"

        bloque        := "{" { declaracion } "}"
        term          := ";" | salto_de_linea
        asignacion    := IDENT "=" expr term

        expr          := logico_o
        logico_o      := logico_y { "o" logico_y }
        logico_y      := igualdad { "y" igualdad }
        igualdad      := comparacion { ("==" | "!=") comparacion }
        comparacion   := termino { ("<" | ">" | "<=" | ">=") termino }
        termino       := factor { ("+" | "-") factor }
        factor        := unario { ("*" | "/" | "%") unario }
        unario        := ("-" | "no") unario | primario
        primario      := NUMERO | CADENA | "verdadero" | "falso"
                       | IDENT [ "(" [ expr { "," expr } ] ")" ]
                       | "(" expr ")"
    """),
    H1("Apéndice B. Chuleta rápida"),
    COD("""
        // Programa mínimo
        programa P {
            ventana v = crear_ventana("App", 400, 300)
            boton b = crear_boton("OK")
            al hacer_clic(b) { mostrar_mensaje("Hola", "Mundo") }
            agregar(v, b)
            mostrar(v)
        }

        // Variables
        entero x = 10
        decimal d = 3.14
        booleano ok = verdadero
        cadena s = "texto"

        // Control
        si (x > 0) { ... } sino { ... }
        mientras (x > 0) { x = x - 1 }
        para i desde 0 hasta 10 { ... }

        // Funciones
        funcion f(entero a, entero &b) -> entero { retornar a + b }
    """),
    H1("Apéndice C. Recursos"),
    B("Código fuente del motor: carpeta `motor/` (Rust)."),
    B("Ejemplos completos: carpeta `ejemplos/` (6 programas)."),
    B("Extensión de VS Code: carpeta `extension/` y paquete `lce-0.1.0.vsix`."),
    B("Paquete de instalación: `lce_0.1.0_amd64.deb`."),
    Spacer(1, 8 * mm),
    P("**LCE** · Manual del lenguaje · Versión 0.1.0 · Noviembre de 2026"),
]

# ----------------------------------------------------------------------------
# Construcción con índice de dos pasadas
# ----------------------------------------------------------------------------
doc = ManualDoc(
    SALIDA,
    pagesize=A4,
    leftMargin=MARGEN_X,
    rightMargin=MARGEN_X,
    topMargin=MARGEN_SUP,
    bottomMargin=MARGEN_INF,
    title="LCE — Manual del lenguaje",
    author="Equipo LCE",
    subject="Guía de programación del lenguaje LCE (Qt)",
    creator="LCE",
)

# Se reemplaza el marcador por el TOC real antes de construir.
historia[marcador_toc] = doc.toc

os.makedirs(os.path.dirname(SALIDA), exist_ok=True)
doc.multiBuild(historia)
print(f"PDF generado: {SALIDA}")
