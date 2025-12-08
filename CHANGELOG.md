# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
and [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

## [2.0.1] - 2025-12-09

### 🐛 Bug Fixes

- Fixed templates not embedded in release builds (missing submodules checkout)
- Fixed install scripts to extract archives (zip/tar.gz) before installing
- Fixed asset pattern matching to use Rust target triples

## [2.0.0] - 2025-12-09

### 🚀 Features

- Complete Rust rewrite of gitignore-gen CLI
- Embedded templates from github/gitignore submodule (no network required)
- Automatic project analysis for technology detection (28+ technologies)
- Self-update command (`gitignore-gen update`)
- Uninstall command (`gitignore-gen uninstall`)
- Custom template selection (`gitignore-gen rust node python`)
- One-line installers for Windows/Linux/macOS
- Multi-platform binaries (x86_64, aarch64)

### 🔒 Security

- SHA-256 checksum verification for updates
- Command injection prevention in uninstall
- WOW64-aware architecture detection

### 📚 Documentation

- README with installation and usage instructions
- Dual MIT/Apache-2.0 licensing
