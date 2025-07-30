# Library Build Management App

A **cross-platform** desktop application built with Dioxus 0.7 for managing build and testing of local development libraries.

## 🌟 Features

### 🖥️ **Cross-Platform Desktop Interface**
- ✅ **Project Management**: Add, edit and delete library projects
- ✅ **Data Persistence**: Projects are automatically saved to `~/.library-build-management/projects.json`
- ✅ **Native Folder Picker**: OS integration for directory selection
- ✅ **package.json Analysis**: Automatic detection of available build commands
- ✅ **Multi-Command Selection**: Accordion UI for selecting multiple commands with custom ordering
- ✅ **Target Path Management**: Add and activate/deactivate locations with intuitive checkboxes
- ✅ **Build Automation**: Automatic patch version increment and file copying
- ✅ **Modern Interface**: Responsive UI with Tailwind CSS and modular components
- ✅ **Settings Page**: System configuration and cross-platform CLI integration

### ⌨️ **Cross-Platform CLI (Command Line Interface)**
- ✅ **Global Command**: Available from any terminal once installed in PATH
- ✅ **Project Listing**: `library-build-management list`
- ✅ **Automatic Build**: `library-build-management build --project "Name"`
- ✅ **Flexible Search**: By project name or ID
- ✅ **Robust Validation**: Verifies commands and targets before execution
- ✅ **Automatic PATH Integration**: Platform-specific installation from GUI

### 🌍 **Cross-Platform Support**
- ✅ **Windows**: User PATH (no admin permissions required)
- ✅ **macOS**: Symlinks with automatic admin permission requests
- ✅ **Linux**: Standard Unix symlinks with appropriate fallbacks
- ✅ **Smart UI**: Platform-specific instructions
- ✅ **Native Commands**: `where` on Windows, `which` on Unix

## Main Functionalities

### 1. Main Menu
- List of existing projects with summary information
- Empty state when no projects exist
- Button to add new projects
- Each project card shows:
  - Project name and path
  - Number of configured paths
  - Selected build command
  - Number of active paths

### 2. Project Detail View
- **Build Commands**: 
  - Accordion UI for selecting multiple commands
  - Custom ordering with up/down buttons
  - Badges showing number of selected commands
  - Persistence of order and selection
- **Target Paths**: 
  - Management of locations where to update the library
  - Checkboxes to activate/deactivate paths
  - Shows project name extracted from path
  - Full path visible as subtitle
- **Actions**:
  - **Build & Update**: Execute multiple commands in order and update targets
  - **Refresh Commands**: Update command list from package.json

### 3. Settings Page
- **CLI Integration**: 
  - Automatic PATH status verification
  - One-click global command installation
  - CLI usage examples
  - Detailed configuration instructions
- **Visual States**: 
  - ✅ Green: CLI available in PATH
  - ⚠️ Yellow: CLI not in PATH
  - ❌ Red: Verification error

### 4. Update Logic (based on update-pkg.sh)
When "Build & Update" is executed:
1. Verifies that `dist` directory exists in the project
2. For each active target path:
   - Gets current version from target's package.json
   - Increments patch version (e.g., 1.0.0 → 1.0.1)
   - Copies project's `dist` directory to target
   - Copies project's `package.json` to target
   - Updates version in target's package.json
3. Shows results summary with successes and errors

## Project Structure

```
library-build-management/
├─ assets/           # Static assets (CSS, icons, favicon)
│  ├─ favicon.ico
│  ├─ main.css
│  └─ tailwind.css
├─ src/
│  ├─ main.rs        # Entry point, CLI parsing, and configuration
│  ├─ types.rs       # Type definitions (Project, TargetPath)
│  ├─ logic.rs       # Business logic and persistence
│  ├─ pages/         # Application pages
│  │  ├─ mod.rs
│  │  ├─ home.rs     # Main page with project list
│  │  ├─ project_detail.rs  # Project detail view
│  │  └─ settings.rs # Configuration page and CLI integration
│  └─ components/    # Reusable components
│     ├─ mod.rs
│     └─ project_card.rs  # Project card for the list
├─ Cargo.toml       # Dependencies and package configuration
├─ Dioxus.toml      # Dioxus-specific configuration
└─ README.md        # This file
```

## Prerequisites

- Rust 1.70+
- Dioxus CLI: `cargo install dioxus-cli`

### 🔧 Installation

### Compile from source code

```bash
# Clone the repository
git clone https://github.com/your-user/library-build-management.git
cd library-build-management

# Compile in release mode
cargo build --release

# Run the application
./target/release/library-build-management
```

### 🌍 Install CLI Globally (Cross-Platform)

#### **Automatic Method** (Recommended for all platforms):
1. Run the application
2. Go to Settings (⚙️)
3. Click "Add to PATH"
4. **Windows**: Added to user PATH automatically
5. **macOS/Linux**: Administrator permissions requested if needed
6. Restart your terminal

#### **Manual Methods by Platform**:

**🪟 Windows:**
```powershell
# Option 1: PowerShell (add to user PATH)
$env:PATH += ';C:\path\to\your\app'; [Environment]::SetEnvironmentVariable('PATH', $env:PATH, 'User')

# Option 2: Via System Properties
# System Properties → Advanced → Environment Variables → Edit user PATH
```

**🍎 macOS:**
```bash
# Create symlink with admin permissions
sudo ln -sf /path/to/LibraryBuildManagement.app/Contents/MacOS/library-build-management /usr/local/bin/library-build-management

# Or add to shell profile
echo 'export PATH="$PATH:/path/to/LibraryBuildManagement.app/Contents/MacOS"' >> ~/.zshrc
```

