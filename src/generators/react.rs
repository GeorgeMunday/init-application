use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use std::process::Command;

pub fn main() {
    println!("\nSelected: React project\n");

    print!("\nEnter Name of Project: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: &str = input.trim();
    
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

    let project_path = folder_path.join(input);
    if project_path.exists() {
        println!("\nProject {} already exists in {}", input, folder_path.display());
    } else {
        println!("Creating Next.js project {}", input);
        // npx create-react-app my-app
        let status = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "npx", "create-react-app", input])
                .current_dir(&folder_path)
                .status()
        } else {
            Command::new("npx")
                .args(&["create-react-app", input])
                .current_dir(&folder_path)
                .status()
        };

        match status {
            Ok(s) if s.success() => println!("Successfully created Next.js project!"),
            Ok(s) => eprintln!("Failed to create project. Exit status: {}", s),
            Err(e) => eprintln!("Failed to execute npx command: {}", e),
        }
    }

    print!("\nPress Anything to continue: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}
