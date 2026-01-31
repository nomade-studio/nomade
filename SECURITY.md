# Security Policy

## Supported Versions

Nomade is currently in early development. Security updates will be provided for the latest release.

| Version | Supported          |
| ------- | ------------------ |
| main    | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

The Nomade team takes security vulnerabilities seriously. We appreciate your efforts to responsibly disclose your findings.

### How to Report

**Please DO NOT file a public issue for security vulnerabilities.**

Instead, please report security vulnerabilities by emailing:

**[security@nomade.studio](mailto:security@nomade.studio)**

### What to Include

Please include the following information in your report:

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact of the vulnerability
- Suggested fix (if you have one)
- Your contact information for follow-up

### Response Timeline

- **Initial Response**: Within 48 hours, we will acknowledge receipt of your report
- **Assessment**: Within 7 days, we will provide an initial assessment and timeline
- **Updates**: We will keep you informed of our progress throughout the investigation
- **Resolution**: Once the vulnerability is fixed, we will notify you and coordinate public disclosure

### What to Expect

- We will work with you to understand and validate the vulnerability
- We will develop and test a fix
- We will coordinate disclosure timing with you
- We will credit you in our security advisory (unless you prefer to remain anonymous)

## Security Best Practices

### For Users

- Always use the latest version of Nomade
- Enable encryption at rest for sensitive data (enabled by default for embeddings)
- Use strong pairing codes when setting up device synchronization
- Keep your operating system and dependencies up to date
- Review network permissions and firewall settings

### For Developers

- Review our [threat model documentation](docs/threat-model.md)
- Follow secure coding practices outlined in our [contributing guide](CONTRIBUTING.md)
- Never commit secrets, API keys, or credentials to the repository
- Use the provided encryption helpers for sensitive data
- Validate all inputs, especially from network sources
- Run security checks before submitting PRs:
  ```bash
  # Rust security audit
  cargo audit
  
  # Dart security analysis
  dart analyze
  ```

## Security Features

Nomade implements several security measures:

1. **No Third-Party Relay**: All synchronization occurs directly between your devices or through manually configured endpoints
2. **Encryption at Rest**: Embedding artifacts are encrypted by default
3. **QR-Based Pairing**: One-time QR code pairing with key pinning
4. **QUIC Transport**: Modern, encrypted transport protocol
5. **Local-First**: Your data stays on your devices

See our [threat model documentation](docs/threat-model.md) for a comprehensive overview of security considerations.

## Known Security Considerations

As an early-stage project, please be aware:

- The project is under active development and has not undergone formal security audits
- Default configurations prioritize ease of use; adjust security settings for production use
- LAN-first architecture assumes trusted local networks
- Manual port forwarding configurations require proper firewall rules

## Security Roadmap

- [ ] Formal security audit (planned for v1.0)
- [ ] Penetration testing
- [ ] Bug bounty program (post v1.0)
- [ ] Regular dependency security scanning
- [ ] Automated vulnerability detection in CI/CD

## Questions

For general security questions (not vulnerabilities), please open a [GitHub Discussion](https://github.com/nomade-studio/nomade/discussions) or reach out via our community channels.

Thank you for helping keep Nomade secure!
