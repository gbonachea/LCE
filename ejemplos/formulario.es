// Ejemplo: formulario que lee texto de una caja y responde (Qt)
// Compila con:  lce formulario.es -o formulario

programa Formulario {

    ventana v = crear_ventana("Saludo", 420, 220)
    caja_texto nombre = crear_caja_texto("Escribe tu nombre")
    boton b = crear_boton("Saludar")

    // Al pulsar el botón se lee el texto de la caja y se saluda
    al hacer_clic(b) {
        cadena n = texto(nombre)
        mostrar_mensaje("Saludo", "Hola, " + n)
    }

    agregar(v, nombre)
    agregar(v, b)
    mostrar(v)
}
