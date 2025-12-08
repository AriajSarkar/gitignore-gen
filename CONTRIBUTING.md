# Contributing to gitignore-gen

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable, MSRV: 1.75.0)
- Git

### Getting Started

```bash
# Clone the repository
git clone https://github.com/AriajSarkar/gitignore-gen.git
cd gitignore-gen

# Build the project
cargo build

# Run tests
cargo test

# Run the CLI
cargo run
```

## Code Style

This project uses standard Rust tooling for code quality:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

All code must pass `cargo fmt --check` and `cargo clippy` before merging.

## Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Formatting, no code change |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `perf` | Performance improvement |
| `test` | Adding or updating tests |
| `chore` | Maintenance tasks |

### Examples

```
feat(analyzer): add TypeScript detection
fix(fetcher): handle API rate limiting
docs: update installation instructions
```

## Pull Request Process

1. **Fork** the repository and create your branch from `dev`
2. **Make changes** following our code style guidelines
3. **Add tests** for new functionality
4. **Update documentation** if needed
5. **Submit PR** against the `dev` branch

### PR Checklist

- [ ] Code compiles without warnings (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt --check`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] Commit messages follow Conventional Commits
- [ ] Documentation updated if needed

## Reporting Issues

### Bug Reports

Please include:
- OS and version
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior

### Feature Requests

Please describe:
- The problem you're trying to solve
- Your proposed solution
- Alternative approaches considered

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).
