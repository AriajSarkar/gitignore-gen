$ErrorActionPreference = "Stop"

$Owner = "AriajSarkar"
$Repo = "gitignore-gen"
$AppName = "gitignore-gen"
$InstallDir = "$env:USERPROFILE\.gitignore-gen\bin"
$ExePath = "$InstallDir\$AppName.exe"

# 1. Determine Architecture
$Arch = if ([System.Environment]::Is64BitOperatingSystem) { "x86_64" } else { "aarch64" }
$AssetPattern = "$Arch-windows"

Write-Host "Installing $AppName for Windows ($Arch)..." -ForegroundColor Cyan

# 2. Get Latest Release Info from GitHub API
$ReleasesUrl = "https://api.github.com/repos/$Owner/$Repo/releases/latest"
try {
    Write-Host "Fetching latest release info..."
    # Use TLS 1.2+
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    $Response = Invoke-RestMethod -Uri $ReleasesUrl -UseBasicParsing
    $TagName = $Response.tag_name
    
    # Find matching asset
    $Asset = $Response.assets | Where-Object { $_.name -like "*$AssetPattern*" } | Select-Object -First 1
    
    if (-not $Asset) {
        throw "No release binary found for your platform ($AssetPattern) in release $TagName"
    }
    
    $DownloadUrl = $Asset.browser_download_url
    Write-Host "Found version: $TagName" -ForegroundColor Green
} catch {
    Write-Error "Failed to fetch release info: $_"
    exit 1
}

# 3. Prepare Install Directory
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

# 4. Download Binary
Write-Host "Downloading from: $DownloadUrl"
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ExePath
} catch {
    Write-Error "Download failed: $_"
    exit 1
}

# 5. Add to PATH
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    Write-Host "Adding $InstallDir to User PATH..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    $env:Path += ";$InstallDir"
    Write-Host "Path updated. Restart your terminal to use '$AppName' everywhere." -ForegroundColor Yellow
} else {
    Write-Host "Path already configured." -ForegroundColor Green
}

Write-Host "`nSuccessfully installed $AppName $TagName!" -ForegroundColor Green
Write-Host "Location: $ExePath"
