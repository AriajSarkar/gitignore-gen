use std::env;

#[cfg(unix)]
use std::fs;

/// Uninstall the binary by deleting itself
pub fn uninstall() -> Result<(), String> {
    let exe_path =
        env::current_exe().map_err(|e| format!("Failed to get executable path: {}", e))?;

    println!("Uninstalling gitignore-gen from: {}", exe_path.display());

    #[cfg(unix)]
    {
        // On Unix, we can delete ourselves directly
        fs::remove_file(&exe_path).map_err(|e| format!("Failed to remove binary: {}", e))?;
        println!("Uninstalled successfully!");
    }

    #[cfg(windows)]
    {
        // On Windows, schedule deletion after process exits
        // Using a simple approach: spawn a delayed delete command
        use std::process::Command;

        let exe_str = exe_path.to_string_lossy();
        let script = format!("Start-Sleep -Seconds 1; Remove-Item -Force '{}'", exe_str);

        Command::new("powershell")
            .args(["-WindowStyle", "Hidden", "-Command", &script])
            .spawn()
            .map_err(|e| format!("Failed to schedule removal: {}", e))?;

        println!("Uninstall scheduled. The binary will be removed shortly.");
    }

    Ok(())
}
