use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use std::process::Command;

pub fn main() {
    println!("\nSelected: Rust CLI project\n");

    let home_dir = env::var("USERPROFILE").or_else(|_| env::var("HOME")).expect("Could not find home directory");
    let folder_path = Path::new(&home_dir).join("projects");
    
    if folder_path.exists() {
        println!("Folder already exists: {}", folder_path.display());
    } else {
        match fs::create_dir_all(&folder_path) {
            Ok(_) => println!("Successfully created folder: {}", folder_path.display()),
            Err(e) => eprintln!("Error creating folder: {}", e),
        }
    }

    let project_path = folder_path.join("rust1");
    if project_path.exists() {
        println!("\nProject 'rust1' already exists at {}. Skipping creation.", project_path.display());
    } else {
        println!("Creating Rust CLI project...");
        let status = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "cargo", "new", "rust1"])
                .current_dir(&folder_path)
                .status()
        } else {
            Command::new("cargo")
                .args(&["new", "rust1"])
                .current_dir(&folder_path)
                .status()
        };

        match status {
            Ok(s) if s.success() => println!("Successfully created Rust project!"),
            Ok(s) => eprintln!("Failed to create project. Exit status: {}", s),
            Err(e) => eprintln!("Failed to execute cargo command: {}", e),
        }
    }

    print!("\nPress Anything to continue: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}