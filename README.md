# Library Build Management App

Una aplicación de escritorio desarrollada con Dioxus 0.7 para gestionar el build y testing de librerías en desarrollo local.

## Características

### 🖥️ **Interfaz Desktop**
- ✅ **Gestión de Proyectos**: Agregar, editar y eliminar proyectos de librerías
- ✅ **Persistencia de Datos**: Los proyectos se guardan automáticamente en `~/.library-build-management/projects.json`
- ✅ **Selector Nativo de Carpetas**: Integración con el sistema operativo para seleccionar directorios
- ✅ **Análisis de package.json**: Detección automática de comandos de build disponibles
- ✅ **Multi-selección de Comandos**: Accordion UI para seleccionar múltiples comandos con orden personalizado
- ✅ **Gestión de Paths de Destino**: Agregar y activar/desactivar ubicaciones con checkboxes intuitivos
- ✅ **Automatización de Build**: Incremento automático de versión patch y copia de archivos
- ✅ **Interfaz Moderna**: UI responsive con Tailwind CSS y componentes modulares
- ✅ **Página de Settings**: Configuración del sistema y integración CLI

### ⌨️ **CLI (Command Line Interface)**
- ✅ **Comando Global**: Disponible desde cualquier terminal una vez instalado en PATH
- ✅ **Lista de Proyectos**: `library-build-management list`
- ✅ **Build Automático**: `library-build-management build --project "Nombre"`
- ✅ **Búsqueda Flexible**: Por nombre o ID de proyecto
- ✅ **Validación Robusta**: Verifica comandos y targets antes de ejecutar
- ✅ **Integración PATH**: Instalación automática desde la GUI

## Funcionalidades Principales

### 1. Menú Principal
- Lista de proyectos existentes con información resumida
- Estado vacío cuando no hay proyectos
- Botón para agregar nuevos proyectos
- Cada tarjeta de proyecto muestra:
  - Nombre y ruta del proyecto
  - Número de paths configurados
  - Comando de build seleccionado
  - Número de paths activos

### 2. Vista de Detalle del Proyecto
- **Comandos de Build**: 
  - Accordion UI para seleccionar múltiples comandos
  - Ordenamiento personalizado con botones up/down
  - Badges que muestran el número de comandos seleccionados
  - Persistencia del orden y selección
- **Paths de Destino**: 
  - Gestión de ubicaciones donde actualizar la librería
  - Checkboxes para activar/desactivar paths
  - Muestra nombre del proyecto extraído de la ruta
  - Ruta completa visible como subtítulo
- **Acciones**:
  - **Build & Update**: Ejecuta múltiples comandos en orden y actualiza targets
  - **Refresh Commands**: Actualiza la lista de comandos desde package.json

### 3. Página de Settings
- **Integración CLI**: 
  - Verificación automática del estado del PATH
  - Instalación con un clic del comando global
  - Ejemplos de uso del CLI
  - Instrucciones detalladas de configuración
- **Estados Visuales**: 
  - ✅ Verde: CLI disponible en PATH
  - ⚠️ Amarillo: CLI no está en PATH
  - ❌ Rojo: Error en la verificación

### 4. Lógica de Actualización (basada en update-pkg.sh)
Cuando se ejecuta "Build & Update":
1. Verifica que exista el directorio `dist` en el proyecto
2. Para cada path de destino activo:
   - Obtiene la versión actual del package.json del destino
   - Incrementa la versión patch (ej: 1.0.0 → 1.0.1)
   - Copia el directorio `dist` del proyecto al destino
   - Copia el `package.json` del proyecto al destino
   - Actualiza la versión en el package.json del destino
3. Muestra un resumen de resultados con éxitos y errores

## Estructura del Proyecto

```
library-build-management/
├─ assets/           # Assets estáticos (CSS, iconos, favicon)
│  ├─ favicon.ico
│  ├─ main.css
│  └─ tailwind.css
├─ src/
│  ├─ main.rs        # Punto de entrada, CLI parsing, y configuración
│  ├─ types.rs       # Definiciones de tipos (Project, TargetPath)
│  ├─ logic.rs       # Lógica de negocio y persistencia
│  ├─ pages/         # Páginas de la aplicación
│  │  ├─ mod.rs
│  │  ├─ home.rs     # Página principal con lista de proyectos
│  │  ├─ project_detail.rs  # Vista detalle del proyecto
│  │  └─ settings.rs # Página de configuración y CLI integration
│  └─ components/    # Componentes reutilizables
│     ├─ mod.rs
│     └─ project_card.rs  # Tarjeta de proyecto para la lista
├─ Cargo.toml       # Dependencias y configuración del paquete
├─ Dioxus.toml      # Configuración específica de Dioxus
├─ package-lock.json # Lock file para dependencias de Node.js
└─ README.md        # Este archivo
```

## Dependencias Principales

- **dioxus**: Framework de UI reactivo para Rust (v0.7.0-alpha.3)
- **serde**: Serialización/deserialización de datos JSON
- **uuid**: Generación de IDs únicos para proyectos y targets
- **dirs**: Acceso a directorios del sistema (home, etc.)
- **rfd**: Diálogos nativos de archivos y carpetas
- **tokio**: Runtime asíncrono para operaciones I/O
- **clap**: Parser de argumentos CLI con derive macros

## Instalación y Uso

### Prerrequisitos
- Rust 1.70+
- Dioxus CLI: `cargo install dioxus-cli`

### Compilar y Ejecutar

