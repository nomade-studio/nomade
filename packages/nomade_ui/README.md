# Nomade UI

Reusable UI components and widgets for the Nomade application.

## Overview

This package contains all shared UI components, widgets, and themes used across the Nomade Flutter application. It provides a consistent design language and reusable building blocks.

## Features

- **Artifact Card**: Display artifact items with appropriate icons and formatting
- **Device List Tile**: Show device information with online status
- **Sync Status Indicator**: Visual indicator for synchronization state
- **Consistent Theming**: Shared styles and design tokens

## Usage

```dart
import 'package:nomade_ui/nomade_ui.dart';
import 'package:nomade_domain/nomade_domain.dart';
import 'package:nomade_protocol/nomade_protocol.dart';

// Display an artifact
ArtifactCard(
  artifact: artifact,
  onTap: () => print('Artifact tapped'),
)

// Display a device
DeviceListTile(
  device: device,
  onTap: () => print('Device selected'),
)

// Show sync status
SyncStatusIndicator(
  status: syncStatus,
)
```

## Widget Catalog

### ArtifactCard

Displays an artifact (clipboard item, file, etc.) with:
- Type-appropriate icon
- Content preview
- Source device
- Timestamp

### DeviceListTile

Shows device information including:
- Device type icon
- Device name
- Platform and status
- Online/offline indicator

### SyncStatusIndicator

A status badge showing:
- Current sync state (idle, syncing, connected, etc.)
- Number of connected devices
- Color-coded status
- Appropriate icon

## Design Principles

- **Material Design 3**: Uses the latest Material Design components
- **Accessibility**: All widgets support screen readers and high contrast
- **Responsive**: Adapts to different screen sizes
- **Themeable**: Respects system theme and custom themes
