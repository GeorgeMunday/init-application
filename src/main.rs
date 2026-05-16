mod build;
mod helpers;

use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::process::Command;
use std::{process, thread, time};
use std::path::PathBuf;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct ProjectTemplate {
    name: String,
    description: String,
    category: String,
    dependencies: Option<Vec<Dependency>>,
    tool_windows: String,
    tool_linux: String,
    args_windows: Vec<String>,
    args_linux: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Dependency {
    name: String,
    link: String,
}

use build::{
    generate_project,
};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";

struct MenuItem {
    key: String,
    description: String,
    template: ProjectTemplate,
}

fn styled(text: &str, color: &str) -> String {
    format!("{color}{text}{RESET}")
}

fn section_title(title: &str) {
    println!("{}", styled(title, CYAN));
}

fn time_sleep(num: u64){
    let duration = time::Duration::from_secs(num);
    let now = time::Instant::now();
    thread::sleep(duration);
    assert!(now.elapsed() >= duration);
}

fn clear_console(){
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("clear")
            .status()
            .unwrap();
    }
}

fn render_section(
    title: &str,
    category: &str,
    templates: &[ProjectTemplate],
) {

    section_title(title);

    for (index, template) in templates.iter().enumerate() {
        if template.category == category {
            println!(
                "{}. {} - {}",
                index + 1,
                template.name,
                template.description
            );
        }
    }

}

fn welcome(templates: &[ProjectTemplate]){
    println!();
    println!("{}", styled("========================================", CYAN));
    println!("{}", styled("           EASY INIT APPLICATION", BOLD));
    println!("{}", styled("========================================", CYAN));

    let mut seen: Vec<String> = Vec::new();
    for t in templates.iter() {
        if !seen.contains(&t.category) {
            seen.push(t.category.clone());
        }
    }

    for category in seen.iter() {
        println!();
        render_section(category, category.as_str(), templates);
    }
    println!();
    section_title("COMMON OPTIONS");
    println!("H. Help");
    println!("D. View dependencies");
    println!("C. Contribute a template");
    println!("Q. Quit");
}

fn help() {
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

fn dependencies() {
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

fn config_templates_path() -> PathBuf {
    if let Some(mut dir) = dirs_next::config_dir() {
        dir.push("init-application");
        let _ = fs::create_dir_all(&dir);
        dir.push("templates.json");
        dir
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join("templates.json")
    }
}

fn load_templates() -> Vec<ProjectTemplate> {
    let path = config_templates_path();

    if path.exists() {
        if let Ok(s) = fs::read_to_string(&path) {
            if let Ok(templates) = serde_json::from_str(&s) {
                return templates;
            }
        }
    }

    let default: Vec<ProjectTemplate> =
        serde_json::from_str(include_str!("data/templates.json")).expect("Failed to parse embedded templates.json");

    if let Ok(json) = serde_json::to_string_pretty(&default) {
        let _ = fs::write(&path, json);
    }

    default
}

fn contribute() {
    clear_console();
    println!();
    section_title("CONTRIBUTE TEMPLATE");
    println!("Opening data/templates.json in VS Code...");

    let path_buf = config_templates_path();
    let path = path_buf.to_string_lossy().to_string();

    let open_result = Command::new("code").arg(&path).status();

    match open_result {
        Ok(status) if status.success() => {}
        _ => {
            if cfg!(target_os = "windows") {
                let _ = Command::new("cmd")
                    .args(["/c", "start", "", &path])
                    .status();
            } else if cfg!(target_os = "macos") {
                let _ = Command::new("open").arg(&path).status();
            } else {
                let _ = Command::new("xdg-open").arg(&path).status();
            }
        }
    }
    print!("\n{}Press Enter to continue...{}", YELLOW, RESET);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
}

fn main() {

    loop {

        clear_console();

        let templates = load_templates();

        welcome(&templates);

        print!("\nEnter option: ");

        io::stdout()
            .flush()
            .expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim().to_lowercase();

        match input.as_str() {

            "h" | "H" => {
                help();
            }

            "d" | "D" => {
                dependencies();
            }

            "c" | "C" => {
                contribute();
                let templates = load_templates();
            }

            "q" | "Q" => {

                println!("\nGoodbye!");

                time_sleep(1);

                clear_console();

                process::exit(0);

            }

            _ => {

                match input.parse::<usize>() {

                    Ok(choice) => {

                        if choice > 0 && choice <= templates.len() {

                            let template =
                                &templates[choice - 1];

                            generate_project(template);

                        } else {

                            println!(
                                "\nInvalid option"
                            );

                            time_sleep(1);

                        }

                    }

                    Err(_) => {

                        println!(
                            "\nPlease enter a valid number"
                        );

                        time_sleep(1);

                    }

                }

            }

        }

    }

}