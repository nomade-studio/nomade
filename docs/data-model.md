# Data Model (CRDT Overview)

## Introduction

Nomade uses Conflict-free Replicated Data Types (CRDTs) to enable seamless multi-device synchronization without requiring a central server or complex conflict resolution logic.

## Why CRDTs?

### Problem: Distributed Conflicts

When multiple devices modify the same data offline, conflicts can occur:

```
Initial state: document = "Hello"

Device A (offline): document = "Hello World"
Device B (offline): document = "Hello Everyone"

When synced: What should the final value be? 🤔
```

### Solution: Conflict-Free Replication

CRDTs ensure that all devices converge to the same state without manual conflict resolution:

- **Commutative**: Operations can be applied in any order
- **Associative**: Grouping of operations doesn't matter
- **Idempotent**: Applying same operation twice is safe
- **Convergent**: All devices eventually reach same state

## CRDT Types in Nomade

### 1. Last-Write-Wins Register (LWW-Register)

**Use Case**: Simple fields like document title, metadata

**How It Works**:
```rust
struct LWWRegister<T> {
    value: T,
    timestamp: Timestamp,
    device_id: DeviceId,
}

impl<T> LWWRegister<T> {
    fn update(&mut self, value: T, timestamp: Timestamp, device_id: DeviceId) {
        if timestamp > self.timestamp ||
           (timestamp == self.timestamp && device_id > self.device_id) {
            self.value = value;
            self.timestamp = timestamp;
            self.device_id = device_id;
        }
    }
}
```

**Example**:
```
Device A: title = "Report" @ T1
Device B: title = "Annual Report" @ T2

After sync: title = "Annual Report" (T2 > T1)
```

**Limitations**: Last writer wins; some edits may be lost

### 2. Observed-Remove Set (OR-Set)

**Use Case**: Collections like tags, labels, shared users

**How It Works**:
```rust
struct ORSet<T> {
    elements: HashMap<T, HashSet<(DeviceId, u64)>>, // element -> unique IDs
}

impl<T: Hash + Eq> ORSet<T> {
    fn add(&mut self, element: T, device_id: DeviceId, counter: u64) {
        self.elements.entry(element)
            .or_insert_with(HashSet::new)
            .insert((device_id, counter));
    }

    fn remove(&mut self, element: &T, observed_ids: HashSet<(DeviceId, u64)>) {
        if let Some(ids) = self.elements.get_mut(element) {
            for id in observed_ids {
                ids.remove(&id);
            }
            if ids.is_empty() {
                self.elements.remove(element);
            }
        }
    }
}
```

**Example**:
```
Device A: add tag "work" with ID (A, 1)
Device B: add tag "important" with ID (B, 1)
Device A: remove tag "work" (observing ID (A, 1))

After sync: tags = ["important"]
No duplicates, no phantom resurrections!
```

### 3. Version Vector

**Use Case**: Tracking causality and detecting conflicts

**How It Works**:
```rust
struct VersionVector {
    clocks: HashMap<DeviceId, u64>,
}

impl VersionVector {
    fn increment(&mut self, device_id: DeviceId) {
        *self.clocks.entry(device_id).or_insert(0) += 1;
    }

    fn merge(&mut self, other: &VersionVector) {
        for (device, &clock) in &other.clocks {
            let entry = self.clocks.entry(*device).or_insert(0);
            *entry = (*entry).max(clock);
        }
    }

    fn happens_before(&self, other: &VersionVector) -> bool {
        self.clocks.iter().all(|(device, &clock)| {
            clock <= *other.clocks.get(device).unwrap_or(&0)
        }) && self != other
    }
}
```

**Example**:
```
Device A: {A: 5, B: 3, C: 2}
Device B: {A: 4, B: 6, C: 2}

After merge: {A: 5, B: 6, C: 2}

Causality:
{A: 3, B: 2} happens before {A: 5, B: 3, C: 2} ✓
{A: 5, B: 3} concurrent with {A: 4, B: 6} (conflict!)
```

### 4. Hybrid Logical Clock (HLC)

**Use Case**: Timestamp with causality tracking

**How It Works**:
```rust
struct HybridLogicalClock {
    physical: SystemTime, // Wall clock time
    logical: u64,         // Logical counter
}

impl HybridLogicalClock {
    fn send(&mut self) -> HybridLogicalClock {
        let now = SystemTime::now();
        if now > self.physical {
            self.physical = now;
            self.logical = 0;
        } else {
            self.logical += 1;
        }
        *self
    }

    fn receive(&mut self, remote: &HybridLogicalClock) {
        let now = SystemTime::now();
        let max_physical = self.physical.max(remote.physical).max(now);
        
        if max_physical == self.physical && max_physical == remote.physical {
            self.logical = self.logical.max(remote.logical) + 1;
        } else if max_physical == remote.physical {
            self.logical = remote.logical + 1;
        } else {
            self.logical = 0;
        }
        self.physical = max_physical;
    }
}
```

