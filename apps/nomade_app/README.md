# Nomade App

The main Flutter application for Nomade - a cross-device clipboard and file sharing tool.

## Supported Platforms

- macOS
- Windows
- iOS
- Android

## Getting Started

### Prerequisites

- Flutter SDK (>=3.0.0)
- For iOS/macOS: Xcode
- For Android: Android Studio
- For Windows: Visual Studio 2022

### Running the App

```bash
# Get dependencies
flutter pub get

# Run on connected device
flutter run

# Run on specific platform
flutter run -d macos
flutter run -d windows
flutter run -d android
flutter run -d ios
```

### Building

```bash
# Build for macOS
flutter build macos

# Build for Windows
flutter build windows

# Build for Android
flutter build apk

# Build for iOS
flutter build ios
```

## Architecture

The app follows a layered architecture:

- **UI Layer**: Uses `nomade_ui` package for shared widgets and themes
- **Domain Layer**: Uses `nomade_domain` package for business logic and models
- **Protocol Layer**: Uses `nomade_protocol` package for sync coordination
- **Core Layer**: Integrates with Rust core via flutter_rust_bridge

## Development

This app is part of the Nomade monorepo. See the root README for more information.
