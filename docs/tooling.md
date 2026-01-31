# Tooling & Plugin Architecture

## Overview

Nomade is designed with extensibility in mind. This document describes the tooling architecture, plugin system, and the future plan for repository split as the project matures.

## Design Philosophy

1. **Core First**: Stable core functionality before plugins
2. **Well-Defined APIs**: Clear interfaces for extension
3. **Security**: Sandboxed execution, permission model
4. **Cross-Platform**: Plugins work on all supported platforms
5. **Developer-Friendly**: Simple plugin development workflow

## Plugin Architecture (Future)

### Plugin Types

#### 1. Tool Plugins

Extend AI assistant capabilities with custom tools/functions:

```rust
pub trait ToolPlugin: Send + Sync {
    /// Unique identifier for the tool
    fn id(&self) -> &str;
    
    /// Human-readable name
    fn name(&self) -> &str;
    
    /// Description for LLM to understand when to use
    fn description(&self) -> &str;
    
    /// JSON schema for parameters
    fn parameters_schema(&self) -> serde_json::Value;
    
    /// Execute the tool
    async fn execute(&self, params: serde_json::Value) -> Result<serde_json::Value>;
}
```

**Example Tool Plugin**:
```rust
pub struct WebSearchPlugin;

impl ToolPlugin for WebSearchPlugin {
    fn id(&self) -> &str { "web_search" }
    
    fn name(&self) -> &str { "Web Search" }
    
    fn description(&self) -> &str {
        "Search the web for current information. Use when user asks about recent events or data not in local documents."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search query"
                },
                "num_results": {
                    "type": "integer",
                    "description": "Number of results to return",
                    "default": 5
                }
            },
            "required": ["query"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> Result<serde_json::Value> {
        let query: String = params["query"].as_str().unwrap().to_string();
        let num_results: usize = params["num_results"].as_u64().unwrap_or(5) as usize;
        
        // Implementation...
        let results = perform_web_search(&query, num_results).await?;
        
        Ok(json!({
            "results": results
        }))
    }
}
```

#### 2. Data Source Plugins

Import data from external sources:

```rust
pub trait DataSourcePlugin: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn supported_formats(&self) -> Vec<String>;
    
    async fn import(&self, source: &str) -> Result<Vec<Artifact>>;
    async fn sync(&self, last_sync: Timestamp) -> Result<Vec<Artifact>>;
}
```

**Examples**:
- Email import (Gmail, Outlook)
- Note apps (Notion, Evernote, Obsidian)
- Cloud storage (Dropbox, Google Drive)
- Code repositories (GitHub, GitLab)

#### 3. Embedding Model Plugins

Custom or alternative embedding models:

```rust
pub trait EmbeddingPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn dimensions(&self) -> usize;
    fn max_tokens(&self) -> usize;
    
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    async fn embed_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>>;
}
```

**Examples**:
- Local models (sentence-transformers)
- Custom fine-tuned models
- Multilingual models
- Domain-specific embeddings

#### 4. UI Extension Plugins

Extend the Flutter UI:

```dart
abstract class UIPlugin {
  String get id;
  String get name;
  
  /// Widget for settings page
  Widget buildSettingsWidget(BuildContext context);
  
  /// Additional menu items
  List<MenuItem> getMenuItems();
  
  /// Custom artifact viewers
  Widget? buildArtifactViewer(Artifact artifact);
}
```

**Examples**:
- Custom artifact types (mind maps, diagrams)
- Specialized viewers (PDF annotator, code editor)
- Dashboard widgets
- Themes and visual customizations

### Plugin Distribution

#### Development Phase

```
plugins/
├── builtin/                    # Bundled with app
│   ├── web_search/
│   ├── email_import/
│   └── pdf_viewer/
└── third_party/                # User-installed
    ├── notion_sync/
    ├── github_integration/
    └── custom_theme/
```

#### Post-Split Phase

Separate repositories:
- `nomade-plugin-web-search`
- `nomade-plugin-notion`
- `nomade-plugin-github`

Distributed via:
- Official plugin registry
- GitHub releases
- Direct installation from source

### Plugin Security

#### Sandboxing

```rust
pub struct PluginSandbox {
    permissions: PermissionSet,
    resource_limits: ResourceLimits,
}

pub struct PermissionSet {
    pub network_access: bool,
    pub file_system_access: bool,
    pub local_ai_access: bool,
    pub artifact_read: bool,
    pub artifact_write: bool,
}

pub struct ResourceLimits {
    pub max_memory_mb: usize,
    pub max_cpu_percent: f32,
    pub max_execution_time_ms: u64,
}
```

**Implementation Options**:
- **WebAssembly**: Compile plugins to WASM for sandboxing
- **Process Isolation**: Run plugins in separate processes
- **Permission Manifest**: Declare required permissions

#### Permission Model

```json
{
  "plugin": {
    "id": "web_search",
    "version": "1.0.0",
    "permissions": [
      "network.http",
      "network.https"
    ],
    "resource_limits": {
      "max_memory_mb": 100,
      "max_execution_time_ms": 5000
    }
  }
}
```

