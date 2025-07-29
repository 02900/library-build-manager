use dioxus::prelude::*;
use dioxus::desktop::{WindowBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/project/:id")]
    ProjectDetail { id: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub build_commands: Vec<String>,
    pub selected_build_command: Option<String>,
    pub target_paths: Vec<TargetPath>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetPath {
    pub id: String,
    pub path: String,
    pub is_active: bool,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(make_config())
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn ProjectCard(project: Project) -> Element {
    rsx! {
        div {
            class: "bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow cursor-pointer",
            onclick: move |_| {
                navigator().push(Route::ProjectDetail { id: project.id.clone() });
            },
            
            h3 { class: "text-xl font-semibold text-gray-800 mb-2", "{project.name}" }
            p { class: "text-gray-600 text-sm mb-3", "{project.path}" }
            
            div { class: "flex items-center justify-between",
                div { class: "flex items-center space-x-2",
                    span { class: "text-xs bg-blue-100 text-blue-800 px-2 py-1 rounded", 
                        "{project.target_paths.len()} paths"
                    }
                    if let Some(cmd) = &project.selected_build_command {
                        span { class: "text-xs bg-green-100 text-green-800 px-2 py-1 rounded", 
                            "{cmd}"
                        }
                    }
                }
                div { class: "text-right",
                    span { class: "text-xs text-gray-500", 
                        "{project.target_paths.iter().filter(|p| p.is_active).count()} active"
                    }
                }
            }
        }
    }
}

/// Home page - Main project list
#[component]
fn Home() -> Element {
    let mut projects = use_signal(|| load_projects());
    let mut show_add_modal = use_signal(|| false);
    let mut new_project_name = use_signal(|| String::new());
    let mut new_project_path = use_signal(|| String::new());

    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            // Header
            div { class: "max-w-6xl mx-auto mb-8",
                div { class: "flex items-center justify-between",
                    div {
                        h1 { class: "text-3xl font-bold text-gray-900", "Library Manager" }
                        p { class: "text-gray-600 mt-1", "Manage your development libraries and build processes" }
                    }
                    button {
                        class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg flex items-center space-x-2 transition-colors",
                        onclick: move |_| show_add_modal.set(true),
                        span { "+ Add Project" }
                    }
                }
            }

            // Projects Grid
            div { class: "max-w-6xl mx-auto",
                if projects().is_empty() {
                    // Empty State
                    div { class: "text-center py-12",
                        div { class: "max-w-md mx-auto",
                            div { class: "w-24 h-24 mx-auto mb-4 bg-gray-200 rounded-full flex items-center justify-center",
                                span { class: "text-4xl text-gray-400", "📦" }
                            }
                            h3 { class: "text-xl font-semibold text-gray-900 mb-2", "No projects yet" }
                            p { class: "text-gray-600 mb-6", "Start by adding your first library project to manage builds and deployments." }
                            button {
                                class: "bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg transition-colors",
                                onclick: move |_| show_add_modal.set(true),
                                "Add Your First Project"
                            }
                        }
                    }
                } else {
                    // Projects Grid
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                        for project in projects().iter() {
                            ProjectCard { project: project.clone() }
                        }
                    }
                }
            }

            // Add Project Modal
            if show_add_modal() {
                div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                    div { class: "bg-white rounded-lg p-6 w-full max-w-md mx-4",
                        h2 { class: "text-xl font-semibold mb-4", "Add New Project" }
                        
                        div { class: "space-y-4",
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-1", "Project Name" }
                                input {
                                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    r#type: "text",
                                    placeholder: "My Library",
                                    value: new_project_name(),
                                    oninput: move |e| new_project_name.set(e.value())
                                }
                            }
                            
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-1", "Project Path" }
                                div { class: "flex space-x-2",
                                    input {
                                        class: "flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                        r#type: "text",
                                        placeholder: "/path/to/project",
                                        value: new_project_path(),
                                        oninput: move |e| new_project_path.set(e.value())
                                    }
                                    button {
                                        class: "px-3 py-2 bg-gray-200 hover:bg-gray-300 rounded-md transition-colors",
                                        onclick: move |_| {
                                            spawn(async move {
                                                if let Some(path) = open_folder_dialog().await {
                                                    new_project_path.set(path);
                                                }
                                            });
                                        },
                                        "Browse"
                                    }
                                }
                            }
                        }
                        
                        div { class: "flex justify-end space-x-3 mt-6",
                            button {
                                class: "px-4 py-2 text-gray-600 hover:text-gray-800 transition-colors",
                                onclick: move |_| {
                                    show_add_modal.set(false);
                                    new_project_name.set(String::new());
                                    new_project_path.set(String::new());
                                },
                                "Cancel"
                            }
                            button {
                                class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition-colors",
                                disabled: new_project_name().trim().is_empty() || new_project_path().trim().is_empty(),
                                onclick: move |_| {
                                    let project = create_project(
                                        new_project_name().trim().to_string(),
                                        new_project_path().trim().to_string()
                                    );
                                    let mut current_projects = projects();
                                    current_projects.push(project);
                                    save_projects(&current_projects);
                                    projects.set(current_projects);
                                    
                                    show_add_modal.set(false);
                                    new_project_name.set(String::new());
                                    new_project_path.set(String::new());
                                },
                                "Add Project"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Project Detail page
#[component]
pub fn ProjectDetail(id: String) -> Element {
    let projects = load_projects();
    let project = projects.iter().find(|p| p.id == id);
    
    match project {
        Some(project) => {
            let mut current_project = use_signal(|| project.clone());
            let mut show_add_path_modal = use_signal(|| false);
            let mut new_path = use_signal(|| String::new());
            let mut show_result_modal = use_signal(|| false);
            let mut result_message = use_signal(|| String::new());
            let mut is_processing = use_signal(|| false);
            
            rsx! {
                div { class: "min-h-screen bg-gray-50 p-6",
                    div { class: "max-w-4xl mx-auto",
                        // Header with back button
                        div { class: "flex items-center mb-6",
                            Link {
                                to: Route::Home {},
                                class: "flex items-center text-blue-600 hover:text-blue-800 mr-4",
                                span { "← Back to Projects" }
                            }
                            div {
                                h1 { class: "text-2xl font-bold text-gray-900", "{current_project().name}" }
                                p { class: "text-gray-600", "{current_project().path}" }
                            }
                        }
                        
                        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                            // Build Commands Section
                            div { class: "bg-white rounded-lg shadow p-6",
                                h2 { class: "text-lg font-semibold mb-4", "Build Commands" }
                                {
                                    let build_commands = current_project().build_commands.clone();
                                    if build_commands.is_empty() {
                                        rsx! {
                                            p { class: "text-gray-500 text-sm", "No build commands found in package.json" }
                                        }
                                    } else {
                                        rsx! {
                                            div { class: "space-y-2",
                                                for command in build_commands.iter() {
                                                    div {
                                                        class: "flex items-center space-x-2",
                                                        input {
                                                            r#type: "radio",
                                                            name: "build_command",
                                                            checked: current_project().selected_build_command.as_ref() == Some(command),
                                                            onchange: {
                                                                let command = command.clone();
                                                                move |_| {
                                                                    let mut proj = current_project();
                                                                    proj.selected_build_command = Some(command.clone());
                                                                    current_project.set(proj.clone());
                                                                    update_project(&proj);
                                                                }
                                                            }
                                                        }
                                                        label { class: "text-sm font-mono bg-gray-100 px-2 py-1 rounded", "{command}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // Target Paths Section
                            div { class: "bg-white rounded-lg shadow p-6",
                                div { class: "flex items-center justify-between mb-4",
                                    h2 { class: "text-lg font-semibold", "Target Paths" }
                                    button {
                                        class: "bg-blue-600 hover:bg-blue-700 text-white px-3 py-1 rounded text-sm transition-colors",
                                        onclick: move |_| show_add_path_modal.set(true),
                                        "+ Add Path"
                                    }
                                }
                                
                                {
                                    let target_paths = current_project().target_paths.clone();
                                    if target_paths.is_empty() {
                                        rsx! {
                                            p { class: "text-gray-500 text-sm", "No target paths configured" }
                                        }
                                    } else {
                                        rsx! {
                                            div { class: "space-y-3",
                                                for target_path in target_paths.iter() {
                                                    div {
                                                        class: "flex items-center justify-between p-3 border rounded-lg",
                                                        div { class: "flex items-center space-x-3",
                                                            input {
                                                                r#type: "checkbox",
                                                                checked: target_path.is_active,
                                                                onchange: {
                                                                    let target_id = target_path.id.clone();
                                                                    move |e| {
                                                                        let mut proj = current_project();
                                                                        if let Some(path) = proj.target_paths.iter_mut().find(|p| p.id == target_id) {
                                                                            path.is_active = e.checked();
                                                                        }
                                                                        current_project.set(proj.clone());
                                                                        update_project(&proj);
                                                                    }
                                                                }
                                                            }
                                                            div {
                                                                p { class: "text-sm font-medium", "{target_path.path}" }
                                                                p { 
                                                                    class: if target_path.is_active { "text-xs text-green-600" } else { "text-xs text-gray-500" },
                                                                    if target_path.is_active { "Active" } else { "Inactive" }
                                                                }
                                                            }
                                                        }
                                                        button {
                                                            class: "text-red-600 hover:text-red-800 text-sm",
                                                            onclick: {
                                                                let target_id = target_path.id.clone();
                                                                move |_| {
                                                                    let mut proj = current_project();
                                                                    proj.target_paths.retain(|p| p.id != target_id);
                                                                    current_project.set(proj.clone());
                                                                    update_project(&proj);
                                                                }
                                                            },
                                                            "Remove"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Actions Section
                        div { class: "bg-white rounded-lg shadow p-6 mt-6",
                            h2 { class: "text-lg font-semibold mb-4", "Actions" }
                            div { class: "flex space-x-4",
                                button {
                                    class: if is_processing() { 
                                        "bg-gray-400 text-white px-4 py-2 rounded-lg cursor-not-allowed" 
                                    } else { 
                                        "bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg transition-colors" 
                                    },
                                    disabled: is_processing() || current_project().selected_build_command.is_none() || current_project().target_paths.iter().filter(|p| p.is_active).count() == 0,
                                    onclick: move |_| {
                                        if !is_processing() {
                                            is_processing.set(true);
                                            let project = current_project();
                                            
                                            // Run the build and update process
                                            match build_and_update_project(&project) {
                                                Ok(success_msg) => {
                                                    result_message.set(success_msg);
                                                }
                                                Err(error_msg) => {
                                                    result_message.set(format!("❌ Error: {}", error_msg));
                                                }
                                            }
                                            
                                            is_processing.set(false);
                                            show_result_modal.set(true);
                                        }
                                    },
                                    if is_processing() { "Processing..." } else { "Build & Update" }
                                }
                                button {
                                    class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg transition-colors",
                                    onclick: move |_| {
                                        let mut proj = current_project();
                                        refresh_project_commands(&mut proj);
                                        current_project.set(proj.clone());
                                        update_project(&proj);
                                        println!("Refreshed build commands for: {}", proj.name);
                                    },
                                    "Refresh Commands"
                                }
                            }
                        }
                    }
                    
                    // Add Path Modal
                    if show_add_path_modal() {
                        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                            div { class: "bg-white rounded-lg p-6 w-full max-w-md mx-4",
                                h2 { class: "text-xl font-semibold mb-4", "Add Target Path" }
                                
                                div { class: "space-y-4",
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "Target Path" }
                                        div { class: "flex space-x-2",
                                            input {
                                                class: "flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                                r#type: "text",
                                                placeholder: "/path/to/target",
                                                value: new_path(),
                                                oninput: move |e| new_path.set(e.value())
                                            }
                                            button {
                                                class: "px-3 py-2 bg-gray-200 hover:bg-gray-300 rounded-md transition-colors",
                                                onclick: move |_| {
                                                    spawn(async move {
                                                        if let Some(path) = open_target_folder_dialog().await {
                                                            new_path.set(path);
                                                        }
                                                    });
                                                },
                                                "Browse"
                                            }
                                        }
                                    }
                                }
                                
                                div { class: "flex justify-end space-x-3 mt-6",
                                    button {
                                        class: "px-4 py-2 text-gray-600 hover:text-gray-800 transition-colors",
                                        onclick: move |_| {
                                            show_add_path_modal.set(false);
                                            new_path.set(String::new());
                                        },
                                        "Cancel"
                                    }
                                    button {
                                        class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition-colors",
                                        disabled: new_path().trim().is_empty(),
                                        onclick: move |_| {
                                            let mut proj = current_project();
                                            let target_path = TargetPath {
                                                id: uuid::Uuid::new_v4().to_string(),
                                                path: new_path().trim().to_string(),
                                                is_active: true,
                                            };
                                            proj.target_paths.push(target_path);
                                            current_project.set(proj.clone());
                                            update_project(&proj);
                                            
                                            show_add_path_modal.set(false);
                                            new_path.set(String::new());
                                        },
                                        "Add Path"
                                    }
                                }
                            }
                        }
                    }
                    
                    // Result Modal
                    if show_result_modal() {
                        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                            div { class: "bg-white rounded-lg p-6 w-full max-w-2xl mx-4 max-h-96 overflow-y-auto",
                                h2 { class: "text-xl font-semibold mb-4", "Build & Update Results" }
                                
                                div { class: "mb-6",
                                    pre { 
                                        class: "whitespace-pre-wrap text-sm bg-gray-100 p-4 rounded-lg border",
                                        "{result_message()}"
                                    }
                                }
                                
                                div { class: "flex justify-end",
                                    button {
                                        class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition-colors",
                                        onclick: move |_| {
                                            show_result_modal.set(false);
                                            result_message.set(String::new());
                                        },
                                        "Close"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "min-h-screen bg-gray-50 flex items-center justify-center",
                    div { class: "text-center",
                        h1 { class: "text-2xl font-bold text-gray-900 mb-4", "Project Not Found" }
                        Link {
                            to: Route::Home {},
                            class: "text-blue-600 hover:text-blue-800",
                            "← Back to Projects"
                        }
                    }
                }
            }
        }
    }
}

// Helper functions for data persistence
fn get_data_dir() -> std::path::PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push(".update-packages");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap_or_else(|e| {
            eprintln!("Failed to create data directory: {}", e);
        });
    }
    path
}

fn get_projects_file() -> std::path::PathBuf {
    let mut path = get_data_dir();
    path.push("projects.json");
    path
}

fn load_projects() -> Vec<Project> {
    let file_path = get_projects_file();
    if !file_path.exists() {
        return vec![];
    }
    
    match std::fs::read_to_string(&file_path) {
        Ok(content) => {
            match serde_json::from_str::<Vec<Project>>(&content) {
                Ok(projects) => projects,
                Err(e) => {
                    eprintln!("Failed to parse projects file: {}", e);
                    vec![]
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read projects file: {}", e);
            vec![]
        }
    }
}

fn save_projects(projects: &[Project]) {
    let file_path = get_projects_file();
    match serde_json::to_string_pretty(projects) {
        Ok(content) => {
            if let Err(e) = std::fs::write(&file_path, content) {
                eprintln!("Failed to save projects: {}", e);
            } else {
                println!("Saved {} projects to {:?}", projects.len(), file_path);
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize projects: {}", e);
        }
    }
}

fn parse_package_json(project_path: &str) -> Vec<String> {
    let mut package_path = std::path::PathBuf::from(project_path);
    package_path.push("package.json");
    
    if !package_path.exists() {
        return vec![];
    }
    
    match std::fs::read_to_string(&package_path) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(json) => {
                    if let Some(scripts) = json.get("scripts").and_then(|s| s.as_object()) {
                        scripts.keys().cloned().collect()
                    } else {
                        vec![]
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse package.json: {}", e);
                    vec![]
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read package.json: {}", e);
            vec![]
        }
    }
}

fn create_project(name: String, path: String) -> Project {
    let build_commands = parse_package_json(&path);
    Project {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        path,
        build_commands,
        selected_build_command: None,
        target_paths: vec![],
    }
}

fn update_project(project: &Project) {
    let mut projects = load_projects();
    if let Some(existing) = projects.iter_mut().find(|p| p.id == project.id) {
        *existing = project.clone();
        save_projects(&projects);
    }
}

fn refresh_project_commands(project: &mut Project) {
    project.build_commands = parse_package_json(&project.path);
    // Reset selected command if it no longer exists
    if let Some(selected) = &project.selected_build_command {
        if !project.build_commands.contains(selected) {
            project.selected_build_command = None;
        }
    }
}

fn get_package_version(package_path: &str) -> Option<String> {
    let mut path = std::path::PathBuf::from(package_path);
    path.push("package.json");
    
    if !path.exists() {
        return None;
    }
    
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(json) => {
                    json.get("version")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                }
                Err(e) => {
                    eprintln!("Failed to parse package.json: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read package.json: {}", e);
            None
        }
    }
}

fn increment_patch_version(version: &str) -> String {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() >= 3 {
        let major = parts[0];
        let minor = parts[1];
        let patch: u32 = parts[2].parse().unwrap_or(0);
        format!("{}.{}.{}", major, minor, patch + 1)
    } else {
        format!("{}.0.1", version)
    }
}

fn update_package_version(package_path: &str, new_version: &str) -> Result<(), String> {
    let mut path = std::path::PathBuf::from(package_path);
    path.push("package.json");
    
    if !path.exists() {
        return Err("package.json not found".to_string());
    }
    
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(mut json) => {
                    if let Some(version_field) = json.get_mut("version") {
                        *version_field = serde_json::Value::String(new_version.to_string());
                        
                        match serde_json::to_string_pretty(&json) {
                            Ok(updated_content) => {
                                match std::fs::write(&path, updated_content) {
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(format!("Failed to write package.json: {}", e))
                                }
                            }
                            Err(e) => Err(format!("Failed to serialize JSON: {}", e))
                        }
                    } else {
                        Err("No version field found in package.json".to_string())
                    }
                }
                Err(e) => Err(format!("Failed to parse package.json: {}", e))
            }
        }
        Err(e) => Err(format!("Failed to read package.json: {}", e))
    }
}

fn copy_directory(src: &std::path::Path, dst: &std::path::Path) -> Result<(), String> {
    if !src.exists() {
        return Err(format!("Source directory does not exist: {:?}", src));
    }
    
    // Remove destination if it exists
    if dst.exists() {
        std::fs::remove_dir_all(dst)
            .map_err(|e| format!("Failed to remove destination directory: {}", e))?;
    }
    
    // Create parent directories if they don't exist
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create parent directories: {}", e))?;
    }
    
    // Copy the directory recursively
    copy_dir_recursive(src, dst)
}

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(dst)
        .map_err(|e| format!("Failed to create directory {:?}: {}", dst, e))?;
    
    for entry in std::fs::read_dir(src)
        .map_err(|e| format!("Failed to read directory {:?}: {}", src, e))? {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)
                .map_err(|e| format!("Failed to copy file {:?} to {:?}: {}", src_path, dst_path, e))?;
        }
    }
    
    Ok(())
}

async fn open_folder_dialog() -> Option<String> {
    let folder = rfd::AsyncFileDialog::new()
        .set_title("Select Project Folder")
        .pick_folder()
        .await;
    
    folder.map(|f| f.path().to_string_lossy().to_string())
}

async fn open_target_folder_dialog() -> Option<String> {
    let folder = rfd::AsyncFileDialog::new()
        .set_title("Select Target Folder")
        .pick_folder()
        .await;
    
    folder.map(|f| f.path().to_string_lossy().to_string())
}

fn build_and_update_project(project: &Project) -> Result<String, String> {
    if project.selected_build_command.is_none() {
        return Err("No build command selected".to_string());
    }
    
    let active_targets: Vec<_> = project.target_paths.iter()
        .filter(|p| p.is_active)
        .collect();
    
    if active_targets.is_empty() {
        return Err("No active target paths".to_string());
    }
    
    let project_path = std::path::Path::new(&project.path);
    let dist_path = project_path.join("dist");
    let package_json_path = project_path.join("package.json");
    
    // Check if dist directory exists
    if !dist_path.exists() {
        return Err("dist directory not found. Please build the project first.".to_string());
    }
    
    // Check if package.json exists
    if !package_json_path.exists() {
        return Err("package.json not found in project directory".to_string());
    }
    
    let mut results = Vec::new();
    
    // Process each active target
    for target in active_targets {
        let target_path = std::path::Path::new(&target.path);
        
        // Get current version from target's package.json
        let current_version = get_package_version(&target.path)
            .unwrap_or_else(|| "0.0.0".to_string());
        
        // Increment patch version
        let new_version = increment_patch_version(&current_version);
        
        // Copy dist directory
        let target_dist = target_path.join("dist");
        if let Err(e) = copy_directory(&dist_path, &target_dist) {
            results.push(format!("❌ Failed to copy dist to {}: {}", target.path, e));
            continue;
        }
        
        // Copy package.json
        let target_package_json = target_path.join("package.json");
        if let Err(e) = std::fs::copy(&package_json_path, &target_package_json) {
            results.push(format!("❌ Failed to copy package.json to {}: {}", target.path, e));
            continue;
        }
        
        // Update version in target's package.json
        if let Err(e) = update_package_version(&target.path, &new_version) {
            results.push(format!("❌ Failed to update version in {}: {}", target.path, e));
            continue;
        }
        
        results.push(format!("✅ Updated {} (v{} → v{})", target.path, current_version, new_version));
    }
    
    Ok(results.join("\n"))
}


fn make_config() -> dioxus::desktop::Config {
    dioxus::desktop::Config::default().with_window(make_window())
}

fn make_window() -> WindowBuilder {
    WindowBuilder::new()
        .with_resizable(true)
        .with_always_on_top(false)
        .with_title("Update Packages")
}

