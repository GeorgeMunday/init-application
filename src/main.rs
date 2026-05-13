mod generators;

use std::io::{self, Write};
use std::process::{Command};
use std::{thread, time};
use std::process;

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
    println!("\nWelcome to easy init\n");

    println!("Please Select a Option \n");

    println!("Web Frameworks\n");

    println!("1: Next Js Project");
    println!("2: Vite React Project");
    println!("3: React Project");
    println!("4: Vue Project");
    println!("5: Svelte Project");
    println!("6: Angular Project");
    println!("7: Blazor Project");

    println!("\nCLI Applications\n");

    println!("8: Rust CLI Project");
    println!("9: Python CLI Project");
    println!("10: C# CLI Project");
    println!("13: JavaScript CLI Project");
    println!("12: TypeScript CLI Project");
    
    println!("\nHelp\n");
    println!("H: More Help/Documentation");
    println!("D: Dependencies");
    println!("O: View Option Details");
    println!("Q: Quit");
}

fn details(){
    clear_console();
    println!("\nOption Details:\n");
    println!("Web Frameworks:");
    println!("1 -> Creates a Next.js full-stack React framework project");
    println!("2 -> Creates a Vite-powered React project (fast build tool)");
    println!("3 -> Creates a standard React project (create-react-app)");
    println!("4 -> Creates a Vue.js progressive framework project");
    println!("5 -> Creates a Svelte cybernetically enhanced project");
    println!("6 -> Creates an Angular platform project");
    println!("7 -> Creates a Blazor WebAssembly C# project");
    println!("\nCLI Applications:");
    println!("8 -> Creates a Rust CLI project");
    println!("9 -> Creates a Python CLI project");
    println!("10 -> Creates a C# console project");
    println!("12 -> Creates a TypeScript CLI project");
    println!("13 -> Creates a Node.js JavaScript CLI project");
    
    print!("\nPress Anything to continue: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}

fn dependencies() {
    clear_console();
    println!("\nProject Dependencies & Links:\n");
    println!("Web Frameworks:");
    println!("1: Next.js       -> Node.js (https://nodejs.org/)");
    println!("2: Vite React    -> Node.js (https://nodejs.org/)");
    println!("3: React         -> Node.js (https://nodejs.org/)");
    println!("4: Vue           -> Node.js (https://nodejs.org/)");
    println!("5: Svelte        -> Node.js (https://nodejs.org/)");
    println!("6: Angular       -> Node.js (https://nodejs.org/)");
    println!("7: Blazor        -> .NET SDK (https://dotnet.microsoft.com/download)");
    
    println!("\nCLI Applications:");
    println!("8: Rust CLI        -> Rust/Cargo (https://rustup.rs/)");
    println!("9: Python CLI      -> Python 3+ (https://www.python.org/downloads/)");
    println!("10: C# CLI         -> .NET SDK (https://dotnet.microsoft.com/download)");
    println!("13: JavaScript CLI -> Node.js (https://nodejs.org/)");
    println!("12: TypeScript CLI -> Node.js (https://nodejs.org/), TypeScript (https://www.typescriptlang.org/)");

    print!("\nPress Anything to continue: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
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
            "o" => {
                details();
            }
            "d" => {
                dependencies();
            }
            "q" => {
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
