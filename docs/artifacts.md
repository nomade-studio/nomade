# Artifacts

## Overview

Artifacts are content-addressed, immutable data objects in Nomade. They include embeddings, thumbnails, and attachments. This document describes the artifact storage system, encryption envelopes, and synchronization semantics.

## Content-Addressed Storage

### Artifact ID

Each artifact is identified by its content hash:

```rust
type ArtifactId = [u8; 32]; // SHA-256 hash of content

fn compute_artifact_id(content: &[u8]) -> ArtifactId {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(content);
    hasher.finalize().into()
}
```

**Properties**:
- **Immutable**: Content never changes for a given ID
- **Deduplication**: Identical content stored once
- **Verifiable**: Recipients can verify integrity
- **Content-addressed**: ID is derived from content

### Artifact Types

```rust
enum ArtifactType {
    Embedding {
        model: String,
        version: String,
        dimensions: u32,
    },
    Thumbnail {
        format: ImageFormat,
        width: u32,
        height: u32,
    },
    Attachment {
        mime_type: String,
        filename: String,
    },
}

enum ImageFormat {
    PNG,
    JPEG,
    WebP,
}
```

## Artifact Envelope

### Plaintext Envelope

For unencrypted artifacts (thumbnails, etc.):

```rust
struct PlaintextArtifact {
    id: ArtifactId,              // SHA-256 of `data`
    artifact_type: ArtifactType,
    data: Vec<u8>,               // Raw content
    size: u64,
    created_at: DateTime<Utc>,
    metadata: ArtifactMetadata,
}

struct ArtifactMetadata {
    source_entity_id: String,    // Document/chunk that created this
    source_entity_type: String,  // "document", "chunk", etc.
    mime_type: Option<String>,
    checksum: [u8; 32],          // Redundant verification
}
```

### Encrypted Envelope

For sensitive artifacts (embeddings, private attachments):

```rust
struct EncryptedArtifact {
    id: ArtifactId,              // SHA-256 of *plaintext* data
    artifact_type: ArtifactType,
    encrypted_data: Vec<u8>,     // ChaCha20-Poly1305 ciphertext
    nonce: [u8; 12],             // Random nonce (never reused)
    tag: [u8; 16],               // Authentication tag
    encryption_key_id: String,   // Which key was used
    encrypted_for: Vec<String>,  // Device IDs that can decrypt
    created_at: DateTime<Utc>,
    metadata: ArtifactMetadata,
}
```

**Important**: The `id` is the hash of *plaintext* content, not encrypted content. This allows:
- Deduplication across devices
- Verification after decryption
- Content-addressed references

## Encryption at Rest

### Key Derivation

Each device derives artifact encryption keys from its identity key:

```rust
use hkdf::Hkdf;
use sha2::Sha256;

fn derive_artifact_key(device_key: &[u8], salt: &[u8]) -> [u8; 32] {
    let hkdf = Hkdf::<Sha256>::new(Some(salt), device_key);
    let mut key = [0u8; 32];
    hkdf.expand(b"nomade-artifact-v1", &mut key)
        .expect("32 bytes is valid length");
    key
}
```

### Encryption Process

```rust
use chacha20poly1305::{ChaCha20Poly1305, KeyInit, AeadInPlace};
use chacha20poly1305::aead::Aead;

fn encrypt_artifact(
    plaintext: &[u8],
    key: &[u8; 32],
) -> Result<EncryptedArtifact, Error> {
    // Generate random nonce
    let mut nonce = [0u8; 12];
    getrandom::getrandom(&mut nonce)?;
    
    // Encrypt
    let cipher = ChaCha20Poly1305::new(key.into());
    let ciphertext = cipher.encrypt(&nonce.into(), plaintext)
        .map_err(|_| Error::EncryptionFailed)?;
    
    // Compute plaintext ID
    let id = compute_artifact_id(plaintext);
    
    Ok(EncryptedArtifact {
        id,
        encrypted_data: ciphertext,
        nonce,
        // ... other fields
    })
}
```

### Decryption Process

```rust
fn decrypt_artifact(
    artifact: &EncryptedArtifact,
    key: &[u8; 32],
) -> Result<Vec<u8>, Error> {
    let cipher = ChaCha20Poly1305::new(key.into());
    let plaintext = cipher.decrypt(&artifact.nonce.into(), artifact.encrypted_data.as_ref())
        .map_err(|_| Error::DecryptionFailed)?;
    
    // Verify content hash
    let computed_id = compute_artifact_id(&plaintext);
    if computed_id != artifact.id {
        return Err(Error::ContentHashMismatch);
    }
    
    Ok(plaintext)
}
```

