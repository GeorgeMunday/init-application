use std::process::Command;
use std::{thread, time};

fn time_sleep(num: u64){
    let duration = time::Duration::from_secs(num);
    let now = time::Instant::now();
    thread::sleep(duration);
    assert!(now.elapsed() >= duration);
}

pub fn main() {
    println!("\nSelected: .NET SDK Test");
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "dotnet", "--version"])
            .status()
    } else {
        Command::new("dotnet")
            .arg("--version")
            .status()
    };
    match status {
        Ok(exit_status) if exit_status.success() => {
            println!(".NET SDK is installed and working!");
        }
        Ok(_) => {
            eprintln!("Failed to run dotnet --version");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    time_sleep(2);
}