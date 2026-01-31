# Nomade Core (Rust)

Core Rust library for Nomade, providing cryptography, networking, storage, and event handling.

## Crates

- **nomade_core**: Main library crate integrating all components
- **nomade_crypto**: Cryptographic primitives (Ed25519, AES-256-GCM, HKDF, QR payloads)
- **nomade_quic**: QUIC client/server for secure sync protocol
- **nomade_storage**: Artifact store interface and implementations
- **nomade_events**: Event stream system for real-time updates

## Building

```bash
cargo build
```

## Testing

```bash
cargo test --all
```

## Linting

```bash
cargo fmt --all
cargo clippy --all -- -D warnings
```

## Architecture

This workspace provides the performance-critical and security-sensitive components of Nomade:

- **Identity Management**: Ed25519 keypairs for device authentication
- **QR Code Pairing**: Encode/decode pairing offers for secure device connection
- **Encryption**: AES-256-GCM encryption for embeddings and sensitive data
- **QUIC Transport**: (Planned) High-performance, multiplexed sync protocol
- **Storage**: Content-addressed artifact storage
- **Events**: Real-time event notifications across the application

## Flutter Integration

This Rust code is exposed to Flutter via `flutter_rust_bridge` (FRB). Bindings are generated automatically and used by the Flutter packages.

## License

MIT License - see LICENSE file in repository root
