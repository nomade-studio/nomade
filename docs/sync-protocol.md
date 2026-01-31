# Sync Protocol (QUIC)

## Overview

Nomade uses QUIC (Quick UDP Internet Connections) as the transport protocol for device-to-device synchronization. QUIC provides secure, multiplexed, low-latency communication between paired devices.

## Why QUIC?

### Advantages Over TCP/TLS

1. **Integrated Security**: TLS 1.3 built into protocol
2. **Lower Latency**: 0-RTT connection resumption
3. **Connection Migration**: Survives IP address changes (important for mobile)
4. **Multiplexing**: Multiple streams without head-of-line blocking
5. **Modern Design**: Built on lessons from HTTP/2 and SPDY

### Comparison

| Feature | QUIC | TCP+TLS |
|---------|------|---------|
| Handshake RTTs | 1 (or 0) | 2-3 |
| Multiplexing | Native | HTTP/2 required |
| Head-of-line blocking | No | Yes |
| Connection migration | Yes | No |
| Built-in encryption | Yes | Separate TLS |

## Protocol Architecture

### Layers

```
┌─────────────────────────────────────────┐
│     Application Layer (Nomade)          │
│  - Artifact sync                        │
│  - Metadata exchange                    │
│  - Event notifications                  │
├─────────────────────────────────────────┤
│     QUIC Streams                        │
│  - Control stream (bidirectional)       │
│  - Artifact streams (unidirectional)    │
│  - Event stream (server-initiated)      │
├─────────────────────────────────────────┤
│     QUIC Transport                      │
│  - Reliable delivery                    │
│  - Flow control                         │
│  - Congestion control                   │
├─────────────────────────────────────────┤
│     TLS 1.3 (Integrated)                │
│  - Mutual authentication                │
│  - Encryption (ChaCha20/AES-GCM)        │
│  - Forward secrecy                      │
├─────────────────────────────────────────┤
│     UDP                                 │
└─────────────────────────────────────────┘
```

## Connection Establishment

### Initial Connection (1-RTT)

```
Client                                  Server
──────                                  ──────
1. Initial Packet
   - Client Hello (TLS)
   - Device ID
   ────────────────────────────────────>

2. Handshake + 1-RTT Packet
   <────────────────────────────────────
   - Server Hello (TLS)
   - Certificate (Device Identity)
   - Encrypted data

3. Handshake Complete
   - Certificate Verify
   - Finished (TLS)
   ────────────────────────────────────>

4. Application Data
   <═══════════════════════════════════>
   Encrypted bidirectional streams
```

### Connection Resumption (0-RTT)

For subsequent connections between paired devices:

```
Client                                  Server
──────                                  ──────
1. Initial Packet (0-RTT)
   - Session ticket
   - Early data (encrypted)
   ════════════════════════════════════>

2. Response
   <════════════════════════════════════
   - Accept or reject 0-RTT

Connection established immediately! 🚀
```

**Security Note**: 0-RTT has replay attack considerations, so only idempotent operations allowed in early data.

## Stream Types

### Control Stream (Bidirectional, ID: 0)

Long-lived stream for coordination:

```
Messages:
- HELLO: Initial handshake
- PING: Keep-alive
- SYNC_REQUEST: Request specific artifacts
- SYNC_RESPONSE: Response to requests
- ERROR: Error conditions
- GOODBYE: Graceful disconnect
```

**Example Control Flow**:
```
Client -> Server: HELLO { device_id, protocol_version }
Server -> Client: HELLO { device_id, protocol_version, capabilities }
Client -> Server: SYNC_REQUEST { since_timestamp, artifact_types }
Server -> Client: SYNC_RESPONSE { artifact_count, total_size }
[Artifact streams opened...]
```

### Artifact Streams (Unidirectional)

One stream per artifact being synchronized:

```
Stream Data:
┌──────────────────────────────────────┐
│ Header (fixed size)                  │
│  - artifact_id (32 bytes)            │
│  - content_type (4 bytes)            │
│  - size (8 bytes)                    │
│  - chunk_count (4 bytes)             │
├──────────────────────────────────────┤
│ Metadata (variable size)             │
│  - JSON with artifact metadata       │
│  - Compressed with zstd              │
├──────────────────────────────────────┤
│ Embedding (optional, encrypted)      │
│  - Encrypted vector data             │
│  - Size specified in metadata        │
├──────────────────────────────────────┤
│ Checksum (32 bytes)                  │
│  - BLAKE3 hash for integrity         │
└──────────────────────────────────────┘
```

