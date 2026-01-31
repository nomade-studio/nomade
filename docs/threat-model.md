# Threat Model

## Introduction

This document analyzes potential security threats to Nomade and describes mitigations. It follows a structured threat modeling approach to ensure comprehensive security coverage.

## Assets

### High-Value Assets

1. **User Documents**: Sensitive personal/professional content
2. **AI Embeddings**: Vector representations of document content
3. **Identity Keys**: Device private keys for authentication
4. **Encryption Keys**: Keys protecting data at rest
5. **Sync Metadata**: Information about user's document graph
6. **User Preferences**: Settings and configuration data

### Trust Boundaries

```
┌───────────────────────────────────────┐
│         User's Device                 │
│  ┌─────────────────────────────────┐ │
│  │    Nomade Application           │ │ <- Trust Boundary 1
│  │  (Flutter + Rust)               │ │
│  └─────────────────────────────────┘ │
│  ┌─────────────────────────────────┐ │
│  │    Operating System             │ │ <- Trust Boundary 2
│  └─────────────────────────────────┘ │
└───────────────────────────────────────┘
         │
         │ Network (LAN or Internet)      <- Trust Boundary 3
         │
┌───────────────────────────────────────┐
│      Peer Device (Optional)           │
└───────────────────────────────────────┘
```

## Threat Categories (STRIDE)

### 1. Spoofing Identity

**Threat**: Attacker impersonates legitimate device during sync

**Scenarios**:
- Fake device attempts to pair via QR code
- Man-in-the-middle during initial pairing
- Compromised device uses stolen identity keys

**Mitigations**:
- ✅ Device identity based on public/private key pairs (Ed25519)
- ✅ QR code includes cryptographic device fingerprint
- ✅ Visual confirmation required during pairing
- ✅ QUIC connection authenticated with TLS 1.3 + mutual auth
- ✅ Key rotation capability (future)

**Residual Risk**: Low - requires physical access during pairing or key theft

### 2. Tampering with Data

**Threat**: Attacker modifies data in transit or at rest

**Scenarios**:
- Network attacker modifies sync data
- Malware modifies local storage
- Attacker with file system access tampers with artifacts

**Mitigations**:
- ✅ QUIC/TLS 1.3 provides transport integrity
- ✅ Content-addressed storage (hash-based IDs) detects tampering
- ✅ Encrypted embeddings prevent modification without key
- ✅ OS-level file permissions protect storage directory
- ✅ Cryptographic signatures on synced artifacts (future)

**Residual Risk**: Medium - OS malware with elevated privileges can tamper

### 3. Repudiation

**Threat**: User denies creating/modifying content

**Scenarios**:
- Dispute over document authorship
- Claim that device didn't perform sync action

**Mitigations**:
- ⚠️ Minimal audit logging (privacy-first design)
- ⚠️ Device identity in metadata (optional)
- ⚠️ Timestamp on artifacts (local clock)

**Residual Risk**: High - privacy trade-off; not a primary concern for personal tool

### 4. Information Disclosure

**Threat**: Unauthorized access to sensitive data

**Scenarios**:
- Attacker gains file system access
- Network eavesdropping during sync
- Memory dump exposes keys or content
- Side-channel attacks on embeddings

**Mitigations**:
- ✅ Embeddings encrypted at rest (AES-256-GCM)
- ✅ QUIC/TLS 1.3 encrypts all network traffic
- ✅ OS-level encryption (FileVault, BitLocker) recommended
- ✅ Keys stored in platform keychain where available
- ⚠️ Document text plaintext at rest (user responsibility to use disk encryption)
- ⚠️ Memory protection relies on OS (no in-memory encryption)

**Residual Risk**: Medium - requires OS-level disk encryption for full protection

### 5. Denial of Service

**Threat**: Attacker prevents legitimate use

**Scenarios**:
- Network flood during sync
- Malformed packets crash QUIC server
- Storage exhaustion via large artifacts
- CPU exhaustion via expensive operations

**Mitigations**:
- ✅ QUIC rate limiting and backpressure
- ✅ Input validation on all network data
- ✅ Storage quotas and size limits (future)
- ✅ Sync can be disabled/paused
- ⚠️ Limited protection against local DoS (malware)

**Residual Risk**: Medium - remote DoS mitigated; local DoS harder to prevent

### 6. Elevation of Privilege

**Threat**: Attacker gains unauthorized access or capabilities

**Scenarios**:
- Rust code vulnerability leads to arbitrary code execution
- Flutter plugin vulnerability exploited
- Escape from app sandbox to OS

**Mitigations**:
- ✅ Memory-safe Rust reduces vulnerability surface
- ✅ Minimal dependencies (reduced supply chain risk)
- ✅ Regular dependency audits (`cargo audit`)
- ✅ OS sandboxing on mobile platforms (iOS, Android)
- ✅ Code signing and notarization (macOS, Windows)
- ⚠️ Desktop platforms less sandboxed (macOS/Windows)

**Residual Risk**: Low-Medium - depends on OS security model

## Attack Scenarios

### Scenario 1: Malicious Peer Device

**Description**: Attacker compromises a device that was previously paired

**Attack Steps**:
1. Steal device or extract keys from compromised device
2. Connect to legitimate device using stolen identity
3. Attempt to sync malicious artifacts or exfiltrate data

