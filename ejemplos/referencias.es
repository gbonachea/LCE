// Ejemplo: parámetros por referencia (&)
// Compila con:  lce referencias.es -o referencias

programa Referencias {

    // 'valor' se pasa por referencia: los cambios afectan al llamador.
    funcion incrementar(entero &valor) {
        valor = valor + 1
    }

    entero contador = 0
    incrementar(contador)
    incrementar(contador)

    mostrar_mensaje("Listo", "El contador se incrementó dos veces")
}
