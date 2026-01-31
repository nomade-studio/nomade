# RAG Pipeline (Determinism)

## Overview

Nomade's Retrieval-Augmented Generation (RAG) pipeline provides deterministic, reproducible AI-powered document understanding and question answering. This document describes the architecture, implementation, and determinism guarantees.

## What is RAG?

**Retrieval-Augmented Generation** combines:
1. **Retrieval**: Finding relevant documents/chunks from a knowledge base
2. **Augmentation**: Adding retrieved context to a prompt
3. **Generation**: LLM generates answer using context

**Benefits**:
- Grounds LLM responses in user's actual documents
- Reduces hallucination
- Provides citations and sources
- Works with local or cloud LLMs

## Pipeline Architecture

### High-Level Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    User Query                               │
│                 "What is our Q1 goal?"                      │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              1. Query Embedding                             │
│  - Generate embedding for query                             │
│  - Model: text-embedding-ada-002 (or local)                 │
│  - Output: 1536-dim vector                                  │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              2. Retrieval                                   │
│  - Cosine similarity search in vector index                 │
│  - Decrypt embeddings (if needed)                           │
│  - Top-k most similar artifacts/chunks                      │
│  - Filter by artifact type, date, tags (optional)           │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              3. Reranking (Optional)                        │
│  - Cross-encoder reranks top-k results                      │
│  - More accurate but slower                                 │
│  - Output: Top-n best results (n < k)                       │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              4. Context Assembly                            │
│  - Load full text of top-n artifacts                        │
│  - Format with metadata (title, date, source)               │
│  - Truncate to fit LLM context window                       │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              5. Prompt Construction                         │
│  - System prompt + context + user query                     │
│  - Deterministic template                                   │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              6. LLM Generation                              │
│  - Send prompt to LLM (local or cloud)                      │
│  - Temperature = 0 for determinism                          │
│  - Parse response                                           │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              7. Response + Citations                        │
│  - Display answer with sources                              │
│  - Link to original artifacts                               │
│  - Show confidence/relevance scores                         │
└─────────────────────────────────────────────────────────────┘
```

## Determinism Guarantees

### Goal: Reproducible Results

Given the same:
- Query
- Document corpus
- Models
- Parameters

→ Produce the same answer

### Sources of Non-Determinism

1. **Embeddings**: Model updates, numeric precision
2. **Retrieval**: Floating-point comparison, tie-breaking
3. **LLM**: Non-zero temperature, different API versions
4. **Time**: Timestamp-based filtering

### Mitigation Strategies

#### 1. Fixed Embedding Models

```rust
pub struct EmbeddingConfig {
    pub model: String,           // "text-embedding-ada-002"
    pub model_version: String,   // "2023-05-15"
    pub dimensions: usize,       // 1536
}

// Pin model version
const EMBEDDING_CONFIG: EmbeddingConfig = EmbeddingConfig {
    model: "text-embedding-ada-002".to_string(),
    model_version: "2023-05-15".to_string(),
    dimensions: 1536,
};
```

**Policy**: Never auto-update embedding models; require explicit migration

#### 2. Deterministic Similarity Search

```rust
pub fn cosine_similarity_deterministic(a: &[f32], b: &[f32]) -> f32 {
    // Use consistent numeric precision
    let dot_product: f64 = a.iter()
        .zip(b.iter())
        .map(|(x, y)| (*x as f64) * (*y as f64))
        .sum();
    
    let norm_a: f64 = a.iter().map(|x| (*x as f64).powi(2)).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| (*x as f64).powi(2)).sum::<f64>().sqrt();
    
    (dot_product / (norm_a * norm_b)) as f32
}

pub fn top_k_deterministic(
    scores: Vec<(ArtifactId, f32)>,
    k: usize,
) -> Vec<(ArtifactId, f32)> {
    let mut sorted = scores;
    
    // Stable sort (preserves relative order of equal elements)
    sorted.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            // Tie-breaker: lexicographic order of IDs
            .then_with(|| a.0.cmp(&b.0))
    });
    
    sorted.into_iter().take(k).collect()
}
```

#### 3. Zero-Temperature LLM

```rust
pub struct LLMConfig {
    pub model: String,        // "gpt-4-turbo-preview"
    pub temperature: f32,     // 0.0 for determinism
    pub top_p: f32,           // 1.0 (disabled)
    pub seed: Option<u64>,    // Fixed seed if supported
}

