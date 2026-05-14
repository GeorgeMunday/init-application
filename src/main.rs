mod generators;
mod tests;

use std::io::{self, Write};
use std::process::Command;
use std::{process, thread, time};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";

fn styled(text: &str, color: &str) -> String {
    format!("{color}{text}{RESET}")
}

fn section_title(title: &str) {
    println!("{}", styled(title, CYAN));
}

fn menu_item(key: &str, label: &str, description: &str) {
    println!("{key:>2}  {label:<24} {DIM}{description}{RESET}");
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

fn welcome(){
    println!();
    println!("{}", styled("========================================", CYAN));
    println!("{}", styled("           EASY INIT APPLICATION", BOLD));
    println!("{}", styled("========================================", CYAN));
    println!();

    section_title("Web Frameworks");
    menu_item("1", "Next.js", "Full-stack React framework");
    menu_item("2", "Vite React", "Fast modern React starter");
    menu_item("3", "React", "Classic React application");
    menu_item("4", "Vue", "Progressive frontend framework");
    menu_item("5", "Svelte", "Lean component-driven app");
    menu_item("6", "Angular", "Structured enterprise frontend");
    menu_item("7", "Blazor", "C# web UI application");

    println!();
    section_title("CLI Applications");
    menu_item("8", "Rust CLI", "Native command-line project");
    menu_item("9", "Python CLI", "Python command-line project");
    menu_item("10", "C# CLI", "Dotnet console application");
    menu_item("12", "TypeScript CLI", "Node.js CLI with TypeScript");
    menu_item("13", "JavaScript CLI", "Node.js CLI with JavaScript");

    println!();
    section_title("Help");
    menu_item("H", "Help", "View documentation and usage");
    menu_item("D", "Dependencies", "Show required tools and links");
    menu_item("T", "Test", "Run troubleshooting tests");
    menu_item("Q", "Quit", "Exit the application");
}

fn help() {
    clear_console();
    println!();
    println!("{}", styled("HELP", BOLD));
    println!("{}", styled("----", CYAN));
    println!();
    println!("1-7  -> Create web framework projects");
    println!("8-10 -> Create CLI projects");
    println!("12-13 -> Create JavaScript and TypeScript CLI projects");
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
    section_title("Web Frameworks");
    println!("1  Next.js       -> Node.js (https://nodejs.org/)");
    println!("2  Vite React    -> Node.js (https://nodejs.org/)");
    println!("3  React         -> Node.js (https://nodejs.org/)");
    println!("4  Vue           -> Node.js (https://nodejs.org/)");
    println!("5  Svelte        -> Node.js (https://nodejs.org/)");
    println!("6  Angular       -> Node.js (https://nodejs.org/)");
    println!("7  Blazor        -> .NET SDK (https://dotnet.microsoft.com/download)");
    
    println!();
    section_title("CLI Applications");
    println!("8  Rust CLI      -> Rust/Cargo (https://rustup.rs/)");
    println!("9  Python CLI    -> Python 3+ (https://www.python.org/downloads/)");
    println!("10  C# CLI        -> .NET SDK (https://dotnet.microsoft.com/download)");
    println!("12  TypeScript    -> Node.js (https://nodejs.org/), TypeScript (https://www.typescriptlang.org/)");
    println!("13  JavaScript    -> Node.js (https://nodejs.org/)");

    print!("\n{}Press Enter to continue...{}", YELLOW, RESET);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}

fn tests() {
    loop {
    clear_console();
    println!("{}", styled("TESTS", BOLD));
    println!("{}", styled("----", CYAN));
    println!("1  node -v");
    println!("2  dotnet --version");
    println!("3  cargo --version");
    println!("4  python --version");
    println!("B  Back to main menu");
    print!("\n{}Enter option:{}", YELLOW, RESET);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    match input.trim().to_lowercase().as_str() {
            "1" => {
                tests::node::main();
            }
            "2" => {
                tests::dotnet::main();
            }
            "3" =>{
                tests::cargo::main();
            }
            "4" => {
                tests::python::main();
            }
            "b" | "B" => {
                return;
            }
            _ => {           
                println!("\nInvalid option. Please try again.\n");  
                time_sleep( 1);
            }
        }
    }
}

fn main() {
    loop {
        clear_console();
        welcome();
        print!("\nEnter option: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().to_lowercase().as_str() {
            "1" => {
                generators::nextjs::main();
            }
            "2" => {
                generators::vite::main();
            }
            "3" =>{
                generators::react::main();
            }
            "4" => {
                generators::vue::main();
            }
            "5" => {
                generators::svelte::main();
            }
            "6" => {
                generators::angular::main();
            }
            "7" => {
                generators::blazor::main();
            }
            "8" => {
                generators::rust::main();
            }
            "9" => {
                generators::python::main();
            }
            "10" => {
                generators::csharp::main();
            }
            "12" => {
                generators::typescript::main();
            }
            "13" => {
                generators::javascript::main();
            }
            "h" | "H" => {
                help();
            }
            "t" | "T" => {
                tests();
            }
            "d" | "D" => {
                dependencies();
            }
            "q" | "Q" => {
                println!("\nGoodbye!");
                time_sleep( 1);
                clear_console();
                process::exit(0);
            }
            _ => {           
                println!("\nInvalid option. Please try again.\n");  
                time_sleep( 1);
            }
        }
    }
}
