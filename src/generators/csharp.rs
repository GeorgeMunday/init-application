use std::fs;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use super::helpers::folder;

pub fn main() {
    println!("\nSelected: C# CLI Project");

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
        println!("Creating C# project {}", input);
        let status = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "dotnet", "new", "console", "-n", input])
                .current_dir(project_path.parent().unwrap())
                .status()
        } else {
            Command::new("dotnet")
                .args(&["new", "console", "-n", input])
                .current_dir(project_path.parent().unwrap())
                .status()
        };

        match status {
            Ok(s) if s.success() => println!("Successfully created C# project!"),
            Ok(s) => eprintln!("Failed to create project. Exit status: {}", s),
            Err(e) => eprintln!("Failed to execute dotnet command: {}", e),
        }
    }

    create_csharp_structure(&project_path);

    print!("\nPress Anything to continue: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn create_csharp_structure(project_path: &Path) {
    let dirs = vec![
        "src",
        "src/Program",
        "src/Models",
        "src/Services",
        "src/Utilities",
        "tests",
        "docs",
        "bin",
        "obj",
    ];

    for dir in dirs {
        let path = project_path.join(dir);
        if !path.exists() {
            fs::create_dir_all(&path).expect(&format!("Failed to create directory: {}", path.display()));
            println!("✓ Created: {}", path.display());
        }
    }

    let files = vec![
        ("src/Program.cs", "namespace MyApp\n{\n    class Program\n    {\n        static void Main(string[] args)\n        {\n            Console.WriteLine(\"Hello, World!\");\n        }\n    }\n}\n"),
        (".gitignore", "bin/\nobj/\n.vs/\n*.csproj.user\n"),
        ("README.md", "# My C# Application\n\nA C# CLI project.\n"),
    ];

    for (filepath, content) in files {
        let path = project_path.join(filepath);
        if !path.exists() {
            fs::write(&path, content).expect(&format!("Failed to create file: {}", path.display()));
            println!("✓ Created: {}", path.display());
        }
    }

    println!("\nC# project structure created successfully!");
}
