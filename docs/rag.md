# RAG (Retrieval-Augmented Generation) Pipeline

## Overview

Nomade's RAG pipeline enables semantic search and context-aware AI interactions by indexing documents, generating embeddings, and retrieving relevant context. This document describes the ingestion pipeline, embedding generation, retrieval strategies, and versioning.

## Pipeline Architecture

```
Document Input
    ↓
Text Extraction
    ↓
Chunking Strategy
    ↓
Chunk Normalization
    ↓
Embedding Generation
    ↓
Vector Index
    ↓
Retrieval
    ↓
Context Assembly
    ↓
LLM Prompt
```

## Document Ingestion

### Supported Formats

```rust
enum DocumentFormat {
    PlainText,
    Markdown,
    PDF,          // via pdf-extract (future)
    DOCX,         // via docx-rs (future)
    HTML,         // via html2text (future)
    Code { language: String },
}
```

### Text Extraction

```rust
trait TextExtractor {
    fn extract(&self, input: &[u8]) -> Result<ExtractedText, Error>;
}

struct ExtractedText {
    content: String,
    metadata: TextMetadata,
}

struct TextMetadata {
    format: DocumentFormat,
    language: Option<String>,
    author: Option<String>,
    title: Option<String>,
    page_count: Option<u32>,
}
```

### Preprocessing

```rust
fn preprocess_text(text: &str) -> String {
    text
        .trim()
        .replace('\r', "")           // Normalize line endings
        .replace("\t", "    ")       // Tabs to spaces
        // Remove excessive whitespace
        .split('\n')
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}
```

## Chunking Strategies

### 1. Fixed-Size Chunking

```rust
struct FixedSizeChunker {
    chunk_size: usize,      // Characters per chunk
    overlap: usize,         // Overlap between chunks
}

impl Chunker for FixedSizeChunker {
    fn chunk(&self, text: &str) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let mut start = 0;
        
        while start < text.len() {
            let end = (start + self.chunk_size).min(text.len());
            let content = &text[start..end];
            
            chunks.push(Chunk {
                content: content.to_string(),
                position: chunks.len() as u32,
                char_count: content.len() as u32,
            });
            
            start += self.chunk_size - self.overlap;
        }
        
        chunks
    }
}
```

**Use Case**: Simple, predictable chunks for uniform content.

### 2. Semantic Chunking

```rust
struct SemanticChunker {
    min_chunk_size: usize,
    max_chunk_size: usize,
    similarity_threshold: f32,
}

impl Chunker for SemanticChunker {
    fn chunk(&self, text: &str) -> Vec<Chunk> {
        // Split into sentences
        let sentences = split_into_sentences(text);
        
        // Group semantically similar sentences
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        
        for sentence in sentences {
            if should_start_new_chunk(&current_chunk, sentence, self.similarity_threshold) {
                if !current_chunk.is_empty() {
                    chunks.push(create_chunk(current_chunk));
                    current_chunk = String::new();
                }
            }
            current_chunk.push_str(sentence);
            current_chunk.push(' ');
        }
        
        if !current_chunk.is_empty() {
            chunks.push(create_chunk(current_chunk));
        }
        
        chunks
    }
}
```

**Use Case**: Maintain semantic coherence across chunk boundaries.

### 3. Sentence-Based Chunking

```rust
struct SentenceChunker {
    sentences_per_chunk: usize,
}

impl Chunker for SentenceChunker {
    fn chunk(&self, text: &str) -> Vec<Chunk> {
        let sentences = split_into_sentences(text);
        
        sentences
            .chunks(self.sentences_per_chunk)
            .map(|sentence_group| {
                Chunk {
                    content: sentence_group.join(" "),
                    // ... metadata
                }
            })
            .collect()
    }
}
```

**Use Case**: Natural language boundaries, good for articles.

### 4. Paragraph-Based Chunking