```bash
# Modo desarrollo con GUI
cargo run

# Modo release (recomendado para uso)
cargo run --release

# O usando Dioxus CLI para desarrollo
dx serve --platform desktop

# Compilar binario optimizado
cargo build --release
```

## 🚀 Uso del CLI

### Instalación Global

1. **Desde la GUI** (Recomendado):
   - Abre la aplicación
   - Ve a Settings (⚙️)
   - Haz clic en "Add to PATH"
   - Reinicia tu terminal

2. **Manual**:
   ```bash
   # Crear symlink manualmente
   sudo ln -sf $(pwd)/target/release/library-build-management /usr/local/bin/library-build-management
   ```

### Comandos Disponibles

```bash
# Mostrar ayuda
library-build-management --help

# Listar todos los proyectos
library-build-management list

# Ejecutar build de un proyecto específico
library-build-management build --project "Nombre del Proyecto"

# También funciona con ID del proyecto
library-build-management build --project "uuid-del-proyecto"

# Mostrar proyectos disponibles
library-build-management build --list
```

### Ejemplos de Uso CLI

```bash
# Ver todos los proyectos configurados
$ library-build-management list
Available projects:
------------------------------------------------------------
📦 Builder Blocks (317eca26-6da9-4356-b1dd-55ad2d8cbb5f)
   Path: /Users/juan/Documents/wiggot-mini-sites-builder-blocks
   Build commands: ["build", "generate-exports"]
   Active targets: 1

# Ejecutar build y actualizar targets
$ library-build-management build --project "Builder Blocks"
🔨 Building project: Builder Blocks
📁 Path: /Users/juan/Documents/wiggot-mini-sites-builder-blocks
🚀 Executing 2 build commands...
   1. build
   2. generate-exports
📤 Will update 1 active targets

✅ Build and update completed successfully!
```

## 🖥️ Uso de la GUI

### 1. **Configuración Inicial**

**Agregar un Proyecto**:
- Haz clic en "+ Add Project" en la página principal
- Ingresa el nombre del proyecto
- Selecciona la ruta usando "Browse" o escríbela manualmente
- La aplicación detectará automáticamente los comandos de build disponibles

**Configurar CLI Global** (Opcional pero recomendado):
- Ve a Settings (⚙️) desde la página principal
- En "CLI Integration", haz clic en "Add to PATH"
- Reinicia tu terminal para usar comandos globales

### 2. **Configuración del Proyecto**

**Seleccionar Comandos de Build**:
- Entra al detalle del proyecto haciendo clic en su tarjeta
- En "Build Commands", usa el accordion para:
  - ✅ Seleccionar múltiples comandos
  - 🔄 Ordenar comandos con botones up/down
  - 👀 Ver badges con el número de comandos seleccionados

**Gestionar Paths de Destino**:
- En "Target Paths", haz clic en "+ Add Path"
- Selecciona la carpeta de destino usando el selector nativo
- Usa checkboxes ☑️ para activar/desactivar paths
- Visualiza el nombre del proyecto extraído automáticamente
- La ruta completa aparece como subtítulo

### 3. **Ejecutar Builds**

**Desde la GUI**:
- Asegúrate de tener comandos seleccionados y paths activos
- Haz clic en "Build & Update"
- Los comandos se ejecutan en el orden configurado
- Revisa los resultados detallados en el modal

**Desde el CLI** (si está configurado):
```bash
# Listar proyectos
library-build-management list

# Ejecutar build específico
library-build-management build --project "Nombre del Proyecto"
```

### 4. **Flujos de Trabajo Recomendados**

**Setup Inicial** (Una sola vez):
1. Configurar proyectos en la GUI
2. Seleccionar comandos de build múltiples
3. Agregar y activar target paths
4. Instalar CLI en PATH desde Settings

**Uso Diario**:
- **Desarrollo**: Usar CLI para builds rápidos
- **Configuración**: Usar GUI para cambios y nuevos proyectos
- **Monitoreo**: GUI para ver estado y resultados detallados

## 💾 Almacenamiento de Datos

Los proyectos se guardan automáticamente en: `~/.library-build-management/projects.json`

**Características del almacenamiento**:
- ✅ **Persistencia automática**: Cambios se guardan inmediatamente
- ✅ **Backup seguro**: Validación JSON antes de escribir
- ✅ **Migración transparente**: Compatibilidad con versiones anteriores
- ✅ **Ubicación estándar**: Directorio home del usuario

Estructura del archivo:
```json
[
  {
    "id": "uuid-único",
    "name": "Nombre del Proyecto",
    "path": "/ruta/al/proyecto",
    "build_commands": ["build", "dev", "test"],
    "selected_build_command": "build",
    "target_paths": [
      {
        "id": "uuid-único",
        "path": "/ruta/destino",
        "is_active": true
      }
    ]
  }
]
```

## Desarrollo

### Arquitectura
- **Dioxus 0.7**: Framework de UI con componentes reactivos
- **Signals**: Manejo de estado local con `use_signal`
- **Router**: Navegación entre vistas (Home, ProjectDetail)
- **Async**: Operaciones asíncronas para diálogos de archivos

### Componentes Principales
- `App`: Componente raíz con router
- `Home`: Vista principal con lista de proyectos
- `ProjectDetail`: Vista de detalle y configuración
- `ProjectCard`: Tarjeta individual de proyecto

### Funciones Utilitarias
- `load_projects()` / `save_projects()`: Persistencia
- `parse_package_json()`: Análisis de comandos
- `build_and_update_project()`: Lógica principal de actualización
- `open_folder_dialog()`: Selector nativo de carpetas