// Deterministic configuration
const LLM_CONFIG: LLMConfig = LLMConfig {
    model: "gpt-4-turbo-preview".to_string(),
    temperature: 0.0,
    top_p: 1.0,
    seed: Some(42),
};
```

**Note**: Some LLMs (e.g., GPT-4) may still have minor non-determinism even at temperature=0 due to internal sampling or model updates.

#### 4. Versioned Prompts

```rust
pub struct PromptTemplate {
    pub version: String,
    pub template: String,
}

const PROMPT_V1: PromptTemplate = PromptTemplate {
    version: "v1.0.0".to_string(),
    template: r#"
You are a helpful assistant. Answer the user's question based on the provided context.

Context:
{context}

Question: {query}

Answer:
"#.to_string(),
};

// Store prompt version with each query result for reproducibility
pub struct RAGResult {
    pub answer: String,
    pub prompt_version: String,
    pub embedding_version: String,
    pub llm_config: LLMConfig,
    pub retrieved_artifacts: Vec<ArtifactId>,
}
```

#### 5. Reproducibility Metadata

```rust
pub struct QueryExecution {
    pub query: String,
    pub timestamp: Timestamp,
    pub embedding_model: String,
    pub llm_model: String,
    pub prompt_version: String,
    pub parameters: RAGParameters,
    pub result_hash: Blake3Hash, // Hash of final answer for verification
}

pub struct RAGParameters {
    pub top_k: usize,
    pub rerank: bool,
    pub temperature: f32,
    pub max_context_tokens: usize,
}
```

## Implementation Details

### Embedding Generation

```rust
use tokenizers::Tokenizer;

pub async fn generate_embedding(text: &str) -> Result<Vec<f32>> {
    // 1. Tokenization (deterministic)
    let tokenizer = Tokenizer::from_pretrained("openai/text-embedding-ada-002")?;
    let tokens = tokenizer.encode(text, false)?;
    
    // 2. Truncate to max length (8192 tokens for ada-002)
    let truncated = &tokens.get_ids()[..tokens.len().min(8192)];
    
    // 3. Call embedding API (or local model)
    let embedding = call_embedding_api(truncated).await?;
    
    // 4. Normalize (unit vector for cosine similarity)
    let normalized = normalize_embedding(&embedding);
    
    Ok(normalized)
}

fn normalize_embedding(embedding: &[f32]) -> Vec<f32> {
    let norm: f64 = embedding.iter()
        .map(|x| (*x as f64).powi(2))
        .sum::<f64>()
        .sqrt();
    
    embedding.iter()
        .map(|x| (*x as f64 / norm) as f32)
        .collect()
}
```

### Retrieval

```rust
pub async fn retrieve_relevant_artifacts(
    query_embedding: &[f32],
    top_k: usize,
    filters: Option<ArtifactFilter>,
) -> Result<Vec<(Artifact, f32)>> {
    // 1. Load all artifact embeddings (or use vector index)
    let candidates = load_artifact_embeddings(filters).await?;
    
    // 2. Compute similarities
    let mut scores: Vec<(ArtifactId, f32)> = candidates.iter()
        .map(|(id, embedding)| {
            let decrypted = decrypt_embedding(embedding)?;
            let similarity = cosine_similarity_deterministic(query_embedding, &decrypted);
            Ok((id.clone(), similarity))
        })
        .collect::<Result<Vec<_>>>()?;
    
    // 3. Sort deterministically
    let top_k_ids = top_k_deterministic(scores, top_k);
    
    // 4. Load full artifacts
    let artifacts = load_artifacts(&top_k_ids).await?;
    
    Ok(artifacts)
}
```

### Context Assembly

```rust
pub fn assemble_context(
    artifacts: &[(Artifact, f32)],
    max_tokens: usize,
) -> String {
    let mut context = String::new();
    let mut token_count = 0;
    
    for (i, (artifact, score)) in artifacts.iter().enumerate() {
        let chunk = format!(
            "[Document {}] (Relevance: {:.2})\nTitle: {}\nDate: {}\n\n{}\n\n",
            i + 1,
            score,
            artifact.metadata.title,
            artifact.metadata.created_at,
            artifact.content.as_ref()
                .map(|c| std::str::from_utf8(&c.data).unwrap_or(""))
                .unwrap_or("")
        );
        
        let chunk_tokens = estimate_tokens(&chunk);
        if token_count + chunk_tokens > max_tokens {
            break; // Truncate if exceeds limit
        }
        
        context.push_str(&chunk);
        token_count += chunk_tokens;
    }
    
    context
}

