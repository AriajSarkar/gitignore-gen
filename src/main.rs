mod analyzer;
mod fetcher;

use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::process;

#[derive(Parser)]
#[command(name = "gitignore-gen")]
#[command(about = "Generate .gitignore files based on project analysis")]
#[command(
    long_about = "A CLI tool that analyzes your project structure and generates\nappropriate .gitignore files by detecting technologies and frameworks.\nUses the gitignore.io API to fetch the latest templates."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Force overwrite existing .gitignore file
    #[arg(short, long)]
    force: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the binary location for uninstallation
    Uninstall,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Uninstall) => match env::current_exe() {
            Ok(path) => {
                println!("To uninstall, manually delete the binary at: {}", path.display());
                println!("For more information, visit: https://github.com/AriajSarkar/gitignore-gen#uninstallation");
            }
            Err(e) => {
                eprintln!("Failed to get executable path: {}", e);
                process::exit(1);
            }
        },
        None => {
            // Main command: analyze and generate .gitignore
            let path = match env::current_dir() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to get current directory: {}", e);
                    process::exit(1);
                }
            };

            let detected = analyzer::analyze_project(&path);
            if detected.is_empty() {
                eprintln!("No supported technologies detected in the project");
                process::exit(1);
            }

            let content = match fetcher::fetch_gitignore_template(&detected) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to fetch gitignore template: {}", e);
                    process::exit(1);
                }
            };

            if let Err(e) = fs::write(".gitignore", &content) {
                eprintln!("Failed to write .gitignore file: {}", e);
                process::exit(1);
            }

            println!("Generated .gitignore file for: {}", detected.join(", "));
        }
    }
}
