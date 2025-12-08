# gitignore-gen

[![CI](https://github.com/AriajSarkar/gitignore-gen/actions/workflows/ci.yml/badge.svg)](https://github.com/AriajSarkar/gitignore-gen/actions/workflows/ci.yml)
[![Release](https://github.com/AriajSarkar/gitignore-gen/actions/workflows/release.yml/badge.svg)](https://github.com/AriajSarkar/gitignore-gen/releases)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![MSRV](https://img.shields.io/badge/MSRV-1.70.0-orange.svg)](https://www.rust-lang.org/)

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

### Pre-built Binaries (Recommended)

1. Go to [Releases](https://github.com/AriajSarkar/gitignore-gen/releases)
2. Download the binary for your platform:
   - **Linux**: `gitignore-gen-x86_64-unknown-linux-gnu.tar.gz`
   - **Windows**: `gitignore-gen-x86_64-pc-windows-msvc.zip`
   - **macOS (Intel)**: `gitignore-gen-x86_64-apple-darwin.tar.gz`
   - **macOS (Apple Silicon)**: `gitignore-gen-aarch64-apple-darwin.tar.gz`

3. Extract and install:

   **Windows (PowerShell):**
   ```powershell
   Expand-Archive gitignore-gen-x86_64-pc-windows-msvc.zip -DestinationPath .
   Move-Item gitignore-gen.exe C:\Windows\System32\
   ```

   **macOS/Linux:**
   ```bash
   tar -xzf gitignore-gen-*.tar.gz
   chmod +x gitignore-gen
   sudo mv gitignore-gen /usr/local/bin/
   ```

### Build from Source

```bash
cargo install --git https://github.com/AriajSarkar/gitignore-gen
```

**Minimum Supported Rust Version (MSRV):** 1.70.0

## Usage

```bash
# Generate .gitignore in current directory
gitignore-gen
```

## How It Works

1. **Scan** - Analyzes your project directory for technology markers
2. **Detect** - Identifies languages and frameworks from file patterns
3. **Fetch** - Downloads official templates from [github/gitignore](https://github.com/github/gitignore)
4. **Generate** - Combines templates and writes `.gitignore`

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

## Uninstallation

**Windows:**
```powershell
Remove-Item C:\Windows\System32\gitignore-gen.exe
```

**macOS/Linux:**
```bash
sudo rm /usr/local/bin/gitignore-gen
```

## License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
