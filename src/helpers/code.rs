use std::process::Command;
use std::io;

pub fn open(path: &str) -> io::Result<()> {
        Command::new("cmd")
                .args(&["/C", "code", path])
                .status()
                .map(|_| ())
}