**Mitigations**:
- Device revocation feature (planned)
- User can detect unknown devices in sync list
- Audit log shows sync activity
- Limit blast radius: only metadata/embeddings sync by default

**Risk Level**: Medium

### Scenario 2: Network Eavesdropper (LAN)

**Description**: Attacker on same local network intercepts sync traffic

**Attack Steps**:
1. Position on same WiFi/LAN as victim devices
2. Capture QUIC packets during sync
3. Attempt to decrypt or analyze traffic

**Mitigations**:
- QUIC/TLS 1.3 encryption prevents eavesdropping
- Mutual authentication prevents MITM
- Forward secrecy limits damage from key compromise

**Risk Level**: Low - mitigated by TLS 1.3

### Scenario 3: Malware on Device

**Description**: Malware running with user privileges on device

**Attack Steps**:
1. Malware gains user-level access (phishing, exploit, etc.)
2. Read document storage directory
3. Exfiltrate documents and keys

**Mitigations**:
- Encryption at rest protects embeddings
- Platform keychain protects encryption keys (iOS, Android)
- OS-level disk encryption recommended (FileVault, BitLocker)
- Regular security updates

**Risk Level**: High - requires OS-level security and user vigilance

### Scenario 4: Supply Chain Attack

**Description**: Malicious dependency introduced via Rust/Dart package

**Attack Steps**:
1. Compromised dependency published to crates.io or pub.dev
2. Nomade builds with malicious code
3. User installs compromised version

**Mitigations**:
- Pin dependencies to specific versions
- Regular `cargo audit` and `flutter pub audit`
- Minimal dependency footprint
- Code review of updates
- Reproducible builds (future)

**Risk Level**: Medium - industry-wide challenge

## Security Design Decisions

### 1. Default Sync Policy v1

**Decision**: Plaintext metadata synced; embeddings encrypted; no blob/chunk text

**Rationale**:
- Balances usability (search across devices) with privacy
- Embedding encryption protects semantic content
- Users can disable sync for sensitive documents
- Future versions allow granular control

### 2. No Third-Party Relay

**Decision**: LAN-first, optional manual port-forward, no cloud relay

**Rationale**:
- Eliminates trusted third party
- Reduces attack surface
- User maintains control
- Trade-off: more complex setup for remote sync

### 3. QR Code Pairing

**Decision**: Visual QR code for device pairing (not NFC, Bluetooth)

**Rationale**:
- User controls when pairing happens (explicit action)
- Visual confirmation of device identity
- Works across all platforms
- Resistant to remote attacks

### 4. Rust Core Library

**Decision**: Implement security-critical code in Rust

**Rationale**:
- Memory safety prevents common vulnerabilities
- Strong type system prevents logic errors
- Excellent cryptography libraries (rustls, ring, etc.)
- Performance for crypto operations

## Security Testing

### Planned Testing Approaches

1. **Static Analysis**:
   - `cargo clippy` for Rust
   - `dart analyze` for Flutter
   - Dependency vulnerability scanning

2. **Dynamic Analysis**:
   - Fuzzing QUIC parser with AFL/libFuzzer
   - Penetration testing of pairing flow
   - Network traffic analysis

3. **Code Review**:
   - Security-focused review for all crypto code
   - Review of authentication and authorization logic
   - Third-party audit (planned before 1.0)

4. **Threat Model Updates**:
   - Regular reviews as features evolve
   - Post-incident analysis
   - Community security feedback

## Known Limitations

1. **Text at Rest**: Document text not encrypted by default (relies on OS disk encryption)
2. **Memory Protection**: No in-memory encryption (keys/content in process memory)
3. **Desktop Sandboxing**: Limited on macOS/Windows compared to mobile
4. **Key Rotation**: Not yet implemented (planned)
5. **Formal Verification**: Crypto code not formally verified
6. **Audit Trail**: Minimal for privacy reasons

## Compliance Considerations

### GDPR/Privacy Laws

- User controls all data (right to access)
- Easy deletion (right to erasure)
- No third-party data sharing (by design)
- Transparent data practices

### Platform Requirements

- iOS: App sandbox, keychain usage, privacy manifest
- Android: Permissions, secure storage
- macOS: Notarization, hardened runtime
- Windows: Code signing, SmartScreen compatibility

## Incident Response

### Security Issue Process

1. **Report**: Private disclosure via SECURITY.md
2. **Triage**: Assess severity and impact
3. **Fix**: Develop and test patch
4. **Disclose**: Coordinated disclosure with fix
5. **Learn**: Update threat model and processes

### Severity Levels

- **Critical**: Remote code execution, key compromise
- **High**: Authentication bypass, data exfiltration
- **Medium**: DoS, information disclosure (non-key)
- **Low**: UI spoofing, minor info leak

## Conclusion

Nomade's security design prioritizes:
1. **User Control**: Local-first, optional sync
2. **Defense in Depth**: Multiple layers of protection
3. **Transparent Risks**: Clear communication of limitations
4. **Continuous Improvement**: Regular reviews and updates

This threat model will evolve as Nomade matures. Security is an ongoing process, not a one-time achievement.

**Next Steps**:
- Implement key rotation
- Add device revocation
- Conduct third-party security audit
- Implement advanced audit logging (opt-in)
