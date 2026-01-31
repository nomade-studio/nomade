# Data Model

## Overview

Nomade's data model is built on CRDT (Conflict-free Replicated Data Types) principles to enable seamless synchronization without conflicts. This document describes the core entities, their CRDT representations, and relationships.

## Core Entities

### 1. Document

A document represents a single piece of content (note, article, etc.).

```rust
struct Document {
    id: DocumentId,           // UUID
    title: LWWRegister<String>,  // Last-Write-Wins
    content: RGA<char>,          // Replicated Growable Array (text)
    metadata: Metadata,
    chunks: Vec<ChunkRef>,       // References to chunks
    embeddings: Vec<ArtifactRef>, // References to embedding artifacts
    created_at: DateTime<Utc>,
    updated_at: LWWRegister<DateTime<Utc>>,
}

type DocumentId = String; // UUIDv4
```

**CRDT Properties**:
- `title`: LWW-Register (last write wins based on timestamp)
- `content`: RGA for collaborative text editing
- `metadata`: OR-Map for key-value pairs
- Collections use OR-Set for add/remove operations

### 2. Chunk

A chunk is a semantic segment of a document for RAG purposes.

```rust
struct Chunk {
    id: ChunkId,              // Content-addressed: SHA-256 of content
    document_id: DocumentId,
    content: String,          // Plaintext chunk (not synced by default)
    content_hash: [u8; 32],   // SHA-256 of content
    position: u32,            // Position in document
    metadata: ChunkMetadata,
    embedding_ref: Option<ArtifactRef>,
}

type ChunkId = [u8; 32]; // Content hash

struct ChunkMetadata {
    token_count: u32,
    char_count: u32,
    language: Option<String>,
    chunk_strategy: ChunkStrategy,
}

enum ChunkStrategy {
    FixedSize { size: u32 },
    Semantic { model: String },
    Sentence,
    Paragraph,
}
```

**Sync Policy**: Chunks are NOT synced by default (only metadata and references).

### 3. Embedding Artifact

An embedding is a vector representation of a chunk.

```rust
struct EmbeddingArtifact {
    id: ArtifactId,           // Content hash of the embedding vector
    chunk_id: ChunkId,
    model_id: String,         // e.g., "text-embedding-ada-002"
    model_version: String,    // e.g., "v2"
    dimensions: u32,
    vector: Vec<f32>,         // The actual embedding
    metadata: EmbeddingMetadata,
    created_at: DateTime<Utc>,
}

type ArtifactId = [u8; 32]; // SHA-256 hash

struct EmbeddingMetadata {
    input_tokens: u32,
    generation_time_ms: u64,
    deterministic: bool,      // Was generation deterministic?
}
```

**Sync Policy**: Synced encrypted at rest by default.

### 4. Conversation

A conversation represents an interaction with the AI.

```rust
struct Conversation {
    id: ConversationId,
    title: LWWRegister<String>,
    messages: RGA<Message>,   // Ordered list of messages
    context: Vec<ChunkRef>,   // Referenced chunks
    metadata: ConversationMetadata,
    created_at: DateTime<Utc>,
    updated_at: LWWRegister<DateTime<Utc>>,
}

type ConversationId = String; // UUIDv4

struct Message {
    id: MessageId,
    role: MessageRole,
    content: String,
    timestamp: DateTime<Utc>,
    metadata: MessageMetadata,
}

enum MessageRole {
    User,
    Assistant,
    System,
}

struct MessageMetadata {
    model: Option<String>,
    tokens: Option<u32>,
    retrieval_context: Vec<ChunkRef>,
}
```

### 5. Task

A task represents an action item.

```rust
struct Task {
    id: TaskId,
    title: LWWRegister<String>,
    description: LWWRegister<String>,
    status: LWWRegister<TaskStatus>,
    priority: LWWRegister<Priority>,
    due_date: LWWRegister<Option<DateTime<Utc>>>,
    tags: ORSet<String>,
    related_documents: ORSet<DocumentId>,
    created_at: DateTime<Utc>,
    completed_at: LWWRegister<Option<DateTime<Utc>>>,
}

enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Archived,
}

enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}
```

### 6. Tag

Tags for organizing content.