```rust
struct ParagraphChunker {
    max_paragraphs: usize,
}

impl Chunker for ParagraphChunker {
    fn chunk(&self, text: &str) -> Vec<Chunk> {
        text.split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .collect::<Vec<_>>()
            .chunks(self.max_paragraphs)
            .map(|para_group| {
                Chunk {
                    content: para_group.join("\n\n"),
                    // ... metadata
                }
            })
            .collect()
    }
}
```

**Use Case**: Document structure preservation.

## Embedding Generation

### Embedding Pipeline

```rust
trait EmbeddingGenerator {
    async fn generate(&self, text: &str) -> Result<Embedding, Error>;
    fn dimensions(&self) -> u32;
    fn model_id(&self) -> &str;
}

struct Embedding {
    vector: Vec<f32>,
    model_id: String,
    model_version: String,
}
```

### Local Model Integration

```rust
struct LocalEmbeddingGenerator {
    model: Box<dyn EmbeddingModel>,
    config: EmbeddingConfig,
}

struct EmbeddingConfig {
    normalize: bool,         // L2 normalization
    batch_size: usize,       // Batch multiple inputs
    max_length: usize,       // Token limit
    truncation: bool,        // Truncate if too long
}

impl EmbeddingGenerator for LocalEmbeddingGenerator {
    async fn generate(&self, text: &str) -> Result<Embedding, Error> {
        // Tokenize
        let tokens = self.model.tokenize(text, self.config.max_length)?;
        
        // Generate embedding
        let mut vector = self.model.encode(&tokens).await?;
        
        // Normalize if configured
        if self.config.normalize {
            normalize_l2(&mut vector);
        }
        
        Ok(Embedding {
            vector,
            model_id: self.model.model_id().to_string(),
            model_version: self.model.version().to_string(),
        })
    }
}
```

### Deterministic Embeddings

To ensure reproducibility across devices:

```rust
struct DeterministicEmbeddingGenerator {
    generator: Box<dyn EmbeddingGenerator>,
    seed: u64,
}

impl EmbeddingGenerator for DeterministicEmbeddingGenerator {
    async fn generate(&self, text: &str) -> Result<Embedding, Error> {
        // Set deterministic seed for model
        set_random_seed(self.seed);
        
        // Generate embedding
        let embedding = self.generator.generate(text).await?;
        
        // Verify determinism by checking input hash
        let input_hash = compute_hash(text);
        verify_determinism(&embedding, input_hash)?;
        
        Ok(embedding)
    }
}
```

### Batch Generation

```rust
async fn generate_embeddings_batch(
    generator: &dyn EmbeddingGenerator,
    chunks: Vec<&str>,
) -> Result<Vec<Embedding>, Error> {
    // Process in batches to optimize throughput
    let batch_size = 32;
    
    let mut embeddings = Vec::with_capacity(chunks.len());
    
    for batch in chunks.chunks(batch_size) {
        let batch_results = futures::future::try_join_all(
            batch.iter().map(|chunk| generator.generate(chunk))
        ).await?;
        
        embeddings.extend(batch_results);
    }
    
    Ok(embeddings)
}
```

## Vector Indexing

### Index Structure

```rust
trait VectorIndex {
    fn insert(&mut self, id: ChunkId, vector: &[f32]) -> Result<(), Error>;
    fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>, Error>;
    fn delete(&mut self, id: &ChunkId) -> Result<(), Error>;
    fn size(&self) -> usize;
}

struct SearchResult {
    chunk_id: ChunkId,
    score: f32,        // Cosine similarity
    metadata: ChunkMetadata,
}
```

### HNSW Index (Future)

```rust
struct HNSWIndex {
    index: hnsw::Hnsw<f32, hnsw::CosineDistance>,
    id_mapping: HashMap<usize, ChunkId>,
    reverse_mapping: HashMap<ChunkId, usize>,
}

impl VectorIndex for HNSWIndex {
    fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>, Error> {
        let results = self.index.search(query, k, 100)?; // ef_search=100
        
        Ok(results.into_iter().map(|result| {
            let chunk_id = self.id_mapping[&result.id];
            SearchResult {
                chunk_id,
                score: result.distance,
                metadata: self.get_metadata(&chunk_id).unwrap(),
            }
        }).collect())
    }
}
```

