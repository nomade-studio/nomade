# Nomade App

The main Flutter application for Nomade.

## Getting Started

### Prerequisites

- Flutter SDK (latest stable)
- Rust toolchain (for core library)

### Running the App

```bash
# Get dependencies
flutter pub get

# Run on desktop (macOS)
flutter run -d macos

# Run on desktop (Windows)
flutter run -d windows

# Run on mobile (iOS)
flutter run -d ios

# Run on mobile (Android)
flutter run -d android
```

### Building

```bash
# Build for macOS
flutter build macos

# Build for Windows
flutter build windows

# Build for iOS
flutter build ios

# Build for Android
flutter build apk
```

## Architecture

This app follows a layered architecture:

- **UI Layer**: Screens and widgets
- **Service Layer**: FFI bridge to Rust core
- **Domain Layer**: Business logic (in `nomade_domain` package)
- **Protocol Layer**: Message schemas (in `nomade_protocol` package)

## FFI Bridge

The app communicates with the Rust core via `flutter_rust_bridge`. See `lib/services/` for FFI service wrappers.

## State Management

Uses Riverpod for state management. Providers are defined in `lib/providers/`.

## Testing

```bash
# Run all tests
flutter test

# Run with coverage
flutter test --coverage
```
