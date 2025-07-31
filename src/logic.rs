use crate::types::*;
use serde_json;
use dioxus::prelude::{Writable, Readable};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use sysinfo::{System, Pid};

// Global process tracking for cleanup on app exit
static ACTIVE_PROCESSES: Lazy<Arc<Mutex<HashMap<String, tokio::process::Child>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// Register a process for tracking
pub fn register_process(id: String, child: tokio::process::Child) {
    if let Ok(mut processes) = ACTIVE_PROCESSES.lock() {
        processes.insert(id, child);
    }
}

// Unregister a process when it completes
pub fn unregister_process(id: &str) {
    if let Ok(mut processes) = ACTIVE_PROCESSES.lock() {
        processes.remove(id);
    }
}

// Kill a process tree (parent + all children) - cross-platform
pub async fn kill_process_tree(pid: u32) -> Result<(), String> {
    let mut system = System::new_all();
    system.refresh_all();
    
    // Find all child processes recursively
    let mut processes_to_kill = Vec::new();
    collect_child_processes(&system, Pid::from(pid as usize), &mut processes_to_kill);
    
    // Add the parent process
    processes_to_kill.push(pid);
    
    println!("🧹 Killing process tree for PID {}: {} processes", pid, processes_to_kill.len());
    
    // For bash processes, also try to kill by process group
    #[cfg(unix)]
    {
        if let Some(process) = system.process(Pid::from(pid as usize)) {
            let process_name = process.name();
            if process_name.contains("bash") || process_name.contains("sh") {
                println!("🔪 Detected bash process, using process group kill");
                // Kill the entire process group
                let _ = std::process::Command::new("kill")
                    .arg("-TERM")
                    .arg(format!("-{}", pid))
                    .output();
                
                // Wait a bit for graceful shutdown
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                
                // Force kill the process group
                let _ = std::process::Command::new("kill")
                    .arg("-KILL")
                    .arg(format!("-{}", pid))
                    .output();
            }
        }
    }
    
    // Kill all processes (children first, then parent)
    for &process_pid in processes_to_kill.iter().rev() {
        kill_single_process(process_pid).await;
    }
    
    // Double-check: refresh and kill any remaining npm/node processes
    system.refresh_all();
    for (proc_pid, process) in system.processes() {
        let process_name = process.name().to_lowercase();
        if process_name.contains("npm") || process_name.contains("node") {
            if let Some(parent_pid) = process.parent() {
                if parent_pid.as_u32() == pid || processes_to_kill.contains(&parent_pid.as_u32()) {
                    println!("🔪 Killing orphaned {}: {}", process_name, proc_pid.as_u32());
                    kill_single_process(proc_pid.as_u32()).await;
                }
            }
        }
    }
    
    Ok(())
}

// Recursively collect all child processes
fn collect_child_processes(system: &System, parent_pid: Pid, processes: &mut Vec<u32>) {
    for (pid, process) in system.processes() {
        if let Some(process_parent_pid) = process.parent() {
            if process_parent_pid == parent_pid {
                let child_pid = pid.as_u32();
                processes.push(child_pid);
                // Recursively find children of this child
                collect_child_processes(system, *pid, processes);
            }
        }
    }
}

// Kill a single process - cross-platform
async fn kill_single_process(pid: u32) {
    #[cfg(unix)]
    {
        // On Unix systems, use SIGTERM first, then SIGKILL if needed
        use std::process::Command;
        
        println!("🔪 Terminating process {} (SIGTERM)", pid);
        let _ = Command::new("kill")
            .arg("-TERM")
            .arg(pid.to_string())
            .output();
        
        // Wait a bit for graceful shutdown
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        // Force kill if still running
        println!("🔪 Force killing process {} (SIGKILL)", pid);
        let _ = Command::new("kill")
            .arg("-KILL")
            .arg(pid.to_string())
            .output();
    }
    
    #[cfg(windows)]
    {
        // On Windows, use taskkill
        use std::process::Command;
        
        println!("🔪 Terminating process {} (Windows)", pid);
        let _ = Command::new("taskkill")
            .arg("/F")
            .arg("/PID")
            .arg(pid.to_string())
            .output();
    }
}

