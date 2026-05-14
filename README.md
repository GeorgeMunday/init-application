# init-application

A Rust CLI for creating starter project scaffolds from a simple menu.

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

## Downloadable Releases

This project is set up with [cargo-dist](https://axodotdev.github.io/cargo-dist/), which will generate GitHub Releases with downloadable binaries and installers.

After you publish a tagged release, users can:

1. Download the archive or installer from GitHub Releases
2. Extract or install it
3. Run it from a terminal after it is on `PATH`, for example:

```powershell
init-application
```

For publishing, the generated workflow is in [.github/workflows/release.yml](.github/workflows/release.yml).