use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use std::process::Command;
use super::helpers::folder;

pub fn main() {
    println!("\nSelected: Vite React project\n");

    print!("\nEnter Name of Project: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: &str = input.trim();
    
    let project_path = match folder::project_path(input) {
        Ok(p) => p,
        Err(e) => { eprintln!("Failed to determine project path: {}", e); return; }
    };
    if project_path.exists() {
        let base = project_path.parent().map(|p| p.display().to_string()).unwrap_or_else(|| ".".to_string());
        println!("\nProject {} already exists in {}", input, base);
    } else {
        println!("Creating Vite React project {}", input);
        let status = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&[
                    "/C",
                    "npm",
                    "create",
                    "vite@latest",
                    input,
                    "--",
                    "--template",
                    "react",
                ])
                .current_dir(project_path.parent().unwrap())
                .status()
        } else {
            Command::new("npm")
                .args(&["create", "vite@latest", input, "--", "--template", "react"])
                .current_dir(project_path.parent().unwrap())
                .status()
        };

        match status {
            Ok(s) if s.success() => println!("Successfully created Vite React project!"),
            Ok(s) => eprintln!("Failed to create project. Exit status: {}", s),
            Err(e) => eprintln!("Failed to execute npm command: {}", e),
        }
    }

    print!("\nPress Anything to continue: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}
