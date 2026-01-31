# Architecture Overview

## System Architecture

Nomade is built on a **local-first, peer-to-peer architecture** that prioritizes privacy, security, and user control. The system combines Flutter for cross-platform UI with Rust for performance-critical operations, connected via FFI bridges.

## High-Level Architecture

```
┌───────────────────────────────────────────────────────────────┐
│                     User Devices                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │   Desktop    │  │    Mobile    │  │   Tablet     │        │
│  │  (Mac/Win)   │  │  (iOS/And)   │  │  (iOS/And)   │        │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘        │
│         │                 │                 │                 │
│         └─────────────────┴─────────────────┘                 │
│                           │                                    │
│                    Local Network                               │
│                    (LAN + Optional WAN)                        │
│                           │                                    │
│                    QUIC Transport                              │
│                    (Encrypted P2P)                             │
└───────────────────────────────────────────────────────────────┘
```

## Component Architecture

### Flutter Layer (UI)

```
apps/
└── nomade_app/                    # Main application
    ├── lib/
    │   ├── main.dart             # Entry point
    │   ├── screens/              # UI screens
    │   ├── widgets/              # App-specific widgets
    │   └── services/             # FFI service wrappers
    └── pubspec.yaml

packages/
├── nomade_ui/                     # Shared UI components
│   ├── lib/
│   │   ├── components/           # Reusable widgets
│   │   ├── theme/                # Design tokens
│   │   └── utils/                # UI utilities
│   └── pubspec.yaml
│
├── nomade_domain/                 # Domain models
│   ├── lib/
│   │   ├── models/               # Data classes
│   │   ├── entities/             # Domain entities
│   │   └── value_objects/        # Value objects
│   └── pubspec.yaml
│
└── nomade_protocol/               # Protocol definitions
    ├── lib/
    │   ├── messages/             # Message schemas
    │   ├── codecs/               # Serialization
    │   └── constants/            # Protocol constants
    └── pubspec.yaml
```

### Rust Core Layer

```
core/nomade_core_rs/
├── Cargo.toml                     # Workspace manifest
├── nomade_core/                   # Main core crate
│   ├── src/
│   │   ├── lib.rs                # Library root + FFI exports
│   │   ├── identity/             # Key management
│   │   ├── pairing/              # QR pairing logic
│   │   ├── networking/           # QUIC client/server
│   │   ├── artifacts/            # Content-addressed store
│   │   ├── sync/                 # CRDT sync engine
│   │   ├── crypto/               # Encryption helpers
│   │   └── bridge/               # Flutter bridge code
│   └── Cargo.toml
└── README.md                      # Build instructions
```

## Data Flow

### 1. UI Interaction
```
User Action
    ↓
Flutter Widget
    ↓
Dart Service Layer
    ↓
FFI Bridge (flutter_rust_bridge)
    ↓
Rust Core
```

### 2. Device Pairing
```
Device A (Initiator)
    ↓
Generate QR Code (public key + endpoint)
    ↓
Device B scans QR
    ↓
Extract public key + endpoint
    ↓
Establish QUIC connection
    ↓
Exchange identity keys
    ↓
Pin keys for future connections
```

### 3. Synchronization
```
Local Change (Device A)
    ↓
Update Local CRDT State
    ↓
Generate Change Event
    ↓
Send via QUIC to Paired Devices
    ↓
Device B receives event
    ↓
Merge with Local CRDT State
    ↓
Resolve conflicts (CRDT guarantees)
    ↓
Update UI
```

### 4. Artifact Storage
```
New Document/Embedding
    ↓
Generate Content Hash (SHA-256)
    ↓
Encrypt Artifact (if embedding)
    ↓
Store in Local Artifact Store
    ↓
Reference in CRDT State
    ↓
Sync artifact to paired devices
    ↓
Devices download by content hash
```

## Key Technologies

### Flutter Stack
- **Flutter SDK**: Cross-platform UI framework
- **Dart**: Primary language for UI
- **flutter_rust_bridge**: FFI bridge to Rust
- **Riverpod/Provider**: State management (TBD)
- **sqflite/hive**: Local database (TBD)

