// Ejemplo mínimo: variables y control de flujo
programa HolaMundo {

    entero edad = 25
    cadena nombre = "Ana"
    booleano es_mayor = edad >= 18

    si (es_mayor) {
        mostrar_mensaje("Bienvenida", nombre)
    } sino {
        mostrar_mensaje("Aviso", "Acceso denegado")
    }

    entero contador = 0
    mientras (contador < 3) {
        contador = contador + 1
    }
}
