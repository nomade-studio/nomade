# Synchronization Protocol

## Overview

Nomade's synchronization protocol enables real-time, conflict-free state replication across paired devices using QUIC transport and CRDT data structures. This document describes the wire protocol, message formats, and synchronization semantics.

## Protocol Stack

```
┌─────────────────────────────────────┐
│  Application (CRDT Operations)      │
├─────────────────────────────────────┤
│  Sync Protocol (Messages)           │
├─────────────────────────────────────┤
│  QUIC (Streams + Datagrams)         │
├─────────────────────────────────────┤
│  TLS 1.3 (Encryption + Auth)        │
├─────────────────────────────────────┤
│  UDP (Transport)                    │
└─────────────────────────────────────┘
```

## QUIC Stream Layout

Nomade uses multiple QUIC streams for different purposes:

### Stream Types

| Stream ID | Type | Direction | Purpose |
|-----------|------|-----------|---------|
| 0 | Bidirectional | Both | Control messages, handshake |
| 2, 6, 10... | Bidirectional | Both | CRDT sync operations |
| 4, 8, 12... | Bidirectional | Both | Artifact transfer |
| 1, 5, 9... | Unidirectional | Client→Server | Heartbeats, presence |
| 3, 7, 11... | Unidirectional | Server→Client | Push notifications |

### Stream Lifecycle

```
Open Stream
    ↓
Send Header (stream type, version)
    ↓
Exchange Messages
    ↓
Close Stream (FIN)
```

## Message Format

All messages use a common envelope format:

```rust
struct MessageEnvelope {
    version: u8,           // Protocol version (currently 1)
    message_type: u16,     // Message type identifier
    message_id: [u8; 16],  // Unique message ID (UUID)
    timestamp: i64,        // Unix timestamp (milliseconds)
    payload_length: u32,   // Payload size in bytes
    payload: Vec<u8>,      // Message-specific payload
    checksum: [u8; 32],    // Blake3 hash of payload
}
```

### Serialization

Messages are serialized using MessagePack (msgpack) for efficiency:
- Compact binary format
- Support for complex types
- Fast serialization/deserialization
- Self-describing

## Message Types

### 1. Handshake Messages (0x00xx)

#### `HELLO (0x0001)`
First message sent after connection establishment.

```rust
struct HelloMessage {
    device_id: String,
    device_name: String,
    protocol_version: u8,
    capabilities: Vec<String>, // ["crdt-sync", "artifact-transfer"]
    sync_state_hash: [u8; 32], // Hash of local CRDT state
}
```

#### `HELLO_ACK (0x0002)`
Response to HELLO with peer capabilities.

```rust
struct HelloAckMessage {
    device_id: String,
    device_name: String,
    protocol_version: u8,
    capabilities: Vec<String>,
    sync_state_hash: [u8; 32],
    sync_required: bool, // True if hashes differ
}
```

### 2. Sync Messages (0x01xx)

#### `SYNC_REQUEST (0x0101)`
Request synchronization of specific entities.

```rust
struct SyncRequestMessage {
    entity_types: Vec<EntityType>, // e.g., ["document", "conversation"]
    since_timestamp: Option<i64>,  // Incremental sync from this time
    full_sync: bool,               // Request full state
}
```

#### `SYNC_OPERATION (0x0102)`
Individual CRDT operation to be applied.

```rust
struct SyncOperationMessage {
    entity_id: String,
    entity_type: EntityType,
    operation: CRDTOperation, // Specific to CRDT implementation
    timestamp: i64,
    actor_id: String,         // Device that created the operation
}

enum CRDTOperation {
    Set { key: String, value: Value },
    Delete { key: String },
    Insert { index: usize, value: Value },
    Remove { index: usize },
    // ... more operations based on CRDT type
}
```

#### `SYNC_COMPLETE (0x0103)`
Indicates sync is complete for requested entities.

```rust
struct SyncCompleteMessage {
    entity_types: Vec<EntityType>,
    operations_count: u32,
    new_state_hash: [u8; 32],
}
```

### 3. Artifact Messages (0x02xx)

#### `ARTIFACT_REQUEST (0x0201)`
Request artifact by content hash.

```rust
struct ArtifactRequestMessage {
    artifact_id: [u8; 32],  // Content hash (SHA-256)
    priority: u8,            // 0=low, 255=high
}
```

#### `ARTIFACT_METADATA (0x0202)`
Artifact metadata before content transfer.

```rust
struct ArtifactMetadataMessage {
    artifact_id: [u8; 32],
    size: u64,
    chunk_count: u32,
    mime_type: Option<String>,
    encrypted: bool,
}
```

#### `ARTIFACT_CHUNK (0x0203)`
Chunk of artifact content.

```rust
struct ArtifactChunkMessage {
    artifact_id: [u8; 32],
    chunk_index: u32,
    chunk_data: Vec<u8>,  // Max 64KB
    is_final: bool,
}
```

#### `ARTIFACT_COMPLETE (0x0204)`
Confirms successful artifact transfer.

```rust
struct ArtifactCompleteMessage {
    artifact_id: [u8; 32],
    received_hash: [u8; 32], // Verify integrity
}
```

### 4. Presence Messages (0x03xx)

#### `HEARTBEAT (0x0301)`
Periodic keepalive message.

```rust
struct HeartbeatMessage {
    timestamp: i64,
    battery_level: Option<u8>, // 0-100, mobile only
    connectivity: ConnectivityStatus,
}

enum ConnectivityStatus {
    Wifi,
    Cellular,
    Ethernet,
    Unknown,
}
```

#### `PRESENCE_UPDATE (0x0302)`
Device status changed.

