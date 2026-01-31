# Threat Model

## Overview

This document analyzes potential security threats to Nomade and describes mitigations. As a local-first, privacy-focused application, security is fundamental to our design.

## Trust Boundaries

### Trusted Components
- User's own devices running Nomade
- Local operating system (to an extent)
- User's local network (with caveats)

### Untrusted Components
- Network infrastructure (ISPs, routers outside user control)
- Any third-party services (none used by default)
- Other devices before pairing
- Public WiFi networks

## Threat Actors

### 1. Network Adversary
**Capability**: Can observe/manipulate network traffic
**Motivation**: Data theft, surveillance
**Scope**: Internet, public WiFi, compromised routers

### 2. Malicious App/Device
**Capability**: Attempts to connect without authorization
**Motivation**: Data theft, unauthorized access
**Scope**: Any device that can reach the QUIC endpoint

### 3. Physical Access Attacker
**Capability**: Physical access to unlocked device
**Motivation**: Data theft, device compromise
**Scope**: Individual device

### 4. Compromised Peer
**Capability**: Previously paired device is compromised
**Motivation**: Access to synced data
**Scope**: All paired devices

### 5. Supply Chain Attack
**Capability**: Malicious dependencies
**Motivation**: Backdoor, data exfiltration
**Scope**: Build artifacts, dependencies

## Assets to Protect

### Critical Assets
1. **Private keys**: Used for device identity and encryption
2. **Document content**: User's notes, documents, conversations
3. **Embeddings**: Vector representations of content
4. **Conversation history**: User interactions with AI

### Sensitive Assets
5. **Metadata**: Document titles, timestamps, tags
6. **Device identity**: Device names, public keys
7. **Network endpoints**: IP addresses of paired devices

### Low-Sensitivity Assets
8. **Application logs**: Debugging information
9. **Performance metrics**: Local usage statistics

## Threat Analysis

### T1: Man-in-the-Middle (MITM) Attack

**Threat**: Attacker intercepts network traffic between devices

**Attack Vector**:
- Compromised router
- Public WiFi network
- ISP-level interception

**Impact**: HIGH
- Could read/modify synced data
- Could impersonate a peer

**Mitigations**:
✅ QUIC with TLS 1.3 provides transport encryption
✅ Key pinning on first pairing prevents MITM after initial connection
✅ QR code pairing ensures first connection authenticity (out-of-band)

**Residual Risk**: LOW
- First pairing must occur over secure channel (QR code)
- User should verify devices are on trusted network during pairing

---

### T2: Unauthorized Device Connection

**Threat**: Malicious device attempts to connect without pairing

**Attack Vector**:
- Port scanning to find QUIC endpoint
- Brute force connection attempts

**Impact**: MEDIUM
- Could attempt to access synced data
- Could cause DoS

**Mitigations**:
✅ All connections require valid device certificate
✅ Mutual TLS authentication
✅ Only paired device public keys are accepted
✅ Rate limiting on connection attempts (planned)

**Residual Risk**: LOW
- Attacker cannot connect without paired credentials

---

### T3: Data at Rest Compromise

**Threat**: Attacker gains physical access to device

**Attack Vector**:
- Device theft
- Forensic analysis of storage
- Unlocked device access

**Impact**: HIGH
- Could read all stored data
- Could extract private keys

**Mitigations**:
✅ Embeddings/artifacts encrypted at rest
⚠️ Metadata stored in plaintext for search performance
✅ OS-level encryption (FileVault, BitLocker) recommended
⚠️ Private keys stored in OS keychain/keyring (platform-dependent)

**Residual Risk**: MEDIUM
- Metadata is accessible if device is unlocked
- User must enable OS-level disk encryption
- Screen lock and strong passwords recommended

---

### T4: Compromised Paired Device

**Threat**: Previously trusted device is compromised

**Attack Vector**:
- Malware on paired device
- Device stolen and unlocked
- Account takeover

**Impact**: HIGH
- Can access all synced data
- Can modify/delete synced data
- Can impersonate device

**Mitigations**:
⚠️ No automatic revocation mechanism (v1)
✅ User can manually unpair devices
✅ Each device has separate encryption keys
⚠️ Synced data accessible to all paired devices

**Residual Risk**: HIGH
- User must manually detect and unpair compromised device
- Planned: Device trust score and anomaly detection (future)
- Planned: Per-document encryption with ACLs (future)

---

### T5: Dependency Vulnerability

**Threat**: Vulnerability in third-party library

**Attack Vector**:
- Vulnerable version of QUIC library
- Vulnerable crypto library
- Supply chain attack

**Impact**: VARIES
- Could compromise network security
- Could leak data
- Could enable remote code execution

**Mitigations**:
✅ Pin dependency versions
✅ Regular security audits of dependencies
✅ Use well-vetted libraries (quinn, rustls)
✅ Automated dependency scanning in CI
✅ Minimal dependency footprint

**Residual Risk**: MEDIUM
- Zero-day vulnerabilities possible
- Must stay current with security patches

---

### T6: Memory Extraction

**Threat**: Attacker extracts sensitive data from memory

**Attack Vector**:
- Memory dump of running process
- Cold boot attack
- RAM scraping malware

**Impact**: HIGH
- Could extract decrypted data
- Could extract private keys

