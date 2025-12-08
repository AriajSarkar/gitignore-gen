# Architecture

## Module Structure

```
src/
├── main.rs           # CLI entry point (clap)
├── commands/
│   ├── mod.rs        # Command exports
│   ├── generate.rs   # Generate .gitignore
│   ├── update.rs     # Self-update binary
│   └── uninstall.rs  # Self-delete binary
├── analyzer.rs       # Project technology detection
└── templates.rs      # Template loader from submodule
```

## Templates

Templates are loaded from the `templates/` Git submodule, which links to [github/gitignore](https://github.com/github/gitignore).

At compile time, templates are embedded using `include_str!()`.

## Detection Flow

```
1. Scan project directory
2. Detect technologies (Cargo.toml → Rust, package.json → Node, etc.)
3. Load matching templates from embedded content
4. Combine templates with headers
5. Write .gitignore
```
