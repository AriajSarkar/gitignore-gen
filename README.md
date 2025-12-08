# gitignore-gen

[![CI](https://github.com/AriajSarkar/gitignore-gen/actions/workflows/ci.yml/badge.svg)](https://github.com/AriajSarkar/gitignore-gen/actions/workflows/ci.yml)
[![Release](https://github.com/AriajSarkar/gitignore-gen/actions/workflows/release.yml/badge.svg)](https://github.com/AriajSarkar/gitignore-gen/releases)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![MSRV](https://img.shields.io/badge/MSRV-1.83.0-orange.svg)](https://www.rust-lang.org/)

Automatically generate `.gitignore` files based on project analysis.

## Features

- 🔍 **Smart Detection** - Automatically detects project technologies by scanning files
- 🌐 **GitHub Templates** - Fetches official gitignore templates from GitHub
- 🦀 **Cross-Platform** - Works on Linux, Windows, and macOS
- ⚡ **Fast** - Written in Rust for maximum performance

## Supported Technologies

| Technology | Detection Method |
|------------|------------------|
| Rust | `Cargo.toml` |
| Go | `go.mod` |
| Node.js | `package.json` |
| Python | `requirements.txt`, `pyproject.toml` |
| Java | `pom.xml`, `.java` files |
| Maven | `pom.xml` |

## Installation

### One-Line Install (Recommended)

**Windows (PowerShell):**
```powershell
iwr https://github.com/AriajSarkar/gitignore-gen/raw/main/scripts/install.ps1 -useb | iex
```

**Linux / macOS:**
```bash
curl -fsSL https://github.com/AriajSarkar/gitignore-gen/raw/main/scripts/install.sh | bash
```

### Build from Source
If you have Rust installed:
```bash
cargo install --git https://github.com/AriajSarkar/gitignore-gen
```

## Usage

```bash
# Generate .gitignore in current directory
gitignore-gen
```

## How It Works

1. **Scan** - Analyzes your project directory for technology markers
2. **Detect** - Identifies languages and frameworks from file patterns
3. **Match** - Finds templates from embedded [github/gitignore](https://github.com/github/gitignore) collection
4. **Generate** - Combines templates and writes `.gitignore`

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

## Uninstallation

```bash
gitignore-gen uninstall
```

This will automatically remove the binary from your system.

## License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
