mod analyzer;
mod commands;
mod templates;

use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "gitignore-gen")]
#[command(version)]
#[command(about = "Generate .gitignore files based on project analysis")]
#[command(long_about = "A CLI tool that analyzes your project structure and generates
appropriate .gitignore files by detecting technologies and frameworks.

Examples:
  gitignore-gen              # Auto-detect and generate
  gitignore-gen rust node    # Generate for specific technologies
  gitignore-gen --list       # Show available templates")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Technologies to include (e.g., rust, node, python)
    #[arg(value_name = "TECH")]
    technologies: Vec<String>,

    /// Force overwrite existing .gitignore file
    #[arg(short, long)]
    force: bool,

    /// List available templates
    #[arg(short, long)]
    list: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Uninstall gitignore-gen (removes the binary)
    Uninstall,
    /// Check for updates
    Update,
}

fn main() {
    let cli = Cli::parse();

    // Handle --list flag
    if cli.list {
        println!("Available templates:");
        for template in templates::list_templates() {
            println!("  - {}", template);
        }
        return;
    }

    let result = match &cli.command {
        Some(Commands::Uninstall) => commands::uninstall(),
        Some(Commands::Update) => commands::update(),
        None => commands::generate(cli.force, &cli.technologies),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
