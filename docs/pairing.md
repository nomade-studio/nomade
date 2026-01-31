# Device Pairing Protocol

## Overview

Nomade uses QR code-based pairing to establish trust between devices. This approach provides a secure, user-friendly way to authenticate devices without relying on third-party services or complex manual configuration.

## Goals

1. **Security**: Prevent unauthorized device connections
2. **Usability**: Simple one-time setup process
3. **Privacy**: No data sent to third parties
4. **Verifiability**: User can verify pairing visually
5. **Key Pinning**: Future connections use pinned keys

## Pairing Flow

```
Device A (Initiator)                    Device B (Joiner)
─────────────────────                   ─────────────────
        │                                      │
        │ 1. Generate identity keypair         │
        │    (if not exists)                   │
        │                                      │
        │ 2. Create pairing payload:           │
        │    - Public key                      │
        │    - Endpoint (IP:Port)              │
        │    - Device ID                       │
        │    - Timestamp                       │
        │                                      │
        │ 3. Encode as QR code                 │
        │                                      │
        │ 4. Display QR code                   │
        │ ◄────────────────────────────────────┤
        │                                      │ 5. Scan QR code
        │                                      │
        │                                      │ 6. Parse payload
        │                                      │
        │                                      │ 7. Show device info
        │                                      │    for confirmation
        │                                      │
        │                                      │ 8. User confirms
        │                                      │
        │ ◄────────────────────────────────────┤ 9. Initiate QUIC
        │                                      │    connection
        │                                      │
        │ 10. Accept connection                │
        │                                      │
        │ 11. Mutual TLS handshake             │
        │ ◄───────────────────────────────────►│
        │                                      │
        │ 12. Exchange device metadata         │
        │ ◄───────────────────────────────────►│
        │                                      │
        │ 13. Pin public keys                  │
        │                                      │
        │ 14. Store pairing locally            │
        │                                      │
        │ 15. Pairing complete ✓               │
        │                                      │
```

## QR Code Payload Format

The QR code contains a JSON payload with the following structure:

```json
{
  "version": 1,
  "device_id": "uuid-v4-string",
  "device_name": "John's MacBook Pro",
  "public_key": "base64-encoded-ed25519-public-key",
  "endpoint": "192.168.1.100:8443",
  "timestamp": 1706745600,
  "signature": "base64-encoded-signature"
}
```

### Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `version` | integer | Protocol version (currently 1) |
| `device_id` | string | Unique device identifier (UUIDv4) |
| `device_name` | string | Human-readable device name |
| `public_key` | string | Base64-encoded Ed25519 public key (32 bytes) |
| `endpoint` | string | IP address and port in format `IP:PORT` |
| `timestamp` | integer | Unix timestamp when QR was generated |
| `signature` | string | Base64-encoded signature of all fields |

### Signature Generation

The signature ensures the payload hasn't been tampered with:

```rust
// Pseudocode
let payload_to_sign = format!(
    "{}:{}:{}:{}:{}:{}",
    version,
    device_id,
    device_name,
    public_key,
    endpoint,
    timestamp
);

let signature = ed25519_sign(private_key, payload_to_sign);
```

### QR Code Format

- **Encoding**: UTF-8 JSON string
- **Error Correction**: Level H (30% recovery)
- **Size**: Minimum version 5 (37×37 modules)
- **Prefix**: `nomade://pair/` followed by base64-encoded JSON

Full QR content:
```
nomade://pair/eyJ2ZXJzaW9uIjoxLCJkZXZpY2VfaWQiOi...
```

## Security Properties

### 1. Out-of-Band Authentication
The QR code provides an out-of-band channel that is difficult to intercept without physical access. This mitigates MITM attacks during initial pairing.

### 2. Key Pinning
After successful pairing, both devices store each other's public keys. Future connections validate against these pinned keys, preventing MITM attacks.

### 3. Timestamp Validation
The timestamp prevents replay attacks. Receivers should reject QR codes older than a configurable threshold (default: 5 minutes).