// Kill all active processes (called on app exit)
pub async fn cleanup_all_processes() {
    if let Ok(mut processes) = ACTIVE_PROCESSES.lock() {
        for (id, mut child) in processes.drain() {
            println!("🧹 Cleaning up process tree: {}", id);
            if let Some(pid) = child.id() {
                let _ = kill_process_tree(pid).await;
            } else {
                // Fallback to simple kill if PID not available
                let _ = child.kill().await;
            }
        }
    }
}

// Helper function to find npm binary path
fn find_npm_path() -> Option<String> {
    // Common npm locations on macOS
    let possible_paths = [
        "/usr/local/bin/npm",
        "/opt/homebrew/bin/npm",
        "/usr/bin/npm",
    ];
    
    for path in &possible_paths {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    
    // Try to find npm using which command
    if let Ok(output) = std::process::Command::new("which")
        .arg("npm")
        .output() {
        if output.status.success() {
            if let Ok(path) = String::from_utf8(output.stdout) {
                let trimmed_path = path.trim();
                if !trimmed_path.is_empty() {
                    return Some(trimmed_path.to_string());
                }
            }
        }
    }
    
    None
}

// Helper function to create and execute a bash script with npm commands
fn create_build_script(commands: &[String], project_path: &str) -> Result<String, String> {
    use std::io::Write;
    
    // Find npm binary path
    let npm_path = find_npm_path().unwrap_or_else(|| "npm".to_string());
    
    // Create temporary script file
    let script_path = format!("{}/build_script.sh", project_path);
    
    // Generate script content with full npm path and proper PATH setup
    let mut script_content = String::from("#!/bin/bash\nset -e\n\n");
    
    // Add common Node.js paths to PATH
    script_content.push_str("export PATH=\"/usr/local/bin:/opt/homebrew/bin:/usr/bin:$PATH\"\n\n");
    
    for command in commands {
        script_content.push_str(&format!("echo 'Running: {} run {}'\n", npm_path, command));
        script_content.push_str(&format!("{} run {}\n", npm_path, command));
    }
    
    // Write script to file
    match std::fs::File::create(&script_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(script_content.as_bytes()) {
                return Err(format!("Failed to write script: {}", e));
            }
        }
        Err(e) => {
            return Err(format!("Failed to create script file: {}", e));
        }
    }
    
    // Make script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Err(e) = std::fs::set_permissions(&script_path, std::fs::Permissions::from_mode(0o755)) {
            return Err(format!("Failed to set script permissions: {}", e));
        }
    }
    
    Ok(script_path)
}

// Data persistence functions
pub fn get_data_dir() -> std::path::PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push(".library-build-management");
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



// Main build and update logic
pub async fn build_and_update_project(project: &Project) -> Result<String, String> {
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
    let package_json_path = project_path.join("package.json");
    
    // Check if package.json exists
    if !package_json_path.exists() {
        return Err("package.json not found in project directory".to_string());
    }
    
    let mut results = Vec::new();
    
    // Step 1: Execute build commands using bash script
    results.push(format!("🚀 Executing {} build commands in order...", project.selected_build_commands.len()));
    
    // Create and execute bash script with all commands
    match create_build_script(&project.selected_build_commands, &project.path) {
        Ok(script_path) => {
            // Execute the bash script
            let output = tokio::process::Command::new("bash")
                .arg(&script_path)
                .current_dir(&project.path)
                .output()
                .await;
            
            // Clean up script file
            let _ = std::fs::remove_file(&script_path);
            
            match output {
                Ok(output) => {
                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        results.push(format!("✅ All build commands completed successfully\n{}", stdout));
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(format!("❌ Build script failed: {}", stderr));
                    }
                }
                Err(e) => {
                    return Err(format!("❌ Failed to execute build script: {}", e));
                }
            }
        }
        Err(e) => {
            return Err(format!("❌ Failed to create build script: {}", e));
        }
    }
    
    results.push("\n📦 Build commands completed successfully!".to_string());
    
    // Step 2: Check if dist directory exists after build
    let dist_path = project_path.join("dist");
    if !dist_path.exists() {
        return Err("dist directory not found after build. Build commands may have failed.".to_string());
    }
    
    results.push("\n📤 Updating target paths...".to_string());
    
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

