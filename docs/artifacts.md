# Artifacts System

## Overview

Nomade's artifact system provides content-addressed storage with encrypted embeddings for semantic search and AI-powered features. This document describes the design, implementation, and security properties of the artifacts system.

## Design Principles

1. **Content-Addressed**: Immutable artifacts identified by content hash
2. **Encrypted at Rest**: Embeddings encrypted by default
3. **Modular Storage**: Metadata, content, and embeddings stored separately
4. **Efficient Retrieval**: Fast lookup via content-addressed IDs
5. **Version Control**: Track artifact evolution over time

## Content-Addressed Storage

### What is Content Addressing?

Instead of arbitrary IDs, artifacts identified by hash of their content:

```
Traditional: id = UUID() = "550e8400-e29b-41d4-a716-446655440000"
Content-Addressed: id = BLAKE3(content) = "blake3-af1349b9..."
```

**Benefits**:
- **Deduplication**: Same content = same ID (automatic dedup)
- **Integrity**: ID verifies content hasn't been tampered with
- **Immutability**: Content change = new ID (versioning built-in)
- **Merkle Trees**: Efficient hierarchical verification

### Hash Function: BLAKE3

**Why BLAKE3?**
- Fast: 1-2 GB/s on modern CPUs
- Secure: 256-bit output, collision-resistant
- Parallelizable: Uses SIMD and multi-threading
- Extensible: Supports keyed hashing, KDF, MAC

```rust
use blake3;

fn compute_artifact_id(content: &[u8]) -> ArtifactId {
    let hash = blake3::hash(content);
    ArtifactId::from_hash(hash)
}
```

**ID Format**:
```
blake3-<hex-encoded-hash>
blake3-af1349b9c87b7e24b0f3e3c7e8c7d1e5a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5
       └─────────────────────────────────────────────────────────────┘
                         32 bytes (256 bits)
```

## Artifact Structure

### Core Components

```rust
pub struct Artifact {
    // Identity
    pub id: ArtifactId, // BLAKE3 hash
    
    // Metadata (plaintext, synced)
    pub metadata: ArtifactMetadata,
    
    // Content (optional, not synced by default)
    pub content: Option<ArtifactContent>,
    
    // Embedding (encrypted, synced)
    pub embedding: Option<EncryptedEmbedding>,
}

pub struct ArtifactMetadata {
    pub title: String,
    pub artifact_type: ArtifactType,
    pub created_at: Timestamp,
    pub modified_at: Timestamp,
    pub tags: Vec<String>,
    pub size: u64,
    pub content_hash: Blake3Hash,
    pub embedding_model: Option<String>,
    pub version: u32,
    pub parent: Option<ArtifactId>,
}

pub enum ArtifactType {
    Document,
    Note,
    Code,
    Image,
    Audio,
    Video,
    Link,
    Email,
    Calendar,
    Contact,
}

pub struct ArtifactContent {
    pub mime_type: String,
    pub data: Vec<u8>,
    pub chunks: Vec<ContentChunk>,
}

pub struct ContentChunk {
    pub id: ChunkId,
    pub offset: u64,
    pub size: u64,
    pub text: String,
    pub embedding_index: Option<usize>,
}
```

### Example Artifact

```json
{
  "id": "blake3-af1349b9c87b7e24b0f3e3c7e8c7d1e5...",
  "metadata": {
    "title": "2026 Q1 Product Roadmap",
    "artifact_type": "Document",
    "created_at": 1706745600,
    "modified_at": 1706832000,
    "tags": ["product", "roadmap", "2026"],
    "size": 45678,
    "content_hash": "blake3-1a2b3c4d...",
    "embedding_model": "text-embedding-ada-002",
    "version": 3,
    "parent": null
  },
  "content": null,
  "embedding": {
    "encrypted_data": "base64-encoded-ciphertext...",
    "nonce": "base64-encoded-nonce",
    "algorithm": "AES-256-GCM"
  }
}
```

## Encrypted Embeddings