```rust
const MAX_QR_AGE_SECONDS: i64 = 300; // 5 minutes

fn validate_timestamp(timestamp: i64) -> Result<(), Error> {
    let now = current_unix_timestamp();
    let age = now - timestamp;
    
    if age > MAX_QR_AGE_SECONDS {
        return Err(Error::QRCodeExpired);
    }
    if age < -60 { // Allow 1 minute clock skew
        return Err(Error::QRCodeFromFuture);
    }
    
    Ok(())
}
```

### 4. Signature Verification
The signature ensures authenticity and prevents payload tampering. The signature is verified using the public key contained in the payload.

### 5. Visual Confirmation
Before completing pairing, the joiner displays device information for user confirmation:
- Device name
- Device type (Desktop/Mobile)
- Last 4 characters of device ID

### 6. Mutual Authentication
During the QUIC handshake, both devices authenticate using their private keys. This prevents one-sided trust relationships.

## Implementation Details

### Device Identity Generation

Each device generates an identity keypair on first launch:

```rust
use ed25519_dalek::{Keypair, PublicKey, SecretKey};

fn generate_identity() -> Keypair {
    let mut csprng = OsRng;
    Keypair::generate(&mut csprng)
}
```

Keys are stored in OS-specific secure storage:
- **macOS**: Keychain
- **Windows**: Credential Manager
- **iOS**: Keychain
- **Android**: KeyStore

### Endpoint Discovery

The endpoint is determined automatically:

```rust
fn get_local_endpoint() -> String {
    let ip = get_local_ip(); // Detect primary LAN IP
    let port = 8443; // Default QUIC port
    format!("{}:{}", ip, port)
}
```

For devices behind NAT, users can manually configure a public endpoint with port forwarding.

### QR Code Generation

```rust
use qrcode::{QrCode, EcLevel};

fn generate_pairing_qr(payload: &PairingPayload) -> Result<QrCode, Error> {
    let json = serde_json::to_string(payload)?;
    let encoded = base64::encode(json);
    let uri = format!("nomade://pair/{}", encoded);
    
    QrCode::with_error_correction_level(&uri, EcLevel::H)
        .map_err(|e| Error::QRGenerationFailed(e))
}
```

### QR Code Parsing

```rust
fn parse_pairing_qr(uri: &str) -> Result<PairingPayload, Error> {
    // Validate prefix
    if !uri.starts_with("nomade://pair/") {
        return Err(Error::InvalidQRCode);
    }
    
    // Extract and decode payload
    let encoded = &uri[14..]; // Skip "nomade://pair/"
    let json = base64::decode(encoded)?;
    let payload: PairingPayload = serde_json::from_slice(&json)?;
    
    // Validate signature
    validate_signature(&payload)?;
    
    // Validate timestamp
    validate_timestamp(payload.timestamp)?;
    
    Ok(payload)
}
```

### Connection Establishment

After parsing the QR code, the joiner initiates a QUIC connection:

```rust
async fn connect_to_peer(payload: PairingPayload) -> Result<Connection, Error> {
    let endpoint = payload.endpoint.parse()?;
    let peer_public_key = decode_public_key(&payload.public_key)?;
    
    // Configure QUIC client with custom certificate validation
    let client_config = configure_quic_client(peer_public_key)?;
    
    // Connect
    let connection = quinn::Endpoint::client()?
        .connect_with(client_config, endpoint, "nomade-peer")?
        .await?;
    
    // Exchange metadata
    exchange_device_info(&connection).await?;
    
    Ok(connection)
}
```

## Pairing States

### State Machine

```
┌─────────────┐
│   Unpaired  │
└──────┬──────┘
       │
       │ Generate/Scan QR
       ▼
┌─────────────┐
│  Pairing    │
└──────┬──────┘
       │
       │ Connection Established
       ▼
┌─────────────┐
│ Authenticat-│
│     ing     │
└──────┬──────┘
       │
       │ Keys Exchanged
       ▼
┌─────────────┐
│   Paired    │
└─────────────┘
```

