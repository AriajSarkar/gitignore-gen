# gitignore-gen installer
# Usage: iwr https://github.com/AriajSarkar/gitignore-gen/raw/master/scripts/install.ps1 -useb | iex
$ErrorActionPreference = "Stop"

$Owner = "AriajSarkar"
$Repo = "gitignore-gen"
$App = "gitignore-gen"
$Dir = "$env:USERPROFILE\.gitignore-gen\bin"

Write-Host "🦀 Installing $App..." -ForegroundColor Cyan

# Detect architecture (handle WOW64 properly)
# PROCESSOR_ARCHITEW6432 is set when running 32-bit process on 64-bit Windows
$RealArch = if ($env:PROCESSOR_ARCHITEW6432) { $env:PROCESSOR_ARCHITEW6432 } else { $env:PROCESSOR_ARCHITECTURE }
$Arch = switch ($RealArch) {
    "AMD64"  { "x86_64" }
    "ARM64"  { "aarch64" }
    "x86"    { Write-Host "❌ 32-bit Windows is not supported" -ForegroundColor Red; exit 1 }
    default  { Write-Host "❌ Unsupported architecture: $RealArch" -ForegroundColor Red; exit 1 }
}

# Get latest release
$Api = "https://api.github.com/repos/$Owner/$Repo/releases/latest"
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
$Release = Invoke-RestMethod -Uri $Api

# Match asset with Windows target triple (e.g., x86_64-pc-windows-msvc)
$Asset = $Release.assets | Where-Object { $_.name -match "$Arch.*windows" -and $_.name -notlike "*.sha256" } | Select-Object -First 1
$ShaAsset = $Release.assets | Where-Object { $_.name -like "*checksums*" -or $_.name -like "*sha256*" } | Select-Object -First 1

if (-not $Asset) {
    Write-Host "❌ No binary found for $Arch on Windows" -ForegroundColor Red
    Write-Host "   Available: $($Release.assets.name -join ', ')"
    exit 1
}

Write-Host "📦 Downloading $($Release.tag_name)..." -ForegroundColor Yellow

# Install directory
if (-not (Test-Path $Dir)) { New-Item -ItemType Directory -Path $Dir -Force | Out-Null }
$ExePath = "$Dir\$App.exe"
Invoke-WebRequest -Uri $Asset.browser_download_url -OutFile $ExePath

# Verify checksum
if ($ShaAsset) {
    Write-Host "🔒 Verifying checksum..." -ForegroundColor Yellow
    try {
        $ChecksumContent = (Invoke-WebRequest -Uri $ShaAsset.browser_download_url -UseBasicParsing).Content
        # Parse checksum file - find line containing our asset name
        $Expected = ($ChecksumContent -split "`n" | Where-Object { $_ -like "*$($Asset.name)*" } | Select-Object -First 1) -split '\s+' | Select-Object -First 1
        if (-not $Expected) {
            # Try first line if single checksum file
            $Expected = ($ChecksumContent.Trim() -split '\s+')[0]
        }
        $Actual = (Get-FileHash -Path $ExePath -Algorithm SHA256).Hash.ToLower()
        if ($Actual -ne $Expected.ToLower()) {
            Remove-Item $ExePath -Force
            Write-Host "❌ Checksum mismatch!" -ForegroundColor Red
            Write-Host "   Expected: $Expected"
            Write-Host "   Actual:   $Actual"
            exit 1
        }
        Write-Host "✓ Verified" -ForegroundColor Green
    } catch {
        Write-Host "⚠️  Checksum verification failed: $_" -ForegroundColor Yellow
    }
} else {
    Write-Host "⚠️  No checksum file found, skipping verification" -ForegroundColor Yellow
}

# Add to PATH (precise matching)
$Path = [Environment]::GetEnvironmentVariable("Path", "User")
$PathEntries = $Path -split ';' | ForEach-Object { $_.Trim().TrimEnd('\') } | Where-Object { $_ -ne '' }
if (-not ($PathEntries -contains $Dir.TrimEnd('\'))) {
    [Environment]::SetEnvironmentVariable("Path", "$Path;$Dir", "User")
    Write-Host "📝 Added to PATH (restart terminal to use)" -ForegroundColor Yellow
}

Write-Host "✅ Installed $App $($Release.tag_name) to $ExePath" -ForegroundColor Green
Write-Host "   Run: $App --help"
