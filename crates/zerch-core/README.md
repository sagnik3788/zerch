# zerch-core

Pure math utilities for vector similarity search.

## Features

- **Cosine Similarity** - Compute similarity between vectors
- **Euclidean Distance** - Compute distance between vectors

## Usage

```rust
use zerch_core::{cosine_similarity, euclidean_distance};

let similarity = cosine_similarity(&vec1, &vec2);
let distance = euclidean_distance(&vec1, &vec2);
```

## License

MIT
