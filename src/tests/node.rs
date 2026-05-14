use std::process::Command;
use std::{thread, time};

fn time_sleep(num: u64){
    let duration = time::Duration::from_secs(num);
    let now = time::Instant::now();
    thread::sleep(duration);
    assert!(now.elapsed() >= duration);
}

pub fn main() {
    println!("\nSelected: Node.js Test");
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "node", "-v"])
            .status()
    } else {
        Command::new("node")
            .arg("-v")
            .status()
    };
    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("Node.js is installed and working!");
        }
        Ok(_) => {
            eprintln!("Failed to run node -v");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    time_sleep(2);
}