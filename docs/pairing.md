# Device Pairing

## Overview

Device pairing in Nomade establishes a secure, authenticated connection between two devices for synchronization. The pairing process uses QR codes for initial key exchange and establishes trust without relying on third-party services.

## Design Goals

1. **Secure**: Cryptographically authenticated device identity
2. **User-Friendly**: Simple QR code scan
3. **Private**: No third-party involvement
4. **Verifiable**: Visual confirmation of device identity
5. **Resistant to Attack**: Protected against MITM and spoofing

## Pairing Protocol

### High-Level Flow

```
Device A (Initiator)                    Device B (Responder)
─────────────────                       ─────────────────────
1. Generate key pair (Ed25519)          1. Generate key pair (Ed25519)
2. Create pairing offer:                2. Start listening for pairs
   - Device ID (public key hash)        
   - Device name                        
   - Connection info (IP:port)          
   - Nonce (random)                     
3. Encode as QR code                    
4. Display QR code                      
                                        5. Scan QR code
                                        6. Display confirmation prompt:
                                           "Pair with Device A?"
                                        7. User confirms
                                        8. Establish QUIC connection
                                        9. Mutual TLS handshake
                                        10. Exchange device info
                                        11. Store peer identity
12. Receive confirmation              
13. Store peer identity               
14. Pairing complete ✅                 15. Pairing complete ✅
```

### Detailed Protocol Steps

#### Step 1: Key Generation

Each device generates a long-term identity key pair:

```rust
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

let mut csprng = OsRng{};
let keypair: Keypair = Keypair::generate(&mut csprng);

// Derive device ID from public key
let device_id = blake3::hash(keypair.public.as_bytes());
```

**Properties**:
- Ed25519: Fast, small keys, strong security (128-bit security level)
- Device ID: Content-addressed (deterministic from public key)
- Keys stored in platform-specific secure storage

#### Step 2: Pairing Offer Creation

Device A (initiator) creates a pairing offer:

```json
{
  "version": 1,
  "device_id": "blake3-hash-of-public-key",
  "device_name": "John's MacBook Pro",
  "public_key": "base64-encoded-ed25519-public-key",
  "endpoints": [
    "192.168.1.100:8765",
    "10.0.0.5:8765"
  ],
  "nonce": "random-32-bytes",
  "timestamp": 1706745600,
  "signature": "ed25519-signature-of-above-fields"
}
```

**Fields**:
- `version`: Protocol version for future compatibility
- `device_id`: Unique device identifier
- `device_name`: Human-readable name (user-configured)
- `public_key`: Ed25519 public key for authentication
- `endpoints`: List of IP:port pairs (LAN addresses)
- `nonce`: Prevents replay attacks
- `timestamp`: Unix timestamp (freshness check)
- `signature`: Signs all above fields with device's private key

#### Step 3: QR Code Encoding

Offer encoded as QR code:

```
nomade://pair?v=1&d=<compressed-base64-offer>
```

**Properties**:
- URL scheme `nomade://` for app linking
- Compressed with zstd to fit in QR code
- QR code version: auto-selected based on size
- Error correction: High (30% recovery)

#### Step 4: QR Code Scanning

Device B scans the QR code:

1. **Parse**: Extract and decompress offer
2. **Validate**: 
   - Check signature matches public key
   - Verify timestamp (within 5 minutes)
   - Ensure nonce is fresh (not seen before)
3. **Display**: Show confirmation prompt with device info

**User sees**:
```
┌─────────────────────────────────────┐
│  Pair with Device?                  │
│                                     │
│  Name: John's MacBook Pro           │
│  ID:   blake3-abc...def (truncated) │
│                                     │
│  [ Confirm ]  [ Cancel ]            │
└─────────────────────────────────────┘
```

#### Step 5: QUIC Connection Establishment

After user confirms, Device B initiates QUIC connection to Device A:

```rust
// Device B connects to Device A
let quic_client = QuicClient::new()
    .with_tls_config(tls_config)
    .with_peer_certificate(device_a_public_key)
    .connect(&endpoints[0])
    .await?;

// Mutual authentication via TLS 1.3
// Both devices verify each other's certificates
```

**TLS Configuration**:
- TLS 1.3 only (no downgrade)
- Mutual authentication (client and server certs)
- Cipher suites: Modern, authenticated encryption only
- Device identity key used as TLS certificate

#### Step 6: Device Info Exchange

Over established QUIC connection:

```
Device B -> Device A:
{
  "type": "pairing_request",
  "device_id": "blake3-hash-of-b-public-key",
  "device_name": "Jane's iPhone",
  "public_key": "base64-encoded-public-key",
  "signature": "..."
}

Device A -> Device B:
{
  "type": "pairing_accept",
  "device_id": "blake3-hash-of-a-public-key",
  "sync_capabilities": ["metadata", "embeddings"],
  "signature": "..."
}
```

#### Step 7: Trust Establishment

Both devices store peer identity:

```rust
struct PairedDevice {
    device_id: DeviceId,
    device_name: String,
    public_key: PublicKey,
    paired_at: Timestamp,
    last_seen: Timestamp,
    trusted: bool,
}
```

**Storage**:
- Persisted to local database
- Encrypted at rest
- Used for future connection authentication

### Security Properties

#### 1. Authentication

