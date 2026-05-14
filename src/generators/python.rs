use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use super::helpers::{folder, code};

pub fn main() {
    println!("\nSelected: Python CLI Project");

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
        println!("Creating Python project {}", input);
    }

    create_python_structure(&project_path);
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn create_python_structure(project_path: &Path) {
    let dirs = vec!["src", "tests", "docs"];

    for dir in dirs {
        let path = project_path.join(dir);
        if !path.exists() {
            fs::create_dir_all(&path).expect(&format!("Failed to create directory: {}", path.display()));
            println!("✓ Created: {}", path.display());
        }
    }

    let files = vec![
        (
            "src/main.py",
            "def main() -> None:\n    print(\"Hello, World!\")\n\n\nif __name__ == \"__main__\":\n    main()\n",
        ),
        ("src/__init__.py", ""),
        (".gitignore", "__pycache__/\n*.pyc\n.venv/\n.pytest_cache/\n"),
        ("requirements.txt", ""),
        ("README.md", "# My Python Application\n\nA Python CLI project.\n"),
    ];

    for (filepath, content) in files {
        let path = project_path.join(filepath);
        if !path.exists() {
            fs::write(&path, content).expect(&format!("Failed to create file: {}", path.display()));
            println!("✓ Created: {}", path.display());
        }
    }

    println!("\nPython project structure created successfully!");
}