fn estimate_tokens(text: &str) -> usize {
    // Rough estimate: ~4 characters per token for English
    (text.len() as f32 / 4.0).ceil() as usize
}
```

### Prompt Construction

```rust
pub fn construct_prompt(
    template: &PromptTemplate,
    context: &str,
    query: &str,
) -> String {
    template.template
        .replace("{context}", context)
        .replace("{query}", query)
}
```

### LLM Call

```rust
pub async fn call_llm(
    prompt: &str,
    config: &LLMConfig,
) -> Result<String> {
    let client = openai::Client::new();
    
    let response = client.chat()
        .create(ChatCompletionRequest {
            model: config.model.clone(),
            messages: vec![
                ChatMessage::system("You are a helpful assistant."),
                ChatMessage::user(prompt),
            ],
            temperature: config.temperature,
            top_p: config.top_p,
            seed: config.seed,
            ..Default::default()
        })
        .await?;
    
    Ok(response.choices[0].message.content.clone())
}
```

## Testing for Determinism

### Reproducibility Test

```rust
#[tokio::test]
async fn test_rag_determinism() {
    let query = "What is our Q1 goal?";
    
    // Run RAG pipeline twice
    let result1 = execute_rag_pipeline(query).await.unwrap();
    let result2 = execute_rag_pipeline(query).await.unwrap();
    
    // Assert identical results
    assert_eq!(result1.answer, result2.answer);
    assert_eq!(result1.retrieved_artifacts, result2.retrieved_artifacts);
    assert_eq!(result1.result_hash, result2.result_hash);
}
```

### Regression Test

```rust
#[tokio::test]
async fn test_rag_regression() {
    let test_cases = load_test_cases("tests/rag_golden.json");
    
    for case in test_cases {
        let result = execute_rag_pipeline(&case.query).await.unwrap();
        
        // Compare against golden answer
        assert_eq!(result.answer, case.expected_answer);
        assert_eq!(result.retrieved_artifacts, case.expected_artifacts);
    }
}
```

## Performance Optimization

### Caching

```rust
use lru::LruCache;

static EMBEDDING_CACHE: Lazy<Mutex<LruCache<String, Vec<f32>>>> =
    Lazy::new(|| Mutex::new(LruCache::new(1000)));

pub async fn get_embedding_cached(text: &str) -> Result<Vec<f32>> {
    let cache_key = blake3::hash(text.as_bytes()).to_hex().to_string();
    
    {
        let mut cache = EMBEDDING_CACHE.lock().unwrap();
        if let Some(embedding) = cache.get(&cache_key) {
            return Ok(embedding.clone());
        }
    }
    
    let embedding = generate_embedding(text).await?;
    
    {
        let mut cache = EMBEDDING_CACHE.lock().unwrap();
        cache.put(cache_key, embedding.clone());
    }
    
    Ok(embedding)
}
```

### Parallel Processing

```rust
pub async fn batch_generate_embeddings(
    texts: Vec<String>,
) -> Result<Vec<Vec<f32>>> {
    let tasks: Vec<_> = texts.into_iter()
        .map(|text| tokio::spawn(generate_embedding(text)))
        .collect();
    
    let results = futures::future::try_join_all(tasks).await?;
    Ok(results)
}
```

### Vector Index (HNSW)

For large document collections:

```rust
use hnswlib::{Index, Hnsw};

