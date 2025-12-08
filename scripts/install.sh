#!/bin/bash
# gitignore-gen installer
# Usage: curl -fsSL https://github.com/AriajSarkar/gitignore-gen/raw/master/scripts/install.sh | bash
set -e

OWNER="AriajSarkar"
REPO="gitignore-gen"
APP="gitignore-gen"
DIR="$HOME/.gitignore-gen/bin"

echo "🦀 Installing $APP..."

# Detect platform - use target triple patterns
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
[[ "$ARCH" == "arm64" ]] && ARCH="aarch64"
# Map to Rust target patterns
[[ "$OS" == "darwin" ]] && OS_PATTERN="apple-darwin" || OS_PATTERN="unknown-linux-gnu"

# Fetch release info with error handling
API="https://api.github.com/repos/$OWNER/$REPO/releases/latest"
if ! JSON=$(curl -fsSL "$API" 2>/dev/null); then
    echo "❌ Failed to fetch release info from GitHub"; exit 1
fi

# Parse JSON - use jq if available, fallback to grep
if command -v jq >/dev/null 2>&1; then
    URL=$(echo "$JSON" | jq -r ".assets[] | select(.name | contains(\"$ARCH-$OS_PATTERN\") and (contains(\".sha256\") | not)) | .browser_download_url" | head -1)
    SHA_URL=$(echo "$JSON" | jq -r ".assets[] | select(.name | (contains(\"$ARCH-$OS_PATTERN\") and contains(\".sha256\")) or contains(\"checksums\")) | .browser_download_url" | head -1)
    VER=$(echo "$JSON" | jq -r ".tag_name")
    ASSET_NAME=$(echo "$JSON" | jq -r ".assets[] | select(.name | contains(\"$ARCH-$OS_PATTERN\") and (contains(\".sha256\") | not)) | .name" | head -1)
else
    URL=$(echo "$JSON" | grep -o "browser_download_url.*$ARCH-$OS_PATTERN[^\"]*" | grep -v ".sha256" | cut -d'"' -f3 | head -1)
    SHA_URL=$(echo "$JSON" | grep -o "browser_download_url.*$ARCH-$OS_PATTERN[^\"]*.sha256" | cut -d'"' -f3 | head -1)
    VER=$(echo "$JSON" | grep -o '"tag_name": "[^"]*"' | cut -d'"' -f4)
    ASSET_NAME=$(basename "$URL" 2>/dev/null || echo "")
fi

# Validate extracted values
if [[ -z "$URL" || ! "$URL" =~ ^https?:// ]]; then
    echo "❌ No binary found for $ARCH-$OS_PATTERN"; exit 1
fi
if [[ -z "$VER" ]]; then
    echo "❌ Could not determine version"; exit 1
fi

echo "📦 Downloading $VER..."
mkdir -p "$DIR"
TEMP_FILE="$DIR/$APP.tmp"

if ! curl -fsSL "$URL" -o "$TEMP_FILE"; then
    echo "❌ Download failed"; exit 1
fi

# Verify checksum if available
if [[ -n "$SHA_URL" && "$SHA_URL" =~ ^https?:// ]]; then
    echo "🔒 Verifying checksum..."
    if CHECKSUM_CONTENT=$(curl -fsSL "$SHA_URL" 2>/dev/null); then
        # Find checksum for our asset or use first line
        if [[ -n "$ASSET_NAME" ]]; then
            EXPECTED=$(echo "$CHECKSUM_CONTENT" | grep "$ASSET_NAME" | awk '{print $1}')
        fi
        [[ -z "$EXPECTED" ]] && EXPECTED=$(echo "$CHECKSUM_CONTENT" | head -1 | awk '{print $1}')
        
        if [[ -n "$EXPECTED" ]]; then
            # Handle both sha256sum (Linux) and shasum (macOS)
            if command -v sha256sum >/dev/null 2>&1; then
                ACTUAL=$(sha256sum "$TEMP_FILE" | awk '{print $1}')
            elif command -v shasum >/dev/null 2>&1; then
                ACTUAL=$(shasum -a 256 "$TEMP_FILE" | awk '{print $1}')
            else
                echo "⚠️  No checksum utility found, skipping verification"
                ACTUAL="$EXPECTED"
            fi
            
            if [[ "${EXPECTED,,}" != "${ACTUAL,,}" ]]; then
                rm -f "$TEMP_FILE"
                echo "❌ Checksum mismatch!"
                echo "   Expected: $EXPECTED"
                echo "   Actual:   $ACTUAL"
                exit 1
            fi
            echo "✓ Verified"
        else
            echo "⚠️  Could not parse checksum file"
        fi
    else
        echo "⚠️  Could not fetch checksum file"
    fi
else
    echo "⚠️  No checksum file found, skipping verification"
fi

# Install
mv "$TEMP_FILE" "$DIR/$APP"
chmod +x "$DIR/$APP"

# Update PATH - use $SHELL to detect user's default shell (not current script runner)
if [[ ":$PATH:" != *":$DIR:"* ]]; then
    # Determine RC file based on user's login shell, not current shell
    USER_SHELL=$(basename "${SHELL:-/bin/bash}")
    RC=""
    
    case "$USER_SHELL" in
        zsh)  [[ -f "$HOME/.zshrc" ]] && RC="$HOME/.zshrc" ;;
        bash) [[ -f "$HOME/.bashrc" ]] && RC="$HOME/.bashrc" || [[ -f "$HOME/.bash_profile" ]] && RC="$HOME/.bash_profile" ;;
        *)    [[ -f "$HOME/.profile" ]] && RC="$HOME/.profile" ;;
    esac
    
    # Fallback: try common RC files if none found
    if [[ -z "$RC" ]]; then
        for f in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.profile"; do
            [[ -f "$f" ]] && RC="$f" && break
        done
    fi
    
    if [[ -n "$RC" ]]; then
        if printf "\nexport PATH=\"\$PATH:$DIR\"" >> "$RC" 2>/dev/null; then
            echo "📝 Added to $RC (restart shell to use)"
        else
            echo "⚠️  Could not update $RC. Add manually:"
            echo "   export PATH=\"\$PATH:$DIR\""
        fi
    else
        echo "⚠️  Could not detect shell config. Add manually:"
        echo "   export PATH=\"\$PATH:$DIR\""
    fi
fi

echo "✅ Installed $APP $VER to $DIR/$APP"
echo "   Run: $APP --help"
