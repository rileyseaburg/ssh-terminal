# SSH Terminal

[![Test](https://github.com/rileyseaburg/ssh-terminal/workflows/Test/badge.svg)](https://github.com/rileyseaburg/ssh-terminal/actions)
[![Release](https://github.com/rileyseaburg/ssh-terminal/workflows/Release/badge.svg)](https://github.com/rileyseaburg/ssh-terminal/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A cross-platform, secure SSH terminal application built with Tauri. Features tabbed terminals, session management, customizable themes, and robust security with encrypted credential storage.

## Features

- **Tabbed Terminal Interface**: Manage multiple SSH sessions in tabs
- **Secure SSH Connections**: Support for password, SSH key, and SSH agent authentication
- **Encrypted Credential Storage**: AES-256-GCM encryption for saved session credentials
- **Session Management**: Save and quickly reconnect to frequently used servers
- **Customizable Themes**: Dark, light, and Dracula themes with customizable colors
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Lightweight**: Minimal resource usage with native performance
- **Keyboard Shortcuts**: Efficient workflow with customizable shortcuts

## Security Features

- **AES-256-GCM Encryption**: All saved credentials are encrypted using industry-standard encryption
- **Keyring Integration**: Encryption keys are stored in the OS keyring (Windows Credential Manager, macOS Keychain, Linux Secret Service)
- **SSH Key Support**: Connect using private keys with optional passphrase protection
- **Host Key Verification**: Optional strict host key checking for enhanced security
- **File Permissions**: Session files are created with restricted permissions (600) on Unix systems
- **No Credential Logging**: Passwords and keys are never logged or exposed in error messages

## Download

Download the latest release for your platform:

| Platform | Download |
|----------|----------|
| Windows (x64) | [SSH-Terminal-Setup.exe](https://github.com/rileyseaburg/ssh-terminal/releases/latest) |
| macOS (Apple Silicon) | [SSH-Terminal-aarch64.dmg](https://github.com/rileyseaburg/ssh-terminal/releases/latest) |
| macOS (Intel) | [SSH-Terminal-x86_64.dmg](https://github.com/rileyseaburg/ssh-terminal/releases/latest) |
| Linux (x64) | [ssh-terminal_amd64.deb](https://github.com/rileyseaburg/ssh-terminal/releases/latest) |
| Linux (AppImage) | [ssh-terminal.AppImage](https://github.com/rileyseaburg/ssh-terminal/releases/latest) |

> **Note**: Replace `rileyseaburg` with your actual GitHub username after forking/cloning.

### Nightly Builds

Get the latest development builds from the [Nightly Releases](https://github.com/rileyseaburg/ssh-terminal/releases/tag/nightly).

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (1.75 or later)
- [Node.js](https://nodejs.org/) (18 or later)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/ssh-terminal.git
cd ssh-terminal
```

2. Install dependencies:
```bash
cd src-tauri
cargo build
cd ..
```

3. Run in development mode:
```bash
cd src-tauri
cargo tauri dev
```

4. Build for production:
```bash
cd src-tauri
cargo tauri build
```

## Usage

### Connecting to a Server

1. Click the **+** button or press `Ctrl/Cmd + N`
2. Fill in the connection details:
   - **Host**: Server IP address or hostname
   - **Port**: SSH port (default: 22)
   - **Username**: Your SSH username
   - **Authentication**: Password, SSH Key, or SSH Agent
3. Click **Connect**

### Saving Sessions

1. Enter connection details in the connection panel
2. Provide a session name
3. Click **Connect** - the session will be automatically saved
4. Or click **Load Saved** to view and manage saved sessions

### Managing Tabs

- **New Tab**: Click the **+** button or press `Ctrl/Cmd + T`
- **Close Tab**: Click the × on the tab or press `Ctrl/Cmd + W`
- **Switch Tabs**: Click on the tab or use `Ctrl/Cmd + Tab`

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl/Cmd + T` | New tab |
| `Ctrl/Cmd + W` | Close tab |
| `Ctrl/Cmd + N` | New connection |
| `Ctrl/Cmd + ,` | Settings |

### Customizing Appearance

1. Click the settings (gear) icon or press `Ctrl/Cmd + ,`
2. Choose from available themes: Dark, Light, or Dracula
3. Adjust font size, font family, and cursor style
4. Set window opacity for a transparent effect

## Configuration

Configuration files are stored in:

- **Windows**: `%APPDATA%\com.sshterminal.app\`
- **macOS**: `~/Library/Application Support/com.sshterminal.app/`
- **Linux**: `~/.config/ssh-terminal/`

### Files

- `config.json`: Application settings (themes, fonts, security options)
- `sessions.json`: Encrypted saved session configurations

## Building from Source

### Development

```bash
# Install Tauri CLI
cargo install tauri-cli

# Run development server
cargo tauri dev
```

### Production Build

```bash
# Build for current platform
cargo tauri build

# Build for specific target
cargo tauri build --target x86_64-pc-windows-msvc
cargo tauri build --target x86_64-apple-darwin
cargo tauri build --target x86_64-unknown-linux-gnu
```

### Cross-Compilation

See [Tauri Cross-Compilation Guide](https://tauri.app/v1/guides/building/cross-platform/)

## Security Considerations

### SSH Keys

- Store private keys in the default SSH directory (`~/.ssh/`)
- Use strong passphrases for encrypted keys
- Set proper file permissions: `chmod 600 ~/.ssh/id_rsa`

### Firewall

The application itself doesn't require incoming firewall rules. Ensure your SSH server:
- Uses non-standard ports (not 22) for production servers
- Has fail2ban or similar intrusion prevention
- Requires key-based authentication for production

### Credential Storage

- Credentials are encrypted using AES-256-GCM
- Encryption keys are stored in the OS keyring
- Never share or commit configuration files containing credentials

## Troubleshooting

### Connection Issues

**"Connection refused"**
- Verify the host and port are correct
- Check if the SSH service is running on the server
- Verify firewall rules allow SSH connections

**"Authentication failed"**
- Double-check username and password
- For key authentication, verify the key path is correct
- Ensure the public key is added to `~/.ssh/authorized_keys` on the server

**"Host key verification failed"**
- This is a security feature
- Manually connect via command line SSH first to accept the host key
- Or disable strict host key checking in Settings (not recommended for production)

### Display Issues

**Terminal not rendering correctly**
- Try adjusting font size in settings
- Check if your graphics drivers are up to date
- Disable hardware acceleration if needed

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## CI/CD

This project uses GitHub Actions for automated testing and releases:

### Automated Workflows

- **Test** - Runs on every PR and push to main (formatting, linting, tests, security audit)
- **Release** - Creates production builds when version tags are pushed
- **Nightly** - Builds development versions on every push to main
- **Dependencies** - Weekly dependency updates with security audits

### Creating a Release

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create and push a tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
4. GitHub Actions will automatically:
   - Build for all platforms
   - Create a draft release
   - Upload artifacts
   - Publish when ready

### Local Development

Use the provided build scripts:

```bash
# Linux/macOS
./scripts/build.sh --release

# Windows
.\scripts\build.ps1 --release
```

See [docs/CI_CD.md](docs/CI_CD.md) for detailed CI/CD documentation.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/)
- Terminal emulator powered by [xterm.js](https://xtermjs.org/)
- SSH functionality via [libssh2](https://www.libssh2.org/)

## Support

For issues, questions, or feature requests, please open an issue on GitHub.
