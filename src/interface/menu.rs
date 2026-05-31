use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use crate::core::template::{ProjectTemplate, load_templates};
use crate::interface::terminal::{styled, section_title, clear_console, time_sleep, BOLD, CYAN, YELLOW, RESET};

pub fn welcome(templates: &[ProjectTemplate]) -> (Vec<String>, HashMap<String, Vec<usize>>) {
    println!();
    println!("{}", styled("========================================", CYAN));
    println!("{}", styled("           EASY INIT APPLICATION", BOLD));
    println!("{}", styled("========================================", CYAN));

    let mut categories: Vec<String> = Vec::new();
    let mut category_map: HashMap<String, Vec<usize>> = HashMap::new();

    for (idx, t) in templates.iter().enumerate() {
        if !categories.contains(&t.category) {
            categories.push(t.category.clone());
        }
        category_map.entry(t.category.clone()).or_default().push(idx);
    }

    println!("\n{}", styled("SELECT A CATEGORY:", BOLD));
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
    println!("{}", styled("HELP", BOLD));
    println!("{}", styled("----", CYAN));
    println!();
    println!("Use the number or letter shown in the menu, then press Enter.");
    println!("Choose D for dependency links or Q to quit.");
    print!("\n{}Press Enter to continue...{}", YELLOW, RESET);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}

pub fn dependencies() {
    clear_console();
    println!();
    println!("{}", styled("DEPENDENCIES", BOLD));
    println!("{}", styled("------------", CYAN));
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

    for (category, deps) in map.iter() {
        section_title(category);
        for (name, link) in deps.iter() {
            println!("- {} -> {}", name, link);
        }
        println!();
    }

    print!("\n{}Press Enter to continue...{}", YELLOW, RESET);
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
    println!("Current destination: {}", if in_folder { "Inside current folder" } else { "In a new folder" });
    println!("Toggling destination...");
    time_sleep(1);
}
