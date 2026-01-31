# Nomade Core (Rust)

The Rust core library for Nomade, implementing:

- Identity and key management
- QR pairing protocol
- QUIC networking
- Artifact storage
- CRDT synchronization
- Encryption helpers
- FFI bridge to Flutter

## Building

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Project Structure

```
nomade_core_rs/
├── Cargo.toml              # Workspace manifest
├── nomade_core/            # Main crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs          # Library root + FFI exports
│       ├── identity.rs     # Key management
│       ├── pairing.rs      # QR pairing logic
│       ├── networking.rs   # QUIC client/server
│       ├── artifacts.rs    # Content-addressed store
│       ├── sync.rs         # CRDT sync engine
│       ├── crypto.rs       # Encryption helpers
│       └── bridge.rs       # Flutter bridge code
└── README.md
```

## FFI Bridge (flutter_rust_bridge)

To set up the FFI bridge:

1. Add `flutter_rust_bridge` dependency to `Cargo.toml`
2. Annotate FFI functions with `#[flutter_rust_bridge::frb]`
3. Generate Dart bindings: `flutter_rust_bridge_codegen generate`
4. Build the native library for each target platform

See [flutter_rust_bridge documentation](https://cjycode.com/flutter_rust_bridge/) for details.

## Dependencies

- **quinn**: QUIC implementation
- **rustls**: TLS library
- **ed25519-dalek**: Ed25519 signatures
- **chacha20poly1305**: Encryption
- **serde**: Serialization
- **tokio**: Async runtime

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_generate_identity

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

## Benchmarking

```bash
# Run benchmarks (requires criterion)
cargo bench
```

## Platform Support

- Linux: ✅
- macOS: ✅
- Windows: ✅
- iOS: ✅ (via Flutter)
- Android: ✅ (via Flutter)

## Notes

- This library is designed to be called from Flutter via FFI
- Core functionality is platform-agnostic
- Platform-specific code (e.g., keychain access) should be handled in Flutter
- The library uses `cdylib` for dynamic linking and `staticlib` for static linking

## Security

- Private keys are kept in memory only
- Sensitive operations should be wrapped in secure contexts
- See [docs/threat-model.md](../../docs/threat-model.md) for security analysis

## Performance

- Release builds are optimized for performance
- Use `--release` flag for production builds
- Profile with `cargo flamegraph` for performance analysis

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for contribution guidelines.
