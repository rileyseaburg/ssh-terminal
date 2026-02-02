# Contributing to SSH Terminal

Thank you for your interest in contributing to SSH Terminal! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Submitting Changes](#submitting-changes)
- [Coding Standards](#coding-standards)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [Security Issues](#security-issues)

## Code of Conduct

This project and everyone participating in it is governed by our commitment to:

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Respect different viewpoints and experiences

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Create a new branch for your feature or fix
4. Make your changes
5. Test thoroughly
6. Submit a pull request

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Node.js 18 or later
- Platform-specific dependencies (see [SETUP.md](docs/SETUP.md))

### Setting Up

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/ssh-terminal.git
cd ssh-terminal

# Install dependencies
cd src-tauri
cargo build
cd ..

# Run in development mode
cargo tauri dev
```

### Running Tests

```bash
# Rust tests
cd src-tauri
cargo test

# Formatting check
cargo fmt -- --check

# Linting
cargo clippy -- -D warnings

# Security audit
cargo audit
```

## Submitting Changes

### Types of Contributions

- **Bug fixes**: Fix issues reported in GitHub issues
- **Features**: Add new functionality
- **Documentation**: Improve docs, add examples
- **Tests**: Add or improve test coverage
- **Performance**: Optimize existing code
- **Security**: Security improvements and fixes

### Before Submitting

- [ ] Code follows the coding standards
- [ ] All tests pass
- [ ] Documentation is updated (if needed)
- [ ] CHANGELOG.md is updated (if applicable)
- [ ] Commit messages are clear and descriptive

## Coding Standards

### Rust Code

- Follow the [Rust Style Guide](https://doc.rust-lang.org/style-guide/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Add documentation comments for public APIs
- Write tests for new functionality

Example:
```rust
/// Establishes an SSH connection to the specified host.
/// 
/// # Arguments
/// 
/// * `host` - The hostname or IP address
/// * `port` - The SSH port number
/// 
/// # Returns
/// 
/// Returns a `Result` containing the session ID on success
pub async fn connect(host: String, port: u16) -> Result<String, SshError> {
    // Implementation
}
```

### JavaScript Code

- Use ES6+ features
- Follow existing code style
- Add comments for complex logic
- Use meaningful variable names

Example:
```javascript
/**
 * Creates a new terminal tab with the given session
 * @param {string} sessionId - The SSH session ID
 * @returns {string} The tab ID
 */
function createNewTab(sessionId = null) {
    // Implementation
}
```

### CSS

- Use CSS custom properties (variables) for theming
- Follow BEM naming convention
- Keep selectors simple and specific

## Commit Messages

Use conventional commits format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, semicolons, etc)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Build process or auxiliary tool changes

### Examples

```
feat(ssh): add support for SSH agent authentication

Implements SSH agent forwarding support using the system's
SSH agent. This allows users to authenticate without storing
private keys in the application.

Closes #123
```

```
fix(ui): resolve tab close button alignment issue

The close button was misaligned on high-DPI displays.
Fixed by using relative units instead of fixed pixels.

Fixes #456
```

```
docs(readme): update installation instructions

Added clearer steps for Windows users and troubleshooting
tips for common installation issues.
```

## Pull Request Process

1. **Create a Branch**
   ```bash
   git checkout -b feature/my-new-feature
   # or
   git checkout -b fix/issue-123
   ```

2. **Make Changes**
   - Write code
   - Add tests
   - Update documentation

3. **Test**
   ```bash
   cargo test
   cargo fmt -- --check
   cargo clippy -- -D warnings
   ```

4. **Commit**
   ```bash
   git add .
   git commit -m "feat: add new feature"
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/my-new-feature
   ```
   Then create a pull request on GitHub.

### PR Guidelines

- Provide a clear description of the changes
- Reference any related issues
- Include screenshots for UI changes
- Ensure all CI checks pass
- Respond to review feedback promptly
- Keep PRs focused and reasonably sized

### PR Template

```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Code refactoring

## Testing
Describe the tests you ran

## Screenshots (if applicable)
Add screenshots for UI changes

## Checklist
- [ ] My code follows the coding standards
- [ ] I have performed a self-review
- [ ] I have commented my code
- [ ] I have made corresponding documentation changes
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix/feature works
- [ ] New and existing tests pass
```

## Security Issues

**DO NOT** open a public issue for security vulnerabilities.

Instead:
1. Email security concerns to: security@example.com
2. Include detailed description
3. Provide reproduction steps
4. Allow 90 days for remediation before public disclosure

See [SECURITY.md](docs/SECURITY.md) for more details.

## Questions?

- Check existing [issues](https://github.com/YOUR_USERNAME/ssh-terminal/issues)
- Start a [discussion](https://github.com/YOUR_USERNAME/ssh-terminal/discussions)
- Join our community chat (if applicable)

## Recognition

Contributors will be:
- Listed in the project's README
- Mentioned in release notes
- Added to the contributors graph

Thank you for contributing to SSH Terminal!
