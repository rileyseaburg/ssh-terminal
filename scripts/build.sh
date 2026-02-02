#!/bin/bash
set -e

echo "=== SSH Terminal Build Script ==="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check prerequisites
echo "Checking prerequisites..."

if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Rust not found. Please install Rust from https://rustup.rs/${NC}"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Cargo not found. Please install Rust from https://rustup.rs/${NC}"
    exit 1
fi

if ! command -v node &> /dev/null; then
    echo -e "${RED}Node.js not found. Please install from https://nodejs.org/${NC}"
    exit 1
fi

echo -e "${GREEN}Prerequisites check passed!${NC}"
echo ""

# Parse arguments
BUILD_TYPE="debug"
TARGET=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            BUILD_TYPE="release"
            shift
            ;;
        --target)
            TARGET="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --release       Build release version"
            echo "  --target        Specify target triple (e.g., x86_64-pc-windows-msvc)"
            echo "  --help          Show this help message"
            echo ""
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Install dependencies based on OS
echo "Installing platform dependencies..."
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if command -v apt-get &> /dev/null; then
        echo "Installing dependencies for Ubuntu/Debian..."
        sudo apt-get update
        sudo apt-get install -y libwebkit2gtk-4.0-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
    elif command -v dnf &> /dev/null; then
        echo "Installing dependencies for Fedora..."
        sudo dnf install -y webkit2gtk4.0-devel openssl-devel libappindicator-gtk3-devel librsvg2-devel
    elif command -v pacman &> /dev/null; then
        echo "Installing dependencies for Arch..."
        sudo pacman -S --needed webkit2gtk-4.1 base-devel openssl libappindicator-gtk3 librsvg
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "macOS detected. No additional dependencies needed."
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
    echo "Windows detected. Make sure Visual Studio Build Tools are installed."
fi

echo ""

# Build
cd src-tauri

if [ "$BUILD_TYPE" == "release" ]; then
    echo -e "${GREEN}Building release version...${NC}"
    if [ -n "$TARGET" ]; then
        cargo build --release --target "$TARGET"
    else
        cargo build --release
    fi
    echo ""
    echo -e "${GREEN}Build complete!${NC}"
    echo ""
    echo "Binary location:"
    if [ -n "$TARGET" ]; then
        echo "  target/$TARGET/release/ssh-terminal"
    else
        echo "  target/release/ssh-terminal"
    fi
else
    echo -e "${YELLOW}Building debug version...${NC}"
    if [ -n "$TARGET" ]; then
        cargo build --target "$TARGET"
    else
        cargo build
    fi
    echo ""
    echo -e "${GREEN}Build complete!${NC}"
    echo ""
    echo "To run the application:"
    if [ -n "$TARGET" ]; then
        echo "  ./target/$TARGET/debug/ssh-terminal"
    else
        echo "  ./target/debug/ssh-terminal"
    fi
fi

echo ""
echo "To build for distribution, run:"
echo "  cargo tauri build"
echo ""
