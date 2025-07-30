use dioxus::prelude::*;
use dioxus::desktop::{WindowBuilder};
use clap::{Parser, Subcommand};
use std::process;

mod types;
mod logic;
mod pages;
mod components;

use pages::{Home, ProjectDetail, Settings};
use logic::*;

#[derive(Parser)]
#[command(name = "update-packages")]
#[command(about = "A tool to manage library builds and deployments")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build and update targets for a project
    Build {
        /// Project name or ID to build
        #[arg(short, long)]
        project: String,
        /// List all available projects
        #[arg(short, long)]
        list: bool,
    },
    /// List all projects
    List,
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/project/:id")]
    ProjectDetail { id: String },
    #[route("/settings")]
    Settings {},
}



const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    use std::env;
    
    let args: Vec<String> = env::args().collect();
    
    // If no arguments provided (like from dx serve), launch GUI directly
    if args.len() == 1 {
        launch_gui();
        return;
    }
    
    // Check if we have empty string arguments (common with dx serve)
    if args.len() == 2 && args[1].is_empty() {
        launch_gui();
        return;
    }
    
    // If arguments are provided, try to parse them as CLI commands
    match Cli::try_parse() {
        Ok(cli) => {
            match cli.command {
                Some(Commands::Build { project, list }) => {
                    if list {
                        list_projects_cli();
                    } else {
                        build_project_cli(&project);
                    }
                }
                Some(Commands::List) => {
                    list_projects_cli();
                }
                None => {
                    // This shouldn't happen with proper clap setup, but launch GUI as fallback
                    launch_gui();
                }
            }
        }
        Err(err) => {
            // Check if it's a help or version request
            if args.iter().any(|arg| arg == "--help" || arg == "-h" || arg == "--version" || arg == "-V") {
                // For help/version, use the normal clap behavior
                let _ = Cli::parse();
            } else {
                // For other parsing errors, show the error and exit
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
    }
}

fn launch_gui() {
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


fn make_config() -> dioxus::desktop::Config {
    dioxus::desktop::Config::default().with_window(make_window())
}

fn make_window() -> WindowBuilder {
    WindowBuilder::new()
        .with_resizable(true)
        .with_always_on_top(false)
        .with_title("Update Packages")
}

// CLI Functions
fn list_projects_cli() {
    let projects = load_projects();
    
    if projects.is_empty() {
        println!("No projects found. Use the GUI to add projects first.");
        return;
    }
    
    println!("Available projects:");
    println!("{:-<60}", "");
    
    for project in projects {
        println!("📦 {} ({})", project.name, project.id);
        println!("   Path: {}", project.path);
        println!("   Build commands: {:?}", project.selected_build_commands);
        println!("   Active targets: {}", 
            project.target_paths.iter().filter(|p| p.is_active).count()
        );
        println!();
    }
}

fn build_project_cli(project_identifier: &str) {
    let projects = load_projects();
    
    // Find project by name or ID
    let project = projects.iter().find(|p| {
        p.name.to_lowercase() == project_identifier.to_lowercase() || 
        p.id == project_identifier
    });
    
    match project {
        Some(project) => {
            println!("🔨 Building project: {}", project.name);
            println!("📁 Path: {}", project.path);
            
            if project.selected_build_commands.is_empty() {
                println!("❌ Error: No build commands selected for this project.");
                println!("   Use the GUI to configure build commands first.");
                process::exit(1);
            }
            
            let active_targets = project.target_paths.iter().filter(|p| p.is_active).count();
            if active_targets == 0 {
                println!("❌ Error: No active target paths for this project.");
                println!("   Use the GUI to configure target paths first.");
                process::exit(1);
            }
            
            println!("🚀 Executing {} build commands...", project.selected_build_commands.len());
            for (i, cmd) in project.selected_build_commands.iter().enumerate() {
                println!("   {}. {}", i + 1, cmd);
            }
            
            println!("📤 Will update {} active targets", active_targets);
            println!();
            
            // Execute the build and update
            match build_and_update_project(project) {
                Ok(output) => {
                    println!("✅ Build and update completed successfully!");
                    println!();
                    println!("📋 Results:");
                    println!("{}", output);
                }
                Err(error) => {
                    println!("❌ Build failed: {}", error);
                    process::exit(1);
                }
            }
        }
        None => {
            println!("❌ Error: Project '{}' not found.", project_identifier);
            println!();
            println!("💡 Available projects:");
            list_projects_cli();
            process::exit(1);
        }
    }
}

