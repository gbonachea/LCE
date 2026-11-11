// Ejemplo: funciones definidas por el usuario
// Compila con:  lce funciones.es -o calculadora

programa Calculadora {

    // Función con tipo de retorno
    funcion suma(entero a, entero b) -> entero {
        retornar a + b
    }

    funcion duplicar(entero x) -> entero {
        retornar x * 2
    }

    // Función sin retorno (equivale a 'nada' / void)
    funcion saludar(cadena nombre) {
        mostrar_mensaje("Bienvenida", nombre)
    }

    // Llamadas a las funciones definidas arriba
    entero total = suma(3, 4)
    entero doble = duplicar(total)
    saludar("Ana")
}
