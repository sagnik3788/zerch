# CLAUDE.md — Zerch-pvt Project Guide

## What is Zerch-pvt?

Zerch-pvt is a **vector-powered semantic search engine for logs**, built in Rust with a React frontend. Users upload log files through the UI, each log line is embedded into a 384-dimensional vector using a local sentence-transformer model (all-MiniLM-L6-v2 via ONNX Runtime), stored in a compact binary format, and then searchable via cosine similarity. An AI summarization feature (via Groq API) lets users click on search results to get LLM-generated insights.

**License:** MIT — Copyright (c) 2026 Sagnik Das

---

## Repository Layout

```
Zerch-pvt/
├── Cargo.toml                 # Workspace root (resolver = "2")
├── .env                       # GROQ_API_KEY for AI summarization (gitignored)
├── .github/workflows/ci.yaml  # GitHub Actions CI — build on push/PR
├── crates/
│   ├── zerch-pvt-core/            # Distance metrics (cosine similarity, euclidean distance)
│   ├── zerch-pvt-embed/           # Local embedding model loading & inference (ONNX Runtime)
│   ├── zerch-pvt-storage/         # Binary vector store (append-only .bin file)
│   ├── zerch-pvt-engine/          # CLI tool — index log files & search from terminal
│   └── zerch-pvt-api/             # HTTP API server (Actix-Web) — upload, search, summarize
├── ui/                        # React frontend (Vite + React 19)
├── zerch_data.bin             # The binary vector store file (gitignored)
└── models--sentence-transformers--all-MiniLM-L6-v2/  # Cached HF model (gitignored)
```

---

## Crate Details

### `zerch-core` (library)
Pure math utilities — no external dependencies.
- **`cosine.rs`** — `cosine_similarity(a: &[f32], b: &[f32]) -> CosineSimilarity` returns a similarity score.
- **`euclidean.rs`** — `euclidean_distance(a: &[f32], b: &[f32]) -> EuclideanDistance` returns a distance value.
- Both return sentinel values (NEG_INFINITY / INFINITY) on dimension mismatch.

### `zerch-embed` (library)
Handles downloading and running the embedding model locally.
- **`downloader.rs`** — Downloads `sentence-transformers/all-MiniLM-L6-v2` from Hugging Face Hub (ONNX format + tokenizer). Uses `hf-hub` crate.
- **`model.rs`** — `LocalEmbedder` struct wraps an ONNX `Session` + `Tokenizer`.
  - `load()` — Downloads model (if not cached) and creates the inference session. CUDA execution provider is configured (falls back to CPU automatically via ORT 2.0).
  - `embed(&mut self, text: &str) -> Result<Vec<f32>>` — Tokenizes, truncates to 512 tokens, runs inference, applies mean pooling + L2 normalization. Returns a 384-dim vector.
- **Key deps:** `ort` (ONNX Runtime with CUDA feature), `tokenizers`, `hf-hub`, `ndarray`.

### `zerch-storage` (library)
Simple append-only binary vector store.
- **`store.rs`** — `VectorStore` struct with a `path: PathBuf`.
  - `append_vector(&self, vector: &[f32], text: &str)` — Appends to the `.bin` file.
  - **Binary format per entry:** `[vec_len: u32 LE][vec_data: vec_len × f32 LE][text_len: u32 LE][text_bytes: UTF-8]`
  - Uses `unsafe` to reinterpret `&[f32]` as `&[u8]` for zero-copy writes.
- **Key deps:** `serde`, `bincode`, `anyhow` (serde/bincode imported but not actively used for the store itself).

### `zerch-engine` (binary)
CLI tool for offline indexing and search.
- **Usage:**
  - Index: `cargo run -p zerch-engine -- <logfile.log>`
  - Search: `cargo run -p zerch-engine -- --search "your query"`
- Reads/writes `zerch_data.bin` in the working directory.
- On search: scans the entire `.bin` file, computes cosine similarity for each stored vector, returns top 5 matches.
- **Depends on:** `zerch-core`, `zerch-embed`, `zerch-storage`.