```rust
struct Tag {
    id: TagId,
    name: LWWRegister<String>,
    color: LWWRegister<String>,  // Hex color
    created_at: DateTime<Utc>,
}

type TagId = String; // UUIDv4
```

## CRDT Types Used

### LWW-Register (Last-Write-Wins)
Used for single-value fields where last update wins.

```rust
struct LWWRegister<T> {
    value: T,
    timestamp: DateTime<Utc>,
    actor_id: String, // Device ID
}
```

**Merge Rule**: Keep value with latest timestamp. If timestamps are equal, use actor_id as tiebreaker.

### OR-Set (Observed-Remove Set)
Used for collections where items can be added/removed.

```rust
struct ORSet<T> {
    entries: HashMap<T, HashSet<UniqueTag>>,
}

struct UniqueTag {
    timestamp: DateTime<Utc>,
    actor_id: String,
}
```

**Merge Rule**: An item is in the set if it has at least one add tag that hasn't been removed.

### RGA (Replicated Growable Array)
Used for ordered lists (text, messages).

```rust
struct RGA<T> {
    items: Vec<RGAItem<T>>,
}

struct RGAItem<T> {
    id: Timestamp,
    value: Option<T>, // None = tombstone
    left: Option<Timestamp>,
}
```

**Merge Rule**: Items are ordered by their causal dependencies.

### OR-Map
Used for key-value mappings.

```rust
struct ORMap<K, V> {
    entries: HashMap<K, LWWRegister<Option<V>>>,
}
```

**Merge Rule**: Each key uses LWW semantics. None value = deleted.

## Relationships

```
Document (1) ──► (N) Chunk
   │                │
   │                ▼
   │          (1) EmbeddingArtifact
   │
   ▼
Conversation (N) ──► (N) Chunk (via context)
   │
   ▼
Message (N)

Document (N) ──◄──► (N) Tag

Task (N) ──► (N) Document
```

## References

### ChunkRef
```rust
struct ChunkRef {
    chunk_id: ChunkId,
    document_id: DocumentId,
    position: u32,
}
```

### ArtifactRef
```rust
struct ArtifactRef {
    artifact_id: ArtifactId,
    artifact_type: ArtifactType,
    size: u64,
    encrypted: bool,
}

enum ArtifactType {
    Embedding,
    Thumbnail,
    Attachment,
}
```

## Metadata

### Document Metadata
```rust
struct DocumentMetadata {
    tags: ORSet<TagId>,
    source: Option<DocumentSource>,
    language: Option<String>,
    word_count: u32,
    character_count: u32,
    last_indexed: DateTime<Utc>,
}

enum DocumentSource {
    Created,
    Imported { from: String },
    Clipped { url: String },
}
```

### Conversation Metadata
```rust
struct ConversationMetadata {
    tags: ORSet<TagId>,
    model: String,
    total_tokens: u32,
    archived: bool,
}
```

## State Representation

The entire CRDT state is represented as:

```rust
struct GlobalState {
    documents: ORMap<DocumentId, Document>,
    chunks: ORMap<ChunkId, Chunk>,
    conversations: ORMap<ConversationId, Conversation>,
    tasks: ORMap<TaskId, Task>,
    tags: ORMap<TagId, Tag>,
    
    // Metadata
    device_id: String,
    state_version: u64,
    last_modified: DateTime<Utc>,
}
```

## Operations

CRDT operations that are synced:

```rust
enum CRDTOperation {
    // Document operations
    CreateDocument { id: DocumentId, title: String, timestamp: DateTime<Utc> },
    UpdateDocumentTitle { id: DocumentId, title: String, timestamp: DateTime<Utc> },
    UpdateDocumentContent { id: DocumentId, ops: Vec<TextOp>, timestamp: DateTime<Utc> },
    DeleteDocument { id: DocumentId, timestamp: DateTime<Utc> },
    
    // Chunk operations
    CreateChunk { id: ChunkId, document_id: DocumentId, metadata: ChunkMetadata },
    UpdateChunkEmbedding { id: ChunkId, artifact_ref: ArtifactRef },
    DeleteChunk { id: ChunkId },
    
    // Conversation operations
    CreateConversation { id: ConversationId, title: String, timestamp: DateTime<Utc> },
    AddMessage { conversation_id: ConversationId, message: Message },
    UpdateConversationContext { conversation_id: ConversationId, chunks: Vec<ChunkRef> },
    
    // Task operations
    CreateTask { id: TaskId, title: String, timestamp: DateTime<Utc> },
    UpdateTaskStatus { id: TaskId, status: TaskStatus, timestamp: DateTime<Utc> },
    UpdateTaskPriority { id: TaskId, priority: Priority, timestamp: DateTime<Utc> },
    
    // Tag operations
    CreateTag { id: TagId, name: String, color: String, timestamp: DateTime<Utc> },
    AddTag { entity_id: String, tag_id: TagId, timestamp: DateTime<Utc> },
    RemoveTag { entity_id: String, tag_id: TagId, timestamp: DateTime<Utc> },
}
```

