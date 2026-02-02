# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Tabbed terminal interface with multiple SSH sessions
- SSH connection support (password, key-based, agent authentication)
- Encrypted session storage using AES-256-GCM
- OS keyring integration for encryption keys
- Customizable themes (Dark, Light, Dracula)
- Session management (save/load/delete connections)
- Keyboard shortcuts (Ctrl/Cmd+T, W, N, ,)
- Cross-platform support (Windows, macOS, Linux)
- GitHub Actions CI/CD workflows for automated builds
- Nightly builds for latest development version

### Security
- AES-256-GCM encryption for all saved credentials
- Encryption keys stored in OS-native keyring
- No plaintext credential storage
- Host key verification support
- Strict file permissions on configuration files

## [0.1.0] - 2026-02-02

### Added
- Initial release of SSH Terminal
- Basic SSH connection functionality
- Terminal interface using xterm.js
- Cross-platform Tauri application
- Documentation and setup guides

[Unreleased]: https://github.com/YOUR_USERNAME/ssh-terminal/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/YOUR_USERNAME/ssh-terminal/releases/tag/v0.1.0