**Stream States**:
- `OPENING`: Stream initiated
- `SENDING`: Data being transmitted
- `SENT`: All data sent, awaiting FIN
- `CLOSED`: Stream closed cleanly
- `RESET`: Stream aborted (error)

### Event Stream (Server-Initiated Unidirectional)

Real-time notifications from server to client:

```
Events:
- ARTIFACT_CREATED: New artifact available
- ARTIFACT_UPDATED: Artifact modified
- ARTIFACT_DELETED: Artifact removed
- DEVICE_JOINED: New device came online
- DEVICE_LEFT: Device went offline
- SYNC_PROGRESS: Progress update
```

**Format**:
```json
{
  "event": "ARTIFACT_CREATED",
  "timestamp": 1706745600,
  "device_id": "blake3-...",
  "artifact_id": "blake3-...",
  "data": { /* event-specific data */ }
}
```

## Sync Policies

### Default Policy v1

**What syncs**:
- ✅ Artifact metadata (plaintext)
- ✅ Embeddings (encrypted)
- ❌ Document blobs (not synced)
- ❌ Chunk text (not synced)

**Rationale**:
- Enables cross-device search via embeddings
- Protects document content
- Reduces bandwidth and storage

**User Control**:
- Can disable sync per-artifact
- Can opt-in to full sync for specific artifacts
- Future: granular policies

### Conflict Resolution

**Strategy**: Last-Write-Wins (LWW) with CRDT semantics

```
Conflict:
Device A: artifact_v1 @ timestamp T1
Device B: artifact_v2 @ timestamp T2

Resolution (T2 > T1):
Both devices converge to artifact_v2

Caveat: Lamport clocks for causality
```

**Implementation**:
```rust
struct ArtifactVersion {
    artifact_id: ArtifactId,
    version_vector: VersionVector, // Lamport clock
    device_id: DeviceId,
    timestamp: Timestamp,
    data_hash: Blake3Hash,
}
```

See [Data Model](data-model.md) for CRDT details.

## Flow Control & Congestion Control

### Flow Control

QUIC provides per-stream and per-connection flow control:

```
Stream Flow Control:
- Sender: Respects MAX_STREAM_DATA limit
- Receiver: Sends MAX_STREAM_DATA when buffer space available

Connection Flow Control:
- Sender: Respects MAX_DATA limit (all streams)
- Receiver: Sends MAX_DATA as aggregate buffer grows
```

**Nomade Configuration**:
- Per-stream limit: 10 MB
- Per-connection limit: 100 MB
- Adjusts based on network conditions

### Congestion Control

QUIC uses modern congestion control algorithms:

- **Default**: CUBIC (similar to TCP)
- **Alternative**: BBR (Bottleneck Bandwidth and RTT)
- **Mobile-aware**: Reduces aggressiveness on metered connections

**Nomade Tuning**:
- Conservative initial congestion window (IW = 10)
- Fast retransmit on packet loss
- Smooth RTT estimation

## Security

### Mutual Authentication

Both client and server authenticate:

```rust
// TLS config for mutual auth
let mut tls_config = rustls::ServerConfig::builder()
    .with_safe_default_cipher_suites()
    .with_safe_default_kx_groups()
    .with_protocol_versions(&[&rustls::version::TLS13])
    .unwrap()
    .with_client_cert_verifier(device_cert_verifier)
    .with_single_cert(server_cert, server_key)
    .unwrap();
```

**Device Certificates**:
- Self-signed certificates based on device identity keys
- Custom certificate verifier checks against paired device list
- No CA required (peer-to-peer trust)

### Encryption

**Cipher Suites** (TLS 1.3):
1. `TLS_CHACHA20_POLY1305_SHA256` (preferred on mobile)
2. `TLS_AES_256_GCM_SHA384` (preferred on desktop)

