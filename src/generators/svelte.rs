use std::io::{self, Write};

pub fn main() {
    println!("\nSelected: Svelte Project");
    println!("(Svelte generator coming soon...)");
    print!("\nPress Anything to continue: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
