# Contributing to Nomade

Thank you for your interest in contributing to Nomade! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

This project adheres to the Contributor Covenant Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## Getting Started

### Prerequisites

- **Flutter**: Latest stable version for cross-platform app development
- **Rust**: Latest stable version (1.70+) for core functionality
- **flutter_rust_bridge**: For Flutter ↔ Rust integration

### Repository Structure

```
nomade/
├── apps/
│   └── nomade_app/          # Flutter application shell
├── packages/
│   ├── nomade_domain/       # Domain models and business logic
│   ├── nomade_protocol/     # Protocol definitions and sync logic
│   └── nomade_ui/           # Reusable UI components
├── core/
│   └── nomade_core_rs/      # Rust core library with FRB bindings
├── docs/                    # Architecture and design documentation
└── .github/                 # CI/CD workflows
```

## Development Workflow

### 1. Fork and Clone

```bash
git clone https://github.com/YOUR_USERNAME/nomade.git
cd nomade
```

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 3. Make Changes

- Write clear, concise commit messages
- Follow existing code style and conventions
- Add tests for new functionality
- Update documentation as needed

### 4. Run Tests and Linting

**Flutter/Dart:**
```bash
cd apps/nomade_app
flutter test
dart format --set-exit-if-changed .
dart analyze
```

**Rust:**
```bash
cd core/nomade_core_rs
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

### 5. Submit a Pull Request

- Push your changes to your fork
- Create a pull request against the `main` branch
- Provide a clear description of your changes
- Reference any related issues

## Code Style Guidelines

### Dart/Flutter

- Follow the [Dart style guide](https://dart.dev/guides/language/effective-dart/style)
- Use `dart format` for consistent formatting
- Prefer composition over inheritance
- Write descriptive variable and function names

### Rust

- Follow the [Rust API guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Document public APIs with `///` doc comments
- Write idiomatic Rust code
- Prefer strong typing and explicit error handling

## Testing

- Write unit tests for all new functionality
- Ensure tests are deterministic and isolated
- Test edge cases and error conditions
- Maintain test coverage above 80%

## Documentation

- Update README.md if adding user-facing features
- Add/update architecture docs in `docs/` for design changes
- Document public APIs and complex algorithms
- Include code examples where helpful

## Security

- Report security vulnerabilities privately (see SECURITY.md)
- Never commit secrets, keys, or credentials
- Follow secure coding practices
- Consider privacy implications of changes

## Pull Request Process

1. **Code Review**: All submissions require review by project maintainers
2. **CI Checks**: All CI checks must pass before merging
3. **Documentation**: Ensure documentation is updated
4. **Changelog**: Update CHANGELOG.md if applicable
5. **Squash Commits**: Maintainers may squash commits before merging

## Communication

- **Issues**: Use GitHub Issues for bug reports and feature requests
- **Discussions**: Use GitHub Discussions for questions and ideas
- **Pull Requests**: Use PR comments for code review feedback

## License

By contributing to Nomade, you agree that your contributions will be licensed under the MIT License.

## Questions?

If you have questions about contributing, please open a GitHub Discussion or reach out to the maintainers.

Thank you for contributing to Nomade! 🚀