**Key Schedule**:
- Ephemeral (EC)DHE key exchange
- Forward secrecy (compromise of long-term key doesn't reveal past sessions)
- Key derivation: HKDF-SHA256

### Integrity

- All data authenticated with AEAD (Authenticated Encryption with Associated Data)
- Per-packet integrity protection
- Artifact-level integrity with BLAKE3 checksums

## Error Handling

### Error Types

```rust
enum SyncError {
    ConnectionFailed,
    AuthenticationFailed,
    StreamReset,
    ProtocolError,
    ArtifactNotFound,
    StorageFull,
    PermissionDenied,
}
```

### Retry Logic

```
Error -> Backoff -> Retry

Backoff schedule:
1st retry: 1 second
2nd retry: 2 seconds
3rd retry: 4 seconds
4th retry: 8 seconds
5th retry: 16 seconds
Max: 60 seconds

After 5 failures: Give up, notify user
```

### Graceful Degradation

- If QUIC unavailable, fallback to manual export/import
- If network unstable, reduce concurrency
- If storage full, prioritize metadata over embeddings

## Performance Optimization

### Multiplexing

Multiple artifacts transferred in parallel:

```
Connection:
├─ Control Stream (ID: 0)
├─ Artifact Stream (ID: 4) ─> artifact_1
├─ Artifact Stream (ID: 8) ─> artifact_2
├─ Artifact Stream (ID: 12) -> artifact_3
└─ Event Stream (ID: 7) <─── server events
```

**Benefits**:
- No head-of-line blocking
- Efficient bandwidth utilization
- Independent stream priorities

### Compression

- Metadata: zstd compression (level 3)
- Embeddings: Already dense (no compression)
- Headers: QPACK (QUIC's header compression)

**Typical Savings**:
- Metadata: 60-80% reduction
- Overall: 30-50% bandwidth savings

### Prioritization

Stream priorities:

1. **High**: Control stream, event stream
2. **Medium**: Metadata-only artifacts
3. **Low**: Artifact embeddings
4. **Lowest**: Bulk transfer operations

### Connection Pooling

- Reuse connections for multiple sync operations
- Connection timeout: 5 minutes idle
- Automatic reconnection with 0-RTT

## Monitoring & Observability

### Metrics

```rust
struct SyncMetrics {
    connection_attempts: u64,
    connection_successes: u64,
    connection_failures: u64,
    bytes_sent: u64,
    bytes_received: u64,
    artifacts_synced: u64,
    average_rtt: Duration,
    packet_loss_rate: f32,
}
```

### Logging

Log levels:
- `ERROR`: Connection failures, protocol errors
- `WARN`: Retry attempts, degraded performance
- `INFO`: Connection established, sync complete
- `DEBUG`: Stream lifecycle, detailed operations
- `TRACE`: Packet-level details (only for debugging)

## Platform Considerations

### Mobile (iOS/Android)

**Challenges**:
- Background execution limits
- Battery constraints
- Network switches (WiFi ↔ Cellular)

**Solutions**:
- Background fetch APIs for periodic sync
- Connection migration on network change
- Efficient cipher suites (ChaCha20 on mobile)
- Adaptive sync frequency based on battery level

### Desktop (macOS/Windows)

**Advantages**:
- No background restrictions
- Stable network connections
- More resources available

**Configuration**:
- More aggressive sync (real-time possible)
- Larger buffers and concurrency
- AES-GCM cipher suite (hardware acceleration)

### Firewall Traversal

**LAN Mode** (Default):
- mDNS for discovery (optional)
- Direct connection on local network
- No firewall issues

**Internet Mode** (Manual Setup):
- User configures port forwarding
- Static IP or DDNS recommended
- QUIC's connection migration helps with IP changes

**NAT Considerations**:
- QUIC works well with NAT (UDP-based)
- Connection migration on IP change
- Keep-alive to maintain NAT binding

## Testing

### Unit Tests

- Stream handling logic
- Error conditions
- Retry behavior
- Compression/decompression

### Integration Tests

- Full connection lifecycle
- Multi-stream sync
- Network failure scenarios
- Connection migration

### Performance Tests

- Throughput benchmarks
- Latency measurements
- Packet loss resilience
- Large artifact sync (stress test)

### Security Tests

- Certificate validation
- Encryption verification
- Replay attack prevention
- MITM resistance

## Future Enhancements

### Planned Features

1. **Differential Sync**: Only sync changed chunks
2. **Resumable Transfers**: Resume interrupted syncs
3. **Bandwidth Limits**: User-configurable rate limits
4. **Smart Scheduling**: Sync during off-peak/charging
5. **Multi-Hop**: Sync through intermediary devices

### Advanced Features

1. **Adaptive Codec**: Adjust compression based on network
2. **Priority Scheduling**: User-defined sync priorities
3. **Conflict UI**: User resolution for complex conflicts
4. **Sync Analytics**: Detailed sync performance insights

## Conclusion

Nomade's QUIC-based sync protocol provides:
- **Secure**: TLS 1.3 with mutual authentication
- **Efficient**: Multiplexed streams, compression, 0-RTT
- **Reliable**: Retry logic, error handling, graceful degradation
- **Modern**: Built on latest networking research

The protocol balances security, performance, and user experience to enable seamless cross-device synchronization.

**Next**: See [Data Model](data-model.md) for how data is structured and [Pairing](pairing.md) for device authentication.
