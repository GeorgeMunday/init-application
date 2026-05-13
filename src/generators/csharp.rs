use std::io::{self, Write};

pub fn main() {
    println!("\nSelected: C# CLI Project");
    println!("(C# generator coming soon...)");
    print!("\nPress Anything to continue: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
