use dioxus::prelude::*;
use crate::logic::*;
use crate::components::ProjectCard;



/// Home page - Main project list
#[component]
pub fn Home() -> Element {
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
                        p { class: "text-gray-600 mt-1",
                            "Manage your development libraries and build processes"
                        }
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
                            h3 { class: "text-xl font-semibold text-gray-900 mb-2",
                                "No projects yet"
                            }
                            p { class: "text-gray-600 mb-6",
                                "Start by adding your first library project to manage builds and deployments."
                            }
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
                                label { class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Project Name"
                                }
                                input {
                                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    r#type: "text",
                                    placeholder: "My Library",
                                    value: new_project_name(),
                                    oninput: move |e| new_project_name.set(e.value()),
                                }
                            }
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Project Path"
                                }
                                div { class: "flex space-x-2",
                                    input {
                                        class: "flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                        r#type: "text",
                                        placeholder: "/path/to/project",
                                        value: new_project_path(),
                                        oninput: move |e| new_project_path.set(e.value()),
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
                                        new_project_path().trim().to_string(),
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
