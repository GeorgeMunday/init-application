use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;

pub fn ensure_projects_dir() -> io::Result<PathBuf> {
    let home_dir = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
    let folder_path = Path::new(&home_dir).join("projects");

    if folder_path.exists() {
        println!("Folder already exists: {}", folder_path.display());
    } else {
        fs::create_dir_all(&folder_path)?;
        println!("Successfully created folder: {}", folder_path.display());
    }

    Ok(folder_path)
}

pub fn project_path(project_name: &str) -> io::Result<PathBuf> {
    let base = ensure_projects_dir()?;
    Ok(base.join(project_name))
}