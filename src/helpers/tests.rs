use std::process::Command;
use std::{thread, time};

fn time_sleep(num: u64){
    let duration = time::Duration::from_secs(num);
    let now = time::Instant::now();
    thread::sleep(duration);
    assert!(now.elapsed() >= duration);
}

pub fn run_cargo_test() {
    println!("\nSelected: Cargo Test");
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cargo", "--version"])
            .status()
    } else {
        Command::new("cargo")
            .arg("--version")
            .status()
    };
    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("Cargo is installed and working!");
        }
        Ok(_) => {
            eprintln!("Failed to run cargo --version");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    time_sleep(2);
}

pub fn run_dotnet_test() {
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

pub fn run_node_test() {
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

pub fn run_python_test() {
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