### Simple Brute-Force (v1)

For initial implementation, use brute-force search:

```rust
struct BruteForceIndex {
    vectors: HashMap<ChunkId, Vec<f32>>,
    metadata: HashMap<ChunkId, ChunkMetadata>,
}

impl VectorIndex for BruteForceIndex {
    fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>, Error> {
        let mut results: Vec<_> = self.vectors
            .iter()
            .map(|(id, vector)| {
                let score = cosine_similarity(query, vector);
                (*id, score)
            })
            .collect();
        
        // Sort by score descending
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Take top k
        Ok(results.into_iter()
            .take(k)
            .map(|(chunk_id, score)| SearchResult {
                chunk_id,
                score,
                metadata: self.metadata[&chunk_id].clone(),
            })
            .collect())
    }
}
```

## Retrieval Strategies

### 1. Vector Similarity Search

```rust
async fn retrieve_by_similarity(
    index: &dyn VectorIndex,
    query_embedding: &[f32],
    k: usize,
) -> Result<Vec<ChunkId>, Error> {
    let results = index.search(query_embedding, k)?;
    Ok(results.into_iter().map(|r| r.chunk_id).collect())
}
```

### 2. Hybrid Search (Vector + Keyword)

```rust
struct HybridRetriever {
    vector_index: Box<dyn VectorIndex>,
    keyword_index: InvertedIndex,
    vector_weight: f32,   // 0.0 to 1.0
}

impl HybridRetriever {
    async fn retrieve(&self, query: &str, k: usize) -> Result<Vec<ChunkId>, Error> {
        // Vector search
        let query_embedding = self.embed(query).await?;
        let vector_results = self.vector_index.search(&query_embedding, k * 2)?;
        
        // Keyword search
        let keyword_results = self.keyword_index.search(query, k * 2)?;
        
        // Combine scores
        let combined = self.combine_results(vector_results, keyword_results);
        
        // Return top k
        Ok(combined.into_iter().take(k).map(|r| r.chunk_id).collect())
    }
}
```

### 3. Context Window Assembly

```rust
struct ContextWindow {
    chunks: Vec<RetrievedChunk>,
    total_tokens: usize,
    max_tokens: usize,
}

impl ContextWindow {
    fn build(
        chunks: Vec<ChunkId>,
        max_tokens: usize,
        expand_adjacent: bool,
    ) -> Self {
        let mut retrieved = Vec::new();
        let mut total_tokens = 0;
        
        for chunk_id in chunks {
            let chunk = load_chunk(&chunk_id)?;
            let tokens = estimate_tokens(&chunk.content);
            
            if total_tokens + tokens > max_tokens {
                break;
            }
            
            // Optionally include adjacent chunks for more context
            if expand_adjacent {
                retrieved.extend(get_adjacent_chunks(&chunk_id));
            }
            
            retrieved.push(chunk);
            total_tokens += tokens;
        }
        
        Self {
            chunks: retrieved,
            total_tokens,
            max_tokens,
        }
    }
}
```

## Pipeline Versioning

### Version Tracking

```rust
struct PipelineVersion {
    version: u32,
    chunking_strategy: ChunkingStrategy,
    embedding_model: ModelInfo,
    indexing_params: IndexParams,
}

struct ModelInfo {
    model_id: String,
    version: String,
    dimensions: u32,
}
```

### Version Compatibility

```rust
fn is_compatible(v1: &PipelineVersion, v2: &PipelineVersion) -> bool {
    // Embeddings must be from same model/version
    v1.embedding_model == v2.embedding_model
}
```

### Reindexing

