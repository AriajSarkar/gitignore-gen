#!/bin/bash
set -e

APP_NAME="gitignore-gen"
INSTALL_DIR="$HOME/.gitignore-gen/bin"
EXE_PATH="$INSTALL_DIR/$APP_NAME"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}Building $APP_NAME (release)...${NC}"
cargo build --release

echo -e "Creating install directory: $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"

echo -e "Installing binary..."
cp "target/release/$APP_NAME" "$EXE_PATH"
chmod +x "$EXE_PATH"

# Check PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}$INSTALL_DIR is not in PATH${NC}"
    
    # Detect shell RC file (in order of preference)
    SHELL_RC=""
    for rc in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.profile" "$HOME/.config/fish/config.fish"; do
        if [ -f "$rc" ]; then
            SHELL_RC="$rc"
            break
        fi
    done

    if [ -n "$SHELL_RC" ]; then
        EXPORT_LINE="export PATH=\"\$PATH:$INSTALL_DIR\""
        [[ "$SHELL_RC" == *"fish"* ]] && EXPORT_LINE="set -gx PATH \$PATH $INSTALL_DIR"
        
        echo -e "Will add to ${GREEN}$SHELL_RC${NC}:"
        echo -e "  $EXPORT_LINE"
        echo ""
        
        # Check for auto-yes flag or environment variable
        if [[ "$1" == "--yes" || "$INSTALL_FROM_SOURCE_AUTO_YES" == "1" ]]; then
            REPLY="y"
        elif [ -t 0 ]; then
            # Interactive: prompt user
            read -p "Proceed? [y/N] " -n 1 -r
            echo ""
        else
            # Non-interactive without flag: skip
            echo -e "${YELLOW}Non-interactive mode. Use --yes or INSTALL_FROM_SOURCE_AUTO_YES=1 to auto-approve.${NC}"
            echo -e "${YELLOW}Add manually: $EXPORT_LINE${NC}"
            REPLY="n"
        fi
        
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            # Create timestamped backup
            BACKUP="$SHELL_RC.bak.$(date +%Y%m%d%H%M%S)"
            cp "$SHELL_RC" "$BACKUP"
            echo -e "Backup created: $BACKUP"
            
            # Append PATH export
            echo "" >> "$SHELL_RC"
            echo "# gitignore-gen" >> "$SHELL_RC"
            echo "$EXPORT_LINE" >> "$SHELL_RC"
            echo -e "${GREEN}Added to $SHELL_RC. Restart shell or run: source $SHELL_RC${NC}"
        else
            echo -e "${YELLOW}Skipped. Add manually:${NC}"
            echo "  $EXPORT_LINE"
        fi
    else
        echo -e "${YELLOW}Could not detect shell RC file. Add manually:${NC}"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    fi
else
    echo -e "${GREEN}Path already configured.${NC}"
fi

echo -e "${GREEN}Successfully installed $APP_NAME!${NC}"
echo "Location: $EXE_PATH"
