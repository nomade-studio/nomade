# Nomade Protocol

Synchronization protocol coordination for the Nomade application.

## Overview

This package handles the synchronization protocol between devices in the Nomade network. It provides high-level coordination logic that integrates with the Rust core library for actual network operations.

## Features

- **Sync Coordination**: Manages synchronization state and lifecycle
- **Device Discovery**: Coordinates device discovery and connection
- **Artifact Synchronization**: Handles artifact distribution across devices
- **State Management**: Tracks sync status and connected devices

## Usage

```dart
import 'package:nomade_protocol/nomade_protocol.dart';

// Create coordinator
final coordinator = SyncCoordinator();

// Initialize
await coordinator.initialize();

// Start syncing
await coordinator.startSync();

// Check status
final status = coordinator.status;
print('Sync state: ${status.state}');
print('Connected devices: ${status.connectedDevices}');

// Sync an artifact
final artifact = Artifact(/* ... */);
await coordinator.syncArtifact(artifact);

// Stop syncing
await coordinator.stopSync();

// Clean up
await coordinator.dispose();
```

## Architecture

The protocol layer acts as a bridge between:
- **Flutter/Dart Layer**: High-level app logic and UI
- **Rust Core**: Low-level networking, cryptography, and storage

This separation allows for:
- Native performance where it matters
- Dart flexibility for business logic
- Cross-platform compatibility

## Future Integration

Will integrate with Rust core via flutter_rust_bridge for:
- mDNS device discovery
- Encrypted peer-to-peer connections
- Efficient binary protocol
- CRDT-based synchronization