**User Consent**:
- Display requested permissions before installation
- User explicitly approves
- Can revoke permissions later

## Development Tooling

### Code Organization

```
nomade/
├── apps/
│   └── nomade_app/             # Flutter app
├── packages/
│   ├── nomade_domain/          # Dart business logic
│   ├── nomade_native/          # Dart FFI Bridge (no Rust source)
│   ├── nomade_protocol/        # Dart sync protocol
│   └── nomade_ui/              # Dart UI components
├── core/
│   └── nomade_core_rs/         # Rust core library
│       ├── nomade_core/        # Main crate with FRB bindings
│       ├── nomade_quic/        # QUIC implementation
│       ├── nomade_crypto/      # Crypto primitives
│       ├── nomade_storage/     # Storage layer
│       └── nomade_events/      # Event system
├── docs/                       # Documentation
├── scripts/                    # Build and dev scripts
└── tools/                      # Development tools
```

### Build System

#### Recommended Workflow (Makefile)

```bash
# Install dependencies
make deps

# Generate Bridge Code (after editing Rust)
make gen

# Run App (macOS)
make run-macos
```

#### Manual Rust Build

```bash
# Build Rust core
cd packages/nomade_native
flutter_rust_bridge_codegen generate
```

#### Flutter Build

```bash
# Get dependencies
cd apps/nomade_app
flutter pub get

# Run on desktop
flutter run -d macos    # or windows, linux

# Run on mobile
flutter run -d ios      # or android

# Build release
flutter build macos --release
flutter build ios --release
flutter build apk --release
flutter build windows --release
```

### Development Scripts

**`scripts/setup.sh`**: One-command setup

```bash
#!/bin/bash
set -e

echo "Setting up Nomade development environment..."

# Check prerequisites
command -v cargo >/dev/null 2>&1 || { echo "Rust not installed"; exit 1; }
command -v flutter >/dev/null 2>&1 || { echo "Flutter not installed"; exit 1; }

# Install flutter_rust_bridge
cargo install flutter_rust_bridge_codegen

# Build Rust core
cd core/nomade_core_rs
cargo build

# Generate FFI bindings
flutter_rust_bridge_codegen \
    --rust-input nomade_core/src/api.rs \
    --dart-output ../../packages/nomade_domain/lib/ffi.dart

# Get Flutter dependencies
cd ../../apps/nomade_app
flutter pub get

echo "✓ Setup complete! Run 'flutter run' to start."
```

**`scripts/test.sh`**: Run all tests

```bash
#!/bin/bash
set -e

# Rust tests
cd core/nomade_core_rs
cargo test --all

# Flutter tests
cd ../../apps/nomade_app
flutter test

# Integration tests
flutter test integration_test
```

**`scripts/format.sh`**: Format all code

```bash
#!/bin/bash

# Rust formatting
cd core/nomade_core_rs
cargo fmt --all

# Dart formatting
cd ../../
find . -name "*.dart" -not -path "*/.*" -exec dart format {} +
```

### IDE Setup

#### VS Code

`.vscode/extensions.json`:
```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "dart-code.flutter",
    "dart-code.dart-code"
  ]
}
```

`.vscode/settings.json`:
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "dart.flutterSdkPath": "/path/to/flutter",
  "[dart]": {
    "editor.formatOnSave": true,
    "editor.rulers": [80]
  },
  "[rust]": {
    "editor.formatOnSave": true,
    "editor.rulers": [100]
  }
}
```

#### IntelliJ / Android Studio

- Install Rust plugin
- Install Flutter/Dart plugin
- Configure Rust toolchain
- Configure Flutter SDK

## Testing Strategy

### Unit Tests

**Rust**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_artifact_id_generation() {
        let content = b"Hello, World!";
        let id = compute_artifact_id(content);
        assert_eq!(id.to_string(), "blake3-...");
    }
    
    #[tokio::test]
    async fn test_quic_connection() {
        let server = start_test_server().await.unwrap();
        let client = connect_to_server(&server.addr()).await.unwrap();
        assert!(client.is_connected());
    }
}
```

**Dart**:
```dart
void main() {
  test('Artifact metadata serialization', () {
    final artifact = Artifact(
      id: 'blake3-...',
      title: 'Test',
      createdAt: DateTime.now(),
    );
    
    final json = artifact.toJson();
    final decoded = Artifact.fromJson(json);
    
    expect(decoded.id, equals(artifact.id));
    expect(decoded.title, equals(artifact.title));
  });
}
```

### Integration Tests

**Flutter**:
```dart
void main() {
  testWidgets('Create and sync artifact', (WidgetTester tester) async {
    await tester.pumpWidget(MyApp());
    
    // Create artifact
    await tester.tap(find.byIcon(Icons.add));
    await tester.pumpAndSettle();
    
    await tester.enterText(find.byType(TextField), 'Test Document');
    await tester.tap(find.text('Save'));
    await tester.pumpAndSettle();
    
    // Verify artifact created
    expect(find.text('Test Document'), findsOneWidget);
  });
}
```