**Benefits**:
- Physical timestamp for human readability
- Logical counter ensures causality
- Bounded divergence from wall clock

## Data Structures

### Artifact

The core unit of data in Nomade:

```rust
struct Artifact {
    // Identity
    id: ArtifactId, // Content-addressed (BLAKE3 hash)
    version_vector: VersionVector,
    
    // Metadata (synced, plaintext by default)
    title: LWWRegister<String>,
    created_at: HybridLogicalClock,
    modified_at: HybridLogicalClock,
    tags: ORSet<String>,
    artifact_type: LWWRegister<ArtifactType>,
    
    // Content (not synced by default)
    content_hash: Option<Blake3Hash>, // Hash of full content
    content_size: u64,
    
    // Embedding (synced, encrypted)
    embedding: Option<EncryptedEmbedding>,
    embedding_model: LWWRegister<String>, // e.g., "text-embedding-ada-002"
    
    // Relationships
    parent: Option<ArtifactId>,
    children: ORSet<ArtifactId>,
    related: ORSet<ArtifactId>,
}
```

**Properties**:
- **Content-Addressed**: ID derived from content (immutable)
- **Versioned**: Version vector tracks causality
- **Modular**: Only necessary fields synced
- **Extensible**: Can add new CRDT fields

### Device Info

Represents a paired device:

```rust
struct DeviceInfo {
    device_id: DeviceId,
    device_name: LWWRegister<String>,
    public_key: PublicKey,
    capabilities: ORSet<Capability>,
    version_vector: VersionVector,
    last_seen: HybridLogicalClock,
}
```

### Sync State

Tracks synchronization progress:

```rust
struct SyncState {
    device_id: DeviceId,
    peer_device_id: DeviceId,
    last_sync: HybridLogicalClock,
    artifacts_synced: u64,
    bytes_transferred: u64,
    version_vector: VersionVector, // What peer has seen
}
```

## Conflict Resolution Strategy

### Automatic Resolution

Most conflicts resolve automatically via CRDT semantics:

```
Scenario: Two devices modify same artifact

Device A: 
  - Change title: "Report" -> "Q1 Report" @ T1
  - Add tag: "draft" @ T1
  
Device B:
  - Change title: "Report" -> "Annual Report" @ T2
  - Add tag: "final" @ T2

After Sync (automatic):
  - Title: "Annual Report" (LWW: T2 > T1)
  - Tags: ["draft", "final"] (OR-Set: both preserved)
```

### Conflict Detection

Some scenarios require user attention:

```rust
enum ConflictType {
    None,                    // No conflict
    Concurrent,              // Concurrent edits (both valid)
    CausalViolation,         // Causality violated (rare)
}

fn detect_conflict(a: &Artifact, b: &Artifact) -> ConflictType {
    let a_before_b = a.version_vector.happens_before(&b.version_vector);
    let b_before_a = b.version_vector.happens_before(&a.version_vector);
    
    match (a_before_b, b_before_a) {
        (true, false) => ConflictType::None, // a superseded by b
        (false, true) => ConflictType::None, // b superseded by a
        (false, false) => ConflictType::Concurrent, // conflict!
        (true, true) => ConflictType::CausalViolation, // shouldn't happen
    }
}
```

### Manual Resolution (Future)

For complex conflicts, user intervention:

```
Conflict Detected: "Q1 Report" vs "Annual Report"

┌─────────────────────────────────────┐
│ Choose Resolution:                  │
│                                     │
│ ○ Keep "Q1 Report" (Device A)      │
│ ○ Keep "Annual Report" (Device B)  │
│ ● Keep both (merge)                 │
│ ○ Custom...                         │
│                                     │
│ [Resolve] [Cancel]                  │
└─────────────────────────────────────┘
```

## Garbage Collection

### Tombstones

Deleted artifacts leave tombstones:

```rust
struct Tombstone {
    artifact_id: ArtifactId,
    deleted_at: HybridLogicalClock,
    deleted_by: DeviceId,
    version_vector: VersionVector,
}
```

**Purpose**:
- Inform other devices of deletion
- Prevent resurrection of deleted items
- Track causality

### Garbage Collection Policy

```
Rules:
1. Keep tombstone for 90 days
2. After 90 days, if all paired devices have seen deletion:
   - Remove tombstone
   - Remove from sync state
3. If new device pairs:
   - Extend tombstone lifetime
   - Ensure new device learns of deletion
```

### Compaction

Periodic compaction of version vectors:

```rust
fn compact_version_vector(vv: &mut VersionVector, active_devices: &[DeviceId]) {
    // Remove entries for devices no longer paired
    vv.clocks.retain(|device_id, _| active_devices.contains(device_id));
    
    // Compute minimum clock across all devices
    let min_clock = vv.clocks.values().min().copied().unwrap_or(0);
    
    // Subtract minimum from all (maintains relative ordering)
    for clock in vv.clocks.values_mut() {
        *clock -= min_clock;
    }
}
```

