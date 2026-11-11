# Módulo 5 — Integración con OpenCode (skill)

## 5.1. Extensión de VS Code vs. skill de OpenCode

Son **mecanismos distintos** y no son intercambiables:

| | Extensión de VS Code (`.vsix` / `.zip`) | Skill de OpenCode (`SKILL.md`) |
| --- | --- | --- |
| Para qué sirve | Resaltado de sintaxis, snippets, tareas y depuración en el **editor** | Instrucciones reutilizables para el **agente** de OpenCode |
| Formato | Paquete VSIX/ZIP con `package.json` y gramática TextMate | Carpeta con `SKILL.md` (frontmatter YAML + Markdown) |
| Quién lo consume | VS Code / editores compatibles | OpenCode (servicio/agente) |
| Instalación | `code --install-extension lce-0.1.0.vsix` | copiar a `.opencode/skills/` o `~/.config/opencode/skills/` |

Conclusión: **el `.vsix` no se instala en OpenCode**. Para que el agente sepa
programar en LCE se crea una **skill**, que es lo que OpenCode descubre y
publica al modelo.

## 5.2. Qué es una skill

Una skill es un archivo `SKILL.md` con:

- **frontmatter YAML**: `name` y `description` (la descripción es lo que ve el
  modelo para decidir cuándo usarla);
- **cuerpo Markdown**: el flujo de trabajo e instrucciones;
- **archivos de apoyo** opcionales (`references/`, `scripts/`, plantillas).

El **ID** de la skill es el nombre de su carpeta (no el `name` del frontmatter).
Las rutas dentro de la skill son relativas al directorio que contiene `SKILL.md`.

## 5.3. Ubicaciones de descubrimiento

| Ámbito | Rutas |
| --- | --- |
| Proyecto | `.opencode/skills/<id>/SKILL.md` |
| Global | `~/.config/opencode/skills/<id>/SKILL.md` |
| Compatibilidad | `.claude/skills`, `.agents/skills`, `~/.claude/skills`, `~/.agents/skills` |

También se pueden añadir directorios o catálogos HTTP con el arreglo `skills`
de `opencode.json` / `opencode.jsonc`.

OpenCode recarga las skills al guardar los archivos; `/reload` fuerza una
recarga completa. No hace falta reiniciar.

## 5.4. Estructura de la skill LCE

```
.opencode/skills/lce/
├── SKILL.md                      # flujo, reglas, chuleta y API
└── references/
    ├── guia-rapida.md            # referencia de sintaxis y funciones
    └── errores-comunes.md        # catálogo de errores y soluciones
```

- **ID**: `lce`
- **name**: `LCE`
- **description**: se activa al escribir, compilar, depurar o explicar programas
  LCE (`.es` / `.lce`).

## 5.5. Instalación

### En el proyecto (ya incluida)

La skill vive en `.opencode/skills/lce/` y OpenCode la descubre al abrir el
proyecto. No requiere configuración adicional.

### Global (para todos los proyectos)

```bash
make skill
# o manualmente:
mkdir -p ~/.config/opencode/skills
cp -r .opencode/skills/lce ~/.config/opencode/skills/
```

## 5.6. Uso

- **Automático**: cuando la tarea menciona LCE, programas `.es`/`.lce`, el
  compilador `lce` o widgets Qt, el agente puede cargar la skill por su cuenta.
- **Explícito**: escribe `@lce` en el prompt, o pide "usa la skill lce".
- El agente carga el cuerpo de `SKILL.md`; los archivos de `references/` se
  leen solo cuando la skill lo indica (no se cargan solos).

## 5.7. Mantenimiento

- Mantén el **ID** (`lce`) igual al nombre de la carpeta.
- Actualiza `SKILL.md` y `references/` cuando cambie la sintaxis o la CLI.
- No dupliques el ID en varias fuentes; la última fuente registrada gana.
- Para skills portables, usa IDs en minúsculas y kebab-case (aquí `lce`).
