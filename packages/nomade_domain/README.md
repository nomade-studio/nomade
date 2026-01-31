# Nomade Domain

Domain models and business logic for the Nomade application.

## Overview

This package contains the core domain models and business logic used throughout the Nomade application. It is platform-agnostic and can be used in both Flutter and Dart applications.

## Features

- **Artifact Models**: Representation of shareable items (clipboard content, files, etc.)
- **Device Models**: Representation of devices in the Nomade network
- **Type Safety**: Strongly-typed models with JSON serialization support
- **Immutability**: All models are immutable with `copyWith` methods for updates

## Usage

```dart
import 'package:nomade_domain/nomade_domain.dart';

// Create an artifact
final artifact = Artifact(
  id: 'unique-id',
  type: ArtifactType.text,
  deviceId: 'device-123',
  timestamp: DateTime.now(),
  content: 'Hello, Nomade!',
);

// Create a device
final device = Device(
  id: 'device-123',
  name: 'My Laptop',
  type: DeviceType.desktop,
  platform: 'macOS',
  lastSeen: DateTime.now(),
  isOnline: true,
);

// JSON serialization
final artifactJson = artifact.toJson();
final deviceFromJson = Device.fromJson(deviceJson);
```

## Future Integration

This package will integrate with the Rust core library via flutter_rust_bridge for:
- Native performance
- Cross-platform compatibility
- Advanced cryptographic operations
- Efficient data synchronization