## Storage Layer

### File System Layout

```
nomade_data/
├── artifacts/
│   ├── 00/
│   │   ├── 00a1b2c3...def.artifact  # First 2 bytes = subdirectory
│   │   └── 00f9e8d7...cba.artifact
│   ├── 01/
│   ├── ...
│   └── ff/
├── index.db                         # SQLite index
└── metadata/
    └── artifacts.json               # Metadata cache
```

### Index Database

SQLite database for fast lookups:

```sql
CREATE TABLE artifacts (
    id BLOB PRIMARY KEY,
    artifact_type TEXT NOT NULL,
    size INTEGER NOT NULL,
    encrypted BOOLEAN NOT NULL,
    created_at INTEGER NOT NULL,
    source_entity_id TEXT,
    source_entity_type TEXT,
    mime_type TEXT,
    file_path TEXT NOT NULL
);

CREATE INDEX idx_source_entity ON artifacts(source_entity_id);
CREATE INDEX idx_artifact_type ON artifacts(artifact_type);
CREATE INDEX idx_created_at ON artifacts(created_at);
```

### Artifact Store Interface

```rust
trait ArtifactStore {
    async fn store(&self, artifact: PlaintextArtifact) -> Result<ArtifactId, Error>;
    async fn store_encrypted(&self, artifact: EncryptedArtifact) -> Result<ArtifactId, Error>;
    async fn retrieve(&self, id: &ArtifactId) -> Result<PlaintextArtifact, Error>;
    async fn retrieve_encrypted(&self, id: &ArtifactId) -> Result<EncryptedArtifact, Error>;
    async fn exists(&self, id: &ArtifactId) -> bool;
    async fn delete(&self, id: &ArtifactId) -> Result<(), Error>;
    async fn list(&self, filter: ArtifactFilter) -> Result<Vec<ArtifactMetadata>, Error>;
    async fn size(&self) -> Result<u64, Error>; // Total size of all artifacts
}
```

## Synchronization

### Sync Strategy

Artifacts are synced lazily:

1. **Metadata First**: CRDT state includes artifact references
2. **On-Demand**: Artifacts fetched when needed
3. **Priority-Based**: Important artifacts (recent embeddings) synced first
4. **Bandwidth-Aware**: Respect mobile data limits

### Sync Flow

```
Device A has new embedding
    ↓
Update CRDT state with artifact reference
    ↓
Sync CRDT operation to Device B
    ↓
Device B sees reference to unknown artifact
    ↓
Device B requests artifact from Device A
    ↓
Device A sends encrypted artifact chunks
    ↓
Device B verifies and stores artifact
```

### Chunk-Based Transfer

Large artifacts are transferred in chunks:

```rust
const ARTIFACT_CHUNK_SIZE: usize = 64 * 1024; // 64 KB

struct ArtifactChunk {
    artifact_id: ArtifactId,
    chunk_index: u32,
    total_chunks: u32,
    data: Vec<u8>,
}
```

### Priority Levels

```rust
enum ArtifactPriority {
    Critical = 0,   // Needed for current UI
    High = 1,       // Recent user activity
    Medium = 2,     // Background indexing
    Low = 3,        // Historical data
}
```

## Embedding Artifacts

### Structure

```rust
struct EmbeddingArtifact {
    model_id: String,        // "text-embedding-ada-002"
    model_version: String,   // "v2"
    dimensions: u32,         // 1536
    vector: Vec<f32>,        // The embedding
    input_text_hash: [u8; 32], // SHA-256 of input (for determinism check)
    metadata: EmbeddingMetadata,
}

struct EmbeddingMetadata {
    input_tokens: u32,
    generation_time_ms: u64,
    timestamp: DateTime<Utc>,
    deterministic: bool,     // Was generation deterministic?
    temperature: Option<f32>, // If non-deterministic
}
```

### Serialization

Embeddings are serialized efficiently:

```rust
fn serialize_embedding(embedding: &EmbeddingArtifact) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(embedding.dimensions as usize * 4 + 1024);
    
    // Header
    bytes.extend_from_slice(&embedding.dimensions.to_le_bytes());
    
    // Vector (little-endian f32)
    for value in &embedding.vector {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    
    // Metadata (msgpack)
    let metadata_bytes = rmp_serde::to_vec(&embedding.metadata).unwrap();
    bytes.extend_from_slice(&metadata_bytes);
    
    bytes
}
```

