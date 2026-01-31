# Nomade Domain

Domain models and business entities for Nomade.

## Structure

- `models/`: Data models (Document, Chunk, Conversation, Message, Task)
- `entities/`: Domain entities (future)
- `value_objects/`: Value objects (DocumentId, ChunkId)

## Models

### Document
Represents a note or document in the system.

### Chunk
A semantic segment of a document for RAG purposes.

### Conversation
A conversation thread with the AI.

### Message
A single message in a conversation.

### Task
An action item or todo.

## Code Generation

This package uses `json_serializable` for JSON serialization:

```bash
dart run build_runner build
```
