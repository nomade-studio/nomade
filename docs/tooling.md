# Tooling & Plugins

## Overview

Nomade is designed with extensibility in mind. While the initial release focuses on core functionality, the architecture supports future plugin and connector systems for integrating external tools, services, and custom workflows.

## Extension Architecture

### Plugin Types

```rust
enum PluginType {
    Connector,        // External service integration
    Processor,        // Document processing pipeline
    Generator,        // Content generation
    UI,              // Custom UI components
    Protocol,        // Custom sync protocols
}
```

### Plugin Interface

```rust
trait Plugin {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn plugin_type(&self) -> PluginType;
    
    async fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError>;
    async fn shutdown(&mut self) -> Result<(), PluginError>;
}

struct PluginContext {
    config: HashMap<String, String>,
    data_dir: PathBuf,
    runtime: Arc<Runtime>,
}
```

## Connector Plugins

Connectors integrate external services and data sources.

### Example: Obsidian Connector

```rust
struct ObsidianConnector {
    vault_path: PathBuf,
    sync_interval: Duration,
}

impl Plugin for ObsidianConnector {
    fn id(&self) -> &str { "com.nomade.obsidian" }
    fn name(&self) -> &str { "Obsidian Vault Connector" }
    fn plugin_type(&self) -> PluginType { PluginType::Connector }
}

impl Connector for ObsidianConnector {
    async fn import(&self) -> Result<Vec<Document>, ConnectorError> {
        // Read markdown files from vault
        let files = read_vault_files(&self.vault_path)?;
        
        // Convert to Nomade documents
        let documents = files.into_iter()
            .map(|file| convert_obsidian_to_document(file))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(documents)
    }
    
    async fn export(&self, documents: Vec<Document>) -> Result<(), ConnectorError> {
        // Write documents back to vault
        for document in documents {
            let markdown = convert_document_to_obsidian(document)?;
            write_to_vault(&self.vault_path, markdown).await?;
        }
        
        Ok(())
    }
    
    async fn watch_changes(&self) -> BoxStream<ConnectorEvent> {
        // Monitor vault for changes
        watch_vault_directory(&self.vault_path).await
    }
}
```

### Example: Notion Connector

```rust
struct NotionConnector {
    api_key: String,
    workspace_id: String,
}

impl Connector for NotionConnector {
    async fn import(&self) -> Result<Vec<Document>, ConnectorError> {
        let client = NotionClient::new(&self.api_key);
        
        // Fetch pages from workspace
        let pages = client.list_pages(&self.workspace_id).await?;
        
        // Convert to Nomade documents
        let documents = pages.into_iter()
            .map(|page| convert_notion_page(page))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(documents)
    }
}
```

## Processor Plugins

Processors extend the document processing pipeline.

### Example: PDF Processor

```rust
struct PdfProcessor;

impl Processor for PdfProcessor {
    fn supported_formats(&self) -> Vec<&str> {
        vec!["pdf"]
    }
    
    async fn process(&self, input: &[u8]) -> Result<ProcessedDocument, ProcessorError> {
        // Extract text from PDF
        let text = extract_pdf_text(input)?;
        
        // Extract metadata
        let metadata = extract_pdf_metadata(input)?;
        
        Ok(ProcessedDocument {
            content: text,
            metadata,
            format: DocumentFormat::PDF,
        })
    }
}
```

### Example: Code Highlighter

```rust
struct CodeHighlighter {
    supported_languages: HashSet<String>,
}

impl Processor for CodeHighlighter {
    fn supported_formats(&self) -> Vec<&str> {
        vec!["rs", "py", "js", "ts", "go", "java", "cpp"]
    }
    
    async fn process(&self, input: &[u8]) -> Result<ProcessedDocument, ProcessorError> {
        let code = String::from_utf8(input.to_vec())?;
        let language = detect_language(&code)?;
        
        // Syntax highlighting
        let highlighted = syntax_highlight(&code, &language)?;
        
        // Extract symbols and structure
        let symbols = extract_code_symbols(&code, &language)?;
        
        Ok(ProcessedDocument {
            content: highlighted,
            metadata: CodeMetadata {
                language,
                symbols,
            },
            format: DocumentFormat::Code { language },
        })
    }
}
```

## Generator Plugins

Generators create new content.

### Example: Summary Generator

```rust
struct SummaryGenerator {
    model: Box<dyn LanguageModel>,
}

impl Generator for SummaryGenerator {
    async fn generate(&self, input: GeneratorInput) -> Result<String, GeneratorError> {
        let prompt = format!(
            "Summarize the following document in 3-5 sentences:\n\n{}",
            input.document.content
        );
        
        let summary = self.model.generate(&prompt).await?;
        
        Ok(summary)
    }
}
```

### Example: Title Generator

