# zerch-storage

Binary vector store with Qdrant integration for storing and retrieving embeddings.

## Features

- Append-only binary vector storage
- Qdrant client integration for vector database operations
- Thread-safe storage operations

## Usage

```rust
use zerch_storage::VectorStore;

let store = VectorStore::new("./data.bin")?;
```

## License

MIT
