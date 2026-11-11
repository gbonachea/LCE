# Módulo 1 — Diseño de la sintaxis y gramática

## 1. Identidad del lenguaje

- **Nombre:** LCE (Qt)
- **Extensión:** `.es`
- **Compilador:** `lce`
- **Paradigma:** imperativo, con funciones nativas de interfaz gráfica (Qt).
- **Destino:** C++17 + Qt6, compilado a binario nativo.

## 2. Palabras clave (reservadas)

| LCE        | C++ / Qt equivalente            | Categoría        |
| -------------- | ------------------------------- | ---------------- |
| `programa`     | `int main(...)`                 | estructura       |
| `funcion`      | función                          | estructura       |
| `si`           | `if`                             | control          |
| `sino`         | `else`                           | control          |
| `mientras`     | `while`                          | control          |
| `para`         | `for`                            | control          |
| `desde`        | `=` (inicio del rango)           | control          |
| `hasta`        | `<` (fin del rango, exclusivo)   | control          |
| `retornar`     | `return`                         | control          |
| `entero`       | `int`                            | tipo             |
| `decimal`      | `double`                         | tipo             |
| `booleano`     | `bool`                           | tipo             |
| `cadena`       | `QString`                        | tipo             |
| `nada`         | `void`                           | tipo             |
| `ventana`      | `QMainWindow*`                   | tipo de interfaz |
| `boton`        | `QPushButton*`                   | tipo de interfaz |
| `etiqueta`     | `QLabel*`                        | tipo de interfaz |
| `caja_texto`   | `QLineEdit*`                     | tipo de interfaz |
| `verdadero`    | `true`                           | literal          |
| `falso`        | `false`                          | literal          |
| `al` `hacer_clic` | `QObject::connect(...clicked)` | evento           |
| `y` / `o` / `no` | `&&` / `\|\|` / `!`            | lógico           |

## 3. Funciones nativas de interfaz

| Función                          | Firma                                  | Comportamiento                                  |
| -------------------------------- | -------------------------------------- | ----------------------------------------------- |
| `crear_ventana(titulo, ancho, alto)` | `(cadena, entero, entero) -> ventana` | Crea una `QMainWindow` con título y tamaño.      |
| `crear_boton(texto)`             | `(cadena) -> boton`                    | Crea un `QPushButton`.                          |
| `crear_etiqueta(texto)`          | `(cadena) -> etiqueta`                 | Crea un `QLabel`.                               |
| `crear_caja_texto(marcador)`     | `(cadena) -> caja_texto`               | Crea un `QLineEdit` con texto de marcador.      |
| `mostrar_mensaje(titulo, texto)` | `(cadena, cadena) -> nada`             | Muestra un `QMessageBox` informativo.           |
| `agregar(ventana, widget)`       | `(ventana, widget) -> nada`            | Añade el widget al layout vertical de la ventana.|
| `mostrar(widget)`                | `(widget) -> nada`                     | Muestra el widget (`show()`).                   |
| `texto(caja)`                    | `(caja_texto) -> cadena`               | Lee el texto actual de un `QLineEdit`.          |
| `establecer_texto(caja, valor)`  | `(caja_texto, cadena) -> nada`         | Cambia el texto de un `QLineEdit`.              |

## 4. Gramática (EBNF resumida)

```ebnf
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

declaracion_variable := tipo IDENT "=" expr term
tipo          := "entero" | "decimal" | "booleano" | "cadena"
               | "ventana" | "boton" | "etiqueta" | "caja_texto"

bloque        := "{" { declaracion } "}"

term          := ";" | salto_de_linea        ; terminador de sentencia
asignacion    := IDENT "=" expr term

expr          := igualdad
igualdad      := comparacion { ("==" | "!=") comparacion }
comparacion   := termino { ("<" | ">" | "<=" | ">=") termino }
termino       := factor { ("+" | "-") factor }
factor        := unario { ("*" | "/" | "%") unario }
unario        := ("-" | "no") unario | primario
primario      := NUMERO | CADENA | "verdadero" | "falso"
               | IDENT [ "(" [ expr { "," expr } ] ")" ]
               | "(" expr ")"
```

> **Nota:** cada sentencia termina con un salto de línea; el punto y coma (`;`)
> es opcional. Las líneas en blanco se ignoran.

> **Nota:** `nada` solo se usa como tipo de retorno. El bucle `para` genera
> `for (int var = inicio; var < fin; var++)`; su variable se infiere como
> `entero` y el límite `hasta` es exclusivo. Los parámetros pueden pasarse por
> referencia con `&` (`entero &x` → `int& x`).

### 4.1. Funciones de usuario

```es
funcion suma(entero a, entero b) -> entero {
    retornar a + b
}

// Sin retorno (equivale a 'nada' / void)
funcion saludar(cadena nombre) {
    mostrar_mensaje("Bienvenida", nombre)
}

entero total = suma(3, 4)     // -> 7
saludar("Ana")
```

El transpilador emite primero los **prototipos** (para permitir llamadas entre
funciones en cualquier orden) y luego las definiciones completas, todo antes de
`main`.

### 4.2. Bucle `para` y condicional `sino si`

```es
entero suma = 0
para i desde 1 hasta 6 {
    suma = suma + i          // i = 1, 2, 3, 4, 5  ->  suma = 15
}

si (suma > 20) {
    mostrar_mensaje("Resultado", "Mayor que 20")
} sino si (suma > 10) {
    mostrar_mensaje("Resultado", "Mayor que 10")
} sino {
    mostrar_mensaje("Resultado", "Menor o igual que 10")
}
```

- `para` se traduce a `for (int i = 1; i < 6; i++)`: variable inferida como
  `entero`, límite `hasta` exclusivo.
- `sino si` se traduce a `else if`, admitiendo encadenamientos arbitrarios.

### 4.3. Referencias, formularios y verificación de tipos

**Parámetros por referencia:**

```es
funcion incrementar(entero &valor) {
    valor = valor + 1        // modifica la variable del llamador
}

entero contador = 0
incrementar(contador)        // contador = 1
```

**Formularios (lectura de `caja_texto`):**

```es
ventana v = crear_ventana("Saludo", 420, 220)
caja_texto nombre = crear_caja_texto("Escribe tu nombre")
boton b = crear_boton("Saludar")

al hacer_clic(b) {
    cadena n = texto(nombre)                 // lee el QLineEdit
    mostrar_mensaje("Saludo", "Hola, " + n)  // concatenación de cadenas
}
```

**Verificación semántica.** Antes de generar C++, `lce` comprueba tipos y
declaraciones y reporta errores en español, por ejemplo:

```
Error semántico: La variable 'z' no está declarada
Error semántico: No se puede asignar un valor de tipo 'cadena' a la variable 'x' de tipo 'entero'
Error semántico: La función 'f' espera 1 argumento(s) pero recibió 2
```

El análisis se puede lanzar aislado con `lce archivo.es --solo-verificar`.

## 5. Ejemplo completo

Ver [`ejemplos/ventana_boton.es`](../ejemplos/ventana_boton.es):

```es
programa HolaMundo {

    ventana v = crear_ventana("Mi Primera Ventana", 400, 300)
    boton b = crear_boton("Haz clic aquí")

    al hacer_clic(b) {
        mostrar_mensaje("Información", "¡Hola, mundo desde LCE + Qt!")
    }

    agregar(v, b)
    mostrar(v)
}
```
