use std::process::Command;
use std::{thread, time};

fn time_sleep(num: u64){
    let duration = time::Duration::from_secs(num);
    let now = time::Instant::now();
    thread::sleep(duration);
    assert!(now.elapsed() >= duration);
}

pub fn main() {
    println!("\nSelected: Python Test");
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "python", "--version"])
            .status()
    } else {
        Command::new("python")
            .arg("--version")
            .status()
    };
    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("Python is installed and working!");
        }
        Ok(_) => {
            eprintln!("Failed to run python --version");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    time_sleep(2);
}