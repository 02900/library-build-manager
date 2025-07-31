use dioxus::prelude::*;
use crate::types::Project;
use crate::logic::delete_project;

#[component]
pub fn ProjectCard(project: Project, on_delete: Option<EventHandler<String>>) -> Element {
    let mut show_delete_confirm = use_signal(|| false);
    
    rsx! {
        div {
            class: "bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow relative group",
            
            // Delete button (appears on hover)
            if let Some(_delete_handler) = on_delete {
                button {
                    class: "absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity bg-red-500 hover:bg-red-600 text-white rounded-full w-8 h-8 flex items-center justify-center text-sm font-bold",
                    onclick: move |e| {
                        e.stop_propagation(); // Prevent card click
                        show_delete_confirm.set(true);
                    },
                    "×"
                }
            }
            
            // Main card content (clickable)
            div {
                class: "cursor-pointer",
                onclick: move |_| {
                    navigator()
                        .push(crate::Route::ProjectDetail {
                            id: project.id.clone(),
                        });
                },
                h3 { class: "text-xl font-semibold text-gray-800 mb-2 pr-10", "{project.name}" }
                p { class: "text-gray-600 text-sm mb-3", "{project.path}" }
                div { class: "flex items-center justify-between",
                    div { class: "flex items-center space-x-2",
                        span { class: "text-xs bg-blue-100 text-blue-800 px-2 py-1 rounded",
                            "{project.target_paths.len()} paths"
                        }
                        if !project.selected_build_commands.is_empty() {
                            span { class: "text-xs bg-green-100 text-green-800 px-2 py-1 rounded",
                                "{project.selected_build_commands.len()} commands"
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
            
            // Delete confirmation modal
            if show_delete_confirm() {
                div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                    onclick: move |_| show_delete_confirm.set(false),
                    div { 
                        class: "bg-white rounded-lg p-6 w-full max-w-md mx-4",
                        onclick: move |e| e.stop_propagation(), // Prevent modal close on content click
                        
                        h2 { class: "text-xl font-semibold mb-4 text-red-600", "Delete Project" }
                        p { class: "text-gray-700 mb-6",
                            "Are you sure you want to delete the project "
                            strong { "\"{project.name}\"" }
                            "? This action cannot be undone."
                        }
                        
                        div { class: "flex justify-end space-x-3",
                            button {
                                class: "px-4 py-2 text-gray-600 hover:text-gray-800 transition-colors",
                                onclick: move |_| show_delete_confirm.set(false),
                                "Cancel"
                            }
                            button {
                                class: "px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-md transition-colors",
                                onclick: move |_| {
                                    match delete_project(&project.name) {
                                        Ok(_) => {
                                            show_delete_confirm.set(false);
                                            if let Some(handler) = on_delete {
                                                handler.call(project.name.clone());
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to delete project: {}", e);
                                            show_delete_confirm.set(false);
                                        }
                                    }
                                },
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
    }
}
