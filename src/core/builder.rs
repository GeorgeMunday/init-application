use std::io::{self, Write};
use std::process::Command;
use crate::helpers::{folder, code};
use crate::core::template::ProjectTemplate;

pub fn generate_project(template: &ProjectTemplate, in_folder: bool) {
    println!("\nSelected: {} project\n", template.name);

    loop {
        print!("\nEnter Name of Project: ");
        io::stdout().flush().expect("Failed to flush stdout");
        
        let mut input: String = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

        
    
        let input: &str = input.trim();

        if input.is_empty() {
            println!("\nProject name cannot be empty");
            continue;
        }

        let folder_path = if in_folder {
            std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
        } else {
            match folder::ensure_projects_dir() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to ensure projects dir: {}", e);
                    return;
                }
            }
        };

        let project_path = folder_path.join(input);
        if project_path.exists() {
            println!("\nProject {} already exists in {}", input, folder_path.display());
            print!("Retry name? (Y/n): ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut answer = String::new();

            io::stdin().read_line(&mut answer).unwrap();
            if answer.trim().to_lowercase().starts_with('n'){
                break; 
            }

            print!("\x1B[7A");

            for _ in 0..7 {
                print!("\x1B[2K");
                print!("\x1B[1B");
            }

            print!("\x1B[7A");

        } else {
            println!("Creating {} project {}", template.name, input);

            let args_windows: Vec<String> = template
                .args_windows
                .iter()
                .map(|arg| {
                    arg.replace(
                        "{project_name}",
                        input,
                    )
                })
                .collect();

            let args_linux: Vec<String> = template
                .args_linux
                .iter()
                .map(|arg| {
                    arg.replace(
                        "{project_name}",
                        input,
                    )
                })
                .collect();

            let status = if cfg!(target_os = "windows") {
                Command::new(&template.tool_windows)
                    .args(&args_windows)
                    .current_dir(&folder_path)
                    .status()
            } else {
                Command::new(&template.tool_linux)
                    .args(&args_linux)
                    .current_dir(&folder_path)
                    .status()
            };

            match status {
                Ok(s) if s.success() => println!("Successfully created {} project!", template.name),
                Ok(s) => eprintln!("Failed to create project. Exit status: {}", s),
                Err(e) => eprintln!("Failed to execute {}/{} command: {}",&template.tool_linux,&template.tool_windows, e),
            }
            
            print!("\nWould you like to open this project in your IDE? (y/N): ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();
            if answer.trim().to_lowercase().starts_with('y') {
                match code::open(project_path.to_str().unwrap_or("")) {
                    Ok(_) => println!("Opened project in IDE."),
                    Err(e) => eprintln!("Failed to open IDE: {}", e),
                }
            }
            break;
        }
    }
    print!("\nPress Anything to continue: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}