### Deterministic Generation

For reproducibility, embeddings should be deterministic:

```rust
struct EmbeddingConfig {
    model: String,
    seed: Option<u64>,        // Fixed seed for determinism
    normalize: bool,          // L2 normalization
    precision: Precision,     // Float precision
}

enum Precision {
    Float32,
    Float16,    // Half precision (future)
    Int8,       // Quantized (future)
}
```

## Garbage Collection

### Reference Counting

Track which entities reference each artifact:

```rust
struct ArtifactRefCount {
    artifact_id: ArtifactId,
    ref_count: u32,
    last_accessed: DateTime<Utc>,
}
```

### Cleanup Policy

```rust
async fn garbage_collect(store: &ArtifactStore) -> Result<u64, Error> {
    let candidates = store.list(ArtifactFilter {
        ref_count: Some(0),
        last_accessed_before: Some(Utc::now() - Duration::days(30)),
    }).await?;
    
    let mut deleted_bytes = 0u64;
    for candidate in candidates {
        store.delete(&candidate.id).await?;
        deleted_bytes += candidate.size;
    }
    
    Ok(deleted_bytes)
}
```

## Verification

### Integrity Checks

Periodically verify artifacts:

```rust
async fn verify_artifact(
    store: &ArtifactStore,
    artifact_id: &ArtifactId,
) -> Result<bool, Error> {
    let artifact = store.retrieve_encrypted(artifact_id).await?;
    let computed_hash = compute_artifact_id(&artifact.encrypted_data);
    
    // For encrypted artifacts, we verify the plaintext hash after decryption
    let key = get_artifact_key()?;
    let plaintext = decrypt_artifact(&artifact, &key)?;
    let plaintext_hash = compute_artifact_id(&plaintext);
    
    Ok(plaintext_hash == *artifact_id)
}
```

## Quota Management

### Device Quotas

```rust
struct ArtifactQuota {
    max_total_size: u64,      // e.g., 10 GB
    max_single_size: u64,     // e.g., 100 MB
    max_artifacts: u64,       // e.g., 100,000
}

impl ArtifactStore {
    async fn check_quota(&self, size: u64) -> Result<(), Error> {
        let current = self.size().await?;
        if current + size > self.quota.max_total_size {
            return Err(Error::QuotaExceeded);
        }
        Ok(())
    }
}
```

## Error Handling

```rust
enum ArtifactError {
    NotFound(ArtifactId),
    CorruptedData(ArtifactId),
    EncryptionFailed,
    DecryptionFailed,
    QuotaExceeded,
    StorageError(std::io::Error),
    HashMismatch { expected: ArtifactId, actual: ArtifactId },
}
```

## Performance Optimizations

### Caching

```rust
struct ArtifactCache {
    cache: LruCache<ArtifactId, Arc<Vec<u8>>>,
    max_size: usize,
}
```

### Batch Operations

```rust
async fn store_batch(
    store: &ArtifactStore,
    artifacts: Vec<PlaintextArtifact>,
) -> Result<Vec<ArtifactId>, Error> {
    // Optimize by batching database writes
}
```

### Compression

```rust
struct CompressedArtifact {
    id: ArtifactId,
    compressed_data: Vec<u8>,
    compression: CompressionAlgorithm,
    uncompressed_size: u64,
}

enum CompressionAlgorithm {
    None,
    Zstd,
    Lz4,
}
```

## Testing

### Test Scenarios

```rust
#[tokio::test]
async fn test_store_and_retrieve() {
    // Store artifact, retrieve, verify content matches
}

#[tokio::test]
async fn test_encryption_roundtrip() {
    // Encrypt, decrypt, verify plaintext matches
}

#[tokio::test]
async fn test_content_hash_verification() {
    // Tamper with encrypted data, verify detection
}

#[tokio::test]
async fn test_deduplication() {
    // Store same content twice, verify single copy
}
```

## Future Enhancements

### v1.1
- [ ] Compression for large embeddings
- [ ] Streaming API for large artifacts
- [ ] Background verification

### v2.0
- [ ] Per-artifact ACLs
- [ ] Artifact versioning
- [ ] Cloud backup (encrypted)
- [ ] P2P artifact sharing

---

**Storage Format Version**: 1
**Encryption**: ChaCha20-Poly1305
**Hash Algorithm**: SHA-256
**Last Updated**: 2026-01-31
