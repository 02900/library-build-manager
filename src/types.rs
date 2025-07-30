use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub build_commands: Vec<String>,
    pub selected_build_command: Option<String>,
    pub target_paths: Vec<TargetPath>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetPath {
    pub id: String,
    pub path: String,
    pub is_active: bool,
}
