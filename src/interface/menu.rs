use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use crate::core::template::{ProjectTemplate, load_templates};
use crate::interface::terminal::{styled, section_title, clear_console, time_sleep, BOLD, CYAN, YELLOW, RESET};

pub fn welcome(templates: &[ProjectTemplate], in_folder: bool) -> (Vec<String>, HashMap<String, Vec<usize>>) {
    println!();
    println!("{}", styled("========================================", CYAN));
    println!("{}", styled("           EASY INIT APPLICATION", BOLD));
    println!("{}", styled("========================================", CYAN));

    println!();
    println!("Current project destination: {}", if in_folder { "Current directory" } else { "In a Projects Folder" });

    let mut categories: Vec<String> = Vec::new();
    let mut category_map: HashMap<String, Vec<usize>> = HashMap::new();

    for (idx, t) in templates.iter().enumerate() {
        if !categories.contains(&t.category) {
            categories.push(t.category.clone());
        }
        category_map.entry(t.category.clone()).or_default().push(idx);
    }
    
    println!();
    section_title("SELECT CATEGORY");
    for (i, category) in categories.iter().enumerate() {
        println!("{}. {}", i + 1, category);
    }

    println!();
    section_title("COMMON OPTIONS");
    println!("H. Help");
    println!("D. View dependencies");
    println!("O. Open template.json");
    println!("C. Change Project Destination");
    println!("Q. Quit");

    (categories, category_map)
}

pub fn show_templates_in_category(category: &str, template_indices: &[usize], all_templates: &[ProjectTemplate]) -> Option<usize> {
    loop {
        clear_console();
        println!();
        println!("{}", styled("========================================", CYAN));
        section_title(&format!("TEMPLATES: {}", category.to_uppercase()));
        println!("{}", styled("========================================", CYAN));
        println!();

        for (display_idx, &template_idx) in template_indices.iter().enumerate() {
            let template = &all_templates[template_idx];
            println!("{}. {} - {}", display_idx + 1, template.name, template.description);
        }

        println!("\nB. Back to Categories");
        print!("\nEnter option: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input == "b" {
            return None;
        }

        if let Ok(choice) = input.parse::<usize>() {
            if choice > 0 && choice <= template_indices.len() {
                return Some(template_indices[choice - 1]);
            }
        }

        println!("\nInvalid option");
        time_sleep(1);
    }
}

pub fn help() {
    clear_console();
    println!();
    println!("{}", styled("           HOW TO USE EASY INIT APPLICATION", BOLD));
    println!("{}", styled("==========================================================", CYAN));
    println!();
    println!("{}", styled("1. CATEGORIES:", BOLD));
    println!("   Select a number from the main menu to browse project templates.");
    println!("   Each category groups similar frameworks (e.g., Backend, Frontend).");
    println!();
    println!("{}", styled("2. TEMPLATES:", BOLD));
    println!("   Once inside a category, choose a template number to start.");
    println!("   You will be prompted for a project name.");
    println!();
    println!("{}", styled("3. DESTINATION:", BOLD));
    println!("   Use 'C' to toggle where your projects are created:");
    println!("   - Current directory: Files are created right where you are.");
    println!("   - Projects Folder: Files are created in your user 'projects' dir.");
    println!();
    println!("{}", styled("4. DEPENDENCIES:", BOLD));
    println!("   Use 'D' to see what tools (Node.js, Rust, .NET, etc.) you need");
    println!("   to have installed for each template to work correctly.");
    println!();
    println!("{}", styled("5. TROUBLESHOOTING & TIPS:", BOLD));
    println!("   - Command not found? Ensure the tool (like 'npm' or 'cargo')");
    println!("     is in your system PATH.");
    println!("   - Permission denied? Try running the app as administrator or");
    println!("     checking folder permissions.");
    println!("   - VS Code not opening? Ensure the 'code' command is installed");
    println!("     via the Command Palette in VS Code.");
    println!();
    println!("{}", styled("6. IDE INTEGRATION:", BOLD));
    println!("   After generation, the app will ask if you want to open the");
    println!("   new project directly in your IDE (e.g., VS Code).");
    println!();
    println!("{}", styled("==========================================================", CYAN));
    print!("\n{}Press Enter to return to menu...{}", YELLOW, RESET);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}

pub fn dependencies() {
    clear_console();
    println!();
    println!("{}", styled("             SYSTEM DEPENDENCY GUIDE", BOLD));
    println!("{}", styled("==========================================================", CYAN));
    println!();
    println!("To use these templates, ensure the following tools are ");
    println!("installed on your system and available in your PATH.");
    println!();
    
    let is_windows = cfg!(target_os = "windows");
    println!("{} detected: {}", 
        styled("System", BOLD), 
        if is_windows { "Windows" } else { "Linux/macOS" }
    );
    println!();

    let templates = load_templates();
    let mut map: HashMap<String, HashSet<(String, String)>> = HashMap::new();

    for t in templates.iter() {
        if let Some(deps) = &t.dependencies {
            let entry = map.entry(t.category.clone()).or_insert_with(HashSet::new);
            for d in deps.iter() {
                entry.insert((d.name.clone(), d.link.clone()));
            }
        }
    }

    // Sort categories for consistent display
    let mut sorted_categories: Vec<_> = map.keys().collect();
    sorted_categories.sort();

    for category in sorted_categories {
        println!("{}", styled(&format!("-- {} --", category.to_uppercase()), YELLOW));
        let deps = &map[category];
        for (name, link) in deps.iter() {
            println!("  • {:<15} -> {}", styled(name, BOLD), link);
        }
        println!();
    }

    println!("{}", styled("OS-SPECIFIC TIPS:", BOLD));
    if is_windows {
        println!("- Use 'PowerShell' or 'CMD' for best compatibility.");
        println!("- For Node.js templates, 'nvm-windows' is recommended.");
    } else {
        println!("- Use 'bash' or 'zsh'.");
        println!("- Ensure build-essential or equivalent is installed for C++.");
    }
    println!();

    println!("{}", styled("==========================================================", CYAN));
    print!("\n{}Press Enter to return to menu...{}", YELLOW, RESET);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}

pub fn change_project_destination(in_folder: bool) {
    clear_console();
    println!();
    section_title("CHANGE PROJECT DESTINATION");
    println!("Current destination: {}", if in_folder { "Current directory" } else { "In a Projects Folder" });
    println!("Toggling destination...");
    time_sleep(1);
}
