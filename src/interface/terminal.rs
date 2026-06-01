use std::{thread, time, process::Command};

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const YELLOW: &str = "\x1b[33m";
pub const CYAN: &str = "\x1b[36m";

pub fn styled(text: &str, color: &str) -> String {
    format!("{color}{text}{RESET}")
}

pub fn section_title(title: &str) {
    println!("{}", styled(title, CYAN));
}

pub fn time_sleep(num: u64) {
    let duration = time::Duration::from_secs(num);
    thread::sleep(duration);
}

pub fn clear_console() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd")
            .args(["/c", "cls"])
            .status();
    } else {
        let _ = Command::new("clear")
            .status();
    }
}
