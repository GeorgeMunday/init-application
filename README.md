# init-application

A Rust-powered CLI tool to quickly initialize project scaffolds from an interactive menu.

[![Release](https://github.com/GeorgeMunday/init-application/actions/workflows/release.yml/badge.svg)](https://github.com/GeorgeMunday/init-application/actions/workflows/release.yml)

## 🚀 Features

- **Interactive Menu:** Easy-to-use CLI interface.
- **Frontend Templates:** Next.js, React, Vue, Svelte, Angular, Vite.
- **Backend/CLI Templates:** Rust, Python, C#, TypeScript, JavaScript.
- **Environment Checks:** Automatically checks for required tools like Node.js, Python, and .NET.
- **VS Code Integration:** Optionally opens your new project directly in VS Code.

## 📦 Installation

### Windows (PowerShell)
Install via the official installer:
```powershell
irm https://github.com/GeorgeMunday/init-application/releases/latest/download/init-application-installer.ps1 | iex
```

### macOS / Linux (Shell)
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/GeorgeMunday/init-application/releases/latest/download/init-application-installer.sh | sh
```

### From Source (Cargo)
If you have [Rust](https://www.rust-lang.org/tools/install) installed:
```bash
cargo install --git https://github.com/GeorgeMunday/init-application.git
```

## 🛠️ Requirements

The tool will prompt you if requirements are missing, but for best results ensure you have:
- **Node.js** (for frontend templates)
- **Python** (for Python templates)
- **.NET SDK** (for C# templates)
- **VS Code** (if you want the "Open in Editor" feature)

## 🚀 Usage

Run the following command in your terminal:
```bash
init-application
```

### Keyboard Shortcuts
In the main menu, you can use these shortcuts:
- `h`: **Help** - Show usage information.
- `d`: **Dependencies** - Check if required tools (Node.js, etc.) are installed.
- `o`: **Contribute** - Information on how to contribute to the project.
- `c`: **Change Destination** - Toggle between creating projects in the current folder or a new subfolder.
- `q`: **Quit** - Exit the application.

## 🤝 Contributing

Contributions are welcome! If you'd like to add a template or fix a bug:
1. Fork the repository.
2. Create a new branch.
3. Submit a pull request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

