# init-application

A Rust CLI for creating starter project scaffolds from a simple menu.
This project is set up with [cargo-dist](https://axodotdev.github.io/cargo-dist/), which will generate GitHub Releases with downloadable binaries and installers.

## What it does

- Creates frontend app starters like Next.js, React, Vue, Svelte, Angular, and Vite
- Creates CLI starters for Rust, Python, C#, TypeScript, and JavaScript
- Includes dependency and help screens inside the terminal menu
- Can optionally open the generated project in VS Code

## Requirements

- Rust toolchain
- Node.js for the JavaScript and frontend templates
- Python for the Python test option
- .NET SDK for the C# and Blazor options
- VS Code if you want the open-in-editor prompt to work

## Run

```bash
cargo run
```

## Build

```bash
cargo build
```

## Installation
 
### Windows
 
**1. Download the source archive:**
 
```powershell
curl.exe -L https://github.com/GeorgeMunday/init-application/archive/refs/tags/v0.1.5.zip -o init-application-v0.1.5.zip
```
 
> Note: Use `curl.exe` rather than `curl` in PowerShell, as `curl` is an alias for `Invoke-WebRequest` and does not support the same flags.
 
**2. Extract the archive:**
 
```powershell
Expand-Archive -Path init-application-v0.1.5.zip -DestinationPath init-application-v0.1.5
```
 
**3. Install via Cargo:**
 
```powershell
cargo install --path init-application-v0.1.5\init-application-0.1.5
```
 
**4. Run:**
 
```powershell
init-application
```
 
---
 
### macOS / Linux
 
**1. Download the source archive:**
 
```bash
curl -L https://github.com/GeorgeMunday/init-application/archive/refs/tags/v0.1.1.tar.gz -o init-application-v0.1.5.tar.gz
```
 
**2. Extract the archive:**
 
```bash
tar -xzf init-application-v0.1.5.tar.gz
```
 
**3. Install via Cargo:**
 
```bash
cargo install --path init-application-v0.1.5/init-application-0.1.5
```
 
**4. Run:**
 
```bash
init-application
```
 
---
 
## Prerequisites
 
- [Rust and Cargo](https://www.rust-lang.org/tools/install) must be installed.
