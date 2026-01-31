# Nomade

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

**A fully local, privacy-first AI assistant for desktop and mobile**

Nomade is an open-source, cross-platform AI assistant that keeps your data under your control. With local-first architecture, end-to-end encryption, and peer-to-peer synchronization, Nomade ensures your conversations, documents, and AI-generated insights never leave your devices without your explicit consent.

## ✨ Vision

We believe AI assistants should be:

- **Private**: Your data belongs to you, not in the cloud
- **Powerful**: Full RAG capabilities with local vector search
- **Secure Sync**: QUIC-based peer-to-peer synchronization all your devices with optional LAN-first mode
- **Extensible**: Plugin architecture for custom workflows
- **Cross-Platform**: Native apps for macOS, Windows, Linux, iOS, and Android
- **Transparent**: Open source and auditable

Nomade enables you to build a personal knowledge base, have intelligent conversations with your documents, and maintain context across devices—all while keeping your data local and secure.

## 🔑 Key Features

### Local-First Architecture
- ✅ No third-party relay servers
- ✅ All processing happens on your devices
- ✅ LAN-first synchronization
- ✅ (Optional) manual port-forwarding for remote access

## 🏗️ Architecture

Nomade is built as a monorepo with clear separation of concerns:

```
nomade/
├── apps/
│   └── nomade_app/          # Flutter cross-platform application
├── packages/
│   ├── nomade_domain/       # Domain models and business logic
│   ├── nomade_native/       # Dart FFI Bridge (links to core)
│   ├── nomade_protocol/     # Sync protocol and communication
│   └── nomade_ui/           # Reusable UI components
├── core/
│   └── nomade_core_rs/      # Rust core library (QUIC, crypto, storage)
└── docs/                    # Architecture and design documentation
```

### Technology Stack

- **Frontend**: Flutter (Dart) for cross-platform UI
- **Backend**: Rust for core functionality, performance, and security
- **Integration**: flutter_rust_bridge (FRB) for seamless Dart ↔ Rust communication
- **Sync Protocol**: QUIC with TLS 1.3 for secure, efficient data transfer
- **Data Model**: CRDT-based for conflict-free synchronization

## 🚀 Getting Started

### Prerequisites

- **Flutter SDK**: Latest stable version (3.x+)
- **Rust**: 1.70 or later
- **Platform Tools**: Xcode (macOS/iOS), Android Studio (Android), Visual Studio (Windows)

### Quick Start

1. **Clone the repository**:
   ```bash
   git clone https://github.com/nomade-studio/nomade.git
   cd nomade
   ```

2. **Setup Dependencies**:
   ```bash
   make deps
   ```

3. **Run Flutter app (macOS)**:
   ```bash
   make run-macos
   ```

### Development Workflow

- **Check Code Quality**:
  ```bash
  make check
  ```
- **Generate Bridge Code (after Rust changes)**:
  ```bash
  make gen
  ```

## 📖 Documentation

Comprehensive documentation is available in the `docs/` directory:

- [Architecture Overview](docs/architecture.md) - System design and component interactions
- [Threat Model](docs/threat-model.md) - Security analysis and mitigations
- [Device Pairing](docs/pairing.md) - Secure device-to-device connection
- [Sync Protocol](docs/sync-protocol.md) - QUIC-based synchronization
- [Data Model](docs/data-model.md) - CRDT implementation and storage
- [Artifacts System](docs/artifacts.md) - Content-addressed storage and embeddings
- [RAG Pipeline](docs/rag-pipeline.md) - Deterministic retrieval and generation
- [Tooling & Plugins](docs/tooling.md) - Extensibility and future architecture

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on:

- Development workflow
- Code style guidelines
- Testing requirements
- Pull request process

Please also read our [Code of Conduct](CODE_OF_CONDUCT.md).

## 🔒 Security

Security is a top priority for Nomade. Please see our [Security Policy](SECURITY.md) for:

- Reporting vulnerabilities
- Security design principles
- Known considerations
- Best practices

## 📋 Project Status

Nomade is in **early development**. Current focus:

- [x] Initial monorepo scaffold
- [x] Core Rust library (QUIC, crypto, storage)
- [x] Flutter app shell with basic UI
- [x] Bridge integration (`nomade_native` + `nomade_core`)
- [ ] Device pairing and identity management
- [ ] Local RAG pipeline implementation
- [ ] Cross-device sync protocol
- [ ] Production-ready releases

## 🎯 Roadmap

### Phase 1: Foundation (Current)
- [x] Monorepo structure and tooling
- [x] Core Rust libraries (QUIC, crypto, storage)
- [x] Flutter app skeleton
- [ ] Basic device pairing

### Phase 2: Core Features
- Local document management
- Semantic embeddings and search
- Basic RAG pipeline
- LAN-based sync

### Phase 3: Advanced Features
- Advanced sync policies
- Plugin system
- Multi-device orchestration
- Performance optimization

### Phase 4: Ecosystem
- Community plugins
- Third-party integrations
- Mobile-optimized features
- Repository split (mono → multi-repo)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2026 Rodrigue Ngalani Touko

## 🙏 Acknowledgments

Built with:
- [Flutter](https://flutter.dev/) - Beautiful cross-platform apps
- [Rust](https://www.rust-lang.org/) - Safe, fast systems programming
- [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) - Seamless Dart-Rust integration
- [QUIC](https://www.chromium.org/quic/) - Modern transport protocol

---

**Nomade** - Your local AI companion, truly yours. 🌍✨
