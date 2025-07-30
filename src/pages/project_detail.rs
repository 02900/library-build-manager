use dioxus::prelude::*;
use crate::types::TargetPath;
use crate::logic::*;

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
            let mut is_success = use_signal(|| true);
            
            let commands = parse_package_json(&current_project().path);

            rsx! {
                div { class: "min-h-screen bg-gray-50 p-6",
                    // Header
                    div { class: "max-w-4xl mx-auto mb-8",
                        div { class: "flex items-center justify-between",
                            div {
                                Link { to: crate::Route::Home {}, 
                                    class: "text-blue-600 hover:text-blue-800 mb-2 inline-block",
                                    "← Back to Projects"
                                }
                                h1 { class: "text-3xl font-bold text-gray-900", "{current_project().name}" }
                                p { class: "text-gray-600 mt-1", "{current_project().path}" }
                            }
                        }
                    }

                    div { class: "max-w-4xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-8",
                        // Build Commands Section
                        div { class: "bg-white rounded-lg shadow-md p-6",
                            h2 { class: "text-xl font-semibold text-gray-900 mb-4", "Build Commands" }
                            
                            if !commands.is_empty() {
                                div { class: "space-y-3",
                                    for cmd in commands.iter() {
                                        div { 
                                            class: format!("p-3 border rounded-lg cursor-pointer transition-colors {}",
                                                if current_project().selected_build_command.as_ref() == Some(cmd) {
                                                    "border-blue-500 bg-blue-50"
                                                } else {
                                                    "border-gray-200 hover:border-gray-300"
                                                }
                                            ),
                                            onclick: {
                                                let cmd = cmd.clone();
                                                move |_| {
                                                    let mut proj = current_project();
                                                    proj.selected_build_command = Some(cmd.clone());
                                                    current_project.set(proj.clone());
                                                    
                                                    let mut all_projects = load_projects();
                                                    if let Some(p) = all_projects.iter_mut().find(|p| p.id == proj.id) {
                                                        p.selected_build_command = Some(cmd.clone());
                                                    }
                                                    save_projects(&all_projects);
                                                }
                                            },
                                            
                                            div { class: "flex items-center justify-between",
                                                div {
                                                    h3 { class: "font-medium text-gray-900", "{cmd}" }
                                                }
                                                if current_project().selected_build_command.as_ref() == Some(cmd) {
                                                    span { class: "text-blue-600", "✓" }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                if current_project().selected_build_command.is_some() {
                                    div { class: "mt-6 pt-4 border-t",
                                        button {
                                            class: "w-full bg-green-600 hover:bg-green-700 text-white py-2 px-4 rounded-lg transition-colors",
                                            onclick: {
                                                let project = current_project();
                                                move |_| {
                                                    let project_clone = project.clone();
                                                    spawn(async move {
                                                        match build_and_update_project(&project_clone) {
                                                            Ok(output) => {
                                                                result_message.set(format!("Build successful!\n\n{}", output));
                                                                is_success.set(true);
                                                            }
                                                            Err(e) => {
                                                                result_message.set(format!("Build failed: {}", e));
                                                                is_success.set(false);
                                                            }
                                                        }
                                                        show_result_modal.set(true);
                                                    });
                                                }
                                            },
                                            "🔨 Build Project"
                                        }
                                    }
                                }
                            } else {
                                div { class: "text-center py-8",
                                    p { class: "text-gray-500", "No package.json found or no scripts available" }
                                }
                            }
                        }

                        // Target Paths Section
                        div { class: "bg-white rounded-lg shadow-md p-6",
                            div { class: "flex items-center justify-between mb-4",
                                h2 { class: "text-xl font-semibold text-gray-900", "Target Paths" }
                                button {
                                    class: "bg-blue-600 hover:bg-blue-700 text-white px-3 py-1 rounded text-sm transition-colors",
                                    onclick: move |_| show_add_path_modal.set(true),
                                    "+ Add Path"
                                }
                            }
                            
                            if current_project().target_paths.is_empty() {
                                div { class: "text-center py-8",
                                    p { class: "text-gray-500", "No target paths configured" }
                                    p { class: "text-sm text-gray-400 mt-1", "Add paths where the built library should be copied" }
                                }
                            } else {
                                div { class: "space-y-3",
                                    for (index, target_path) in current_project().target_paths.iter().enumerate() {
                                        div { class: "p-3 border border-gray-200 rounded-lg",
                                            div { class: "flex items-center justify-between",
                                                div { class: "flex-1",
                                                    p { class: "font-medium text-gray-900", "{target_path.path}" }
                                                    div { class: "flex items-center space-x-2 mt-1",
                                                        span { 
                                                            class: format!("text-xs px-2 py-1 rounded {}",
                                                                if target_path.is_active {
                                                                    "bg-green-100 text-green-800"
                                                                } else {
                                                                    "bg-gray-100 text-gray-600"
                                                                }
                                                            ),
                                                            if target_path.is_active { "Active" } else { "Inactive" }
                                                        }
                                                    }
                                                }
                                                div { class: "flex items-center space-x-2",
                                                    button {
                                                        class: format!("px-3 py-1 text-xs rounded transition-colors {}",
                                                            if target_path.is_active {
                                                                "bg-yellow-100 text-yellow-800 hover:bg-yellow-200"
                                                            } else {
                                                                "bg-green-100 text-green-800 hover:bg-green-200"
                                                            }
                                                        ),
                                                        onclick: {
                                                            let index = index;
                                                            move |_| {
                                                                let mut proj = current_project();
                                                                proj.target_paths[index].is_active = !proj.target_paths[index].is_active;
                                                                current_project.set(proj.clone());
                                                                
                                                                let mut all_projects = load_projects();
                                                                if let Some(p) = all_projects.iter_mut().find(|p| p.id == proj.id) {
                                                                    p.target_paths[index].is_active = proj.target_paths[index].is_active;
                                                                }
                                                                save_projects(&all_projects);
                                                            }
                                                        },
                                                        if target_path.is_active { "Deactivate" } else { "Activate" }
                                                    }
                                                    button {
                                                        class: "px-3 py-1 text-xs bg-red-100 text-red-800 hover:bg-red-200 rounded transition-colors",
                                                        onclick: {
                                                            let index = index;
                                                            move |_| {
                                                                let mut proj = current_project();
                                                                proj.target_paths.remove(index);
                                                                current_project.set(proj.clone());
                                                                
                                                                let mut all_projects = load_projects();
                                                                if let Some(p) = all_projects.iter_mut().find(|p| p.id == proj.id) {
                                                                    p.target_paths.remove(index);
                                                                }
                                                                save_projects(&all_projects);
                                                            }
                                                        },
                                                        "Remove"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                if current_project().target_paths.iter().any(|p| p.is_active) && current_project().selected_build_command.is_some() {
                                    div { class: "mt-6 pt-4 border-t",
                                        button {
                                            class: "w-full bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded-lg transition-colors",
                                            onclick: {
                                                let project = current_project();
                                                move |_| {
                                                    let project_clone = project.clone();
                                                    spawn(async move {
                                                        match build_and_update_project(&project_clone) {
                                                            Ok(output) => {
                                                                result_message.set(format!("Update successful!\n\n{}", output));
                                                                is_success.set(true);
                                                            }
                                                            Err(e) => {
                                                                result_message.set(format!("Update failed: {}", e));
                                                                is_success.set(false);
                                                            }
                                                        }
                                                        show_result_modal.set(true);
                                                    });
                                                }
                                            },
                                            "🚀 Build & Update Targets"
                                        }
                                    }
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
                                                        if let Some(path) = open_folder_dialog().await {
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
                                            let target_path = TargetPath {
                                                id: uuid::Uuid::new_v4().to_string(),
                                                path: new_path().trim().to_string(),
                                                is_active: true,
                                            };
                                            
                                            let mut proj = current_project();
                                            proj.target_paths.push(target_path);
                                            current_project.set(proj.clone());
                                            
                                            let mut all_projects = load_projects();
                                            if let Some(p) = all_projects.iter_mut().find(|p| p.id == proj.id) {
                                                *p = proj;
                                            }
                                            save_projects(&all_projects);
                                            
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
                            div { class: "bg-white rounded-lg p-6 w-full max-w-lg mx-4",
                                h2 { 
                                    class: format!("text-xl font-semibold mb-4 {}",
                                        if is_success() { "text-green-800" } else { "text-red-800" }
                                    ),
                                    if is_success() { "Success!" } else { "Error" }
                                }
                                
                                div { class: "bg-gray-50 rounded-md p-4 mb-4",
                                    pre { class: "text-sm whitespace-pre-wrap", "{result_message()}" }
                                }
                                
                                div { class: "flex justify-end",
                                    button {
                                        class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition-colors",
                                        onclick: move |_| show_result_modal.set(false),
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
                div { class: "min-h-screen bg-gray-50 p-6",
                    div { class: "max-w-4xl mx-auto text-center py-12",
                        h1 { class: "text-2xl font-bold text-gray-900 mb-4", "Project Not Found" }
                        p { class: "text-gray-600 mb-6", "The project you're looking for doesn't exist." }
                        Link { 
                            to: crate::Route::Home {},
                            class: "bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg transition-colors",
                            "Back to Projects"
                        }
                    }
                }
            }
        }
    }
}