```rust
async fn reindex_with_new_version(
    documents: Vec<Document>,
    old_version: PipelineVersion,
    new_version: PipelineVersion,
) -> Result<(), Error> {
    for document in documents {
        // Re-chunk with new strategy
        let chunks = new_version.chunking_strategy.chunk(&document.content)?;
        
        // Re-generate embeddings
        for chunk in chunks {
            let embedding = new_version.embedding_model.generate(&chunk.content).await?;
            store_embedding(chunk.id, embedding).await?;
        }
    }
    
    Ok(())
}
```

## Performance Optimizations

### Caching

```rust
struct EmbeddingCache {
    cache: LruCache<[u8; 32], Arc<Vec<f32>>>,  // Hash → embedding
    max_size: usize,
}

impl EmbeddingCache {
    fn get_or_generate(
        &mut self,
        text: &str,
        generator: &dyn EmbeddingGenerator,
    ) -> Result<Arc<Vec<f32>>, Error> {
        let text_hash = compute_hash(text);
        
        if let Some(cached) = self.cache.get(&text_hash) {
            return Ok(Arc::clone(cached));
        }
        
        let embedding = generator.generate(text)?;
        let arc_embedding = Arc::new(embedding.vector);
        self.cache.put(text_hash, Arc::clone(&arc_embedding));
        
        Ok(arc_embedding)
    }
}
```

### Incremental Updates

```rust
async fn incremental_update(
    document: &Document,
    old_chunks: Vec<Chunk>,
    new_chunks: Vec<Chunk>,
) -> Result<(), Error> {
    // Compute diff
    let to_remove = find_removed_chunks(&old_chunks, &new_chunks);
    let to_add = find_new_chunks(&old_chunks, &new_chunks);
    
    // Update index
    for chunk in to_remove {
        remove_from_index(&chunk.id).await?;
    }
    
    for chunk in to_add {
        let embedding = generate_embedding(&chunk.content).await?;
        add_to_index(chunk.id, embedding).await?;
    }
    
    Ok(())
}
```

## Quality Metrics

### Retrieval Quality

```rust
struct RetrievalMetrics {
    precision_at_k: f32,    // Relevant results in top-k
    recall_at_k: f32,       // Coverage of relevant results
    mrr: f32,               // Mean reciprocal rank
    ndcg: f32,              // Normalized discounted cumulative gain
}

fn evaluate_retrieval(
    results: &[SearchResult],
    ground_truth: &[ChunkId],
) -> RetrievalMetrics {
    // Calculate metrics
    // ...
}
```

### Embedding Quality

```rust
fn evaluate_embedding_quality(
    embeddings: &[(ChunkId, Vec<f32>)],
    similarity_pairs: &[(ChunkId, ChunkId, f32)],
) -> f32 {
    // Compare cosine similarities to ground truth
    // ...
}
```

## Testing

### Test Cases

```rust
#[tokio::test]
async fn test_chunk_generation() {
    let text = "Sample document...";
    let chunker = FixedSizeChunker::new(100, 10);
    let chunks = chunker.chunk(text);
    assert!(!chunks.is_empty());
}

#[tokio::test]
async fn test_embedding_determinism() {
    let generator = DeterministicEmbeddingGenerator::new(seed: 42);
    let emb1 = generator.generate("test").await.unwrap();
    let emb2 = generator.generate("test").await.unwrap();
    assert_eq!(emb1.vector, emb2.vector);
}

#[tokio::test]
async fn test_retrieval_quality() {
    // Index sample documents
    // Query with known relevant documents
    // Verify top results include relevant documents
}
```

## Future Enhancements

### v1.1
- [ ] Multi-modal embeddings (text + images)
- [ ] Query expansion and reranking
- [ ] Adaptive chunking based on document type

### v2.0
- [ ] HNSW or IVF index for scalability
- [ ] Fine-tuned embedding models
- [ ] Federated search across devices
- [ ] Real-time embedding updates

---

**Pipeline Version**: 1
**Default Chunking**: Fixed-size (500 chars, 50 overlap)
**Default Model**: TBD (local model)
**Last Updated**: 2026-01-31
