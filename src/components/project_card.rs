use dioxus::prelude::*;
use crate::types::Project;

#[component]
pub fn ProjectCard(project: Project) -> Element {
    rsx! {
        div {
            class: "bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow cursor-pointer",
            onclick: move |_| {
                navigator().push(crate::Route::ProjectDetail { id: project.id.clone() });
            },
            
            h3 { class: "text-xl font-semibold text-gray-800 mb-2", "{project.name}" }
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
    }
}