```rust
struct TitleGenerator {
    model: Box<dyn LanguageModel>,
}

impl Generator for TitleGenerator {
    async fn generate(&self, input: GeneratorInput) -> Result<String, GeneratorError> {
        let first_paragraph = extract_first_paragraph(&input.document.content);
        
        let prompt = format!(
            "Generate a concise title for a document that starts with:\n\n{}",
            first_paragraph
        );
        
        let title = self.model.generate(&prompt).await?;
        
        Ok(title)
    }
}
```

## UI Plugins

UI plugins extend the Flutter interface.

### Example: Custom Document Viewer

```dart
class MarkdownViewerPlugin extends UiPlugin {
  @override
  String get id => 'com.nomade.markdown-viewer';
  
  @override
  String get name => 'Markdown Viewer';
  
  @override
  Widget buildViewer(BuildContext context, Document document) {
    return MarkdownViewer(
      content: document.content,
      theme: Theme.of(context),
    );
  }
  
  @override
  bool canHandle(Document document) {
    return document.format == DocumentFormat.markdown;
  }
}
```

## Plugin Discovery and Loading

### Plugin Manifest

Each plugin includes a manifest file:

```toml
[plugin]
id = "com.nomade.obsidian"
name = "Obsidian Vault Connector"
version = "1.0.0"
type = "connector"
authors = ["Nomade Team"]

[requirements]
nomade_version = "^1.0"
rust_version = "1.70"

[permissions]
filesystem = ["read", "write"]
network = []

[dependencies]
serde = "1.0"
tokio = "1.0"
```

### Plugin Loader

```rust
struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
    plugin_dirs: Vec<PathBuf>,
}

impl PluginManager {
    async fn discover_plugins(&mut self) -> Result<(), PluginError> {
        for dir in &self.plugin_dirs {
            let manifests = find_plugin_manifests(dir)?;
            
            for manifest in manifests {
                let plugin = load_plugin(&manifest).await?;
                self.register_plugin(plugin)?;
            }
        }
        
        Ok(())
    }
    
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        let id = plugin.id().to_string();
        
        if self.plugins.contains_key(&id) {
            return Err(PluginError::DuplicateId(id));
        }
        
        self.plugins.insert(id, plugin);
        Ok(())
    }
}
```

## Plugin Sandboxing

### Permissions

```rust
struct PluginPermissions {
    filesystem: FileSystemPermissions,
    network: NetworkPermissions,
    data_access: DataAccessPermissions,
}

struct FileSystemPermissions {
    read_paths: Vec<PathBuf>,
    write_paths: Vec<PathBuf>,
}

struct NetworkPermissions {
    allowed_domains: Vec<String>,
    allow_localhost: bool,
}

struct DataAccessPermissions {
    can_read_documents: bool,
    can_write_documents: bool,
    can_read_embeddings: bool,
    can_access_keys: bool,
}
```

### Sandboxed Execution

```rust
async fn execute_plugin_safely(
    plugin: &dyn Plugin,
    operation: PluginOperation,
    permissions: &PluginPermissions,
) -> Result<PluginResult, PluginError> {
    // Create isolated context
    let context = create_sandbox_context(permissions)?;
    
    // Execute with timeout
    let result = tokio::time::timeout(
        Duration::from_secs(30),
        plugin.execute(operation, &context)
    ).await??;
    
    // Validate result
    validate_plugin_result(&result, permissions)?;
    
    Ok(result)
}
```

## Plugin API

### Core APIs Exposed to Plugins

```rust
trait NomadeApi {
    // Document management
    async fn create_document(&self, content: String) -> Result<DocumentId, Error>;
    async fn read_document(&self, id: &DocumentId) -> Result<Document, Error>;
    async fn update_document(&self, id: &DocumentId, content: String) -> Result<(), Error>;
    async fn delete_document(&self, id: &DocumentId) -> Result<(), Error>;
    
    // Search
    async fn search_documents(&self, query: &str) -> Result<Vec<Document>, Error>;
    async fn semantic_search(&self, query: &str, k: usize) -> Result<Vec<ChunkId>, Error>;
    
    // Embeddings
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, Error>;
    
    // Events
    fn subscribe(&self, event_type: EventType) -> BoxStream<Event>;
}
```

## Plugin Communication

### Event System

```rust
enum Event {
    DocumentCreated { document_id: DocumentId },
    DocumentUpdated { document_id: DocumentId },
    DocumentDeleted { document_id: DocumentId },
    ChunkIndexed { chunk_id: ChunkId },
    SyncStarted { peer_device_id: String },
    SyncCompleted { peer_device_id: String },
}

impl PluginManager {
    async fn broadcast_event(&self, event: Event) {
        for plugin in self.plugins.values() {
            if plugin.subscribes_to(&event) {
                plugin.on_event(&event).await.ok();
            }
        }
    }
}
```

## Repository Structure (Future Split)

When plugins mature, split into separate repositories:

