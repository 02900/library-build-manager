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
                    h2 { class: "text-xl font-semibold text-gray-900 mb-4",
                        "CLI Integration"
                    }
                    
                    div { class: "space-y-4",
                        div { class: "flex items-start space-x-4",
                            div { class: "flex-shrink-0 w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center",
                                span { class: "text-blue-600 text-sm font-semibold", "🔧" }
                            }
                            div { class: "flex-1",
                                h3 { class: "font-medium text-gray-900 mb-2",
                                    "Add CLI to System PATH"
                                }
                                p { class: "text-gray-600 text-sm mb-4",
                                    "Make the 'update-packages' command available globally from any terminal. This allows you to run builds from anywhere without specifying the full path."
                                }
                                
                                // Current Status
                                div { class: "mb-4",
                                    match path_status() {
                                        PathStatus::InPath => rsx! {
                                            div { class: "flex items-center space-x-2 text-green-600",
                                                span { "✅" }
                                                span { class: "font-medium", "CLI is available in PATH" }
                                            }
                                            p { class: "text-sm text-gray-600 mt-1",
                                                "You can run 'update-packages' from any terminal"
                                            }
                                        },
                                        PathStatus::NotInPath => rsx! {
                                            div { class: "flex items-center space-x-2 text-yellow-600",
                                                span { "⚠️" }
                                                span { class: "font-medium", "CLI is not in PATH" }
                                            }
                                            p { class: "text-sm text-gray-600 mt-1",
                                                "The command is only available from the project directory"
                                            }
                                        },
                                        PathStatus::Error(ref msg) => rsx! {
                                            div { class: "flex items-center space-x-2 text-red-600",
                                                span { "❌" }
                                                span { class: "font-medium", "Error checking PATH status" }
                                            }
                                            p { class: "text-sm text-gray-600 mt-1", "{msg}" }
                                        }
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
                                    div { class: "mt-4 p-3 rounded-lg",
                                        class: if result.is_ok() { "bg-green-50 border border-green-200" } else { "bg-red-50 border border-red-200" },
                                        match result {
                                            Ok(msg) => rsx! {
                                                div { class: "flex items-center space-x-2 text-green-700",
                                                    span { "✅" }
                                                    span { class: "font-medium", "Success!" }
                                                }
                                                p { class: "text-sm text-green-600 mt-1", "{msg}" }
                                            },
                                            Err(msg) => rsx! {
                                                div { class: "flex items-center space-x-2 text-red-700",
                                                    span { "❌" }
                                                    span { class: "font-medium", "Error" }
                                                }
                                                p { class: "text-sm text-red-600 mt-1", "{msg}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Instructions
                        div { class: "bg-blue-50 border border-blue-200 rounded-lg p-4",
                            h4 { class: "font-medium text-blue-900 mb-2", "How it works:" }
                            ul { class: "text-sm text-blue-800 space-y-1",
                                li { "• Creates a symlink in /usr/local/bin pointing to the current binary" }
                                li { "• /usr/local/bin is typically in the system PATH" }
                                li { "• You may need to restart your terminal for changes to take effect" }
                                li { "• Run 'which update-packages' to verify the installation" }
                            }
                        }
                    }
                }
                
                // CLI Usage Examples
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-6",
                    h2 { class: "text-xl font-semibold text-gray-900 mb-4",
                        "CLI Usage Examples"
                    }
                    
                    div { class: "space-y-4",
                        div { class: "bg-gray-900 text-green-400 p-4 rounded-lg font-mono text-sm",
                            div { "# List all projects" }
                            div { class: "text-white", "$ update-packages list" }
                            br {}
                            div { "# Build a specific project" }
                            div { class: "text-white", "$ update-packages build --project \"My Project\"" }
                            br {}
                            div { "# Show help" }
                            div { class: "text-white", "$ update-packages --help" }
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
    // Check if update-packages is available in PATH
    match Command::new("which").arg("update-packages").output() {
        Ok(output) => {
            if output.status.success() && !output.stdout.is_empty() {
                PathStatus::InPath
            } else {
                PathStatus::NotInPath
            }
        }
        Err(e) => PathStatus::Error(format!("Failed to check PATH: {}", e))
    }
}

async fn add_to_path() -> Result<String, String> {
    // Get the current executable path
    let current_exe = env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {}", e))?;
    
    let target_path = Path::new("/usr/local/bin/update-packages");
    
    // Remove existing symlink if it exists
    if target_path.exists() {
        fs::remove_file(target_path)
            .map_err(|e| format!("Failed to remove existing symlink: {}", e))?;
    }
    
    // Create symlink
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&current_exe, target_path)
            .map_err(|e| format!("Failed to create symlink: {}. You may need to run this with sudo privileges.", e))?;
    }
    
    #[cfg(not(unix))]
    {
        return Err("PATH installation is only supported on Unix-like systems".to_string());
    }
    
    Ok(format!(
        "Successfully added update-packages to PATH!\nSymlink created: {} -> {}\nRestart your terminal and run 'which update-packages' to verify.",
        target_path.display(),
        current_exe.display()
    ))
}
