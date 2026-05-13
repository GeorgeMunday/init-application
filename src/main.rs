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
    println!("\nOption Details:\n");
    println!("1 -> Creates a Python CLI project structure");
    println!("2 -> Creates a Next.js 15 starter structure\n");
    
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

        match input.trim() {
            "1" => {
                generators::nextjs::main();
            }
            "2" => {
                generators::vite::main();
            }
            "3" =>{
                generators::react::main();
            }
            
            "8" => {
                generators::rust::main();
            }
            "O" => {
                details();
            }
            "Q" => {
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
