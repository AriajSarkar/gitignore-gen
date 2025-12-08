$ErrorActionPreference = "Stop"

$AppName = "gitignore-gen"
$InstallDir = "$env:USERPROFILE\.gitignore-gen\bin"
$ExePath = "$InstallDir\$AppName.exe"

Write-Host "Building $AppName (release)..." -ForegroundColor Cyan
cargo build --release

if (-not (Test-Path "target\release\$AppName.exe")) {
    Write-Error "Build failed: target\release\$AppName.exe not found."
}

Write-Host "Creating install directory: $InstallDir"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

Write-Host "Installing binary..."
Copy-Item -Force "target\release\$AppName.exe" $ExePath

# Check PATH
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    Write-Host "Adding $InstallDir to User PATH..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    $env:Path += ";$InstallDir"
    Write-Host "Path updated. You may need to restart your terminal for changes to take effect." -ForegroundColor Yellow
} else {
    Write-Host "Path already configured." -ForegroundColor Green
}

Write-Host "`nSuccessfully installed $AppName!" -ForegroundColor Green
Write-Host "Location: $ExePath"