/// Extract project name from target path
/// For paths like "/Users/random/Documents/project/node_modules/@package/name"
/// Returns "project"
/// For nested node_modules like "/project/node_modules/pkg/node_modules/@nested/lib"
/// Returns "pkg" (parent of the LAST node_modules)
pub fn extract_project_name(path: &str) -> String {
    let path_parts: Vec<&str> = path.split('/').collect();
    
    // Find the last occurrence of "node_modules"
    if let Some(node_modules_index) = path_parts.iter().rposition(|&part| part == "node_modules") {
        // The project name should be the directory before the last "node_modules"
        if node_modules_index > 0 {
            return path_parts[node_modules_index - 1].to_string();
        }
    }
    
    // Fallback: try to get the last meaningful directory name
    // Skip empty parts and common endings
    for part in path_parts.iter().rev() {
        if !part.is_empty() && *part != "node_modules" && !part.starts_with('@') {
            return part.to_string();
        }
    }
    
    // Final fallback: return the full path
    path.to_string()
}

// Build and update with progress reporting for UI
pub async fn build_and_update_project_with_progress(
    project: &Project, 
    mut progress_signal: dioxus::prelude::Signal<String>
) -> Result<String, String> {
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
    let package_json_path = project_path.join("package.json");
    
    // Check if package.json exists
    if !package_json_path.exists() {
        return Err("package.json not found in project directory".to_string());
    }
    
    let mut results = Vec::new();
    
    // Step 1: Execute build commands using bash script
    results.push(format!("🚀 Executing {} build commands in order...", project.selected_build_commands.len()));
    progress_signal.set("Creating build script...".to_string());
    
    // Create and execute bash script with all commands
    match create_build_script(&project.selected_build_commands, &project.path) {
        Ok(script_path) => {
            progress_signal.set("Executing build commands...".to_string());
            
            // Execute the bash script
            let output = tokio::process::Command::new("bash")
                .arg(&script_path)
                .current_dir(&project.path)
                .output()
                .await;
            
            // Clean up script file
            let _ = std::fs::remove_file(&script_path);
            
            match output {
                Ok(output) => {
                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        results.push(format!("✅ All build commands completed successfully\n{}", stdout));
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(format!("❌ Build script failed: {}", stderr));
                    }
                }
                Err(e) => {
                    return Err(format!("❌ Failed to execute build script: {}", e));
                }
            }
        }
        Err(e) => {
            return Err(format!("❌ Failed to create build script: {}", e));
        }
    }
    
    results.push("\n📦 Build commands completed successfully!".to_string());
    
    // Step 2: Check if dist directory exists after build
    progress_signal.set("Verifying build output...".to_string());
    let dist_path = project_path.join("dist");
    if !dist_path.exists() {
        return Err("dist directory not found after build. Build commands may have failed.".to_string());
    }
    
    results.push("\n📤 Updating target paths...".to_string());
    
    // Process each active target
    for (index, target) in active_targets.iter().enumerate() {
        progress_signal.set(format!("Updating target {} of {}: {}", index + 1, active_targets.len(), extract_project_name(&target.path)));
        
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
    
    progress_signal.set("Finalizing...".to_string());
    
    Ok(results.join("\n"))
}

