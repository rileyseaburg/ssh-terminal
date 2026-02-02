# Security Guide

This document outlines the security features, best practices, and considerations for using SSH Terminal securely.

## Security Architecture

### Encryption

#### Credential Encryption
- **Algorithm**: AES-256-GCM (Galois/Counter Mode)
- **Key Size**: 256-bit encryption keys
- **Nonce**: 96-bit unique nonce for each encryption operation
- **Authenticated Encryption**: GCM provides both confidentiality and integrity

#### Key Management
- Encryption keys are generated randomly using cryptographically secure random number generation
- Keys are stored in the OS-native keyring:
  - **Windows**: Windows Credential Manager
  - **macOS**: macOS Keychain
  - **Linux**: Secret Service API / libsecret
- Keys are never written to disk in plaintext

### SSH Security

#### Supported Authentication Methods

1. **Password Authentication**
   - Passwords are transmitted securely via SSH protocol
   - Passwords are never stored unless session is saved
   - Saved passwords are encrypted before storage

2. **SSH Key Authentication**
   - Supports RSA, ECDSA, and Ed25519 keys
   - Keys can be password-protected (recommended)
   - Private keys are never uploaded or transmitted
   - Only the key path is stored (if session saved)

3. **SSH Agent Authentication**
   - Uses system SSH agent (ssh-agent, Pageant, etc.)
   - Most secure method - no credentials stored
   - Requires running SSH agent with loaded keys

#### Host Key Verification

- By default, strict host key checking is enabled
- Known hosts are verified against `~/.ssh/known_hosts`
- Man-in-the-middle attack protection
- Option to disable for testing (not recommended for production)

## Best Practices

### SSH Server Configuration

#### On Your Servers

1. **Use Key-Based Authentication**
   ```bash
   # Disable password authentication
   PasswordAuthentication no
   PubkeyAuthentication yes
   ```

2. **Change Default Port**
   ```bash
   # Use non-standard port
   Port 2222
   ```

3. **Disable Root Login**
   ```bash
   PermitRootLogin no
   ```

4. **Use AllowUsers/AllowGroups**
   ```bash
   AllowUsers user1 user2
   ```

5. **Enable Two-Factor Authentication** (optional)
   - Use with PAM modules like Google Authenticator

### SSH Key Management

1. **Generate Strong Keys**
   ```bash
   # Ed25519 (recommended)
   ssh-keygen -t ed25519 -C "your_email@example.com"
   
   # RSA with sufficient bits
   ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
   ```

2. **Use Passphrases**
   - Always protect private keys with strong passphrases
   - Use ssh-agent to avoid typing passphrases repeatedly

3. **Key Permissions**
   ```bash
   chmod 700 ~/.ssh
   chmod 600 ~/.ssh/id_rsa
   chmod 644 ~/.ssh/id_rsa.pub
   ```

4. **Rotate Keys Regularly**
   - Change keys every 6-12 months
   - Remove old keys from `authorized_keys`

### Application Security

1. **Lock Your Screen**
   - Enable auto-lock in settings
   - Lock screen when away from computer
   - Use system screen lock

2. **Secure Configuration Files**
   - Configuration files have restricted permissions (600)
   - Don't share configuration files
   - Don't commit configuration to version control

3. **Regular Updates**
   - Keep the application updated
   - Monitor security advisories
   - Update SSH keys and passwords periodically

## Security Checklist

### For Administrators

- [ ] SSH server uses key-based authentication only
- [ ] Non-standard SSH port configured
- [ ] Root login disabled
- [ ] Firewall rules restrict SSH access
- [ ] Fail2ban or similar intrusion prevention installed
- [ ] Regular security audits performed
- [ ] SSH keys rotated every 6-12 months
- [ ] Access logs monitored

### For Users

- [ ] Strong passphrases on SSH keys
- [ ] ssh-agent configured for convenience
- [ ] Screen lock enabled
- [ ] Application auto-lock configured
- [ ] Verify host keys on first connection
- [ ] Report suspicious activity
- [ ] Don't save sensitive production credentials
- [ ] Use VPN for additional security when possible

## Threat Model

### What This Application Protects Against

✅ **Eavesdropping**: All traffic encrypted via SSH protocol
✅ **Credential Theft**: Encrypted storage of saved credentials
✅ **Man-in-the-Middle**: Host key verification
✅ **Memory Dumps**: Secure memory handling for passwords
✅ **Unauthorized Access**: OS keyring integration

### What Requires User Action

⚠️ **Physical Access**: Lock your screen when away
⚠️ **Malware**: Keep system secure and updated
⚠️ **Shoulder Surfing**: Be aware of your surroundings
⚠️ **Social Engineering**: Verify connection requests
⚠️ **Weak Passwords**: Use strong, unique passwords

### Limitations

❌ Cannot protect against compromised endpoints
❌ Cannot verify server security configuration
❌ Cannot prevent all forms of credential theft if system is compromised
❌ Does not replace need for VPN in untrusted networks

## Incident Response

### If You Suspect Compromise

1. **Immediately**
   - Disconnect from all sessions
   - Lock the application (Settings → Security → Lock Now)
   - Lock your system screen

2. **Assessment**
   - Check recent connections in logs
   - Verify no unauthorized access to servers
   - Review saved sessions for suspicious entries

3. **Remediation**
   - Change all SSH keys
   - Rotate all passwords
   - Remove saved sessions
   - Reinstall application if necessary

4. **Prevention**
   - Review security settings
   - Enable stricter authentication
   - Implement additional monitoring

## Compliance

### Standards

This application follows security best practices from:
- NIST Cybersecurity Framework
- CIS Controls
- OWASP Top 10

### Data Protection

- No telemetry or analytics collected
- No data sent to external servers
- All data remains local to your machine
- Compliant with GDPR data minimization principles

## Security Updates

### Reporting Vulnerabilities

If you discover a security vulnerability:

1. **DO NOT** open a public issue
2. Email security@example.com with details
3. Include reproduction steps if possible
4. Allow 90 days for remediation before public disclosure

### Security Updates

- Critical updates will be released within 48 hours
- Security patches will be backported to supported versions
- Users will be notified of security updates

## Additional Resources

- [OpenSSH Security Best Practices](https://www.ssh.com/ssh/protocol/)
- [NIST Guidelines on SSH](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-77r1.pdf)
- [CIS SSH Benchmarks](https://www.cisecurity.org/cis-benchmarks/)

## FAQ

**Q: Is it safe to save passwords in the application?**

A: Saved passwords are encrypted with AES-256-GCM and keys are stored in the OS keyring. While this is secure, we recommend using SSH keys for production servers.

**Q: Can someone access my saved sessions if they have my computer?**

A: If they have physical access and know your OS password, they could potentially access saved sessions. Always lock your screen and use disk encryption (BitLocker/FileVault/LUKS).

**Q: Are my SSH keys uploaded anywhere?**

A: No. Private keys never leave your machine. Only the key file path is stored for convenience.

**Q: What happens if I lose my encryption key?**

A: The encryption key is stored in your OS keyring. If you reinstall your OS or reset the keyring, saved credentials will be unrecoverable. Keep backups of important credentials separately.

**Q: Is the SSH traffic encrypted?**

A: Yes, all SSH traffic uses the SSH protocol's built-in encryption (typically AES-256-CTR or ChaCha20-Poly1305).

---

**Last Updated**: 2026-02-02
**Version**: 0.1.0
