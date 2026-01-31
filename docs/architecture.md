# Architecture Overview

## Introduction

Nomade is a privacy-first, local-first AI assistant designed for cross-platform operation (macOS, Windows, iOS, Android). This document describes the high-level architecture, component interactions, and design principles.

## Design Principles

1. **Privacy-First**: Data stays local; user controls what syncs
2. **Local-First**: Works offline; sync is optional and peer-to-peer
3. **Security by Design**: Encryption at rest, secure pairing, authenticated sync
4. **Cross-Platform**: Native experience on all target platforms
5. **Extensible**: Plugin system for future expansion

## System Architecture

### High-Level Overview

```
┌─────────────────────────────────────────────────────┐
│            Flutter Application Layer                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────┐ │
│  │   UI Layer   │  │  Domain Layer│  │ Protocol │ │
│  │ (nomade_ui)  │  │(nomade_domain)│  │  Layer   │ │
│  └──────┬───────┘  └──────┬───────┘  └────┬─────┘ │
│         │                 │                │       │
│         └─────────────────┴────────────────┘       │
│                           │                         │
│             Nomade Native (Bridge)                  │
│                           │                         │
└───────────────────────────┼─────────────────────────┘
                            │
┌───────────────────────────┼─────────────────────────┐
│            Rust Core Library (nomade_core_rs)       │
│  ┌─────────┐  ┌─────────┐  ┌──────────┐  ┌──────┐ │
│  │  QUIC   │  │ Crypto  │  │ Storage  │  │Events│ │
│  │ Client/ │  │Identity │  │Artifacts │  │Stream│ │
│  │ Server  │  │ Keys    │  │  Store   │  │      │ │
│  └─────────┘  └─────────┘  └──────────┘  └──────┘ │
└─────────────────────────────────────────────────────┘
```

### Component Descriptions

#### Flutter Application Layer

**nomade_app/**
- Main application entry point
- Platform-specific initialization
- Navigation and routing
- State management

**packages/nomade_ui/**
- Reusable UI components
- Design system (colors, typography, spacing)
- Platform-adaptive widgets
- Accessibility support

**packages/nomade_domain/**
- Business logic and use cases
- Domain models (Document, Artifact, Device, etc.)
- Service interfaces
- Data transformation logic

**packages/nomade_protocol/**
- Sync protocol coordination
- Device discovery
- Connection management
- Conflict resolution strategy

**packages/nomade_native/**
- Bridge package (Dart FFI)
- Links `nomade_core` via static/dynamic linking
- Contains generated bridge code (from `flutter_rust_bridge`)


#### Rust Core Library

**core/nomade_core_rs/**
- High-performance core functionality
- Cryptographic operations
- QUIC-based networking
- Storage engine
- Event stream for real-time updates

**Crates:**
- `nomade_core`: Main library crate with FRB bindings
- `nomade_quic`: QUIC client/server implementation
- `nomade_crypto`: Identity keys, encryption, QR payloads
- `nomade_storage`: Artifact store, content-addressed storage
- `nomade_events`: Event stream and subscription system

### Data Flow

#### Document Creation Flow

1. User creates/imports document in UI
2. UI layer sends request to domain layer
3. Domain layer prepares artifact (content + metadata)
4. Request crosses FFI boundary via FRB
5. Rust core:
   - Generates content-addressed ID
   - Creates embeddings (if AI enabled)
   - Encrypts embeddings
   - Stores artifact in local store
   - Emits event
6. Event propagates back to Flutter
7. UI updates to reflect new document

#### Sync Flow

1. Device A and Device B are paired (see pairing.md)
2. Device A detects change (new/modified artifact)
3. Sync protocol evaluates policy:
   - Plaintext metadata: sync
   - Encrypted embeddings: sync
   - Blob/chunk text: skip (v1 default)
4. Device A establishes QUIC connection to Device B
5. Authenticated transfer of selected data
6. Device B validates, stores, and emits event
7. Both UIs update

## Technology Choices

### Flutter + Rust

**Why Flutter:**
- Single codebase for iOS, Android, macOS, Windows
- High-performance rendering
- Rich ecosystem and widgets
- Hot reload for fast iteration

**Why Rust:**
- Memory safety without garbage collection
- Excellent cryptography libraries
- High-performance networking (QUIC)
- Strong type system prevents bugs

**Why flutter_rust_bridge:**
- Automatic FFI binding generation
- Type-safe Dart ↔ Rust communication
- Supports async operations
- Minimal boilerplate

### QUIC Protocol

- Built on UDP for lower latency than TCP
- TLS 1.3 integrated for security
- Connection migration (important for mobile)
- Multiplexed streams (parallel transfers)
- 0-RTT connection resumption

### CRDT Data Model

- Conflict-free merges for multi-device sync
- No central coordination needed
- Eventually consistent
- Suitable for offline-first architecture

## Deployment Architecture

### Platform Targets

- **macOS**: Native app bundle (.app)
- **Windows**: Native executable (.exe)
- **iOS**: App Store package (.ipa)
- **Android**: APK/AAB bundle

### Storage Locations

- **macOS**: `~/Library/Application Support/Nomade/`
- **Windows**: `%APPDATA%\Nomade\`
- **iOS**: App sandbox container
- **Android**: App-specific internal storage

### Network Architecture

**LAN-First Mode (Default):**
```
[Device A] <--QUIC/mDNS--> [Device B]
```

**Port-Forward Mode (Optional):**
```
[Device A] <--QUIC/Internet--> [NAT/Firewall] <--QUIC--> [Device B]
```

**No Third-Party Relay:**
- Users maintain control
- No external dependencies
- Manual configuration for Internet access

## Security Architecture

See [Threat Model](threat-model.md) for detailed security analysis.

**Key Principles:**
- Defense in depth
- Minimize attack surface
- Cryptographic authentication
- Encrypted data at rest
- Secure key management

## Performance Considerations

### Optimization Strategies

1. **Lazy Loading**: Load artifacts on-demand
2. **Incremental Sync**: Only sync changes, not full documents
3. **Efficient Embeddings**: Use quantized vectors where possible
4. **Connection Pooling**: Reuse QUIC connections
5. **Background Sync**: Non-blocking UI during sync operations

### Scalability

- **Local Storage**: Supports 10,000+ documents per device
- **Sync**: Optimized for 2-5 devices (typical use case)
- **Embeddings**: Compressed storage format

## Future Architecture Evolution

### Plugin System

- Extensible tool/function calling
- Third-party AI model support
- Custom sync policies
- UI extensions

### Repository Split (Post-MVP)

As project matures, consider splitting monorepo:
- `nomade-app`: Flutter application
- `nomade-core`: Rust core library
- `nomade-plugins`: Plugin ecosystem

This maintains modularity while improving:
- Independent versioning
- Clearer boundaries
- Community contribution focus

## Conclusion

Nomade's architecture balances:
- **User Privacy**: Local-first with optional sync
- **Cross-Platform**: Flutter for UI consistency
- **Performance**: Rust for critical paths
- **Security**: Encryption and authentication throughout
- **Extensibility**: Plugin system for future growth

Next steps: Review specific subsystem documentation (sync protocol, data model, etc.).
