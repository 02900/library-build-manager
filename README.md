# Library Build Management App

Una aplicación de escritorio desarrollada con Dioxus 0.7 para gestionar el build y testing de librerías en desarrollo local.

## Características

- ✅ **Gestión de Proyectos**: Agregar, editar y eliminar proyectos de librerías
- ✅ **Persistencia de Datos**: Los proyectos se guardan automáticamente en `~/.library-build-management/projects.json`
- ✅ **Selector Nativo de Carpetas**: Integración con el sistema operativo para seleccionar directorios
- ✅ **Análisis de package.json**: Detección automática de comandos de build disponibles
- ✅ **Gestión de Paths de Destino**: Agregar y activar/desactivar ubicaciones donde actualizar la librería
- ✅ **Automatización de Build**: Incremento automático de versión patch y copia de archivos
- ✅ **Interfaz Moderna**: UI responsive con Tailwind CSS

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
- **Comandos de Build**: Lista de comandos disponibles desde package.json
- **Paths de Destino**: Gestión de ubicaciones donde actualizar la librería
- **Acciones**:
  - **Build & Update**: Ejecuta la lógica de actualización automática
  - **Refresh Commands**: Actualiza la lista de comandos desde package.json

### 3. Lógica de Actualización (basada en update-pkg.sh)
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
├─ assets/           # Assets estáticos (CSS, iconos)
├─ src/
│  └─ main.rs       # Código principal de la aplicación
├─ Cargo.toml       # Dependencias y configuración
├─ Dioxus.toml      # Configuración de Dioxus
└─ README.md        # Este archivo
```

## Dependencias Principales

- **dioxus**: Framework de UI reactivo para Rust
- **serde**: Serialización/deserialización de datos
- **uuid**: Generación de IDs únicos
- **dirs**: Acceso a directorios del sistema
- **rfd**: Diálogos nativos de archivos
- **tokio**: Runtime asíncrono

## Instalación y Uso

### Prerrequisitos
- Rust 1.70+
- Dioxus CLI: `cargo install dioxus-cli`

### Compilar y Ejecutar

```bash
# Modo desarrollo
cargo run

# Modo release (recomendado para uso)
cargo run --release

# O usando Dioxus CLI
dx serve --platform desktop
```

### Uso de la Aplicación

1. **Agregar un Proyecto**:
   - Haz clic en "+ Add Project"
   - Ingresa el nombre del proyecto
   - Selecciona la ruta usando "Browse" o escríbela manualmente
   - La aplicación detectará automáticamente los comandos de build

2. **Configurar Paths de Destino**:
   - Entra al detalle del proyecto
   - Haz clic en "+ Add Path" en la sección "Target Paths"
   - Selecciona la carpeta de destino
   - Activa/desactiva paths según necesites

3. **Ejecutar Build & Update**:
   - Selecciona un comando de build
   - Asegúrate de tener al menos un path activo
   - Haz clic en "Build & Update"
   - Revisa los resultados en el modal

## Almacenamiento de Datos

Los proyectos se guardan en: `~/.library-build-management/projects.json`

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

