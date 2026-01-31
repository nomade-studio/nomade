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
│   ├── nomade_native/       # Native bridge (FFI) to Rust core
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

### 2. Setup (Important)

We use **Lefthook** for git hooks and a **Makefile** for common tasks.

```bash
# 1. Install dependencies
make deps

# 2. Install Lefthook (if not already installed)
brew install lefthook

# 3. Enable Git Hooks (REQUIRED)
lefthook install
```

### 3. Create a Branch

**Strict Branch Naming**: `type/description`
*   Allowed types: `feat`, `bug`, `fix`, `refactor`, `docs`, `style`, `test`, `chore`, `ci`
*   Example: `feat/login-screen`, `bug/crash-fix`

```bash
git checkout -b feat/your-feature-name
```

### 4. Make Changes

- **Conventional Commits**: Messages **MUST** follow the format `type(scope): subject`.
    - Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`.
    - Example: `feat(auth): add google login`
- Run local checks frequently with `make check`.
- Update documentation as needed.

### 5. Run Verification

We provide meaningful `make` targets to help you verify your code:

```bash
# Run all checks (Format, Analyze, Test) for Rust & Flutter
make check

# Run only Rust checks
make check-rust

# Run only Flutter checks
make check-flutter

# Auto-format code
make format
``` 

## Testing

- Write unit tests for all new functionality.
- Ensure `make check` passes before pushing.
- Maintain test coverage above 80%.

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