## Persistence

### Storage Format

Artifacts stored as:
1. **Metadata**: JSON with CRDT state
2. **Content**: Separate blob (if stored locally)
3. **Embedding**: Encrypted binary

```
Storage Layout:
artifacts/
├── metadata/
│   ├── blake3-abc...def.json
│   └── blake3-123...456.json
├── content/
│   ├── blake3-abc...def.blob
│   └── blake3-123...456.blob
└── embeddings/
    ├── blake3-abc...def.enc
    └── blake3-123...456.enc
```

### Indexing

Efficient queries via indices:

```sql
-- SQLite schema for metadata index
CREATE TABLE artifacts (
    id BLOB PRIMARY KEY,
    title TEXT,
    created_at INTEGER,
    modified_at INTEGER,
    artifact_type TEXT,
    version_vector BLOB
);

CREATE INDEX idx_title ON artifacts(title);
CREATE INDEX idx_modified_at ON artifacts(modified_at);
CREATE INDEX idx_type ON artifacts(artifact_type);

CREATE TABLE tags (
    artifact_id BLOB,
    tag TEXT,
    PRIMARY KEY (artifact_id, tag),
    FOREIGN KEY (artifact_id) REFERENCES artifacts(id)
);

CREATE INDEX idx_tag ON tags(tag);
```

## Performance Considerations

### Sync Efficiency

**Delta Sync**: Only send changes since last sync

```rust
fn compute_delta(
    local_state: &VersionVector,
    peer_state: &VersionVector,
) -> Vec<ArtifactId> {
    // Return artifacts that peer hasn't seen
    artifacts.iter()
        .filter(|a| !peer_state.dominates(&a.version_vector))
        .map(|a| a.id)
        .collect()
}
```

**Compression**: Metadata compressed with zstd

**Batching**: Group small artifacts into batches

### Memory Usage

**Streaming**: Large content streamed, not loaded in memory

**Lazy Loading**: Load artifact content on-demand

**Cache**: LRU cache for frequently accessed artifacts

### Network Optimization

**Bloom Filters**: Quickly check if peer has artifact

**Priority Queue**: Sync metadata before embeddings

**Adaptive Batch Size**: Adjust based on network conditions

## Edge Cases

### Clock Skew

**Problem**: Device clocks out of sync

**Solution**: Hybrid Logical Clocks tolerate bounded skew

**Limit**: If physical clock off by > 1 hour, warn user

### Network Partition

**Problem**: Devices offline for extended period

**Solution**: CRDTs handle arbitrary offline time

**Caveat**: More conflicts likely after long partition

### Device Removal

**Problem**: Device removed but data remains in version vectors

**Solution**: Garbage collection removes old device entries

### Data Corruption

**Problem**: Storage corruption or malicious modification

**Solution**: Content-addressed IDs detect tampering

**Recovery**: Re-sync from peer or restore from backup

## Testing Strategy

### Property-Based Testing

```rust
#[test]
fn crdt_convergence_property() {
    // Generate random operations
    let ops = generate_random_ops(100);
    
    // Apply in different orders to two replicas
    let mut replica1 = Artifact::new();
    let mut replica2 = Artifact::new();
    
    for op in ops.clone() {
        replica1.apply(op);
    }
    
    for op in ops.into_iter().rev() {
        replica2.apply(op);
    }
    
    // Merge replicas
    replica1.merge(&replica2);
    replica2.merge(&replica1);
    
    // Assert convergence
    assert_eq!(replica1, replica2);
}
```

### Chaos Testing

- Random network partitions
- Clock skew simulation
- Concurrent operations
- Device failures

## Future Enhancements

### Planned Features

1. **Causal Trees**: Better text collaboration (Google Docs-like)
2. **Schema Evolution**: Versioned CRDT types
3. **Selective Sync**: Fine-grained sync policies
4. **Peer Discovery**: Automatic CRDT state discovery

### Research Directions

1. **Byzantine Fault Tolerance**: Handle malicious devices
2. **Weak Consistency Models**: Tune consistency vs. availability
3. **Compression**: CRDT-specific compression algorithms
4. **Formal Verification**: Prove convergence properties

## Conclusion

Nomade's CRDT-based data model provides:
- **Conflict-Free**: Automatic conflict resolution
- **Partition-Tolerant**: Works offline indefinitely
- **Eventually Consistent**: All devices converge
- **Scalable**: Efficient delta sync

The design balances theoretical correctness with practical performance, enabling seamless multi-device collaboration.

**Next**: See [Sync Protocol](sync-protocol.md) for how CRDTs are transmitted, and [Artifacts](artifacts.md) for content-addressed storage details.