pub struct VectorIndex {
    index: Hnsw<f32>,
    id_map: HashMap<usize, ArtifactId>,
}

impl VectorIndex {
    pub fn search(&self, query: &[f32], k: usize) -> Vec<(ArtifactId, f32)> {
        let results = self.index.search(query, k);
        results.iter()
            .map(|(idx, dist)| {
                let id = self.id_map.get(idx).unwrap().clone();
                (id, 1.0 - dist) // Convert distance to similarity
            })
            .collect()
    }
}
```

## Evaluation Metrics

### Retrieval Quality

**Precision@k**: Fraction of retrieved documents that are relevant

```rust
fn precision_at_k(retrieved: &[ArtifactId], relevant: &[ArtifactId], k: usize) -> f32 {
    let retrieved_k = &retrieved[..k.min(retrieved.len())];
    let relevant_retrieved = retrieved_k.iter()
        .filter(|id| relevant.contains(id))
        .count();
    relevant_retrieved as f32 / k as f32
}
```

**Recall@k**: Fraction of relevant documents that are retrieved

```rust
fn recall_at_k(retrieved: &[ArtifactId], relevant: &[ArtifactId], k: usize) -> f32 {
    let retrieved_k = &retrieved[..k.min(retrieved.len())];
    let relevant_retrieved = retrieved_k.iter()
        .filter(|id| relevant.contains(id))
        .count();
    relevant_retrieved as f32 / relevant.len() as f32
}
```

**MRR (Mean Reciprocal Rank)**: Average of reciprocal ranks of first relevant result

```rust
fn mean_reciprocal_rank(queries: &[Query]) -> f32 {
    let sum: f32 = queries.iter()
        .map(|q| {
            q.retrieved.iter().position(|id| q.relevant.contains(id))
                .map(|pos| 1.0 / (pos + 1) as f32)
                .unwrap_or(0.0)
        })
        .sum();
    sum / queries.len() as f32
}
```

### Answer Quality

**ROUGE**: Overlap of n-grams between generated and reference answer

**BLEU**: Precision-based metric for comparing generated text

**BERTScore**: Embedding-based semantic similarity

**Human Evaluation**: Ground truth for quality assessment

## Observability

### Logging

```rust
pub struct RAGExecutionLog {
    pub query: String,
    pub timestamp: Timestamp,
    pub duration_ms: u64,
    pub embedding_time_ms: u64,
    pub retrieval_time_ms: u64,
    pub llm_time_ms: u64,
    pub num_retrieved: usize,
    pub context_tokens: usize,
    pub answer_tokens: usize,
}
```

### Tracing

```rust
use tracing::{info, instrument};

#[instrument]
pub async fn execute_rag_pipeline(query: &str) -> Result<RAGResult> {
    info!("Starting RAG pipeline");
    
    let embedding = generate_embedding(query).await?;
    info!("Generated query embedding");
    
    let artifacts = retrieve_relevant_artifacts(&embedding, 5, None).await?;
    info!("Retrieved {} artifacts", artifacts.len());
    
    // ... rest of pipeline
}
```

## Future Enhancements

### Planned Features

1. **Hybrid Search**: Combine semantic and keyword search
2. **Query Expansion**: Expand query for better retrieval
3. **Multi-Step Reasoning**: Chain multiple queries
4. **Fact Checking**: Verify LLM claims against sources
5. **Confidence Scores**: Quantify answer reliability

### Advanced Features

1. **Fine-Tuned Embeddings**: Custom embedding models
2. **Graph RAG**: Leverage knowledge graphs
3. **Adaptive Retrieval**: Adjust k based on query
4. **Interactive RAG**: Clarifying questions
5. **Multimodal RAG**: Images, audio, video

## Conclusion

Nomade's RAG pipeline provides:
- **Deterministic**: Reproducible results for same inputs
- **Grounded**: Answers based on user's documents
- **Transparent**: Citations and sources shown
- **Private**: All processing can be done locally

The design prioritizes determinism and reproducibility while maintaining high-quality AI-powered document understanding.

**Next**: See [Tooling](tooling.md) for extensibility and plugin architecture.
