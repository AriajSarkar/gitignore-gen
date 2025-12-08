#!/bin/bash
set -e

OWNER="AriajSarkar"
REPO="gitignore-gen"
APP_NAME="gitignore-gen"
INSTALL_DIR="$HOME/.gitignore-gen/bin"
EXE_PATH="$INSTALL_DIR/$APP_NAME"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing $APP_NAME...${NC}"

# 1. Detect OS and Arch
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)     PLATFORM="linux" ;;
    Darwin)    PLATFORM="darwin" ;;
    MINGW*)    PLATFORM="windows" ;;
    MSYS*)     PLATFORM="windows" ;;
    *)         echo -e "${RED}Unsupported OS: $OS${NC}"; exit 1 ;;
esac

case "$ARCH" in
    x86_64)    ARCH="x86_64" ;;
    aarch64)   ARCH="aarch64" ;;
    arm64)     ARCH="aarch64" ;;
    *)         echo -e "${RED}Unsupported Architecture: $ARCH${NC}"; exit 1 ;;
esac

ASSET_PATTERN="$ARCH-$PLATFORM"
echo -e "Detected platform: $ASSET_PATTERN"

# 2. Get Latest Release URL
API_URL="https://api.github.com/repos/$OWNER/$REPO/releases/latest"
echo "Fetching release info..."

if command -v curl >/dev/null; then
    JSON=$(curl -s $API_URL)
elif command -v wget >/dev/null; then
    JSON=$(wget -qO- $API_URL)
else
    echo -e "${RED}Error: neither curl nor wget found${NC}"
    exit 1
fi

# Extract download URL for the platform (hacky grep parsing to avoid jq dependency)
DOWNLOAD_URL=$(echo "$JSON" | grep -o "browser_download_url.*$ASSET_PATTERN[^\"]*" | cut -d '"' -f 3 | head -n 1)

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}Error: No binary found for $ASSET_PATTERN in latest release${NC}"
    echo "This might be because no release exists yet or it doesn't support your platform."
    exit 1
fi

VERSION=$(echo "$JSON" | grep -o '"tag_name": ".*"' | cut -d '"' -f 4)
echo -e "${GREEN}Found version: $VERSION${NC}"

# 3. Install
echo "Creating install directory: $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"

echo "Downloading from: $DOWNLOAD_URL"
if command -v curl >/dev/null; then
    curl -L -o "$EXE_PATH" "$DOWNLOAD_URL"
elif command -v wget >/dev/null; then
    wget -O "$EXE_PATH" "$DOWNLOAD_URL"
fi

chmod +x "$EXE_PATH"

# 4. Update PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Adding $INSTALL_DIR to PATH...${NC}"
    
    SHELL_RC=""
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_RC="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        SHELL_RC="$HOME/.bashrc"
    elif [ -f "$HOME/.profile" ]; then
        SHELL_RC="$HOME/.profile"
    fi

    if [ -n "$SHELL_RC" ]; then
        echo "" >> "$SHELL_RC"
        echo "# gitignore-gen" >> "$SHELL_RC"
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_RC"
        echo -e "${GREEN}Added to $SHELL_RC. Restart your shell or run: source $SHELL_RC${NC}"
    else
        echo -e "${YELLOW}Could not detect shell config. Add this manually:${NC}"
        echo "export PATH=\"\$PATH:$INSTALL_DIR\""
    fi
else
    echo -e "${GREEN}Path already configured.${NC}"
fi

echo -e "${GREEN}Successfully installed $APP_NAME $VERSION!${NC}"
echo "Location: $EXE_PATH"