// Build with cancellation support and PID tracking
pub async fn build_and_update_project_cancellable(
    project: &Project,
    mut progress_signal: dioxus::prelude::Signal<String>,
    mut process_handle: dioxus::prelude::Signal<Option<tokio::process::Child>>
) -> Result<String, String> {
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
    let package_json_path = project_path.join("package.json");
    
    // Check if package.json exists
    if !package_json_path.exists() {
        return Err("package.json not found in project directory".to_string());
    }
    
    let mut results = Vec::new();
    
    // Step 1: Execute build commands using bash script with cancellation support
    results.push(format!("🚀 Executing {} build commands in order...", project.selected_build_commands.len()));
    progress_signal.set("Creating build script...".to_string());
    
    // Create and execute bash script with all commands
    match create_build_script(&project.selected_build_commands, &project.path) {
        Ok(script_path) => {
            progress_signal.set("Executing build commands...".to_string());
            
            // Execute the bash script as a cancellable process with process group
            let mut cmd = tokio::process::Command::new("bash");
            cmd.arg(&script_path)
                .current_dir(&project.path);
            
            // Set process group for better process tree management
            #[cfg(unix)]
            {
                use std::os::unix::process::CommandExt;
                cmd.process_group(0);
            }
            
            let child = cmd.spawn()
                .map_err(|e| {
                    let _ = std::fs::remove_file(&script_path);
                    format!("❌ Failed to start build script: {}", e)
                })?;
            
            // Store the process handle for potential cancellation
            process_handle.set(Some(child));
            
            // Wait for the process to complete while keeping handle available for cancellation
            let output = loop {
                // Check if we still have a process handle (not cancelled)
                let has_process = process_handle.read().is_some();
                if !has_process {
                    // Process was cancelled
                    break Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "Process was cancelled"));
                }
                
                // Try to check if process is done
                let mut process_done = false;
                let mut process_result = None;
                
                if let Some(mut child_process) = process_handle.take() {
                    match child_process.try_wait() {
                        Ok(Some(_status)) => {
                            // Process completed, get final output
                            let result = child_process.wait_with_output().await;
                            process_result = Some(result);
                            process_done = true;
                        }
                        Ok(None) => {
                            // Process still running, put it back
                            process_handle.set(Some(child_process));
                        }
                        Err(e) => {
                            // Error checking process status
                            process_result = Some(Err(e));
                            process_done = true;
                        }
                    }
                } else {
                    // Process was cancelled or removed
                    break Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "Process was cancelled"));
                }
                
                if process_done {
                    if let Some(result) = process_result {
                        break result;
                    }
                } else {
                    // Process still running, wait a bit and check again
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            };
            
            // Clear the process handle after completion
            process_handle.set(None);
            
            // Clean up script file
            let _ = std::fs::remove_file(&script_path);
            
            match output {
                Ok(output) => {
                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        results.push(format!("✅ All build commands completed successfully\n{}", stdout));
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(format!("❌ Build script failed: {}", stderr));
                    }
                }
                Err(e) => {
                    return Err(format!("❌ Failed to execute build script: {}", e));
                }
            }
        }
        Err(e) => {
            return Err(format!("❌ Failed to create build script: {}", e));
        }
    }
    
    results.push("\n📦 Build commands completed successfully!".to_string());
    
    // Step 2: Check if dist directory exists after build
    progress_signal.set("Verifying build output...".to_string());
    let dist_path = project_path.join("dist");
    if !dist_path.exists() {
        return Err("dist directory not found after build. Build commands may have failed.".to_string());
    }
    
    results.push("\n📤 Updating target paths...".to_string());
    
    // Process each active target
    for (index, target) in active_targets.iter().enumerate() {
        progress_signal.set(format!("Updating target {} of {}: {}", index + 1, active_targets.len(), extract_project_name(&target.path)));
        
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
    
    progress_signal.set("Finalizing...".to_string());
    
    Ok(results.join("\n"))
}

// Cancel a running build process with tree kill
pub async fn cancel_build_process(mut process_handle: dioxus::prelude::Signal<Option<tokio::process::Child>>) -> Result<(), String> {
    if let Some(mut child) = process_handle.take() {
        if let Some(pid) = child.id() {
            println!("❌ Cancelling build process tree (PID: {})", pid);
            match kill_process_tree(pid).await {
                Ok(_) => {
                    process_handle.set(None);
                    Ok(())
                }
                Err(e) => {
                    // Fallback to simple kill if tree kill fails
                    println!("⚠️ Tree kill failed, using fallback: {}", e);
                    match child.kill().await {
                        Ok(_) => {
                            process_handle.set(None);
                            Ok(())
                        }
                        Err(e) => Err(format!("Failed to cancel process: {}", e))
                    }
                }
            }
        } else {
            // No PID available, use simple kill
            match child.kill().await {
                Ok(_) => {
                    process_handle.set(None);
                    Ok(())
                }
                Err(e) => Err(format!("Failed to cancel process: {}", e))
            }
        }
    } else {
        Err("No running process to cancel".to_string())
    }
}