- Both devices prove ownership of private keys
- TLS mutual authentication ensures authenticity
- Signatures prevent impersonation

#### 2. Confidentiality

- QR code displayed locally (physical proximity required)
- TLS 1.3 encrypts all network traffic
- Keys never transmitted in plaintext

#### 3. Integrity

- Signatures protect against tampering
- Content-addressed device IDs detect key substitution
- Nonce prevents replay attacks

#### 4. Forward Secrecy

- TLS 1.3 provides forward secrecy
- Compromise of long-term key doesn't reveal past sessions

#### 5. Man-in-the-Middle Resistance

- Visual confirmation of device name
- Device ID displayed (user can verify out-of-band)
- Mutual authentication prevents MITM

## Attack Scenarios & Mitigations

### Scenario 1: QR Code Capture

**Attack**: Attacker photographs QR code from distance

**Mitigations**:
- Short validity window (5 minutes)
- Nonce prevents reuse
- User-controlled display (can cancel)
- Physical proximity required for initial pairing

**Residual Risk**: Low - attacker needs quick action

### Scenario 2: Network Interception

**Attack**: Attacker intercepts QUIC handshake

**Mitigations**:
- TLS 1.3 mutual authentication
- Both parties verify certificates
- Device ID verification

**Residual Risk**: Very Low - TLS 1.3 prevents MITM

### Scenario 3: Fake Device

**Attack**: Attacker creates device with similar name

**Mitigations**:
- Device ID shown to user (can verify out-of-band)
- Unique device ID based on cryptographic key
- User confirms before pairing

**Residual Risk**: Low - requires user to not verify ID

### Scenario 4: Compromised Device

**Attack**: Previously paired device is compromised

**Mitigations**:
- Device revocation feature (planned)
- Regular re-authentication (planned)
- User can remove paired devices manually

**Residual Risk**: Medium - requires user awareness

## User Experience

### Pairing Flow (User Perspective)

**Device A (showing QR)**:
1. Open Settings → Devices → Add Device
2. Click "Show QR Code"
3. Wait for Device B to scan
4. See "Device B paired successfully" ✅

**Device B (scanning QR)**:
1. Open Settings → Devices → Pair with Device
2. Click "Scan QR Code"
3. Point camera at Device A's screen
4. Review device info: "Pair with Device A?"
5. Tap "Confirm"
6. See "Paired successfully" ✅

**Time**: ~20 seconds for complete pairing

### Multi-Device Management

Users can manage paired devices:

```
Settings → Devices

Paired Devices:
├─ John's MacBook Pro
│  ID: blake3-abc...def
│  Paired: 2026-01-15
│  Last Seen: 2 hours ago
│  [Remove] [View Details]
│
├─ John's iPhone
│  ID: blake3-fed...cba
│  Paired: 2026-01-20
│  Last Seen: 5 minutes ago
│  [Remove] [View Details]
```

## Implementation Details

### Key Storage

**iOS/Android**:
- Platform keychain/keystore (hardware-backed if available)
- Keys never leave secure enclave

**macOS**:
- Keychain with access control lists
- Requires user authentication for key access

**Windows**:
- Credential Manager or DPAPI
- User-protected keys

### Connection Details

**Ports**:
- Default: 8765 (configurable)
- Random ephemeral port as fallback

**Discovery**:
- QR code provides explicit endpoints
- No mDNS/Bonjour required for pairing
- mDNS optional for automatic reconnection

**Timeouts**:
- QR code validity: 5 minutes
- Connection attempt: 30 seconds
- Handshake timeout: 10 seconds

## Future Enhancements

### Planned Features

1. **Device Groups**: Organize devices into groups (work, personal, etc.)
2. **Trusted Circles**: Transitive trust for easier multi-device setup
3. **Backup Codes**: Paper backup for device recovery
4. **Key Rotation**: Periodic key rotation without re-pairing
5. **Revocation**: Remote device revocation
6. **NFC Alternative**: Tap-to-pair for NFC-enabled devices

### Advanced Security

1. **Hardware Keys**: Support for FIDO2/U2F keys
2. **Biometric Confirmation**: Require fingerprint/face ID for pairing
3. **Multi-Party Pairing**: Group pairing sessions
4. **Audit Logs**: Record all pairing events

## Testing

### Test Scenarios

1. **Happy Path**: Normal pairing between two devices
2. **Expired QR**: Attempt to use QR after 5 minutes
3. **Replay Attack**: Reuse QR code / nonce
4. **Invalid Signature**: Tampered QR data
5. **Network Failure**: Connection drops during handshake
6. **Concurrent Pairing**: Multiple devices pairing simultaneously

### Security Testing

1. **MITM Attempt**: Verify TLS prevents interception
2. **Certificate Validation**: Ensure fake certs rejected
3. **Key Extraction**: Attempt to extract keys from storage
4. **Side-Channel**: Timing attack on cryptographic operations

## Conclusion

Nomade's pairing protocol provides:
- **Strong Security**: Cryptographic authentication
- **Simple UX**: QR code scan
- **No Third Party**: Peer-to-peer trust establishment
- **Attack Resistance**: Multiple layers of protection

The design balances security and usability, making secure pairing accessible to non-technical users while maintaining cryptographic rigor.

**Next**: See [Sync Protocol](sync-protocol.md) for how paired devices synchronize data.
