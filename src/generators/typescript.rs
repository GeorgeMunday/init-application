use std::fs;
use std::io::{self, Write};
use std::path::Path;
use super::helpers::{folder, code};

pub fn main() {
    println!("\nSelected: TypeScript CLI Project");

    print!("\nEnter Name of Project: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let folder_path = match folder::ensure_projects_dir() {
        Ok(p) => p,
        Err(e) => { eprintln!("Failed to ensure projects dir: {}", e); return; }
    };

    let project_path = folder_path.join(input);
    if project_path.exists() {
        println!("\nProject {} already exists in {}", input, folder_path.display());
    } else {
        println!("Creating TypeScript project {}", input);
    }

    create_typescript_structure(&project_path, input);

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

fn create_typescript_structure(project_path: &Path, project_name: &str) {
    let dirs = vec!["src", "tests", "docs"];

    for dir in dirs {
        let path = project_path.join(dir);
        if !path.exists() {
            fs::create_dir_all(&path).expect(&format!("Failed to create directory: {}", path.display()));
            println!("✓ Created: {}", path.display());
        }
    }

    let files: Vec<(&str, String)> = vec![
        (
            "src/main.ts",
            "function main(): void {\n    console.log(\"Hello, World!\");\n}\n\nmain();\n".to_string(),
        ),
        ("src/index.ts", "export * from './main';\n".to_string()),
        (
            "tsconfig.json",
            "{\n  \"compilerOptions\": {\n    \"target\": \"ES2019\",\n    \"module\": \"commonjs\",\n    \"rootDir\": \"src\",\n    \"outDir\": \"dist\",\n    \"strict\": true,\n    \"esModuleInterop\": true,\n    \"forceConsistentCasingInFileNames\": true,\n    \"skipLibCheck\": true,\n    \"moduleResolution\": \"node\"\n  },\n  \"include\": [\"src/**/*.ts\"],\n  \"exclude\": [\"node_modules\", \"dist\"]\n}\n".to_string(),
        ),
        (
            "package.json",
            format!(
                "{{\n  \"name\": \"{}\",\n  \"version\": \"1.0.0\",\n  \"description\": \"A TypeScript CLI project.\",\n  \"main\": \"dist/main.js\",\n  \"scripts\": {{\n    \"build\": \"tsc\",\n    \"start\": \"node dist/main.js\",\n    \"dev\": \"ts-node-dev --respawn --transpile-only src/main.ts\"\n  }},\n  \"dependencies\": {{}},\n  \"devDependencies\": {{\n    \"typescript\": \"^5.0.0\",\n    \"ts-node-dev\": \"^2.0.0\",\n    \"@types/node\": \"^20.0.0\"\n  }}\n}}\n",
                project_name
            ),
        ),
        (
            ".gitignore",
            "node_modules/\ndist/\n.env\ncoverage/\n.DS_Store\n".to_string(),
        ),
        (
            "README.md",
            format!(
                "# {}\n\nA TypeScript CLI project.\n\n## Run\n\n```bash\nnpm install\nnpm run dev\n```\n",
                project_name
            ),
        ),
    ];

    for (filepath, content) in files {
        let path = project_path.join(filepath);
        if !path.exists() {
            fs::write(&path, content).expect(&format!("Failed to create file: {}", path.display()));
            println!("✓ Created: {}", path.display());
        }
    }

    println!("\nTypeScript project structure created successfully!");
}