```
nomade/                      # Core repository
nomade-plugins/              # Official plugins
  ├── obsidian/
  ├── notion/
  ├── google-docs/
  └── ...
nomade-connectors/           # Community connectors
  ├── roam/
  ├── logseq/
  └── ...
```

## Plugin Distribution

### Plugin Registry

```rust
struct PluginRegistry {
    url: String,
}

impl PluginRegistry {
    async fn search(&self, query: &str) -> Result<Vec<PluginInfo>, Error> {
        // Search registry
    }
    
    async fn install(&self, plugin_id: &str) -> Result<(), Error> {
        // Download and install plugin
    }
    
    async fn update(&self, plugin_id: &str) -> Result<(), Error> {
        // Update plugin to latest version
    }
}
```

### Installation Flow

```
User searches for plugin
    ↓
Browse registry
    ↓
Select plugin to install
    ↓
Review permissions
    ↓
Approve installation
    ↓
Download plugin
    ↓
Verify signature
    ↓
Install to plugins directory
    ↓
Restart Nomade (or hot-reload)
    ↓
Plugin activated
```

## Development Tools

### Plugin CLI

```bash
# Create new plugin
nomade plugin new my-connector --type connector

# Test plugin
nomade plugin test ./my-connector

# Build plugin
nomade plugin build ./my-connector --release

# Package plugin
nomade plugin package ./my-connector --output my-connector-1.0.0.nplugin

# Publish to registry
nomade plugin publish my-connector-1.0.0.nplugin
```

### Plugin SDK

```rust
// Simplified plugin development
use nomade_plugin_sdk::prelude::*;

#[plugin]
pub struct MyConnector {
    config: ConnectorConfig,
}

#[plugin_impl]
impl MyConnector {
    #[init]
    async fn initialize(&mut self, ctx: &PluginContext) -> Result<()> {
        // Initialize connector
        Ok(())
    }
    
    #[handler]
    async fn import_documents(&self) -> Result<Vec<Document>> {
        // Import logic
        Ok(vec![])
    }
}
```

## Security Considerations

### Plugin Signing

```rust
struct PluginSignature {
    plugin_hash: [u8; 32],
    signature: Vec<u8>,
    public_key: Vec<u8>,
    timestamp: DateTime<Utc>,
}

fn verify_plugin_signature(
    plugin_bytes: &[u8],
    signature: &PluginSignature,
) -> Result<bool, Error> {
    // Verify signature using public key
    use ed25519_dalek::Verifier;
    
    let public_key = PublicKey::from_bytes(&signature.public_key)?;
    let signature_obj = Signature::from_bytes(&signature.signature)?;
    
    Ok(public_key.verify(plugin_bytes, &signature_obj).is_ok())
}
```

### Audit Trail

```rust
struct PluginAuditLog {
    timestamp: DateTime<Utc>,
    plugin_id: String,
    action: PluginAction,
    user_approved: bool,
}

enum PluginAction {
    Installed,
    Executed { operation: String },
    PermissionRequested { permission: String },
    DataAccessed { data_type: String },
    NetworkRequest { url: String },
}
```

## Testing Plugins

### Integration Tests

```rust
#[tokio::test]
async fn test_obsidian_connector_import() {
    let connector = ObsidianConnector::new(test_vault_path());
    let documents = connector.import().await.unwrap();
    
    assert!(!documents.is_empty());
    assert_eq!(documents[0].format, DocumentFormat::Markdown);
}
```

### Mock Nomade API

```rust
struct MockNomadeApi {
    documents: HashMap<DocumentId, Document>,
}

impl NomadeApi for MockNomadeApi {
    async fn create_document(&self, content: String) -> Result<DocumentId, Error> {
        // Mock implementation
    }
}
```

## Future Enhancements

### v1.1
- [ ] Basic plugin system foundation
- [ ] File system-based plugin loading
- [ ] Limited API surface for plugins

### v2.0
- [ ] Plugin registry and marketplace
- [ ] Hot-reload plugins
- [ ] WebAssembly plugin support
- [ ] UI extension points
- [ ] Advanced sandboxing

### v3.0
- [ ] Plugin marketplace with ratings
- [ ] Paid plugins support
- [ ] Plugin analytics
- [ ] Cross-device plugin sync
- [ ] Plugin version management

## Example Plugins Roadmap

### Phase 1 (Official)
- [ ] Obsidian connector
- [ ] Notion connector
- [ ] Google Docs connector
- [ ] PDF processor
- [ ] Markdown renderer

### Phase 2 (Community)
- [ ] Roam Research connector
- [ ] Logseq connector
- [ ] Apple Notes connector
- [ ] Evernote connector
- [ ] Code editor integration

### Phase 3 (Advanced)
- [ ] Slack integration
- [ ] Email connector
- [ ] Calendar integration
- [ ] Browser extension
- [ ] Mobile share target

---

**Plugin System Status**: Planned (not in v1.0)
**Architecture**: Ready for future implementation
**Last Updated**: 2026-01-31
