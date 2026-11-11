// Ejemplo: ventana con un botón interactivo
// Compila con:  lce ventana_boton.es -o mi_app

programa HolaMundo {

    // Crear la ventana principal (título, ancho, alto)
    ventana v = crear_ventana("Mi Primera Ventana", 400, 300)

    // Crear un botón
    boton b = crear_boton("Haz clic aquí")

    // Asociar un evento: al hacer clic se ejecuta el bloque
    al hacer_clic(b) {
        mostrar_mensaje("Información", "¡Hola, mundo desde LCE + Qt!")
    }

    // Añadir el botón a la ventana y mostrarla
    agregar(v, b)
    mostrar(v)
}
