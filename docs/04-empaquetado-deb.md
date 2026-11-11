# Módulo 4 — Empaquetado y distribución en Linux (.deb)

## 1. Estructura de un paquete Debian

```
empaquetado/
├── build-deb.sh              # script que compila y ensambla el .deb
└── deb/
    └── DEBIAN/
        └── control           # metadatos del paquete
```

Al construirlo, el paquete temporal adopta la estructura de instalación:

```
lce_0.1.0_amd64/
├── DEBIAN/
│   └── control
└── usr/
    └── bin/
        └── lce             # binario compilado (cargo build --release)
```

La ruta `usr/bin/lce` dentro del paquete corresponde a `/usr/bin/lce` en el
sistema destino, que ya forma parte del `PATH` por defecto.

## 2. Archivo `DEBIAN/control`

Campos clave:

| Campo            | Valor                          | Significado                                  |
| ---------------- | ------------------------------ | -------------------------------------------- |
| `Package`        | `lce`                        | nombre del paquete                           |
| `Version`        | `0.1.0`                        | versión (Debian la usa para actualizaciones) |
| `Architecture`   | `amd64`                        | arquitectura destino                         |
| `Depends`        | `qt6-base-dev`, `g++`, etc.    | dependencias instaladas automáticamente      |
| `Section`        | `devel`                        | categoría (herramientas de desarrollo)       |
| `Priority`       | `optional`                     | prioridad                                    |

`Installed-Size` se rellena automáticamente en `build-deb.sh` con `du -sk`.

## 3. Construcción

```bash
chmod +x empaquetado/build-deb.sh
./empaquetado/build-deb.sh
```

El script:

1. Compila el motor: `cargo build --release`.
2. Crea la estructura temporal con el binario en `usr/bin/`.
3. Calcula `Installed-Size` con `du -sk`.
4. Ejecuta `dpkg-deb --build --root-owner-group`.

Resultado: `dist/lce_0.1.0_amd64.deb`.

## 4. Instalación y uso

```bash
# Instalar
sudo apt install ./dist/lce_0.1.0_amd64.deb
#   (o) sudo dpkg -i dist/lce_0.1.0_amd64.deb

# Verificar
which lce                    # -> /usr/bin/lce
lce --help

# Compilar un programa
lce mi_programa.es -o mi_programa
./mi_programa
```

## 5. Desinstalación y limpieza

```bash
sudo apt remove lce          # desinstala conservando configuración
sudo apt purge lce           # desinstala y elimina configuración
```

## 6. Distribución adicional

- **Repositorio APT:** alojar el `.deb` en un servidor y generar un índice
  `Packages` con `dpkg-scanpackages` para servirlo como repositorio.
- **PPA / Launchpad:** subir el paquete fuente para builds automáticos en
  múltiples arquitecturas.
- **AppImage / Flatpak:** alternativas para distribuir *aplicaciones* hechas
  con el lenguaje (no el compilador en sí).
