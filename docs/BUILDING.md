# Building from Source

## Prerequisites

- [Rust](https://rustup.rs/) 1.83.0 or newer
- Git with submodule support

## Clone

```bash
# Clone with submodules (required for templates)
git clone --recurse-submodules https://github.com/AriajSarkar/gitignore-gen.git
cd gitignore-gen

# Or if already cloned
git submodule update --init
```

## Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

## Install

```bash
# Install to cargo bin
cargo install --path .
```

## Test

```bash
cargo test
```

## Update Templates

Templates are linked from [github/gitignore](https://github.com/github/gitignore) via Git submodule.

```bash
# Update to latest templates
git submodule update --remote templates
```