**Mitigations**:
⚠️ Limited memory protection in v1
✅ Minimize plaintext in memory
✅ Zero memory on deallocation (Rust's Drop trait)
⚠️ OS-level protections (ASLR, DEP)

**Residual Risk**: MEDIUM
- Sophisticated attackers can extract memory
- Mobile platforms provide better isolation
- Planned: Secure enclaves for key storage (future)

---

### T7: Relay Impersonation (N/A)

**Threat**: Malicious relay server

**Attack Vector**: Not applicable

**Mitigation**: ✅ No third-party relay by design

---

### T8: Embedding Model Poisoning

**Threat**: Malicious embedding model produces backdoored embeddings

**Attack Vector**:
- User loads compromised model
- Model produces embeddings with hidden channels

**Impact**: LOW-MEDIUM
- Could encode sensitive data in embeddings
- Could bias RAG results

**Mitigations**:
⚠️ User responsibility to trust model source
✅ Document model provenance
✅ Support for deterministic embedding generation
✅ Embeddings are encrypted at rest

**Residual Risk**: LOW
- Advanced attack, low likelihood
- User must verify model integrity

---

### T9: Traffic Analysis

**Threat**: Attacker analyzes encrypted traffic patterns

**Attack Vector**:
- Timing analysis
- Packet size analysis
- Connection patterns

**Impact**: LOW
- Could infer document types
- Could infer sync patterns
- Cannot decrypt content

**Mitigations**:
⚠️ Limited traffic obfuscation in v1
✅ QUIC provides some padding
⚠️ Sync patterns may reveal activity

**Residual Risk**: MEDIUM
- Metadata leakage through traffic analysis
- Not a priority for v1
- Planned: Traffic padding and batching (future)

---

### T10: Denial of Service

**Threat**: Attacker floods QUIC endpoint

**Attack Vector**:
- Connection flood
- Large message spam
- Resource exhaustion

**Impact**: MEDIUM
- Service unavailable
- Battery drain on mobile
- No data loss

**Mitigations**:
⚠️ Basic rate limiting in v1
✅ QUIC has built-in DoS protections
✅ Only accept connections from paired devices
⚠️ Local network makes DoS less likely

**Residual Risk**: LOW
- LAN environment reduces attack surface
- Public endpoints more vulnerable
- User can disable sync if needed

---

## Security Assumptions

1. **Operating System**: We assume the OS is not compromised
2. **First Pairing**: Initial QR code pairing occurs in trusted environment
3. **User Behavior**: User practices basic security hygiene (screen lock, updates)
4. **Local Network**: LAN is more trusted than public internet
5. **Cryptography**: Modern cryptographic primitives are secure

## Defense in Depth

| Layer | Protection |
|-------|------------|
| Transport | QUIC + TLS 1.3 |
| Authentication | Mutual TLS, key pinning |
| Data at Rest | Encryption for embeddings/artifacts |
| Access Control | Pairing-based device authorization |
| Network | LAN-first, no third-party relay |
| Application | Input validation, memory safety (Rust) |
| OS | Leverage OS keychains, sandboxing |

## Security Roadmap

### v1.0
- ✅ QUIC + TLS transport security
- ✅ QR-based pairing
- ✅ Key pinning
- ✅ Encryption at rest for artifacts
- ✅ Mutual TLS authentication

### v1.1
- [ ] Rate limiting and DoS protection
- [ ] Device trust scores
- [ ] Anomaly detection
- [ ] Secure key storage (hardware-backed)

### v2.0
- [ ] Per-document encryption with ACLs
- [ ] Automated device revocation
- [ ] Traffic padding/obfuscation
- [ ] Formal security audit
- [ ] Penetration testing

## Security Best Practices for Users

1. **Enable OS Encryption**: Use FileVault (macOS), BitLocker (Windows)
2. **Strong Passwords**: Protect device with strong password/biometric
3. **Screen Lock**: Enable auto-lock after brief timeout
4. **Trusted Network**: Pair devices on trusted local network
5. **Keep Updated**: Install security updates promptly
6. **Review Pairings**: Regularly audit paired devices
7. **Physical Security**: Don't leave devices unattended
8. **Backup**: Maintain encrypted backups

## Incident Response

If a security vulnerability is discovered:

1. **Report**: Email security@nomade.studio immediately
2. **Assess**: Team evaluates severity and impact
3. **Patch**: Develop and test fix
4. **Disclose**: Coordinate responsible disclosure
5. **Release**: Push security update
6. **Notify**: Inform users of threat and mitigation

## Compliance

Nomade is designed with privacy regulations in mind:

- **GDPR**: No data collection, user controls all data
- **CCPA**: No data sales, no third-party access
- **HIPAA**: Encryption at rest/transit (user responsibility for compliance)
- **SOC 2**: Not applicable (no service provider)

## Conclusion

Nomade's security model prioritizes:
1. **Privacy**: No third-party access to data
2. **Control**: User owns and controls all data
3. **Transparency**: Open source, auditable code
4. **Defense in Depth**: Multiple security layers

The local-first architecture eliminates entire classes of attacks that affect cloud-based systems. However, users must take responsibility for physical device security and network hygiene.

---

**Last Updated**: 2026-01-31
**Next Review**: After major architecture changes or security incidents
