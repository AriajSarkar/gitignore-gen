#!/bin/bash
set -e

APP_NAME="gitignore-gen"
INSTALL_DIR="$HOME/.gitignore-gen/bin"
EXE_PATH="$INSTALL_DIR/$APP_NAME"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
d='\033[0m'

echo -e "${GREEN}Building $APP_NAME (release)...${NC}"
cargo build --release

echo -e "Creating install directory: $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"

echo -e "Installing binary..."
cp "target/release/$APP_NAME" "$EXE_PATH"
chmod +x "$EXE_PATH"

# Check PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Adding $INSTALL_DIR to PATH...${NC}"
    
    SHELL_RC=""
    if [ -f "$HOME/.zshrc" ]; then
        SHELL_RC="$HOME/.zshrc"
    elif [ -f "$HOME/.bashrc" ]; then
        SHELL_RC="$HOME/.bashrc"
    fi

    if [ -n "$SHELL_RC" ]; then
        echo "" >> "$SHELL_RC"
        echo "# gitignore-gen" >> "$SHELL_RC"
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_RC"
        echo -e "${GREEN}Added to $SHELL_RC. Please restart your shell or run source $SHELL_RC${NC}"
    else
        echo -e "${YELLOW}Could not detect shell RC file. Please add the following to your PATH:${NC}"
        echo "export PATH=\"\$PATH:$INSTALL_DIR\""
    fi
else
    echo -e "${GREEN}Path already configured.${NC}"
fi

echo -e "${GREEN}Successfully installed $APP_NAME!${NC}"
echo "Location: $EXE_PATH"
