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
            let mut show_commands_accordion = use_signal(|| false);
            let mut is_building = use_signal(|| false);
            let mut current_command = use_signal(|| String::new());
            
            let commands = parse_package_json(&current_project().path);

            rsx! {
                div { class: "min-h-screen bg-gray-50 p-6",
                    // Header
                    div { class: "max-w-4xl mx-auto mb-8",
                        div { class: "flex items-center justify-between",
                            div {
                                Link {
                                    to: crate::Route::Home {},
                                    class: "text-blue-600 hover:text-blue-800 mb-2 inline-block",
                                    "← Back to Projects"
                                }
                                h1 { class: "text-3xl font-bold text-gray-900",
                                    "{current_project().name}"
                                }
                                p { class: "text-gray-600 mt-1", "{current_project().path}" }
                            }
                        }
                    }

                    div { class: "max-w-4xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-8",
                        // Build Commands Section
                        div { class: "bg-white rounded-lg shadow-md p-6",
                            // Accordion Header
                            div {
                                class: "flex items-center justify-between cursor-pointer p-2 hover:bg-gray-50 rounded-lg",
                                onclick: move |_| show_commands_accordion.set(!show_commands_accordion()),
                                div { class: "flex items-center space-x-3",
                                    h2 { class: "text-xl font-semibold text-gray-900",
                                        "Build Commands"
                                    }
                                    if !current_project().selected_build_commands.is_empty() {
                                        span { class: "bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full",
                                            "{current_project().selected_build_commands.len()} selected"
                                        }
                                    }
                                }
                                div { class: "text-gray-500",
                                    if show_commands_accordion() {
                                        "▼"
                                    } else {
                                        "▶"
                                    }
                                }
                            }
                            // Accordion Content
                            if show_commands_accordion() {
                                div { class: "mt-4 space-y-4",
                                    // Available Commands
                                    if !commands.is_empty() {
                                        div {
                                            h3 { class: "text-lg font-medium text-gray-800 mb-3",
                                                "Available Commands"
                                            }
                                            div { class: "grid grid-cols-1 gap-2 lg:max-h-[240px] overflow-y-scroll",
                                                for cmd in commands.iter() {
                                                    div {
                                                        class: format!(
                                                            "p-3 border rounded-lg cursor-pointer transition-colors {}",
                                                            if current_project().selected_build_commands.contains(cmd) {
                                                                "border-green-500 bg-green-50"
                                                            } else {
                                                                "border-gray-200 hover:border-gray-300"
                                                            },
                                                        ),
                                                        onclick: {
                                                            let cmd = cmd.clone();
                                                            move |_| {
                                                                let mut proj = current_project();
                                                                if proj.selected_build_commands.contains(&cmd) {
                                                                    // Remove command
                                                                    proj.selected_build_commands.retain(|c| c != &cmd);
                                                                } else {
                                                                    // Add command
                                                                    proj.selected_build_commands.push(cmd.clone());
                                                                }
                                                                current_project.set(proj.clone());

                                                                let mut all_projects = load_projects();
                                                                if let Some(p) = all_projects.iter_mut().find(|p| p.id == proj.id) {
                                                                    p.selected_build_commands = proj.selected_build_commands.clone();
                                                                }
                                                                save_projects(&all_projects);
                                                            }
                                                        },
                                                        div { class: "flex items-center justify-between",
                                                            div {
                                                                h4 { class: "font-medium text-gray-900",
                                                                    "{cmd}"
                                                                }
                                                            }
                                                            if current_project().selected_build_commands.contains(cmd) {
                                                                span { class: "text-green-600 font-bold",
                                                                    "✓"
                                                                }
                                                            } else {
                                                                span { class: "text-gray-400",
                                                                    "+"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // Selected Commands (with ordering)
                                    if !current_project().selected_build_commands.is_empty() {
                                        div { class: "border-t pt-4",
                                            h3 { class: "text-lg font-medium text-gray-800 mb-3",
                                                "Execution Order"
                                            }
                                            div { class: "space-y-2",
                                                for (index , cmd) in current_project().selected_build_commands.iter().enumerate() {
                                                    div { class: "flex items-center space-x-3 p-3 bg-blue-50 border border-blue-200 rounded-lg",
                                                        // Order number
                                                        div { class: "flex-shrink-0 w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center text-sm font-bold",
                                                            "{index + 1}"
                                                        }
                                                        // Command name
                                                        div { class: "flex-1",
                                                            span { class: "font-medium text-gray-900",
                                                                "{cmd}"
                                                            }
                                                        }
                                                        // Move buttons
                                                        div { class: "flex space-x-1",
                                                            if index > 0 {
                                                                button {
                                                                    class: "p-1 text-gray-500 hover:text-gray-700 hover:bg-gray-200 rounded",
                                                                    onclick: {
                                                                        let index = index;
                                                                        move |_| {
                                                                            let mut proj = current_project();
                                                                            proj.selected_build_commands.swap(index, index - 1);
                                                                            current_project.set(proj.clone());

                                                                            let mut all_projects = load_projects();
                                                                            if let Some(p) = all_projects.iter_mut().find(|p| p.id == proj.id) {
                                                                                p.selected_build_commands = proj.selected_build_commands.clone();
                                                                            }
                                                                            save_projects(&all_projects);
                                                                        }
                                                                    },
                                                                    "↑"
                                                                }
                                                            }
                                                            if index < current_project().selected_build_commands.len() - 1 {
                                                                button {
                                                                    class: "p-1 text-gray-500 hover:text-gray-700 hover:bg-gray-200 rounded",
                                                                    onclick: {
                                                                        let index = index;
                                                                        move |_| {
                                                                            let mut proj = current_project();
                                                                            proj.selected_build_commands.swap(index, index + 1);
                                                                            current_project.set(proj.clone());

                                                                            let mut all_projects = load_projects();
                                                                            if let Some(p) = all_projects.iter_mut().find(|p| p.id == proj.id) {
                                                                                p.selected_build_commands = proj.selected_build_commands.clone();
                                                                            }
                                                                            save_projects(&all_projects);
                                                                        }
                                                                    },
                                                                    "↓"
                                                                }
                                                            }
                                                            // Remove button
                                                            button {
                                                                class: "p-1 text-red-500 hover:text-red-700 hover:bg-red-100 rounded",
                                                                onclick: {
                                                                    let cmd = cmd.clone();
                                                                    move |_| {
                                                                        let mut proj = current_project();
                                                                        proj.selected_build_commands.retain(|c| c != &cmd);
                                                                        current_project.set(proj.clone());

                                                                        let mut all_projects = load_projects();
                                                                        if let Some(p) = all_projects.iter_mut().find(|p| p.id == proj.id) {
                                                                            p.selected_build_commands = proj.selected_build_commands.clone();
                                                                        }
                                                                        save_projects(&all_projects);
                                                                    }
                                                                },
                                                                "✕"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            // Show message when accordion is closed but commands are selected
                            if !show_commands_accordion() && !current_project().selected_build_commands.is_empty() {
                                div { class: "mt-4 p-3 bg-blue-50 border border-blue-200 rounded-lg",
                                    p { class: "text-sm text-blue-800",
                                        "Click to expand and manage your {current_project().selected_build_commands.len()} selected build commands"
                                    }
                                }
                            }
                            // Show empty state when no commands available
                            if commands.is_empty() {
                                div { class: "text-center py-8",
                                    p { class: "text-gray-500",
                                        "No package.json found or no scripts available"
                                    }
                                }
                            }
                        }

                        // Target Paths Section
                        div { class: "bg-white rounded-lg shadow-md p-6",
                            div { class: "flex items-center justify-between mb-4",
                                h2 { class: "text-xl font-semibold text-gray-900",
                                    "Target Paths"
                                }
                                button {
                                    class: "bg-blue-600 hover:bg-blue-700 text-white px-3 py-1 rounded text-sm transition-colors",
                                    onclick: move |_| show_add_path_modal.set(true),
                                    "+ Add Path"
                                }
                            }
                            if current_project().target_paths.is_empty() {
                                div { class: "text-center py-8",
                                    p { class: "text-gray-500", "No target paths configured" }
                                    p { class: "text-sm text-gray-400 mt-1",
                                        "Add paths where the built library should be copied"
                                    }
                                }
                            } else {
                                div { class: "space-y-3",
                                    for (index , target_path) in current_project().target_paths.iter().enumerate() {
                                        div { class: "p-3 border border-gray-200 rounded-lg",
                                            div { class: "flex items-center space-x-3",
                                                // Checkbox for active/inactive
                                                input {
                                                    r#type: "checkbox",
                                                    class: "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 focus:ring-2",
                                                    checked: target_path.is_active,
                                                    onchange: {
                                                        let index = index;
                                                        move |e| {
                                                            let mut proj = current_project();
                                                            proj.target_paths[index].is_active = e.checked();
                                                            current_project.set(proj.clone());

                                                            let mut all_projects = load_projects();
                                                            if let Some(p) = all_projects.iter_mut().find(|p| p.id == proj.id) {
                                                                p.target_paths[index].is_active = proj.target_paths[index].is_active;
                                                            }
                                                            save_projects(&all_projects);
                                                        }
                                                    },
                                                }
                                                // Path info
                                                div { class: "flex-1",
                                                    div { class: "font-medium text-gray-900",
                                                        "{extract_project_name(&target_path.path)}"
                                                    }
                                                    div { class: "text-xs text-gray-500 mt-1",
                                                        "{target_path.path}"
                                                    }
                                                }
                                                // Remove button
                                                button {
                                                    class: "px-3 py-1 text-xs bg-red-100 text-red-800 hover:bg-red-200 rounded transition-colors flex-shrink-0",
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
                                if current_project().target_paths.iter().any(|p| p.is_active)
                                    && !current_project().selected_build_commands.is_empty()
                                {
                                    div { class: "mt-6 pt-4 border-t",
                                        button {
                                            class: if is_building() {
                                                "w-full bg-blue-500 text-white py-2 px-4 rounded-lg cursor-not-allowed opacity-75"
                                            } else {
                                                "w-full bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded-lg transition-colors"
                                            },
                                            disabled: is_building(),
                                            onclick: {
                                                let project = current_project();
                                                move |_| {
                                                    if is_building() { return; }
                                                    
                                                    let project_clone = project.clone();
                                                    is_building.set(true);
                                                    current_command.set("Starting build...".to_string());
                                                    
                                                    spawn(async move {
                                                        match build_and_update_project_with_progress(&project_clone, current_command.clone()).await {
                                                            Ok(_) => {
                                                                result_message.set("✅ Build and update completed successfully!\n\nAll selected commands were executed and target paths were updated with the new version.".to_string());
                                                                is_success.set(true);
                                                            }
                                                            Err(e) => {
                                                                result_message.set(format!("Update failed: {}", e));
                                                                is_success.set(false);
                                                            }
                                                        }
                                                        is_building.set(false);
                                                        current_command.set(String::new());
                                                        show_result_modal.set(true);
                                                    });
                                                }
                                            },
                                            // Button content with spinner and dynamic text
                                            if is_building() {
                                                div { class: "flex items-center justify-center space-x-2",
                                                    // Spinning icon
                                                    svg {
                                                        class: "animate-spin h-4 w-4",
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        fill: "none",
                                                        "viewBox": "0 0 24 24",
                                                        stroke: "currentColor",
                                                        path {
                                                            "stroke-linecap": "round",
                                                            "stroke-linejoin": "round",
                                                            "stroke-width": "2",
                                                            d: "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                                                        }
                                                    }
                                                    span { "{current_command()}" }
                                                }
                                            } else {
                                                "🚀 Build & Update Targets"
                                            }
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
                                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                                            "Target Path"
                                        }
                                        div { class: "flex space-x-2",
                                            input {
                                                class: "flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                                r#type: "text",
                                                placeholder: "/path/to/target",
                                                value: new_path(),
                                                oninput: move |e| new_path.set(e.value()),
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
                                    class: format!(
                                        "text-xl font-semibold mb-4 {}",
                                        if is_success() { "text-green-800" } else { "text-red-800" },
                                    ),
                                    if is_success() {
                                        "Success!"
                                    } else {
                                        "Error"
                                    }
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
                        p { class: "text-gray-600 mb-6",
                            "The project you're looking for doesn't exist."
                        }
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