**🐧 Linux:**
```bash
# Create symlink
sudo ln -sf $(pwd)/target/release/library-build-management /usr/local/bin/library-build-management

# Or add to shell profile
echo 'export PATH="$PATH:$(pwd)/target/release"' >> ~/.bashrc
```

### Available Commands

```bash
# Show help
library-build-management --help

# List all projects
library-build-management list

# Execute build for a specific project
library-build-management build --project "Project Name"

# Build by project ID
library-build-management build --project "uuid-of-project"
```

### CLI Usage Examples

```bash
# List projects
$ library-build-management list
📋 Found 2 projects:

📦 Wiggot Components (a1b2c3d4-e5f6-7890-abcd-ef1234567890)
   Path: /Users/juan/Documents/wiggot-components
   Build commands: ["build", "build:prod"]
   Active targets: 2

📦 Builder Blocks (317eca26-6da9-4356-b1dd-55ad2d8cbb5f)
   Path: /Users/juan/Documents/wiggot-mini-sites-builder-blocks
   Build commands: ["build", "generate-exports"]
   Active targets: 1

# Execute build and update targets
$ library-build-management build --project "Builder Blocks"
🔨 Building project: Builder Blocks
📁 Path: /Users/juan/Documents/wiggot-mini-sites-builder-blocks
🚀 Executing 2 build commands...
   1. build
   2. generate-exports
📤 Will update 1 active targets

✅ Build and update completed successfully!
```

## 🖥️ GUI Usage

### 1. **Initial Setup**

**Add a Project**:
- Click "+ Add Project" on the main page
- Enter the project name
- Select the path using "Browse" or type it manually
- The application will automatically detect available build commands

**Configure Global CLI** (Optional but recommended):
- Go to Settings (⚙️) from the main page
- In "CLI Integration", click "Add to PATH"
- Restart your terminal to use global commands

### 2. **Project Configuration**

**Select Build Commands**:
- Enter project details by clicking on its card
- In "Build Commands", use the accordion to:
  - ✅ Select multiple commands
  - 🔄 Order commands with up/down buttons
  - 👀 See badges with number of selected commands

**Manage Target Paths**:
- In "Target Paths", click "+ Add Path"
- Select the target folder using the native picker
- Use checkboxes ☑️ to activate/deactivate paths
- View automatically extracted project name
- Full path appears as subtitle

### 3. **Execute Builds**

**From the GUI**:
- Make sure you have selected commands and active paths
- Click "Build & Update"
- Commands execute in configured order
- Review detailed results in the modal

**From the CLI** (if configured):
```bash
# List projects
library-build-management list

# Execute specific build
library-build-management build --project "Project Name"
```

### 4. **Recommended Workflows**

**Initial Setup** (One time):
1. Configure projects in the GUI
2. Select multiple build commands
3. Add and activate target paths
4. Install CLI in PATH from Settings

**Daily Usage**:
- **Development**: Use CLI for quick builds
- **Configuration**: Use GUI for changes and new projects
- **Monitoring**: GUI for status and detailed results

## 💾 Data Storage

Projects are automatically saved to: `~/.library-build-management/projects.json`

**Storage features**:
- ✅ **Automatic persistence**: Changes saved immediately
- ✅ **Safe backup**: JSON validation before writing
- ✅ **Transparent migration**: Compatibility with previous versions
- ✅ **Standard location**: User's home directory

File structure:
```json
[
  {
    "id": "unique-uuid",
    "name": "Project Name",
    "path": "/path/to/project",
    "build_commands": ["build", "dev", "test"],
    "selected_build_command": "build",
    "target_paths": [
      {
        "id": "unique-uuid",
        "path": "/target/path",
        "is_active": true
      }
    ]
  }
]
```

## 🛠️ Development

### Compile and Run

```bash
# Development mode with GUI
cargo run

# Release mode (recommended for usage)
cargo run --release

# Or using Dioxus CLI for development
dx serve --platform desktop

# Compile optimized binary
cargo build --release

# Create bundle for distribution (macOS)
dx bundle
```

### 🏗️ Cross-Platform Architecture
- **Dioxus 0.7**: UI framework with reactive components
- **Signals**: Local state management with `use_signal`
- **Router**: Navigation between views (Home, ProjectDetail, Settings)
- **Async**: Asynchronous operations for file dialogs
- **Conditional Compilation**: `#[cfg(target_os = "...")]` for platform-specific functionality
- **PATH Management**: Native implementations for Windows, macOS and Linux

### 📦 Key Dependencies
- `dioxus`: Main UI framework
- `dioxus-router`: Page navigation
- `serde` / `serde_json`: Data serialization
- `uuid`: Unique ID generation
- `dirs`: System directory access
- `rfd`: Native file dialogs
- `tokio`: Asynchronous runtime
- `clap`: CLI argument parsing

### 🧩 Main Components
- `App`: Root component with router
- `Home`: Main view with project list
- `ProjectDetail`: Detail and configuration view
- `Settings`: Configuration page and CLI integration
- `ProjectCard`: Individual project card

### 🔧 Utility Functions
- `load_projects()` / `save_projects()`: Cross-platform persistence
- `parse_package_json()`: Build command analysis
- `build_and_update_project()`: Main update logic
- `open_folder_dialog()`: Native folder picker
- `add_to_path()` / `remove_from_path()`: Platform-specific PATH management
- `check_path_status()`: CLI status verification

### 🌍 Platform-Specific Functionality

**Windows:**
- User PATH management with PowerShell
- Verification with `where` command
- No administrator permissions required

**macOS:**
- Symlinks in `/usr/local/bin`
- Automatic permission requests with `osascript`
- `.app` bundle detection
- Verification with `which` command

**Linux:**
- Standard Unix symlinks
- Fallbacks with `sudo` instructions
- Verification with `which` command