### `zerch-api` (binary)
HTTP API server built with Actix-Web 4. This is the main server that the UI connects to.

- **Startup:**
  - Loads `.env` via `dotenv` (for `GROQ_API_KEY`).
  - Initializes `LocalEmbedder` (wrapped in `Arc<Mutex<...>>` for thread-safe shared access).
  - Creates `VectorStore` pointing to `zerch_data.bin`.
  - Binds to `127.0.0.1:8080`.
  - CORS is fully permissive (any origin/method/header).

- **Endpoints:**
  | Method | Path              | Description |
  |--------|-------------------|-------------|
  | GET    | `/health`         | Returns `{"status":"healthy","engine":"Running (Rust)"}` |
  | POST   | `/api/upload`     | Multipart file upload — reads log file line-by-line, embeds each line, stores vector+text, returns structured `LogEntry[]` with id, timestamp, level, service, msg |
  | GET    | `/api/search`     | Query params: `q` (search text), `limit` (default 5). Embeds query, scans `.bin` file, returns top-N results sorted by cosine similarity |
  | POST   | `/api/summarize`  | JSON body: `{"text": "..."}`. Calls Groq API (`openai/gpt-oss-120b` model) for AI-powered log summarization |

- **Helper functions:**
  - `extract_log_level(line)` — keyword-based (error/warn/debug/info).
  - `extract_timestamp(line)` — currently returns `Local::now()` (not parsed from log).
  - `extract_service(line)` — extracts text between first `[...]` brackets.
  - `search_vectors(...)` — linear scan over the binary store, returns `Vec<SearchResult>`.

- **Key deps:** `actix-web`, `actix-multipart`, `actix-cors`, `tokio`, `serde_json`, `reqwest` (for Groq API calls), `dotenv`, `uuid`, `chrono`.

---

## Frontend (ui/)

React 19 + Vite 8 SPA. No routing library — single-page layout.

### Tech Stack
- **Framework:** React 19, Vite 8
- **Styling:** Vanilla CSS with CSS custom properties (dark theme)
- **Fonts:** Inter (UI), JetBrains Mono (monospace/code)
- **No component library** — all components are hand-built

### Component Architecture
```
App.jsx                       # Root — manages state, wires up all panels
├── Topbar.jsx                # Header with logo, stats, AI thinking indicator, clock
├── SearchPanel.jsx           # Search input + severity/service/time filters + semantic toggle
├── LogPanel.jsx              # Scrollable uploaded log stream with level badges & similarity bars
├── SearchResultsPanel.jsx    # Semantic search results with score bars — click to summarize
├── RightPanel.jsx            # Sidebar wrapper — upload zone + AI summary display
│   └── UploadSection.jsx     # Drag-and-drop file upload (sends to /api/upload)
└── IncidentDetails.jsx       # Modal overlay for incident details (AI insight, similar logs, fixes)
```

### Key Data Flow
1. **Upload:** User drops a `.log`/`.txt`/`.json` file → `UploadSection` sends `POST /api/upload` → backend embeds + stores → returns parsed logs → rendered in `LogPanel`.
2. **Search:** User types a query in `SearchPanel` → `GET /api/search?q=...` → backend embeds query, scans store → returns ranked results → rendered in `SearchResultsPanel`.
3. **Summarize:** User clicks a search result → `POST /api/summarize` with the log text → Groq LLM generates summary → rendered in `RightPanel`'s AI insights section.

### Design System (index.css)
- **Color tokens:** `--bg-base`, `--bg-surface`, `--bg-card`, `--accent-blue`, `--accent-purple`, `--accent-cyan`, `--log-info/warn/error/success/debug`, `--incident-crit/warn/ok`
- **Reusable classes:** `.card`, `.badge`, `.btn` (primary/ghost/danger/success), `.status-bar`
- **Animations:** `pulse-dot`, `blink-cursor`, `slide-in-up`, `slide-in-right`, `fade-in`, `thinking-dots`, `spin`, `glow-pulse`, `bar-slide`

