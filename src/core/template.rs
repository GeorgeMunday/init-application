use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::io::{self, Write};
use crate::interface::terminal::{YELLOW, RESET, section_title};

#[derive(Deserialize, Serialize, Clone)]
pub struct ProjectTemplate {
    pub name: String,
    pub description: String,
    pub category: String,
    pub dependencies: Option<Vec<Dependency>>,
    pub tool_windows: String,
    pub tool_linux: String,
    pub args_windows: Vec<String>,
    pub args_linux: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Dependency {
    pub name: String,
    pub link: String,
}

pub fn config_templates_path() -> PathBuf {
    if let Some(mut dir) = dirs_next::config_dir() {
        dir.push("init-application");
        let _ = fs::create_dir_all(&dir);
        dir.push("templates.json");
        dir
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join("templates.json")
    }
}

pub fn load_templates() -> Vec<ProjectTemplate> {
    let path = config_templates_path();

    if path.exists() {
        if let Ok(s) = fs::read_to_string(&path) {
            if let Ok(templates) = serde_json::from_str(&s) {
                return templates;
            }
        }
    }

    let default: Vec<ProjectTemplate> =
        serde_json::from_str(include_str!("../data/templates.json")).expect("Failed to parse embedded templates.json");

    if let Ok(json) = serde_json::to_string_pretty(&default) {
        let _ = fs::write(&path, json);
    }

    default
}

pub fn contribute() {
    crate::interface::terminal::clear_console();
    println!();
    section_title("CONTRIBUTE TEMPLATE");
    println!("Opening data/templates.json in VS Code...");

    let path_buf = config_templates_path();
    let path = path_buf.to_string_lossy().to_string();

    let open_result = Command::new("code").arg(&path).status();

    match open_result {
        Ok(status) if status.success() => {}
        _ => {
            if cfg!(target_os = "windows") {
                let _ = Command::new("cmd")
                    .args(["/c", "start", "", &path])
                    .status();
            } else if cfg!(target_os = "macos") {
                let _ = Command::new("open").arg(&path).status();
            } else {
                let _ = Command::new("xdg-open").arg(&path).status();
            }
        }
    }
    print!("\n{}Press Enter to continue...{}", YELLOW, RESET);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
}
