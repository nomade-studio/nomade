# Contributing to Nomade

Thank you for your interest in contributing to Nomade! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it before contributing.

## Getting Started

### Development Environment Setup

#### Prerequisites

- **Flutter SDK** (latest stable channel) for UI development
- **Rust** (latest stable) for core functionality
- **Git** for version control
- **IDE**: VS Code (recommended) or Android Studio with Flutter/Dart plugins

#### macOS Setup

```bash
# Install Flutter
# Download from https://docs.flutter.dev/get-started/install/macos
# or use homebrew:
brew install --cask flutter

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installations
flutter doctor
cargo --version

# Clone the repository
git clone https://github.com/nomade-studio/nomade.git
cd nomade

# Install Flutter dependencies
cd apps/nomade_app
flutter pub get

# Build Rust core
cd ../../core/nomade_core_rs
cargo build
```

#### Windows Setup

```bash
# Install Flutter
# Download from https://docs.flutter.dev/get-started/install/windows
# Follow the installation instructions

# Install Rust
# Download from https://rustup.rs/ and run the installer

# Verify installations
flutter doctor
cargo --version

# Clone the repository
git clone https://github.com/nomade-studio/nomade.git
cd nomade

# Install Flutter dependencies
cd apps/nomade_app
flutter pub get

# Build Rust core
cd ..\..\core\nomade_core_rs
cargo build
```

#### iOS/Android Development

- **iOS**: Requires Xcode on macOS. Run `flutter doctor` to verify setup.
- **Android**: Requires Android Studio with Android SDK. Run `flutter doctor` to verify setup.

## Development Workflow

### 1. Fork and Branch

1. Fork the repository on GitHub
2. Clone your fork locally
3. Create a new branch for your feature or fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

### 2. Make Changes

- Follow the existing code style and conventions
- Write clear, descriptive commit messages
- Keep commits focused and atomic
- Add tests for new functionality
- Update documentation as needed

### 3. Testing

Before submitting your changes, ensure all tests pass:

```bash
# Flutter/Dart tests
cd apps/nomade_app
flutter test
dart analyze
dart format --set-exit-if-changed .

# Rust tests
cd core/nomade_core_rs
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

### 4. Submit a Pull Request

1. Push your branch to your fork
2. Open a pull request against the `main` branch
3. Fill in the PR template with relevant details
4. Link any related issues
5. Wait for review and address feedback

## Code Style Guidelines

### Dart/Flutter

- Follow the [official Dart style guide](https://dart.dev/guides/language/effective-dart/style)
- Use `dart format` to automatically format code
- Maximum line length: 80 characters (with flexibility for readability)
- Use meaningful variable and function names
- Document public APIs with doc comments (`///`)

### Rust

- Follow the [official Rust style guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `cargo fmt` to automatically format code
- Use `cargo clippy` to catch common mistakes
- Document public APIs with doc comments (`///`)
- Prefer explicit types when it improves readability

## Project Structure

```
nomade/
├── apps/
│   └── nomade_app/          # Main Flutter application
├── packages/
│   ├── nomade_ui/           # Shared UI components
│   ├── nomade_domain/       # Domain models
│   └── nomade_protocol/     # Message schemas
├── core/
│   └── nomade_core_rs/      # Rust core workspace
├── docs/                    # Documentation
└── .github/
    └── workflows/           # CI/CD workflows
```

## Areas of Contribution

### High Priority

- Core QUIC networking implementation
- CRDT synchronization logic
- Encryption and security features
- Cross-platform UI components
- Documentation improvements

### Future Work

- Plugin system architecture
- Vector search integration
- Advanced RAG features
- Performance optimizations
- Additional platform support (Linux)

## Documentation

- Update relevant documentation in `docs/` for architectural changes
- Add inline code comments for complex logic
- Update README.md if user-facing features change
- Write clear commit messages following conventional commits format

## Security

If you discover a security vulnerability, please **DO NOT** open a public issue. Instead, follow our [Security Policy](SECURITY.md) and report it privately.

## Questions?

- Open a [GitHub Discussion](https://github.com/nomade-studio/nomade/discussions) for general questions
- Join our community channels (links in README)
- Check existing issues and documentation first

## License

By contributing to Nomade, you agree that your contributions will be licensed under the MIT License.

Thank you for contributing to Nomade! 🚀