### Mock Data
`data/mockData.js` exports empty arrays and a no-op `generateRandomLog()`. The app starts with zero logs — all data comes from the real backend.

---

## Build & Run

### Prerequisites
- **Rust** (stable toolchain, edition 2021)
- **Node.js / npm** (for the UI)
- **ONNX Runtime** — pulled automatically via the `ort` crate. CUDA support is optional (auto-detects GPU).
- **A Groq API key** in `.env` (only needed for AI summarization)

### Backend
```bash
# Build the entire workspace
cargo build

# Run the API server (main entrypoint for production use)
cargo run -p zerch-api

# CLI: Index a log file
cargo run -p zerch-engine -- path/to/logfile.log

# CLI: Search indexed logs
cargo run -p zerch-engine -- --search "your query here"
```

The API server starts on `http://127.0.0.1:8080`.

### Frontend
```bash
cd ui
npm install
npm run dev
```

Vite dev server starts on `http://localhost:5173` (default). The UI expects the backend at `http://localhost:8080` (hardcoded in fetch calls).

---

## CI/CD

GitHub Actions workflow at `.github/workflows/ci.yaml`:
- Triggers on push to `master` and all pull requests.
- Steps: Checkout → Install Rust (stable) → Cache (Swatinem/rust-cache) → `cargo build --verbose`.
- No test step yet (to be added as tests are written).

---

## Environment Variables

| Variable       | Required | Description |
|----------------|----------|-------------|
| `GROQ_API_KEY` | For `/api/summarize` | API key for Groq's OpenAI-compatible endpoint |

Loaded via `dotenv` from the project root `.env` file.

---

## Binary Data Format (zerch_data.bin)

The vector store is a flat append-only binary file. Each record:

```
┌──────────────────┬──────────────────────────┬──────────────────┬────────────────┐
│ vec_len (u32 LE) │ vector data (vec_len×f32) │ text_len (u32 LE)│ text (UTF-8)   │
│ 4 bytes          │ vec_len × 4 bytes         │ 4 bytes          │ text_len bytes │
└──────────────────┴──────────────────────────┴──────────────────┴────────────────┘
```

For the all-MiniLM-L6-v2 model, `vec_len` is always 384, so each record has 4 + 1536 + 4 + text_len bytes.

---

## Coding Conventions

- **Rust edition:** 2021, resolver 2.
- **Error handling:** `anyhow::Result` throughout. The API maps errors to HTTP 500 responses.
- **Concurrency:** The embedder is shared via `Arc<Mutex<LocalEmbedder>>`. CPU-heavy embedding work is wrapped in `web::block()` to avoid blocking the async Actix runtime.
- **No tests yet** — the project is WIP. When adding tests, use standard `#[cfg(test)]` modules.
- **Formatting:** `rustfmt` is used with `#[rustfmt::skip]` annotations where manual formatting is preferred.
- **Logging:** `env_logger` with default filter `info`. Use `log::info!`, `log::error!`, etc.
- **Frontend style:** Functional React components with hooks. CSS modules via per-component `.css` files. No TypeScript — plain JSX.

---

## Known Limitations & TODOs

- **Linear scan search:** The search iterates over the entire `.bin` file. No index or ANN structure yet.
- **Token truncation:** Logs longer than 512 tokens are truncated before embedding (see TODO in `model.rs`).
- **CUDA dependency:** The `ort` crate is compiled with `features = ["cuda"]`. This can cause build issues on systems without CUDA. Needs to be made optional (see TODO in `model.rs`).
- **Timestamp extraction:** `extract_timestamp()` always returns the current time instead of parsing timestamps from log lines.
- **Hardcoded API URL:** The frontend has `http://localhost:8080` hardcoded in fetch calls.
- **No authentication** on any endpoint.
- **Single-threaded embedding:** The `Mutex<LocalEmbedder>` serializes all embedding requests.