### Why Encrypt Embeddings?

Embeddings reveal semantic content:
- Similar embeddings = similar meaning
- Can infer topics, entities, sentiment
- Privacy risk if leaked

**Solution**: Encrypt embeddings at rest

### Encryption Scheme

**Algorithm**: AES-256-GCM (Authenticated Encryption)

```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};

pub fn encrypt_embedding(
    embedding: &[f32],
    key: &Key,
) -> Result<EncryptedEmbedding> {
    // Convert float32 to bytes
    let plaintext: Vec<u8> = embedding.iter()
        .flat_map(|f| f.to_le_bytes())
        .collect();
    
    // Generate random nonce (96 bits for GCM)
    let nonce = Nonce::from_slice(&generate_random_bytes(12));
    
    // Encrypt with AES-256-GCM
    let cipher = Aes256Gcm::new(key);
    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())?;
    
    Ok(EncryptedEmbedding {
        ciphertext,
        nonce: nonce.to_vec(),
        algorithm: "AES-256-GCM".to_string(),
    })
}
```

**Properties**:
- **Confidentiality**: Embedding content hidden
- **Authenticity**: Tamper detection via GCM tag
- **Performance**: Fast encryption/decryption (hardware AES)

### Key Management

**Embedding Key Hierarchy**:

```
Master Key (device-specific, in keychain)
    └─> Artifact Encryption Key (derived via HKDF)
            └─> Per-Embedding Key (derived via HKDF)
```

**Key Derivation**:
```rust
use hkdf::Hkdf;
use sha2::Sha256;

fn derive_embedding_key(
    master_key: &[u8],
    artifact_id: &ArtifactId,
    context: &str,
) -> Key {
    let hkdf = Hkdf::<Sha256>::new(Some(artifact_id.as_bytes()), master_key);
    let mut okm = [0u8; 32]; // 256-bit key
    hkdf.expand(context.as_bytes(), &mut okm)
        .expect("key derivation failed");
    Key::from_slice(&okm).clone()
}
```

**Benefits**:
- Different key per embedding (isolation)
- Deterministic (can re-derive)
- Forward secrecy (if master key rotated)

### Embedding Format

**Vector Dimensions**: Typically 768 or 1536 (depends on model)

**Storage**:
```
Unencrypted: 1536 floats × 4 bytes = 6144 bytes
Encrypted: 6144 bytes + 16 bytes (GCM tag) + 12 bytes (nonce) = 6172 bytes
Overhead: ~0.5%
```

**Compression**: Vectors already dense; minimal benefit from compression

**Quantization** (future):
- Float32 → Int8: 4× reduction
- Minimal accuracy loss for retrieval
- Requires model-specific quantization

## Storage Architecture

### Directory Structure

```
nomade_data/
├── artifacts/
│   ├── metadata/
│   │   ├── blake3-af1349b9...json     # Metadata (plaintext)
│   │   └── blake3-1a2b3c4d...json
│   ├── content/
│   │   ├── blake3-af1349b9...blob     # Content (optional)
│   │   └── blake3-1a2b3c4d...blob
│   └── embeddings/
│       ├── blake3-af1349b9...enc      # Encrypted embeddings
│       └── blake3-1a2b3c4d...enc
├── index/
│   ├── artifacts.db                   # SQLite index
│   └── vector_index.bin               # Vector similarity index
└── keys/
    └── master_key.enc                 # Encrypted master key
```

### SQLite Index

Fast metadata queries:

```sql
CREATE TABLE artifacts (
    id BLOB PRIMARY KEY,
    title TEXT NOT NULL,
    artifact_type TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    modified_at INTEGER NOT NULL,
    size INTEGER NOT NULL,
    has_embedding BOOLEAN NOT NULL,
    version INTEGER NOT NULL
);

CREATE TABLE tags (
    artifact_id BLOB NOT NULL,
    tag TEXT NOT NULL,
    PRIMARY KEY (artifact_id, tag),
    FOREIGN KEY (artifact_id) REFERENCES artifacts(id)
);

CREATE TABLE relationships (
    source_id BLOB NOT NULL,
    target_id BLOB NOT NULL,
    relationship_type TEXT NOT NULL,
    PRIMARY KEY (source_id, target_id),
    FOREIGN KEY (source_id) REFERENCES artifacts(id),
    FOREIGN KEY (target_id) REFERENCES artifacts(id)
);

-- Indices for fast queries
CREATE INDEX idx_artifact_type ON artifacts(artifact_type);
CREATE INDEX idx_created_at ON artifacts(created_at);
CREATE INDEX idx_modified_at ON artifacts(modified_at);
CREATE INDEX idx_tag ON tags(tag);
```

### Vector Index (Future)

For semantic search, approximate nearest neighbor (ANN) index:

**Options**:
- **HNSW** (Hierarchical Navigable Small World): Fast, memory-efficient
- **IVF** (Inverted File Index): Good for large datasets
- **Annoy** (Approximate Nearest Neighbors Oh Yeah): Simple, disk-backed

**Implementation** (placeholder):
```rust
pub trait VectorIndex {
    fn add(&mut self, id: ArtifactId, embedding: &[f32]) -> Result<()>;
    fn search(&self, query: &[f32], k: usize) -> Result<Vec<(ArtifactId, f32)>>;
    fn remove(&mut self, id: &ArtifactId) -> Result<()>;
}
```

## Artifact Lifecycle

### Creation

```rust
async fn create_artifact(
    title: String,
    content: Vec<u8>,
    artifact_type: ArtifactType,
) -> Result<Artifact> {
    // 1. Compute content-addressed ID
    let content_hash = blake3::hash(&content);
    let artifact_id = ArtifactId::from_hash(content_hash);
    
    // 2. Generate embedding (optional)
    let embedding = if should_embed(&artifact_type) {
        let raw_embedding = generate_embedding(&content).await?;
        let key = derive_embedding_key(&MASTER_KEY, &artifact_id, "embedding");
        Some(encrypt_embedding(&raw_embedding, &key)?)
    } else {
        None
    };
    
    // 3. Create metadata
    let metadata = ArtifactMetadata {
        title,
        artifact_type,
        created_at: Timestamp::now(),
        modified_at: Timestamp::now(),
        size: content.len() as u64,
        content_hash,
        embedding_model: embedding.as_ref().map(|_| "text-embedding-ada-002".into()),
        version: 1,
        parent: None,
        tags: vec![],
    };
    
    // 4. Store artifact
    let artifact = Artifact {
        id: artifact_id,
        metadata,
        content: Some(ArtifactContent { /* ... */ }),
        embedding,
    };
    
    store_artifact(&artifact).await?;
    
    Ok(artifact)
}
```

### Retrieval

```rust
async fn retrieve_artifact(id: &ArtifactId) -> Result<Artifact> {
    // 1. Load metadata (fast)
    let metadata = load_metadata(id).await?;
    
    // 2. Load content (lazy, optional)
    let content = None; // Load on-demand via load_content(id)
    
    // 3. Load embedding (lazy, optional)
    let embedding = None; // Load on-demand via load_embedding(id)
    
    Ok(Artifact {
        id: id.clone(),
        metadata,
        content,
        embedding,
    })
}
```

### Update

Artifacts are immutable, so "update" creates new version:

```rust
async fn update_artifact(
    original_id: &ArtifactId,
    new_content: Vec<u8>,
) -> Result<Artifact> {
    // 1. Load original artifact
    let original = retrieve_artifact(original_id).await?;
    
    // 2. Create new artifact with updated content
    let mut new_artifact = create_artifact(
        original.metadata.title.clone(),
        new_content,
        original.metadata.artifact_type.clone(),
    ).await?;
    
    // 3. Link to parent
    new_artifact.metadata.parent = Some(original_id.clone());
    new_artifact.metadata.version = original.metadata.version + 1;
    
    Ok(new_artifact)
}
```

### Deletion

Soft delete with tombstone:

```rust
async fn delete_artifact(id: &ArtifactId) -> Result<()> {
    // 1. Create tombstone
    let tombstone = Tombstone {
        artifact_id: id.clone(),
        deleted_at: Timestamp::now(),
        deleted_by: DEVICE_ID.clone(),
    };
    
    store_tombstone(&tombstone).await?;
    
    // 2. Remove from index
    remove_from_index(id).await?;
    
    // 3. Keep files for sync (garbage collected later)
    // Files in artifacts/ directory remain until GC
    
    Ok(())
}
```

## Security Properties

### Integrity

**Content-Addressed IDs**: Detect tampering

```rust
async fn verify_artifact_integrity(artifact: &Artifact) -> Result<bool> {
    if let Some(content) = &artifact.content {
        let computed_hash = blake3::hash(&content.data);
        Ok(computed_hash == artifact.metadata.content_hash.as_slice())
    } else {
        // No content loaded; trust metadata
        Ok(true)
    }
}
```

### Confidentiality

**Encrypted Embeddings**: Protect semantic content

**Plaintext Metadata**: Trade-off for usability (search by title, tags)

**User Control**: Can opt-out of sync for sensitive artifacts

### Authenticity

**Signed Metadata** (future): Verify artifact creator

```rust
struct SignedMetadata {
    metadata: ArtifactMetadata,
    signature: Signature,
    signer: DeviceId,
}
```

## Performance Optimization

### Lazy Loading

Only load what's needed:

```rust
impl Artifact {
    pub async fn load_content(&mut self) -> Result<()> {
        if self.content.is_none() {
            self.content = Some(load_content_from_disk(&self.id).await?);
        }
        Ok(())
    }
    
    pub async fn load_embedding(&mut self) -> Result<Vec<f32>> {
        if let Some(encrypted) = &self.embedding {
            let key = derive_embedding_key(&MASTER_KEY, &self.id, "embedding");
            decrypt_embedding(encrypted, &key)
        } else {
            Err(Error::NoEmbedding)
        }
    }
}
```

### Caching

LRU cache for frequently accessed artifacts:

```rust
use lru::LruCache;

static ARTIFACT_CACHE: Lazy<Mutex<LruCache<ArtifactId, Artifact>>> =
    Lazy::new(|| Mutex::new(LruCache::new(100)));
```

### Batch Operations

Process multiple artifacts efficiently:

```rust
async fn batch_store_artifacts(artifacts: Vec<Artifact>) -> Result<()> {
    // Write metadata in batch
    let mut tx = db.begin().await?;
    for artifact in &artifacts {
        insert_metadata(&mut tx, &artifact.metadata).await?;
    }
    tx.commit().await?;
    
    // Write files in parallel
    let tasks: Vec<_> = artifacts.iter()
        .map(|a| tokio::spawn(write_artifact_files(a.clone())))
        .collect();
    
    futures::future::try_join_all(tasks).await?;
    Ok(())
}
```

## Future Enhancements

### Planned Features

1. **Deduplication**: Automatic content deduplication
2. **Compression**: Compress large content
3. **Sharding**: Split large files into chunks
4. **Versioning UI**: Visual artifact history
5. **Garbage Collection**: Automated cleanup

### Advanced Features

1. **Merkle Trees**: Efficient sync of artifact hierarchies
2. **Delta Encoding**: Only store differences between versions
3. **Bloom Filters**: Fast existence checks
4. **Encrypted Metadata**: Option to encrypt titles/tags
5. **Homomorphic Search**: Search encrypted embeddings

## Conclusion

Nomade's artifact system provides:
- **Integrity**: Content-addressed storage detects tampering
- **Privacy**: Encrypted embeddings protect semantic content
- **Efficiency**: Lazy loading and caching optimize performance
- **Immutability**: Versioning built into content addressing

The design balances security, performance, and usability for a privacy-first AI assistant.

**Next**: See [RAG Pipeline](rag-pipeline.md) for how artifacts are used in AI workflows.
