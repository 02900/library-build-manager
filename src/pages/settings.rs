use dioxus::prelude::*;
use crate::Route;
use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

/// Settings page - System configuration
#[component]
pub fn Settings() -> Element {
    let mut path_status = use_signal(|| check_path_status());
    let mut operation_result = use_signal(|| None::<Result<String, String>>);

    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            // Header
            div { class: "max-w-4xl mx-auto mb-8",
                div { class: "flex items-center justify-between",
                    div {
                        h1 { class: "text-3xl font-bold text-gray-900", "Settings" }
                        p { class: "text-gray-600 mt-1",
                            "Configure system settings and CLI integration"
                        }
                    }
                    Link {
                        to: Route::Home {},
                        class: "bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded-lg transition-colors",
                        "← Back to Projects"
                    }
                }
            }

            // Settings Content
            div { class: "max-w-4xl mx-auto space-y-6",
                // CLI PATH Integration
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-6",
                    h2 { class: "text-xl font-semibold text-gray-900 mb-4", "CLI Integration" }
                    div { class: "space-y-4",
                        div { class: "flex items-start space-x-4",
                            div { class: "flex-shrink-0 w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center",
                                span { class: "text-blue-600 text-sm font-semibold",
                                    "🔧"
                                }
                            }
                            div { class: "flex-1",
                                h3 { class: "font-medium text-gray-900 mb-2", "Add CLI to System PATH" }
                                p { class: "text-gray-600 text-sm mb-4",
                                    "Make the 'library-build-management' command available globally from any terminal. This allows you to run builds from anywhere without specifying the full path."
                                }
                                // Current Status
                                div { class: "mb-4",
                                    match path_status() {
                                        PathStatus::InPath => rsx! {
                                            div { class: "flex items-center space-x-2 text-green-600",
                                                span { "✅" }
                                                span { class: "font-medium", "CLI is available in PATH" }
                                            }
                                            p { class: "text-sm text-gray-600 mt-1", "You can run 'library-build-management' from any terminal" }
                                        },
                                        PathStatus::NotInPath => rsx! {
                                            div { class: "flex items-center space-x-2 text-yellow-600",
                                                span { "⚠️" }
                                                span { class: "font-medium", "CLI is not in PATH" }
                                            }
                                            p { class: "text-sm text-gray-600 mt-1", "The command is only available from the project directory" }
                                        },
                                        PathStatus::Error(ref msg) => rsx! {
                                            div { class: "flex items-center space-x-2 text-red-600",
                                                span { "❌" }
                                                span { class: "font-medium", "Error checking PATH status" }
                                            }
                                            p { class: "text-sm text-gray-600 mt-1", "{msg}" }
                                        },
                                    }
                                }
                                // Action Buttons
                                div { class: "flex space-x-3",
                                    if !matches!(path_status(), PathStatus::InPath) {
                                        button {
                                            class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg transition-colors",
                                            onclick: move |_| {
                                                spawn(async move {
                                                    let result = add_to_path().await;
                                                    operation_result.set(Some(result.clone()));
                                                    if result.is_ok() {
                                                        path_status.set(check_path_status());
                                                    }
                                                });
                                            },
                                            "Add to PATH"
                                        }
                                    } else {
                                        button {
                                            class: "bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg transition-colors",
                                            onclick: move |_| {
                                                spawn(async move {
                                                    let result = remove_from_path().await;
                                                    operation_result.set(Some(result.clone()));
                                                    if result.is_ok() {
                                                        path_status.set(check_path_status());
                                                    }
                                                });
                                            },
                                            "Remove from PATH"
                                        }
                                    }
                                    button {
                                        class: "bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded-lg transition-colors",
                                        onclick: move |_| {
                                            path_status.set(check_path_status());
                                            operation_result.set(None);
                                        },
                                        "Refresh Status"
                                    }
                                }
                                // Operation Result
                                if let Some(ref result) = operation_result() {
                                    div {
                                        class: "mt-4 p-3 rounded-lg",
                                        class: if result.is_ok() { "bg-green-50 border border-green-200" } else { "bg-red-50 border border-red-200" },
                                        match result {
                                            Ok(msg) => rsx! {
                                                p { class: "text-sm text-green-600 mt-1", "{msg}" }
                                            },
                                            Err(msg) => rsx! {
                                                p { class: "text-sm text-red-600 mt-1", "{msg}" }
                                            },
                                        }
                                    }
                                }
                            }
                        }
                        // Instructions
                        div { class: "bg-blue-50 border border-blue-200 rounded-lg p-4",
                            h4 { class: "font-medium text-blue-900 mb-2", "How it works:" }
                            
                            // Platform-specific instructions
                            {
                                #[cfg(target_os = "windows")]
                                {
                                    rsx! {
                                        ul { class: "text-sm text-blue-800 space-y-1",
                                            li {
                                                "• Adds the application directory to your user PATH environment variable"
                                            }
                                            li { "• No administrator privileges required (uses user PATH only)" }
                                            li {
                                                "• You may need to restart your terminal for changes to take effect"
                                            }
                                            li {
                                                "• Run 'where library-build-management' to verify the installation"
                                            }
                                        }
                                    }
                                }
                                
                                #[cfg(unix)]
                                {
                                    rsx! {
                                        ul { class: "text-sm text-blue-800 space-y-1",
                                            li {
                                                "• Creates a symlink in /usr/local/bin pointing to the current binary"
                                            }
                                            li { "• /usr/local/bin is typically in the system PATH" }
                                            li {
                                                "• May request administrator privileges if needed (via password prompt)"
                                            }
                                            li {
                                                "• You may need to restart your terminal for changes to take effect"
                                            }
                                            li {
                                                "• Run 'which library-build-management' to verify the installation"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        // Alternative installation for permission issues
                        div { class: "bg-yellow-50 border border-yellow-200 rounded-lg p-4 mt-4",
                            {
                                // Windows alternative instructions
                                #[cfg(target_os = "windows")]
                                {
                                    rsx! {
                                        div {
                                            h4 { class: "font-medium text-yellow-900 mb-2",
                                                "⚠️ If automatic installation fails:"
                                            }
                                            div { class: "text-sm text-yellow-800 space-y-2",
                                                p { "Option 1: Add manually via System Properties:" }
                                                ul { class: "list-disc list-inside ml-2 space-y-1",
                                                    li { "Open System Properties → Advanced → Environment Variables" }
                                                    li { "Edit your user PATH variable" }
                                                    li { "Add the application directory to the PATH" }
                                                }
                                                p { class: "mt-2", "Option 2: Use PowerShell manually:" }
                                                div { class: "bg-gray-900 text-green-400 p-2 rounded font-mono text-xs",
                                                    "$env:PATH += ';C:\\path\\to\\your\\app'; [Environment]::SetEnvironmentVariable('PATH', $env:PATH, 'User')"
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                // Unix alternative instructions
                                #[cfg(unix)]
                                {
                                    rsx! {
                                        div {
                                            h4 { class: "font-medium text-yellow-900 mb-2",
                                                "⚠️ If you get permission errors:"
                                            }
                                            div { class: "text-sm text-yellow-800 space-y-2",
                                                p { "Option 1: Run the command manually with sudo:" }
                                                div { class: "bg-gray-900 text-green-400 p-2 rounded font-mono text-xs",
                                                    "sudo ln -sf /path/to/LibraryBuildManagement.app/Contents/MacOS/library-build-management /usr/local/bin/library-build-management"
                                                }
                                                p { class: "mt-2",
                                                    "Option 2: Add to your shell profile (~/.zshrc or ~/.bash_profile):"
                                                }
                                                div { class: "bg-gray-900 text-green-400 p-2 rounded font-mono text-xs",
                                                    "export PATH=\"$PATH:/path/to/LibraryBuildManagement.app/Contents/MacOS\""
                                                }
                                                p { class: "mt-2", "Option 3: Create an alias in your shell profile:" }
                                                div { class: "bg-gray-900 text-green-400 p-2 rounded font-mono text-xs",
                                                    "alias library-build-management='/path/to/LibraryBuildManagement.app/Contents/MacOS/library-build-management'"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // CLI Usage Examples
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-6",
                    h2 { class: "text-xl font-semibold text-gray-900 mb-4", "CLI Usage Examples" }
                    div { class: "space-y-4",
                        div { class: "bg-gray-900 text-green-400 p-4 rounded-lg font-mono text-sm",
                            div { "# List all projects" }
                            div { class: "text-white", "$ library-build-management list" }
                            br {}
                            div { "# Build a specific project" }
                            div { class: "text-white",
                                "$ library-build-management build --project \"My Project\""
                            }
                            br {}
                            div { "# Show help" }
                            div { class: "text-white", "$ library-build-management --help" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PathStatus {
    InPath,
    NotInPath,
    Error(String),
}

fn check_path_status() -> PathStatus {
    #[cfg(target_os = "windows")]
    {
        check_path_status_windows()
    }
    
    #[cfg(unix)]
    {
        let target_path = Path::new("/usr/local/bin/library-build-management");
        
        // First check if the symlink exists
        if target_path.exists() {
            // Double-check with 'which' command
            match Command::new("which").arg("library-build-management").output() {
                Ok(output) => {
                    if output.status.success() && !output.stdout.is_empty() {
                        PathStatus::InPath
                    } else {
                        // Symlink exists but 'which' doesn't find it - might be a PATH issue
                        // Still consider it as installed since the symlink is there
                        PathStatus::InPath
                    }
                }
                Err(_) => {
                    // 'which' command failed, but symlink exists, so consider it installed
                    PathStatus::InPath
                }
            }
        } else {
            // No symlink exists
            PathStatus::NotInPath
        }
    }
}

async fn add_to_path() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        add_to_path_windows().await
    }
    
    #[cfg(unix)]
    {
        add_to_path_unix().await
    }
}

/// Add to PATH on Unix systems (macOS, Linux)
#[cfg(unix)]
async fn add_to_path_unix() -> Result<String, String> {
    // Get the current executable path
    let current_exe = env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {}", e))?;
    
    // Determine the correct binary path for .app bundles
    let binary_path = get_binary_path(&current_exe)?;
    let target_path = Path::new("/usr/local/bin/library-build-management");
    
    // Remove existing symlink if it exists
    if target_path.exists() {
        // Try to remove normally first
        match fs::remove_file(target_path) {
            Ok(_) => {}, // Successfully removed
            Err(_) => {
                // Try with admin privileges if normal removal fails
                #[cfg(target_os = "macos")]
                {
                    remove_symlink_with_admin(target_path)
                        .map_err(|e| format!("Failed to remove existing symlink: {}", e))?;
                }
                #[cfg(not(target_os = "macos"))]
                {
                    return Err("Failed to remove existing symlink: Permission denied. Please run: sudo rm -f /usr/local/bin/library-build-management".to_string());
                }
            }
        }
    }
    
    // Try to create symlink with elevated privileges if needed
    match std::os::unix::fs::symlink(&binary_path, target_path) {
        Ok(_) => Ok("✅ Successfully added library-build-management to PATH".to_string()),
        Err(_e) => {
            match create_symlink_with_admin(&binary_path, target_path) {
                Ok(_) => Ok("✅ Successfully added library-build-management to PATH".to_string()),
                Err(_admin_err) => {
                    Err("❌ Failed to add to PATH".to_string())
                }
            }
        }
    }
}

/// Determine the correct binary path, handling .app bundles
fn get_binary_path(current_exe: &Path) -> Result<std::path::PathBuf, String> {
    let exe_path = current_exe.to_path_buf();
    
    // Check if we're running from a .app bundle
    if let Some(app_path) = find_app_bundle(&exe_path) {
        // We're in a .app bundle, use the path inside Contents/MacOS/
        let macos_dir = app_path.join("Contents").join("MacOS");
        if let Some(binary_name) = current_exe.file_name() {
            let bundle_binary = macos_dir.join(binary_name);
            if bundle_binary.exists() {
                return Ok(bundle_binary);
            }
        }
    }
    
    // Not in a bundle or bundle binary not found, use the current executable
    Ok(exe_path)
}

/// Find the .app bundle containing the given path
fn find_app_bundle(path: &Path) -> Option<std::path::PathBuf> {
    let mut current = path;
    while let Some(parent) = current.parent() {
        if let Some(name) = parent.file_name() {
            if let Some(name_str) = name.to_str() {
                if name_str.ends_with(".app") {
                    return Some(parent.to_path_buf());
                }
            }
        }
        current = parent;
    }
    None
}

/// Create symlink with admin privileges using macOS osascript
#[cfg(target_os = "macos")]
fn create_symlink_with_admin(source: &Path, target: &Path) -> Result<(), String> {
    use std::process::Command;
    
    let script = format!(
        "do shell script \"ln -sf '{}' '{}'\" with administrator privileges",
        source.display(),
        target.display()
    );
    
    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to execute osascript: {}", e))?;
    
    if output.status.success() {
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("osascript failed: {}", error))
    }
}

/// Fallback for non-macOS Unix systems
#[cfg(all(unix, not(target_os = "macos")))]
fn create_symlink_with_admin(source: &Path, target: &Path) -> Result<(), String> {
    // For Linux and other Unix systems, we could use pkexec or similar
    // For now, just return an error suggesting manual installation
    Err(format!(
        "Admin privileges not implemented for this platform. Please run manually:\nsudo ln -sf {} {}",
        source.display(),
        target.display()
    ))
}

/// Remove library-build-management from PATH
async fn remove_from_path() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        remove_from_path_windows().await
    }
    
    #[cfg(unix)]
    {
        remove_from_path_unix().await
    }
}

/// Remove from PATH on Unix systems (macOS, Linux)
#[cfg(unix)]
async fn remove_from_path_unix() -> Result<String, String> {
    let target_path = Path::new("/usr/local/bin/library-build-management");
    
    // Check if symlink exists
    if !target_path.exists() {
        return Err("❌ CLI is not installed in PATH".to_string());
    }
    
    // Try to remove the symlink
    // First try normal removal
    match fs::remove_file(target_path) {
        Ok(_) => {
            Ok("✅ Successfully removed library-build-management from PATH".to_string())
        }
        Err(_e) => {
            // Try with elevated privileges using osascript (macOS)
            match remove_symlink_with_admin(target_path) {
                Ok(_) => {
                    Ok("✅ Successfully removed library-build-management from PATH".to_string())
                }
                Err(_admin_err) => {
                    Err("❌ Failed to remove from PATH".to_string())
                }
            }
        }
    }
}

/// Remove symlink with admin privileges using macOS osascript
#[cfg(target_os = "macos")]
fn remove_symlink_with_admin(target: &Path) -> Result<(), String> {
    use std::process::Command;
    
    let script = format!(
        "do shell script \"rm -f '{}'\" with administrator privileges",
        target.display()
    );
    
    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to execute osascript: {}", e))?;
    
    if output.status.success() {
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("osascript failed: {}", error))
    }
}

/// Fallback for non-macOS Unix systems - remove symlink
#[cfg(all(unix, not(target_os = "macos")))]
fn remove_symlink_with_admin(target: &Path) -> Result<(), String> {
    Err(format!(
        "Admin privileges not implemented for this platform. Please run manually:\nsudo rm -f {}",
        target.display()
    ))
}

// Windows-specific PATH management functions

/// Add to PATH on Windows (user PATH, no admin required)
#[cfg(target_os = "windows")]
async fn add_to_path_windows() -> Result<String, String> {
    use std::process::Command;
    
    let current_exe = env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {}", e))?;
    
    let exe_dir = current_exe.parent()
        .ok_or("Failed to get executable directory")?;
    
    // First, check if already in PATH
    if is_in_path_windows(&exe_dir.display().to_string()) {
        return Ok("✅ Already in PATH".to_string());
    }
    
    // Add to user PATH using PowerShell
    let script = format!(
        "$currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User'); \
         if ($currentPath -notlike '*{}*') {{ \
             $newPath = $currentPath + ';{}'; \
             [Environment]::SetEnvironmentVariable('PATH', $newPath, 'User'); \
             Write-Output 'SUCCESS' \
         }} else {{ \
             Write-Output 'ALREADY_EXISTS' \
         }}",
        exe_dir.display(),
        exe_dir.display()
    );
    
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", &script])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;
    
    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).trim();
        match result {
            "SUCCESS" => Ok("✅ Successfully added library-build-management to PATH".to_string()),
            "ALREADY_EXISTS" => Ok("✅ Already in PATH".to_string()),
            _ => Ok("✅ Successfully added library-build-management to PATH".to_string()),
        }
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("❌ Failed to add to PATH: {}", error))
    }
}

/// Remove from PATH on Windows
#[cfg(target_os = "windows")]
async fn remove_from_path_windows() -> Result<String, String> {
    use std::process::Command;
    
    let current_exe = env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {}", e))?;
    
    let exe_dir = current_exe.parent()
        .ok_or("Failed to get executable directory")?;
    
    // Check if it's in PATH
    if !is_in_path_windows(&exe_dir.display().to_string()) {
        return Err("❌ CLI is not in PATH".to_string());
    }
    
    // Remove from user PATH using PowerShell
    let exe_dir_str = exe_dir.display().to_string();
    let escaped_dir = exe_dir_str.replace("\\", "\\\\").replace("[", "\\[").replace("]", "\\]");
    let script = format!(
        "$currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User'); \
         $newPath = $currentPath -replace '(^|;){}(;|$)', '$1$2' -replace '^;|;$', ''; \
         [Environment]::SetEnvironmentVariable('PATH', $newPath, 'User'); \
         Write-Output 'SUCCESS'",
        escaped_dir
    );
    
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", &script])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;
    
    if output.status.success() {
        Ok("✅ Successfully removed library-build-management from PATH".to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("❌ Failed to remove from PATH: {}", error))
    }
}

/// Check if a directory is in Windows PATH
#[cfg(target_os = "windows")]
fn is_in_path_windows(dir: &str) -> bool {
    use std::process::Command;
    
    // Check both user and system PATH
    let check_user = Command::new("powershell")
        .args(&[
            "-NoProfile", "-Command", 
            &format!("[Environment]::GetEnvironmentVariable('PATH', 'User') -like '*{}*'", dir)
        ])
        .output();
    
    let check_system = Command::new("powershell")
        .args(&[
            "-NoProfile", "-Command", 
            &format!("[Environment]::GetEnvironmentVariable('PATH', 'Machine') -like '*{}*'", dir)
        ])
        .output();
    
    let user_has_it = check_user
        .map(|output| String::from_utf8_lossy(&output.stdout).trim() == "True")
        .unwrap_or(false);
    
    let system_has_it = check_system
        .map(|output| String::from_utf8_lossy(&output.stdout).trim() == "True")
        .unwrap_or(false);
    
    user_has_it || system_has_it
}

/// Check PATH status on Windows
#[cfg(target_os = "windows")]
fn check_path_status_windows() -> PathStatus {
    use std::process::Command;
    
    // Use 'where' command to check if the executable is found
    match Command::new("where").arg("library-build-management.exe").output() {
        Ok(output) => {
            if output.status.success() && !output.stdout.is_empty() {
                PathStatus::InPath
            } else {
                PathStatus::NotInPath
            }
        }
        Err(_) => PathStatus::NotInPath
    }
}