```rust
struct PresenceUpdateMessage {
    status: PresenceStatus,
    message: Option<String>,
}

enum PresenceStatus {
    Online,
    Away,
    Busy,
    Offline,
}
```

### 5. Error Messages (0x04xx)

#### `ERROR (0x0401)`
Error occurred during sync.

```rust
struct ErrorMessage {
    error_code: ErrorCode,
    error_message: String,
    related_message_id: Option<[u8; 16]>,
}

enum ErrorCode {
    UnknownError = 0,
    ProtocolVersionMismatch = 1,
    InvalidMessage = 2,
    ArtifactNotFound = 3,
    SyncConflict = 4,
    QuotaExceeded = 5,
    // ...
}
```

## Synchronization Flow

### Initial Sync (Full State)

```
Device A                          Device B
   │                                 │
   │──────── HELLO ──────────────────►
   │                                 │
   │◄─────── HELLO_ACK ───────────────│
   │         (sync_required=true)    │
   │                                 │
   │──────── SYNC_REQUEST ───────────►
   │         (full_sync=true)        │
   │                                 │
   │◄─────── SYNC_OPERATION ──────────│
   │◄─────── SYNC_OPERATION ──────────│
   │◄─────── SYNC_OPERATION ──────────│
   │         ... (all operations)    │
   │◄─────── SYNC_COMPLETE ───────────│
   │                                 │
   │──────── ARTIFACT_REQUEST ────────►
   │         (for missing artifacts) │
   │                                 │
   │◄─────── ARTIFACT_METADATA ───────│
   │◄─────── ARTIFACT_CHUNK ──────────│
   │◄─────── ARTIFACT_CHUNK ──────────│
   │◄─────── ARTIFACT_COMPLETE ───────│
   │                                 │
```

### Incremental Sync (Real-time)

```
Device A                          Device B
   │                                 │
   │  Local change occurs            │
   │                                 │
   │──────── SYNC_OPERATION ─────────►
   │                                 │
   │         (operation applied)     │
   │◄─────── ACK ─────────────────────│
   │                                 │
```

### Conflict Resolution

Conflicts are resolved at the CRDT level, not the protocol level. The protocol simply delivers operations in causal order.

```
Device A                          Device B
   │                                 │
   │  Edit document at t=100         │  Edit same doc at t=101
   │                                 │
   │──────── SYNC_OPERATION ─────────►
   │         (timestamp=100)         │
   │                                 │
   │◄─────── SYNC_OPERATION ──────────│
   │         (timestamp=101)         │
   │                                 │
   │  CRDT merges operations         │  CRDT merges operations
   │  Result: conflict-free state    │  Result: conflict-free state
   │                                 │
```

## Endpoint Priority

When a device has multiple endpoints (LAN IP, public IP), clients prioritize:

1. **Direct LAN**: Fastest, most reliable
2. **Local Network via Gateway**: Still local, slightly slower
3. **Public IP with Port Forward**: Higher latency
4. **Manual Endpoint**: User-configured

```rust
struct Endpoint {
    address: SocketAddr,
    priority: u8,        // 0=highest
    last_success: Option<DateTime<Utc>>,
    rtt_ms: Option<u64>, // Round-trip time
}
```

Clients attempt endpoints in priority order with exponential backoff for failures.

## Flow Control

QUIC provides built-in flow control, but we add application-level controls:

### Rate Limiting
- Max 100 operations/second per stream
- Max 10 MB/second artifact transfer
- Backpressure signals to slow down senders

### Prioritization
- Control messages: Highest priority
- Real-time sync operations: High priority
- Artifact requests: Medium priority
- Bulk transfers: Low priority

## Encryption

All messages are encrypted via QUIC/TLS, but artifacts are additionally encrypted at rest:

```rust
struct EncryptedArtifact {
    artifact_id: [u8; 32],        // Content hash of plaintext
    encrypted_data: Vec<u8>,       // ChaCha20-Poly1305
    nonce: [u8; 12],              // Random nonce
    encrypted_for: Vec<String>,    // Device IDs with access
}
```

## Error Handling

### Transient Errors
- Network timeout → Retry with exponential backoff
- Stream error → Open new stream
- Rate limit → Wait and retry

### Permanent Errors
- Protocol version mismatch → Alert user to update
- Authentication failure → Require re-pairing
- Quota exceeded → Alert user, stop sync

## Performance Optimizations

### Batching
Group multiple small operations into single message to reduce overhead.

```rust
struct BatchedSyncMessage {
    operations: Vec<SyncOperationMessage>,
}
```

### Delta Sync
Only send changed fields, not full objects.

```rust
struct DeltaSyncMessage {
    entity_id: String,
    changed_fields: HashMap<String, Value>,
}
```

### Compression
Enable QUIC-level compression for large payloads.

## Observability

### Metrics
- Operations synced per second
- Bytes transferred per second
- Sync latency (operation created → applied remotely)
- Error rate per message type

### Logging
- All sync operations logged locally
- Connection events logged
- Errors logged with context

## Testing

### Unit Tests
- Message serialization/deserialization
- Operation ordering
- Error handling

### Integration Tests
- Two-device sync
- Conflict resolution
- Network interruption recovery
- Large artifact transfer

### Chaos Tests
- Random packet loss
- Connection interruptions
- Clock skew
- Concurrent operations

## Future Enhancements

### v1.1
- [ ] Compression for CRDT operations
- [ ] Incremental artifact sync (rsync-like)
- [ ] Peer-assisted relay (optional)

### v2.0
- [ ] Multi-hop sync (A ↔ B ↔ C)
- [ ] Selective sync (filter by entity)
- [ ] Bandwidth-adaptive quality
- [ ] Background sync scheduling

---

**Protocol Version**: 1
**Last Updated**: 2026-01-31
