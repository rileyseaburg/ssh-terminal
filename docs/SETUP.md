# Setup Guide

Complete guide for setting up, building, and running the SSH Terminal application.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Development](#development)
- [Building](#building)
- [Configuration](#configuration)
- [First Connection](#first-connection)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### Required Software

1. **Rust** (1.75 or later)
   - Install from [rustup.rs](https://rustup.rs/)
   - Verify: `rustc --version`

2. **Node.js** (18 or later)
   - Install from [nodejs.org](https://nodejs.org/)
   - Verify: `node --version`

3. **Git**
   - Install from [git-scm.com](https://git-scm.com/)
   - Verify: `git --version`

### Platform-Specific Requirements

#### Windows

- **Microsoft Visual Studio Build Tools** or **Visual Studio 2022**
  - Install "Desktop development with C++" workload
  - Or install via `winget install Microsoft.VisualStudio.2022.BuildTools`

- **WebView2 Runtime**
  - Usually pre-installed on Windows 10/11
  - Download from [Microsoft Edge WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

#### macOS

- **Xcode Command Line Tools**
  ```bash
  xcode-select --install
  ```

- **Homebrew** (optional but recommended)
  ```bash
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  ```

#### Linux

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**Fedora:**
```bash
sudo dnf install -y webkit2gtk4.0-devel \
    openssl-devel \
    curl \
    wget \
    libappindicator-gtk3-devel \
    librsvg2-devel
```

**Arch Linux:**
```bash
sudo pacman -S --needed webkit2gtk-4.1 \
    base-devel \
    curl \
    wget \
    openssl \
    libappindicator-gtk3 \
    librsvg
```

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/ssh-terminal.git
cd ssh-terminal
```

### 2. Install Tauri CLI

```bash
cargo install tauri-cli
```

### 3. Build the Rust Backend

```bash
cd src-tauri
cargo build
```

This will download and compile all Rust dependencies.

## Development

### Running in Development Mode

From the project root:

```bash
cd src-tauri
cargo tauri dev
```

This will:
1. Compile the Rust backend
2. Start the Tauri development server
3. Open the application window
4. Enable hot-reload for frontend changes

### Development Features

- **Hot Reload**: Frontend changes are automatically reflected
- **DevTools**: Developer tools are available in development mode
- **Error Logging**: Detailed error messages in console

### Project Structure

```
ssh-terminal/
├── src-tauri/           # Rust backend code
│   ├── src/
│   │   ├── main.rs      # Application entry point
│   │   ├── ssh.rs       # SSH connection handling
│   │   ├── crypto.rs    # Encryption/decryption
│   │   ├── session.rs   # Session management
│   │   └── config.rs    # Configuration handling
│   ├── Cargo.toml       # Rust dependencies
│   └── tauri.conf.json  # Tauri configuration
├── src-ui/              # Frontend code
│   ├── index.html       # Main HTML
│   ├── css/
│   │   └── styles.css   # Application styles
│   └── js/
│       └── app.js       # Frontend JavaScript
├── docs/                # Documentation
└── README.md           # Main readme
```

## Building

### Build for Current Platform

```bash
cd src-tauri
cargo tauri build
```

Output locations:
- **Windows**: `src-tauri/target/release/`
- **macOS**: `src-tauri/target/release/`
- **Linux**: `src-tauri/target/release/`

### Build for Distribution

#### Windows

```bash
cargo tauri build --target x86_64-pc-windows-msvc
```

Creates:
- `.msi` installer
- `.exe` portable executable

#### macOS

```bash
# Intel Macs
cargo tauri build --target x86_64-apple-darwin

# Apple Silicon (M1/M2)
cargo tauri build --target aarch64-apple-darwin

# Universal binary (both architectures)
cargo tauri build --target universal-apple-darwin
```

Creates:
- `.dmg` disk image
- `.app` application bundle

#### Linux

```bash
cargo tauri build --target x86_64-unknown-linux-gnu
```

Creates:
- `.deb` package (Debian/Ubuntu)
- `.AppImage` portable executable
- `.rpm` package (Fedora/Red Hat)

### Code Signing

#### Windows

1. Obtain a code signing certificate
2. Set environment variable:
   ```powershell
   $env:TAURI_SIGNING_PRIVATE_KEY = "path/to/private.key"
   ```
3. Build with signing:
   ```bash
   cargo tauri build
   ```

#### macOS

1. Enroll in Apple Developer Program
2. Install signing certificate
3. Update `tauri.conf.json`:
   ```json
   {
     "tauri": {
       "bundle": {
         "macOS": {
           "signingIdentity": "Developer ID Application: Your Name"
         }
       }
     }
   }
   ```

## Configuration

### First Run Setup

On first launch, the application creates configuration directories:

- **Windows**: `%APPDATA%\com.sshterminal.app\`
- **macOS**: `~/Library/Application Support/com.sshterminal.app/`
- **Linux**: `~/.config/ssh-terminal/`

### Configuration Files

#### config.json

Application settings file:

```json
{
  "theme": "dark",
  "font_size": 14,
  "font_family": "JetBrains Mono, monospace",
  "cursor_style": "block",
  "bell_enabled": false,
  "copy_on_select": true,
  "scrollback_lines": 10000,
  "window_opacity": 1.0,
  "security": {
    "auto_lock_timeout": 300,
    "require_password_on_startup": false,
    "ssh_key_passphrase_cache": false,
    "verify_host_keys": true,
    "strict_host_key_checking": true
  }
}
```

#### sessions.json

Encrypted saved sessions (DO NOT EDIT MANUALLY).

### Environment Variables

- `SSH_TERMINAL_CONFIG_DIR`: Override config directory path
- `RUST_LOG`: Set logging level (debug, info, warn, error)
- `SSH_AUTH_SOCK`: SSH agent socket path (Linux/macOS)

## First Connection

### 1. Generate SSH Key (Recommended)

```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
```

### 2. Copy Public Key to Server

```bash
ssh-copy-id -i ~/.ssh/id_ed25519.pub user@server
```

### 3. Connect in Application

1. Open SSH Terminal
2. Click **+** or press `Ctrl/Cmd + N`
3. Fill in:
   - Host: Your server IP or hostname
   - Port: 22 (or your custom SSH port)
   - Username: Your username
   - Authentication: SSH Key
   - Key Path: `~/.ssh/id_ed25519`
4. Click **Connect**

### 4. Save Session

1. Enter a session name
2. Click **Connect**
3. Session will be automatically saved for quick access

## Troubleshooting

### Build Errors

**Error: "linker 'link.exe' not found" (Windows)**

Install Visual Studio Build Tools with C++ workload.

**Error: "pkg-config not found" (Linux)**

```bash
# Ubuntu/Debian
sudo apt install pkg-config

# Fedora
sudo dnf install pkgconfig

# Arch
sudo pacman -S pkgconf
```

**Error: "cannot find -lwebkit2gtk"**

Install WebKit development libraries (see Linux prerequisites).

### Runtime Errors

**"Failed to create cipher"**

Encryption key initialization failed. Try:
1. Delete config directory
2. Restart application
3. Re-enter saved sessions

**"Connection refused"**

- Verify SSH service is running on server: `sudo systemctl status sshd`
- Check firewall rules: `sudo ufw status` or `sudo iptables -L`
- Verify port is correct

**"Permission denied (publickey)"**

- Ensure public key is in server's `~/.ssh/authorized_keys`
- Check key file permissions: `chmod 600 ~/.ssh/id_ed25519`
- Verify using correct private key

### Performance Issues

**Slow terminal response**

1. Check network latency: `ping server_ip`
2. Reduce scrollback buffer in settings
3. Disable visual bell
4. Use wired connection instead of WiFi

**High CPU usage**

1. Check for infinite loops in custom scripts
2. Reduce terminal output frequency
3. Close unused tabs

### Platform-Specific Issues

**macOS: "App is damaged and can't be opened"**

This is Gatekeeper. Solutions:
1. Right-click app → Open
2. Or: `xattr -cr /Applications/SSH\ Terminal.app`

**Windows: SmartScreen warning**

Click "More info" → "Run anyway" (if you trust the source).

**Linux: App doesn't launch**

Check dependencies:
```bash
ldd ./ssh-terminal | grep "not found"
```

## Updating

### Automatic Updates

The application currently doesn't have automatic updates enabled. Check the GitHub releases page for new versions.

### Manual Update

1. Backup your configuration:
   ```bash
   cp -r ~/.config/ssh-terminal ~/.config/ssh-terminal-backup
   ```

2. Pull latest changes:
   ```bash
   git pull origin main
   ```

3. Rebuild:
   ```bash
   cd src-tauri
   cargo build --release
   ```

## Getting Help

- **Documentation**: Check `docs/` directory
- **Issues**: [GitHub Issues](https://github.com/yourusername/ssh-terminal/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/ssh-terminal/discussions)
- **Security**: See [SECURITY.md](SECURITY.md) for reporting security issues

---

**Last Updated**: 2026-02-02
**Version**: 0.1.0
