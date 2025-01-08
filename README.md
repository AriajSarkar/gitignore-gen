# gitignore-gen

Automatically generate .gitignore files based on project analysis.

## Installation

### Using pre-built binaries (No Go required)

1. Go to [Releases](https://github.com/AriajSarkar/gitignore-gen/releases)
2. Download the binary for your platform:
   - Windows: `gitignore-gen_windows_amd64.exe`
   - Mac: `gitignore-gen_darwin_amd64` or `gitignore-gen_darwin_arm64` (for M1/M2)
   - Linux: `gitignore-gen_linux_amd64`

3. Installation instructions:
   
   Windows:
   ```powershell
   # Move the binary to a directory in your PATH
   move gitignore-gen_windows_amd64.exe C:\Windows\System32\gitignore-gen.exe
   ```

   Mac/Linux:
   ```bash
   # Make it executable
   chmod +x gitignore-gen_*_amd64
   # Move to a directory in your PATH
   sudo mv gitignore-gen_*_amd64 /usr/local/bin/gitignore-gen
   ```

### Option 1: Using Go Install (Requires Go)
```bash
go install github.com/AriajSarkar/gitignore-gen@latest
```

### Option 2: Using pre-built binaries (No Go required)

1. Go to [Releases](https://github.com/AriajSarkar/gitignore-gen/releases)
2. Download the binary for your platform:
   - Windows: `gitignore-gen_windows_amd64.exe`
   - Mac: `gitignore-gen_darwin_amd64` or `gitignore-gen_darwin_arm64` (for M1/M2)
   - Linux: `gitignore-gen_linux_amd64`

3. Installation instructions:
   
   Windows:
   ```powershell
   # Move the binary to a directory in your PATH
   move gitignore-gen_windows_amd64.exe C:\Windows\System32\gitignore-gen.exe
   ```

   Mac/Linux:
   ```bash
   # Make it executable
   chmod +x gitignore-gen_*_amd64
   # Move to a directory in your PATH
   sudo mv gitignore-gen_*_amd64 /usr/local/bin/gitignore-gen
   ```

## Usage

```bash
gitignore-gen
```

## Uninstallation

Windows:
```powershell
# Remove the binary from your PATH
del C:\Windows\System32\gitignore-gen.exe
```

Mac/Linux:
```bash
# Remove the binary from your PATH
sudo rm /usr/local/bin/gitignore-gen
```
