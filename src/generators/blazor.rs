use std::io::{self, Write};
use std::process::Command;
use super::helpers::{folder, code};

pub fn main() {
    println!("\nSelected: Blazor project\n");

    print!("\nEnter Name of Project: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: &str = input.trim();
    
    let folder_path = match folder::ensure_projects_dir() {
        Ok(p) => p,
        Err(e) => { eprintln!("Failed to ensure projects dir: {}", e); return; }
    };

    let project_path = folder_path.join(input);
    if project_path.exists() {
        println!("\nProject {} already exists in {}", input, folder_path.display());
    } else {
        println!("Creating Blazor WebAssembly project {}", input);
        let status = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "dotnet", "new", "blazorwasm", "-o", input])
                .current_dir(&folder_path)
                .status()
        } else {
            Command::new("dotnet")
                .args(&["new", "blazorwasm", "-o", input])
                .current_dir(&folder_path)
                .status()
        };

        match status {
            Ok(s) if s.success() => println!("Successfully created Blazor project!"),
            Ok(s) => eprintln!("Failed to create project. Exit status: {}", s),
            Err(e) => eprintln!("Failed to execute dotnet command: {}", e),
        }
    }

    print!("\nWould you like to open this project in VS Code and use helpers/code.rs for that? (y/N): ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    if answer.trim().to_lowercase().starts_with('y') {
        match code::open(project_path.to_str().unwrap_or("")) {
            Ok(_) => println!("Opened project in VS Code."),
            Err(e) => eprintln!("Failed to open VS Code: {}", e),
        }
    }

    print!("\nPress Anything to continue: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
