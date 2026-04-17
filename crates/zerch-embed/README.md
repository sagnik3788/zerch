# zerch-embed

Local embedding model for generating vector embeddings from text.

## Features

- Downloads and runs sentence-transformers/all-MiniLM-L6-v2 locally
- ONNX Runtime inference with CUDA support
- Mean pooling and L2 normalization

## Usage

```rust
use zerch_embed::LocalEmbedder;

let mut embedder = LocalEmbedder::load()?;
let vector = embedder.embed("your text here")?;
```

## License

MIT
