//! Self-update from GitHub releases.
//!
//! Downloads and installs the latest binary directly from GitHub releases
//! without requiring cargo or any build tools.

use serde::Deserialize;
use std::{env, fs, io::Write, path::PathBuf};

/// Repository information parsed from Cargo.toml at compile time.
const REPO: &str = env!("CARGO_PKG_REPOSITORY");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// GitHub API response for a release.
#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

/// GitHub API response for a release asset.
#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

/// Check for updates and install the latest version.
pub fn update() -> Result<(), String> {
    println!("Current version: v{VERSION}");
    println!("Checking for updates...\n");

    let (owner, repo) = parse_repo_url(REPO)?;
    let release = fetch_latest_release(&owner, &repo)?;
    let latest = release.tag_name.trim_start_matches('v');

    if latest == VERSION {
        println!("✓ Already on the latest version!");
        return Ok(());
    }

    println!("  New version available: v{VERSION} → {}", release.tag_name);

    let platform = Platform::detect()?;
    let asset = find_asset(&release.assets, &platform)?;

    println!("  Downloading {}...", asset.name);
    let binary = download(&asset.browser_download_url)?;

    println!("  Installing...");
    install_binary(&binary)?;

    println!("\n✓ Updated to {}!", release.tag_name);
    Ok(())
}

// ============================================================================
// Platform Detection
// ============================================================================

struct Platform {
    os: &'static str,
    arch: &'static str,
}

impl Platform {
    fn detect() -> Result<Self, String> {
        let os = match () {
            _ if cfg!(target_os = "windows") => "windows",
            _ if cfg!(target_os = "macos") => "darwin",
            _ if cfg!(target_os = "linux") => "linux",
            _ => return Err("Unsupported operating system".into()),
        };

        let arch = match () {
            _ if cfg!(target_arch = "x86_64") => "x86_64",
            _ if cfg!(target_arch = "aarch64") => "aarch64",
            _ => return Err("Unsupported architecture".into()),
        };

        Ok(Self { os, arch })
    }

    fn matches(&self, asset_name: &str) -> bool {
        let name = asset_name.to_lowercase();
        name.contains(self.os) && name.contains(self.arch)
    }
}

// ============================================================================
// GitHub API
// ============================================================================

fn parse_repo_url(url: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> =
        url.trim_end_matches('/').trim_end_matches(".git").rsplit('/').take(2).collect();

    match parts.as_slice() {
        [repo, owner] => Ok((owner.to_string(), repo.to_string())),
        _ => Err("Invalid repository URL in Cargo.toml".into()),
    }
}

fn fetch_latest_release(owner: &str, repo: &str) -> Result<Release, String> {
    let url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");

    let response = http_client()?.get(&url).send().map_err(|e| format!("Network error: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("GitHub API error: {} - No releases found?", response.status()));
    }

    response.json::<Release>().map_err(|e| format!("Failed to parse release: {e}"))
}

fn find_asset<'a>(assets: &'a [Asset], platform: &Platform) -> Result<&'a Asset, String> {
    assets.iter().find(|a| platform.matches(&a.name)).ok_or_else(|| {
        format!(
            "No binary found for {}-{}. Available: {}",
            platform.arch,
            platform.os,
            assets.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", ")
        )
    })
}

fn download(url: &str) -> Result<Vec<u8>, String> {
    let response = http_client()?.get(url).send().map_err(|e| format!("Download failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("Download error: {}", response.status()));
    }

    response.bytes().map(|b| b.to_vec()).map_err(|e| format!("Failed to read binary: {e}"))
}

fn http_client() -> Result<reqwest::blocking::Client, String> {
    reqwest::blocking::Client::builder()
        .user_agent(concat!("gitignore-gen/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))
}

// ============================================================================
// Installation
// ============================================================================

fn install_binary(binary: &[u8]) -> Result<(), String> {
    let current = env::current_exe().map_err(|e| format!("Cannot find current exe: {e}"))?;

    #[cfg(windows)]
    windows_install(binary, &current)?;

    #[cfg(unix)]
    unix_install(binary, &current)?;

    Ok(())
}

#[cfg(windows)]
fn windows_install(binary: &[u8], target: &PathBuf) -> Result<(), String> {
    let backup = target.with_extension("exe.old");

    // Remove old backup, rename current, write new, cleanup
    let _ = fs::remove_file(&backup);
    fs::rename(target, &backup).map_err(|e| format!("Backup failed: {e}"))?;

    fs::File::create(target)
        .and_then(|mut f| f.write_all(binary))
        .map_err(|e| format!("Install failed: {e}"))?;

    let _ = fs::remove_file(&backup);
    Ok(())
}

#[cfg(unix)]
fn unix_install(binary: &[u8], target: &PathBuf) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;

    let temp = target.with_extension("new");

    fs::File::create(&temp)
        .and_then(|mut f| f.write_all(binary))
        .map_err(|e| format!("Write failed: {e}"))?;

    fs::set_permissions(&temp, fs::Permissions::from_mode(0o755))
        .map_err(|e| format!("Permission denied: {e}"))?;

    fs::rename(&temp, target).map_err(|e| format!("Install failed: {e}"))?;

    Ok(())
}
