use crate::types::Project;
use serde_json;

// Data persistence functions
pub fn get_data_dir() -> std::path::PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push(".update-packages");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap_or_else(|e| {
            eprintln!("Failed to create data directory: {}", e);
        });
    }
    path
}

pub fn get_projects_file() -> std::path::PathBuf {
    let mut path = get_data_dir();
    path.push("projects.json");
    path
}

pub fn load_projects() -> Vec<Project> {
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

pub fn save_projects(projects: &[Project]) {
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

// Package.json parsing functions
pub fn parse_package_json(project_path: &str) -> Vec<String> {
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

pub fn get_package_version(package_path: &str) -> Option<String> {
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

// Project management functions
pub fn create_project(name: String, path: String) -> Project {
    let build_commands = parse_package_json(&path);
    Project {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        path,
        build_commands,
        selected_build_commands: vec![], // Start with empty ordered list
        target_paths: vec![],
    }
}

pub fn update_project(project: &Project) {
    let mut projects = load_projects();
    if let Some(existing) = projects.iter_mut().find(|p| p.id == project.id) {
        *existing = project.clone();
        save_projects(&projects);
    }
}

pub fn refresh_project_commands(project: &mut Project) {
    project.build_commands = parse_package_json(&project.path);
    // Remove selected commands that are no longer available
    project.selected_build_commands.retain(|cmd| project.build_commands.contains(cmd));
}

// Version management functions
pub fn increment_patch_version(version: &str) -> String {
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

pub fn update_package_version(package_path: &str, new_version: &str) -> Result<(), String> {
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

// File operations
pub fn copy_directory(src: &std::path::Path, dst: &std::path::Path) -> Result<(), String> {
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

// Native dialogs
pub async fn open_folder_dialog() -> Option<String> {
    let folder = rfd::AsyncFileDialog::new()
        .set_title("Select Project Folder")
        .pick_folder()
        .await;
    
    folder.map(|f| f.path().to_string_lossy().to_string())
}

pub async fn open_target_folder_dialog() -> Option<String> {
    let folder = rfd::AsyncFileDialog::new()
        .set_title("Select Target Folder")
        .pick_folder()
        .await;
    
    folder.map(|f| f.path().to_string_lossy().to_string())
}

// Main build and update logic
pub fn build_and_update_project(project: &Project) -> Result<String, String> {
    if project.selected_build_commands.is_empty() {
        return Err("No build commands selected".to_string());
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
