// Ejemplo: bucle 'para' y condicional 'sino si'
// Compila con:  lce bucles.es -o bucles

programa Bucles {

    funcion es_par(entero n) -> booleano {
        retornar (n % 2) == 0
    }

    // Bucle 'para': i toma los valores 1, 2, 3, 4, 5 (hasta es exclusivo)
    entero suma = 0
    para i desde 1 hasta 6 {
        suma = suma + i
    }

    // Condicional encadenado 'sino si'
    si (suma > 20) {
        mostrar_mensaje("Resultado", "Mayor que 20")
    } sino si (suma > 10) {
        mostrar_mensaje("Resultado", "Mayor que 10")
    } sino {
        mostrar_mensaje("Resultado", "Menor o igual que 10")
    }
}