### End-to-End Tests

```bash
# Start app on test device
flutter drive \
  --target=test_driver/app.dart \
  --driver=test_driver/integration_test.dart
```

## CI/CD Pipeline

See `.github/workflows/` for GitHub Actions workflows (created below).

## Future Repository Split

### Motivation

As Nomade matures, splitting the monorepo enables:
- **Focused Development**: Teams work on specific repos
- **Independent Versioning**: Core and app release separately
- **Community Contributions**: Easier to contribute to specific areas
- **Clearer Ownership**: Maintainers per repository

### Proposed Structure (Post-MVP)

```
nomade (organization)
├── nomade-core
│   └── Rust core library (QUIC, crypto, storage)
├── nomade-app
│   └── Flutter application (UI, business logic)
├── nomade-protocol
│   └── Protocol specifications (sync, pairing, CRDT)
├── nomade-plugins
│   └── Official plugin collection
├── nomade-docs
│   └── Documentation and architecture
└── nomade-cli
    └── Command-line tool for advanced users
```

### Migration Plan

**Phase 1: Monorepo (Current)**
- All code in single repository
- Fast iteration and experimentation
- Easy to maintain consistency

**Phase 2: Modular Monorepo**
- Clear module boundaries
- Separate versioning per module
- Prepare for extraction

**Phase 3: Multi-Repo**
- Extract modules to separate repos
- Set up inter-repo dependencies
- Coordinate releases

**Phase 4: Ecosystem**
- Community plugins in separate repos
- Third-party integrations
- Plugin marketplace

### Versioning Strategy

**Semantic Versioning**: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking API changes
- **MINOR**: New features, backward-compatible
- **PATCH**: Bug fixes

**Example**:
- `nomade-core`: v1.2.3
- `nomade-app`: v2.0.1
- `web-search-plugin`: v0.5.0

**Compatibility Matrix**:
```
nomade-app v2.0.x requires nomade-core v1.2.x+
nomade-app v2.1.x requires nomade-core v1.3.x+
```

## Documentation Standards

### Code Documentation

**Rust**:
```rust
/// Computes content-addressed ID for artifact.
///
/// Uses BLAKE3 hash function for fast, secure hashing.
///
/// # Arguments
///
/// * `content` - Raw content bytes
///
/// # Returns
///
/// Unique artifact ID based on content hash
///
/// # Examples
///
/// ```
/// let id = compute_artifact_id(b"Hello, World!");
/// assert_eq!(id.to_string(), "blake3-...");
/// ```
pub fn compute_artifact_id(content: &[u8]) -> ArtifactId {
    let hash = blake3::hash(content);
    ArtifactId::from_hash(hash)
}
```

**Dart**:
```dart
/// Creates a new artifact with the given metadata.
///
/// The artifact is stored locally and can be synced to paired devices.
///
/// Returns the newly created [Artifact] with a content-addressed ID.
///
/// Throws [StorageException] if storage fails.
Future<Artifact> createArtifact({
  required String title,
  required Uint8List content,
  required ArtifactType type,
}) async {
  // Implementation...
}
```

### Architecture Decision Records (ADR)

Track important decisions:

```markdown
# ADR-001: Use QUIC for Sync Protocol

## Status
Accepted

## Context
Need efficient, secure protocol for device-to-device sync.

## Decision
Use QUIC instead of TCP+TLS or WebSockets.

## Consequences
- Faster connection establishment (0-RTT)
- Better mobile support (connection migration)
- More complex implementation
- Requires UDP support
```

## Developer Experience

### Quick Start

```bash
# Clone repo
git clone https://github.com/nomade-studio/nomade.git
cd nomade

# Setup environment
./scripts/setup.sh

# Run app
cd apps/nomade_app
flutter run
```

### Hot Reload

Flutter's hot reload for fast iteration:
- Save Dart file → UI updates in <1s
- Rust changes require rebuild

### Debugging

**Flutter**:
```bash
flutter run --debug
# Use DevTools for UI inspection, profiling
```

**Rust**:
```bash
RUST_LOG=debug cargo run
# Or use lldb/gdb for native debugging
```

## Community & Contribution

### Plugin Development Guide

Will provide:
- Plugin templates
- API documentation
- Example plugins
- Testing framework
- Publishing guidelines

### Contribution Process

1. Discuss in GitHub Issues/Discussions
2. Fork repository
3. Create feature branch
4. Implement changes
5. Add tests
6. Submit pull request
7. Address review feedback
8. Merge and celebrate! 🎉

## Conclusion

Nomade's tooling and plugin architecture provides:
- **Extensibility**: Plugin system for customization
- **Developer-Friendly**: Simple setup and development
- **Secure**: Sandboxed plugins with permissions
- **Future-Ready**: Plan for ecosystem growth

The architecture balances current monorepo simplicity with future multi-repo flexibility.

---

**This concludes the documentation section.** For more details, see individual documentation files and code comments.
