# Security Policy

## Supported Versions

Nomade is currently in early development. Security updates will be applied to the latest version.

| Version | Supported          |
| ------- | ------------------ |
| production | :white_check_mark: |

## Security Design Principles

Nomade is designed with security and privacy as core principles:

### 1. Local-First Architecture
- **No relay servers**: We do not operate any servers that relay or store user data.
- **Direct P2P**: Sync happens directly between devices via QUIC.
- **Offline by Default**: The app is fully functional without an internet connection.

### 2. Encryption at Rest & in Transit
- **Zero-Knowledge**: Keys are generated on-device and never leave your control.
- **Authenticated Encryption**: All sync traffic is secured via TLS 1.3 (QUIC) with mutual authentication (mTLS).
- **At Rest**: Sensitive artifacts (embeddings, vectors) are encrypted on disk using robust primitives (ChaCha20-Poly1305).

### 3. Strict Sync Control
- **User Agency**: Nothing leaves the device unless explicitly configured.
- **Granular Scope**: You choose which artifacts or folders to sync.
- **Verifiable**: Open source means you can audit the network stack yourself.

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability in Nomade, please report it privately:

### Reporting Process

1. **Email**: Send details to the project maintainers (contact information will be updated as project matures)
2. **Include**:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)
3. **Response Time**: We aim to respond within 48 hours
4. **Disclosure**: We follow coordinated disclosure principles

### What to Expect

- **Acknowledgment**: Confirmation of receipt within 48 hours
- **Assessment**: Initial assessment within 1 week
- **Fix**: Security patches prioritized and released ASAP
- **Credit**: Public acknowledgment (unless you prefer anonymity)

## Security Best Practices for Contributors

### Code Review
- All code changes require review before merging
- Security-sensitive changes require additional scrutiny
- Automated security scanning in CI/CD pipeline

### Dependencies
- Regular dependency audits (`cargo audit`, `flutter pub audit`)
- Pin dependencies to specific versions
- Review security advisories for all dependencies

### Secrets Management
- Never commit secrets, keys, or credentials
- Use environment variables or secure key stores
- Rotate keys regularly

### Cryptography
- Use well-established cryptographic libraries
- Never roll your own crypto
- Follow current best practices (e.g., TLS 1.3, modern ciphers)

### Testing
- Include security test cases
- Test authentication and authorization
- Validate input sanitization and validation

## Known Security Considerations

### Early Development Stage
Nomade is in early development. While we strive for security:
- Code has not undergone professional security audit
- Cryptographic implementations are minimal
- APIs and protocols may change

### User Responsibilities
- Keep devices secure and updated
- Use strong encryption keys
- Regularly backup data
- Review sync settings and understand what data is shared

## Future Security Enhancements

Planned security improvements:
- Professional security audit before stable release
- Hardware security module (HSM) support
- Advanced key management features
- Enhanced audit logging
- Formal threat modeling and security testing

## Security Updates

Security updates will be announced:
- GitHub Security Advisories
- Release notes with `[SECURITY]` prefix
- Project documentation updates

## Compliance

Nomade aims to follow:
- OWASP security guidelines
- Platform-specific security best practices (iOS, Android, macOS, Windows)
- Data protection principles (GDPR-friendly by design)

## Questions?

For general security questions (not vulnerability reports), please open a GitHub Discussion with the `security` label.

---

**Last Updated**: January 2026

Thank you for helping keep Nomade and its users secure! 🔒