### Stored Pairing Information

After successful pairing, both devices store:

```rust
struct PairedDevice {
    device_id: String,
    device_name: String,
    public_key: PublicKey,
    last_seen: DateTime<Utc>,
    endpoints: Vec<String>, // May have multiple endpoints
    paired_at: DateTime<Utc>,
    trust_level: TrustLevel, // User can adjust
}

enum TrustLevel {
    Trusted,    // Full access
    Limited,    // Read-only (future)
    Revoked,    // Blocked
}
```

## Edge Cases and Error Handling

### Expired QR Code
```
Error: "This QR code has expired. Please generate a new one."
```

### Invalid Signature
```
Error: "Invalid QR code signature. The code may be corrupted."
```

### Connection Timeout
```
Error: "Could not connect to device. Ensure both devices are on the same network."
```

### Duplicate Pairing
```
Warning: "This device is already paired. Do you want to re-pair?"
```

### Network Unreachable
```
Error: "Device endpoint is not reachable. Check firewall settings."
```

## UI/UX Guidelines

### Pairing Initiator (QR Generator)
1. Show large, high-contrast QR code
2. Display device name and ID
3. Show countdown timer (5 minutes)
4. Provide "Refresh" button to generate new code
5. Show waiting indicator when joiner is connecting

### Pairing Joiner (QR Scanner)
1. Camera viewfinder with QR detection
2. Haptic feedback on successful scan
3. Show parsed device info for confirmation
4. "Pair" and "Cancel" buttons
5. Progress indicator during connection

## Security Recommendations

### For Users
1. **Verify Device Name**: Ensure the displayed device name matches expectation
2. **Trusted Network**: Perform pairing on trusted local network
3. **Physical Proximity**: Be physically near the other device during pairing
4. **Review Pairings**: Regularly audit paired devices in settings

### For Developers
1. **Short Expiry**: Keep QR code validity short (5 minutes)
2. **Rate Limiting**: Limit pairing attempts to prevent brute force
3. **Audit Logging**: Log all pairing attempts for security review
4. **User Confirmation**: Require explicit user action to complete pairing

## Future Enhancements

### v1.1
- [ ] NFC pairing as alternative to QR codes
- [ ] Bluetooth LE for initial handshake
- [ ] Backup codes for pairing

### v2.0
- [ ] Multi-device pairing from single QR
- [ ] Delegation (pair on behalf of another device)
- [ ] Hierarchical trust (device families)
- [ ] Time-limited pairing (guest access)

## Testing

### Test Cases
1. ✅ Valid QR code successfully pairs devices
2. ✅ Expired QR code is rejected
3. ✅ Invalid signature is rejected
4. ✅ Tampered payload is rejected
5. ✅ Duplicate pairing is handled
6. ✅ Connection timeout is handled gracefully
7. ✅ Malformed QR code is rejected
8. ✅ Future timestamp is rejected (clock skew)

### Integration Tests
```rust
#[tokio::test]
async fn test_full_pairing_flow() {
    let device_a = Device::new("Device A");
    let device_b = Device::new("Device B");
    
    // Device A generates QR
    let qr_payload = device_a.generate_pairing_qr().unwrap();
    
    // Device B scans and parses
    let parsed = device_b.parse_qr(&qr_payload).unwrap();
    
    // Device B connects
    let connection = device_b.connect_to_peer(parsed).await.unwrap();
    
    // Verify pairing
    assert!(device_a.is_paired(&device_b.device_id()));
    assert!(device_b.is_paired(&device_a.device_id()));
}
```

## References

- RFC 9000: QUIC Protocol
- RFC 8032: Ed25519 Signatures
- ISO/IEC 18004: QR Code Specification

---

**Last Updated**: 2026-01-31
**Protocol Version**: 1
