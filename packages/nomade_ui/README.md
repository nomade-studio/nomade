# Nomade UI

Shared UI components for the Nomade application.

## Components

- `DocumentCard`: A card widget for displaying document previews
- (More components to be added)

## Theme

- `AppTheme`: Application-wide theme configuration

## Usage

```dart
import 'package:nomade_ui/nomade_ui.dart';

DocumentCard(
  title: 'My Document',
  preview: 'Document preview text...',
  updatedAt: DateTime.now(),
  onTap: () {
    // Handle tap
  },
)
```
