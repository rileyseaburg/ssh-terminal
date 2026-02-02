# CI/CD Workflows

This document describes the GitHub Actions workflows used to build, test, and release the SSH Terminal application.

## Workflows Overview

### 1. Test (`test.yml`)
Runs on every push to `main`/`master` and on pull requests.

**What it does:**
- Runs on Ubuntu, Windows, and macOS
- Checks Rust code formatting with `cargo fmt`
- Runs clippy lints
- Executes Rust unit tests
- Builds the application
- Checks JavaScript formatting with Prettier
- Runs security audit with `cargo audit`

**Triggers:**
- Push to `main` or `master`
- Pull requests to `main` or `master`

### 2. Release (`release.yml`)
Creates a production release when a version tag is pushed.

**What it does:**
- Creates a GitHub release (draft initially)
- Builds the application for:
  - macOS (Apple Silicon - aarch64)
  - macOS (Intel - x86_64)
  - Linux (x86_64)
  - Windows (x86_64)
- Publishes release artifacts
- Signs binaries (if certificates configured)

**Triggers:**
- Push of tags matching `v*` (e.g., `v0.1.0`)
- Manual workflow dispatch with version input

**Usage:**
```bash
# Create and push a new version tag
git tag v0.1.0
git push origin v0.1.0
```

### 3. Nightly Build (`nightly.yml`)
Creates nightly/development builds on every push to `main`.

**What it does:**
- Builds the application for all platforms
- Creates/updates a "nightly" pre-release
- Includes commit SHA in release notes

**Triggers:**
- Every push to `main` or `master`
- Manual workflow dispatch

### 4. Dependency Updates (`dependencies.yml`)
Automatically updates dependencies weekly.

**What it does:**
- Runs `cargo update` every Monday
- Creates a pull request with dependency updates
- Runs `cargo audit` to check for security vulnerabilities

**Triggers:**
- Weekly schedule (Mondays at midnight)
- Manual workflow dispatch

## Required Secrets

To enable code signing and secure releases, add these secrets to your GitHub repository:

### macOS Code Signing (Optional but Recommended)

1. **APPLE_CERTIFICATE**
   - Base64-encoded Apple Developer certificate
   - Export from Keychain: `security find-identity -v -p codesigning`
   - Convert to base64: `base64 -i certificate.p12 | pbcopy`

2. **APPLE_CERTIFICATE_PASSWORD**
   - Password for the certificate file

3. **APPLE_SIGNING_IDENTITY**
   - The signing identity name (e.g., "Developer ID Application: Your Name")

4. **APPLE_ID**
   - Your Apple ID email for notarization

5. **APPLE_PASSWORD**
   - App-specific password for your Apple ID
   - Generate at: appleid.apple.com

6. **APPLE_TEAM_ID**
   - Your Apple Developer Team ID

### Windows Code Signing (Optional)

1. **TAURI_SIGNING_PRIVATE_KEY**
   - Private key for Tauri updater signing
   - Generate: `tauri signer generate`

2. **TAURI_SIGNING_PRIVATE_KEY_PASSWORD**
   - Password for the private key

### GitHub Token

The `GITHUB_TOKEN` secret is automatically provided by GitHub Actions and doesn't need to be manually configured.

## Setting Up Secrets

1. Go to your repository on GitHub
2. Navigate to **Settings** > **Secrets and variables** > **Actions**
3. Click **New repository secret**
4. Add each secret name and value

## Workflow Status Badges

Add these badges to your README.md:

```markdown
![Test](https://github.com/YOUR_USERNAME/ssh-terminal/workflows/Test/badge.svg)
![Release](https://github.com/YOUR_USERNAME/ssh-terminal/workflows/Release/badge.svg)
```

## Local Testing

### Run workflows locally with `act`

Install `act` to test workflows locally:

```bash
# macOS
brew install act

# Linux
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
```

Run a workflow:

```bash
# Test workflow
act push

# Release workflow
act workflow_dispatch -W .github/workflows/release.yml
```

## Build Scripts

Use the provided build scripts for local development:

### Linux/macOS
```bash
# Debug build
./scripts/build.sh

# Release build
./scripts/build.sh --release

# Cross-compile for specific target
./scripts/build.sh --release --target x86_64-pc-windows-msvc
```

### Windows
```powershell
# Debug build
.\scripts\build.ps1

# Release build
.\scripts\build.ps1 --release
```

## Release Checklist

Before creating a release:

- [ ] All tests passing
- [ ] Version bumped in `Cargo.toml`
- [ ] CHANGELOG.md updated
- [ ] Documentation updated
- [ ] Secrets configured (for signed builds)
- [ ] Git tag created with format `vX.Y.Z`

## Troubleshooting

### Build Failures

**"linker 'link.exe' not found" (Windows)**
- Install Visual Studio Build Tools with C++ workload

**"cannot find -lwebkit2gtk" (Linux)**
- Install WebKit development libraries (see SETUP.md)

**"No such file or directory" for icons**
- Ensure icon files exist in `src-tauri/icons/`
- Generate icons: `cargo tauri icon /path/to/icon.png`

### Signing Issues

**macOS: "No valid signing identity"**
- Verify certificate is valid and not expired
- Check APPLE_SIGNING_IDENTITY matches exactly

**Windows: "Certificate not found"**
- Ensure certificate is installed in Windows certificate store
- Check certificate thumbprint in tauri.conf.json

### Workflow Timeouts

The release workflow can take 15-30 minutes. If it times out:
- Check if any step is hanging
- Increase timeout in workflow file if needed
- Consider splitting into more parallel jobs

## Customization

### Add a New Platform

Edit `.github/workflows/release.yml` and add to the matrix:

```yaml
- platform: 'ubuntu-22.04'
  args: '--target aarch64-unknown-linux-gnu'
  arch: 'aarch64'
```

### Change Trigger Events

Modify the `on:` section in workflow files:

```yaml
on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]
```

### Add Custom Build Steps

Add steps to the `build-tauri` job:

```yaml
- name: Custom step
  run: |
    echo "Running custom build step"
    # Your commands here
```

## Security Considerations

- Never commit secrets to the repository
- Use GitHub's encrypted secrets for sensitive data
- Regularly rotate signing certificates
- Review dependency updates before merging
- Enable branch protection on `main`

## Support

For issues with CI/CD workflows:

1. Check the [GitHub Actions documentation](https://docs.github.com/en/actions)
2. Review workflow logs in the Actions tab
3. Open an issue with the workflow name and error message
