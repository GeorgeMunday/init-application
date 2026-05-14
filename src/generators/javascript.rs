use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use super::helpers::folder;

pub fn main() {
    println!("\nSelected: Express.js Project");

    print!("\nEnter Name of Project: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let project_path = match folder::project_path(input) {
        Ok(p) => p,
        Err(e) => { eprintln!("Failed to determine project path: {}", e); return; }
    };
    if project_path.exists() {
        let base = project_path.parent().map(|p| p.display().to_string()).unwrap_or_else(|| ".".to_string());
        println!("\nProject {} already exists in {}", input, base);
    } else {
        println!("Creating Express.js project {}", input);
        create_express_project(project_path.parent().unwrap(), input);
    }

    print!("\nPress Anything to continue: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

// Folder creation and location is handled by helpers::folder

fn create_express_project(folder_path: &Path, project_name: &str) {
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args([
                "/C",
                "npm",
                "exec",
                "--yes",
                "express-generator",
                "--",
                "--no-view",
                "--git",
                project_name,
            ])
            .current_dir(folder_path)
            .status()
    } else {
        Command::new("npm")
            .args([
                "exec",
                "--yes",
                "express-generator",
                "--",
                "--no-view",
                "--git",
                project_name,
            ])
            .current_dir(folder_path)
            .status()
    };

    match status {
        Ok(s) if s.success() => println!("Successfully created Express.js project!"),
        Ok(s) => eprintln!("Failed to create project. Exit status: {}", s),
        Err(e) => eprintln!("Failed to execute npm command: {}", e),
    }
}