## Synchronization Semantics

### What Gets Synced

| Entity | Metadata | Content | Artifacts |
|--------|----------|---------|-----------|
| Document | ✅ Always | ✅ Always | ❌ Default |
| Chunk | ✅ Always | ❌ Default | ✅ Encrypted |
| Conversation | ✅ Always | ✅ Always | N/A |
| Task | ✅ Always | ✅ Always | N/A |
| Tag | ✅ Always | ✅ Always | N/A |

**Default Policy (Default B)**:
- Plaintext metadata: Synced
- Embeddings: Synced encrypted
- Chunk text: NOT synced
- Blobs/attachments: NOT synced

Users can override per-document or per-collection.

## Versioning

### Schema Version
```rust
const DATA_MODEL_VERSION: u32 = 1;
```

Future versions may add fields but must remain backward-compatible with merge semantics.

### Migration Strategy
```rust
struct Migration {
    from_version: u32,
    to_version: u32,
    migrate: fn(&mut GlobalState) -> Result<(), MigrationError>,
}
```

## Garbage Collection

### Tombstones
Deleted items become tombstones (empty values) to preserve causal history.

### Compaction
Periodically compact old tombstones:
- Keep tombstones for 30 days
- After 30 days, remove if all devices have seen deletion
- Requires vector clocks for causality tracking

## Indexing

### Local Indexes
```rust
// For fast lookups
index_documents_by_tag: HashMap<TagId, HashSet<DocumentId>>
index_chunks_by_document: HashMap<DocumentId, Vec<ChunkId>>
index_messages_by_conversation: HashMap<ConversationId, Vec<MessageId>>

// For search
index_document_content: InvertedIndex
index_embeddings: VectorIndex
```

Indexes are rebuilt locally, not synced.

## Size Estimates

Rough size estimates per entity:

| Entity | Avg Size | Max Size |
|--------|----------|----------|
| Document | 10 KB | 10 MB |
| Chunk | 500 B | 5 KB |
| Embedding | 5 KB | 20 KB |
| Conversation | 20 KB | 1 MB |
| Message | 1 KB | 100 KB |
| Task | 500 B | 10 KB |
| Tag | 100 B | 1 KB |

## Performance Considerations

- **CRDT Overhead**: ~20-30% metadata overhead
- **Merge Complexity**: O(n) for most operations
- **Memory Usage**: Full state kept in memory (optimized later)
- **Disk Usage**: Content-addressed deduplication

## Testing

### CRDT Properties to Test
1. **Commutativity**: Operations can be applied in any order
2. **Associativity**: Grouping doesn't matter
3. **Idempotency**: Applying same operation multiple times = once
4. **Convergence**: All devices eventually reach same state

### Test Scenarios
```rust
#[test]
fn test_concurrent_document_edits() {
    // Device A and B edit same document
    // Both apply their own then other's operations
    // Verify they converge to same state
}

#[test]
fn test_delete_add_conflict() {
    // Device A deletes tag
    // Device B adds same tag
    // Verify add wins (OR-Set semantics)
}
```

## Future Enhancements

### v1.1
- [ ] Fine-grained sync policies per entity
- [ ] Selective sync by tag/folder
- [ ] Read-only sync mode

### v2.0
- [ ] Per-document encryption with ACLs
- [ ] Hierarchical documents (folders)
- [ ] Multi-format content (rich text, images)
- [ ] Collaborative editing (OT-based text)

---

**Data Model Version**: 1
**CRDT Library**: TBD (automerge or yrs)
**Last Updated**: 2026-01-31
