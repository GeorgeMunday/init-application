mod core;
mod helpers;
mod interface;

use std::io::{self, Write};
use std::process;
use crate::core::builder::generate_project;
use crate::core::template::{load_templates, contribute};
use crate::interface::menu::{welcome, show_templates_in_category, help, dependencies, change_project_destination};
use crate::interface::terminal::{clear_console, time_sleep};

fn main() {
    let mut in_folder: bool = false;
    loop {
        clear_console();
        let templates = load_templates();
        let (categories, category_map) = welcome(&templates, in_folder);

        print!("\nEnter option: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "h" => help(),
            "d" => dependencies(),
            "o" => {
                contribute();
            }
            "c" => {
                in_folder = !in_folder;
                change_project_destination(in_folder);
            }
            "q" => {
                println!("\nGoodbye!");
                time_sleep(1);
                clear_console();
                process::exit(0);
            }
            _ => {
                if let Ok(choice) = input.parse::<usize>() {
                    if choice > 0 && choice <= categories.len() {
                        let category = &categories[choice - 1];
                        let indices = &category_map[category];

                        if let Some(template_idx) = show_templates_in_category(category, indices, &templates) {
                            generate_project(&templates[template_idx], in_folder);
                        }
                    } else {
                        println!("\nInvalid category number");
                        time_sleep(1);
                    }
                } else {
                    println!("\nInvalid input");
                    time_sleep(1);
                }
            }
        }
    }
}