### Rust Stack
- **quinn**: QUIC implementation
- **rustls**: TLS/crypto
- **serde**: Serialization
- **tokio**: Async runtime
- **automerge/yrs**: CRDT library (TBD)
- **blake3/sha2**: Hashing
- **chacha20poly1305**: Encryption

## Deployment Targets

| Platform | Support | Notes |
|----------|---------|-------|
| macOS    | ✅ Priority | Desktop primary target |
| Windows  | ✅ Priority | Desktop primary target |
| iOS      | ✅ Priority | Mobile primary target |
| Android  | ✅ Priority | Mobile primary target |
| Linux    | 🔄 Future | Post v1.0 |
| Web      | ❌ Not planned | Incompatible with local-first architecture |

## Network Architecture

### LAN Discovery
- mDNS/Bonjour for device discovery on local network
- Broadcast presence announcements
- Automatic peer connection

### Connection Types
1. **Direct LAN**: Local network, no configuration needed
2. **Manual Endpoint**: User configures IP:Port for remote access
3. **Port Forward**: User sets up port forwarding on router

### No Third-Party Relay
- All connections are direct peer-to-peer
- No intermediary servers
- No data passes through third parties
- User maintains full control

## Security Layers

1. **Transport Security**: QUIC with TLS 1.3
2. **Key Pinning**: First connection pins device public keys
3. **Encryption at Rest**: Artifacts encrypted with device-specific keys
4. **Access Control**: Only paired devices can connect
5. **No Cloud**: All data stays on user devices

## Extension Points

### Plugin System (Future)
```
plugins/
├── nomade_plugin_obsidian/       # Obsidian integration
├── nomade_plugin_notion/         # Notion connector
└── nomade_plugin_custom/         # User-defined plugins
```

Plugins will have access to:
- Document ingestion API
- Embedding generation API
- Search/retrieval API
- UI extension points

## Performance Considerations

- **Lazy Loading**: Load documents on demand
- **Incremental Sync**: Only sync changes, not full state
- **Background Processing**: Embeddings generated in background
- **Efficient CRDT**: Choose CRDT that minimizes metadata overhead
- **Connection Pooling**: Reuse QUIC connections

## Scalability

- **Vertical**: Supports thousands of documents per device
- **Horizontal**: Supports multiple paired devices (target: 5-10)
- **Network**: Efficient sync for mobile data constraints
- **Storage**: Content-addressed deduplication

## Development Workflow

```
1. Make changes in Rust core
2. Rebuild Rust library (cargo build)
3. Regenerate FFI bindings (if API changed)
4. Update Dart service layer
5. Test in Flutter app
6. Run integration tests
```

## Testing Strategy

- **Rust**: Unit tests + integration tests with cargo test
- **Flutter**: Widget tests + integration tests
- **FFI Bridge**: Cross-language integration tests
- **E2E**: Automated tests across multiple devices
- **Manual**: Multi-device sync testing

## Build & Release

- **Development**: Debug builds for testing
- **Release**: Optimized builds with:
  - Rust: `cargo build --release`
  - Flutter: `flutter build [macos|windows|ios|android]`
- **Signing**: Code signing for macOS/iOS
- **Distribution**: Direct downloads + app stores (future)

## Monitoring & Telemetry

- **Local Logs**: On-device logging for debugging
- **No Analytics**: No usage tracking or telemetry
- **Crash Reports**: Optional, local-only crash logs
- **Performance**: Local performance metrics

## Future Architecture Considerations

- **Vector Search**: Integrate fast vector DB (e.g., usearch, faiss)
- **Repository Split**: Move plugins to separate repos
- **Multi-Model Support**: Allow different embedding models
- **Conflict UI**: Visual conflict resolution for advanced users
- **Backup/Export**: Encrypted backup to user-owned storage

---

For protocol details, see [sync-protocol.md](sync-protocol.md).
For data structures, see [data-model.md](data-model.md).
For security analysis, see [threat-model.md](threat-model.md).
