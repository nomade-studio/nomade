# Nomade

> A fully local, privacy-first AI assistant for desktop and mobile

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

Nomade is an open-source, cross-platform AI assistant that keeps your data under your control. With local-first architecture, end-to-end encryption, and peer-to-peer synchronization, Nomade ensures your conversations, documents, and AI-generated insights never leave your devices without your explicit consent.

## ✨ Vision

We believe AI assistants should be:

- **Private**: Your data belongs to you, not in the cloud
- **Powerful**: Full RAG capabilities with local vector search
- **Portable**: Seamlessly sync across all your devices
- **Transparent**: Open source and auditable
- **Extensible**: Plugin architecture for custom workflows

Nomade enables you to build a personal knowledge base, have intelligent conversations with your documents, and maintain context across devices—all while keeping your data local and secure.

## 🔑 Key Features

### Local-First Architecture
- ✅ No third-party relay servers
- ✅ All processing happens on your devices
- ✅ LAN-first synchronization
- ✅ Optional manual port-forwarding for remote access

### Privacy & Security
- 🔒 Encryption at rest for embeddings and artifacts
- 🔒 One-time QR code pairing with key pinning
- 🔒 QUIC transport protocol for secure communication
- 🔒 Plaintext metadata for local search, encrypted content

### Cross-Platform
- 📱 iOS and Android mobile apps
- 💻 macOS and Windows desktop apps
- 🐧 Linux support (coming soon)

### Intelligent Features
- 🧠 RAG (Retrieval-Augmented Generation) for document Q&A
- 🔍 Local vector search for semantic queries
- 📝 CRDT-based state synchronization
- 🔄 Real-time sync across devices
- 🎯 Extensible plugin system (planned)

## 🏗️ Architecture

Nomade uses a hybrid Flutter + Rust architecture:

- **Frontend**: Flutter for cross-platform UI (mobile + desktop)
- **Core**: Rust for networking, encryption, storage, and sync
- **Bridge**: flutter_rust_bridge for seamless Dart ↔ Rust communication
- **Transport**: QUIC protocol for efficient, secure networking
- **Sync**: CRDT for conflict-free state synchronization

```
┌─────────────────────────────────────┐
│   Flutter UI (Dart)                 │
│   - Mobile & Desktop Apps           │
│   - Shared Components               │
└──────────────┬──────────────────────┘
               │ FFI Bridge
┌──────────────▼──────────────────────┐
│   Rust Core                         │
│   - QUIC Networking                 │
│   - Encryption & Key Management     │
│   - Artifact Store (Content-Addr)   │
│   - CRDT State Machine              │
│   - Vector Search (Planned)         │
└─────────────────────────────────────┘
```

For detailed architecture, see [docs/architecture.md](docs/architecture.md).

## 🚀 Quick Start

### Prerequisites

- **Flutter SDK** (latest stable): [Install Flutter](https://docs.flutter.dev/get-started/install)
- **Rust** (1.70+): [Install Rust](https://rustup.rs/)
- **Git**

### Installation

```bash
# Clone the repository
git clone https://github.com/nomade-studio/nomade.git
cd nomade

# Set up Flutter dependencies
cd apps/nomade_app
flutter pub get
cd ../..

# Build Rust core
cd core/nomade_core_rs
cargo build
cd ../..

# Run on desktop (macOS/Windows)
cd apps/nomade_app
flutter run -d macos  # or 'windows' or 'linux'

# Run on mobile (requires connected device/emulator)
flutter run -d ios      # or 'android'
```

### Development Setup

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed development environment setup instructions.

## 📚 Documentation

- [Architecture Overview](docs/architecture.md) - System design and component interactions
- [Threat Model](docs/threat-model.md) - Security analysis and considerations
- [Pairing Protocol](docs/pairing.md) - QR code format and key exchange
- [Sync Protocol](docs/sync-protocol.md) - QUIC streams and message types
- [Data Model](docs/data-model.md) - CRDT schema and data structures
- [Artifacts](docs/artifacts.md) - Content-addressed storage and encryption
- [RAG Pipeline](docs/rag.md) - Ingestion, embedding, and retrieval
- [Tooling & Plugins](docs/tooling.md) - Extension architecture

## 🗺️ Roadmap

### Phase 1: Core Foundation (Current)
- [x] Repository setup and project structure
- [x] Basic documentation
- [ ] Flutter app skeleton
- [ ] Rust core implementation
  - [ ] Identity and key management
  - [ ] QR pairing
  - [ ] QUIC networking basics
  - [ ] Artifact store interface
- [ ] CI/CD pipeline

### Phase 2: Synchronization
- [ ] CRDT state synchronization
- [ ] LAN device discovery
- [ ] Peer-to-peer connection establishment
- [ ] Conflict resolution
- [ ] Encryption at rest

### Phase 3: RAG Features
- [ ] Document ingestion pipeline
- [ ] Local embedding generation
- [ ] Vector search integration
- [ ] Context-aware retrieval
- [ ] Conversation management

### Phase 4: Production Ready
- [ ] Performance optimization
- [ ] Battery efficiency (mobile)
- [ ] Robust error handling
- [ ] Comprehensive testing
- [ ] Security audit
- [ ] User documentation

### Phase 5: Extensibility
- [ ] Plugin system design
- [ ] Connector architecture
- [ ] Custom model integration
- [ ] Export/import functionality
- [ ] Repository split for plugins

### Phase 6: Advanced Features
- [ ] Linux support
- [ ] Collaborative features
- [ ] Advanced analytics
- [ ] Cloud backup (encrypted, optional)
- [ ] Multi-device conflict visualization

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on:

- Setting up your development environment
- Code style and conventions
- Submitting pull requests
- Project structure and architecture

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

## 🔒 Security

Security is a top priority for Nomade. Please review our [Security Policy](SECURITY.md) for:

- Reporting vulnerabilities responsibly
- Security best practices
- Current security features and roadmap

**Never disclose security vulnerabilities publicly.** Report them to [security@nomade.studio](mailto:security@nomade.studio).

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright © 2026 Nomade Studio

## 🌟 Acknowledgments

- Built with [Flutter](https://flutter.dev/) and [Rust](https://www.rust-lang.org/)
- Powered by [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge)
- Inspired by local-first software principles

## 📞 Contact & Community

- **Issues**: [GitHub Issues](https://github.com/nomade-studio/nomade/issues)
- **Discussions**: [GitHub Discussions](https://github.com/nomade-studio/nomade/discussions)
- **Email**: [hello@nomade.studio](mailto:hello@nomade.studio)

---

Made with ❤️ by the Nomade community
