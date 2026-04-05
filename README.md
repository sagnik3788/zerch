<div align="center">

<br/>

```
                ███████╗███████╗██████╗  ██████╗██╗  ██╗
                ╚══███╔╝██╔════╝██╔══██╗██╔════╝██║  ██║
                  ███╔╝ █████╗  ██████╔╝██║     ███████║
                 ███╔╝  ██╔══╝  ██╔══██╗██║     ██╔══██║
                ███████╗███████╗██║  ██║╚██████╗██║  ██║
                ╚══════╝╚══════╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝
```

### ⚡ Vector-Powered Semantic Log Search Engine

**Built with Rust. Powered by AI. Designed for Engineers.**

---

## Demo

- **Live Demo:** https://zerch-pvt.vercel.app/
- **YouTube Walkthrough:** https://www.youtube.com/watch?v=kKXEW9pN6_E

<br/>

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/React_19-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://reactjs.org/)
[![Actix Web](https://img.shields.io/badge/Actix--Web_4-000000?style=for-the-badge&logo=actix&logoColor=white)](https://actix.rs/)
[![ONNX Runtime](https://img.shields.io/badge/ONNX_Runtime-005CED?style=for-the-badge&logo=onnx&logoColor=white)](https://onnxruntime.ai/)
[![Vite](https://img.shields.io/badge/Vite_8-646CFF?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev/)
[![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-FFD700?style=for-the-badge)](./LICENSE)

<br/>

> *Upload your logs. Search with plain English. Get AI-powered insights — all at sub-millisecond speed.*

<br/>

[🚀 Quick Start](#-quick-start) &nbsp;·&nbsp;
[📖 How It Works](#-how-it-works) &nbsp;·&nbsp;
[🏗️ Architecture](#%EF%B8%8F-architecture) &nbsp;·&nbsp;
[🛠️ Tech Stack](#%EF%B8%8F-tech-stack) &nbsp;·&nbsp;
[📡 API Reference](#-api-reference) &nbsp;·&nbsp;
[🤖 MCP Server](#-mcp-server--ai-agent-integration)

</div>

---

## 📸 Screenshots

<br/>

### 🏠 Home Page — The Landing Experience

![Zerch Home Page](./ProjectDemo/Home_Page.png)

*The Zerch landing page greets users with a bold, neobrutalist design. It highlights the core value proposition — a vector search engine built in Rust — with live stats showing 10M+ vectors indexed, sub-millisecond query times, and 100% Rust-native performance.*

<br/>

### 🔐 Authentication — Secure Access

![Zerch Auth Page](./ProjectDemo/Auth_Page.png)

*A clean split-panel login screen. The left side reinforces the brand identity with a golden yellow background, while the right side hosts the login form with demo credentials visible for easy onboarding.*

<br/>

### ⚙️ Dashboard — The Core Search & Analysis Engine

![Zerch Dashboard Page](./ProjectDemo/Dashboard_Page.png)

*The main AI Log Debugger dashboard. A live topbar shows indexed vectors, active incidents, P99 latency, and service count. Users can search with natural language, filter by severity/service/time, upload log files via drag-and-drop, and trigger AI summarization — all in one view.*

<br/>

### ✨ Features & Technology — What's Under the Hood

![Zerch Features Page](./ProjectDemo/Features_Page.png)

*The marketing features page showcases four core capabilities: Ultra-Fast Search, Semantic Understanding, Log Analysis, and Enterprise Grade reliability — along with the modern technology pillars that power them.*

<br/>

---

## 📌 Overview

**Zerch** is a **semantic search engine purpose-built for log files**. Traditional log search tools rely on exact keyword matching — you get results only when you type the exact string that appears in the log. Zerch is fundamentally different.

Instead of keyword matching, Zerch **embeds every log line into a 384-dimensional semantic vector** using a locally-running sentence transformer model (`all-MiniLM-L6-v2` via ONNX Runtime). These vectors are stored in a compact, append-only binary file. When you search, your natural language query is embedded in real-time and compared against every stored vector using **cosine similarity** — returning the logs that are *semantically closest* to your intent, not just lexically similar.

On top of search, Zerch integrates with the **Groq API** to provide AI-powered log summarization: click any search result and get an instant LLM-generated explanation and fix suggestion.

**Key capabilities at a glance:**

| Capability | Description |
|---|---|
| 🔍 **Semantic Search** | Find logs by meaning, not just keywords |
| 🧠 **Local Embeddings** | All-MiniLM-L6-v2 runs 100% on your machine |
| ⚡ **Rust Performance** | Zero-copy binary I/O, memory-safe, blazing fast |
| 🤖 **AI Summarization** | Groq-powered LLM insights on any log entry |
| 🖥️ **CLI Tool** | Index and search logs directly from the terminal |
| 📡 **MCP Server** | Plug Zerch into any AI agent via Model Context Protocol |
| 🐳 **Docker Ready** | Full containerized deployment with one command |

---

## ✨ Features

### 🔍 Semantic Understanding
Type what you *mean*, not what you remember. Search for `"database connection refused"` and find logs mentioning `"ECONNREFUSED"`, `"timeout on port 5432"`, or `"postgres unreachable"` — because Zerch understands context.

### ⚡ Sub-Millisecond Search
The Rust backend reads vectors directly from disk using zero-copy byte reinterpretation, computes cosine similarity for every record in microseconds, and returns ranked results instantly.

### 📤 Drag-and-Drop Log Ingestion
Upload any `.log`, `.txt`, or `.json` file via the browser UI. Every non-empty line is parsed, embedded, and stored — with metadata extraction (log level, service name, timestamp) happening automatically.

### 🤖 AI-Powered Summarization
Click any search result to send the raw log text to the Groq API. The LLM analyzes the log, explains what happened in plain English, and suggests a fix — displayed live in the right sidebar.

### 🖥️ CLI Engine
Prefer the terminal? Use `zerch-engine` to index entire log files and run semantic searches without touching a browser.

### 📡 MCP Protocol Support
The `zerch-mcp-server` exposes `embed_text` and `search_logs` as MCP tools over **stdio** or **HTTP**, making Zerch a first-class citizen in any AI agent workflow (Claude Desktop, LangChain, custom agents).

### 🏗️ Modular Workspace Architecture
Six independent Rust crates, each with a single responsibility — core math, embedding, storage, CLI engine, HTTP API, and MCP server. Add features without touching unrelated code.

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        CLIENT LAYER                                  │
│                                                                      │
│   ┌──────────────────────────────────────────────────────────────┐  │
│   │              React 19 + Vite 8 (ui/)                         │  │
│   │                                                              │  │
│   │  LandingPage → LoginPage → DashboardPage → LiveLogsPage     │  │
│   │                                                              │  │
│   │  [Upload] ──► POST /api/upload   (multipart file)           │  │
│   │  [Search] ──► GET  /api/search   (query + limit)            │  │
│   │  [Click]  ──► POST /api/summarize (log text → LLM)          │  │
│   └────────────────────────────┬─────────────────────────────────┘  │
└────────────────────────────────│────────────────────────────────────┘
                                 │ HTTP (localhost:8080)
┌────────────────────────────────▼────────────────────────────────────┐
│                        API LAYER  (zerch-api)                        │
│                    Actix-Web 4 · CORS Open · Port 8080               │
│                                                                      │
│   /health ──────────────────────► JSON health status                 │
│   POST /api/upload ─────────────► embed each log line + store        │
│   GET  /api/search?q= ──────────► embed query + linear scan         │
│   POST /api/summarize ──────────► Groq API (LLM)                    │
│                                                                      │
│   Arc<Mutex<LocalEmbedder>>  ·  Arc<VectorStore>  ·  Client         │
└────┬───────────────┬───────────────────────────────────┬────────────┘
     │               │                                   │
     ▼               ▼                                   ▼
┌─────────┐   ┌─────────────┐                  ┌──────────────────┐
│ zerch-  │   │  zerch-     │                  │  Groq API        │
│ embed   │   │  storage    │                  │  (LLM Cloud)     │
│         │   │             │                  │                  │
│ ONNX    │   │ zerch_      │                  │ gpt-oss-120b     │
│ Runtime │   │ data.bin    │                  │                  │
│ HF Hub  │   │ (binary     │                  │ Summarize &      │
│         │   │  store)     │                  │ Fix suggestions  │
│ 384-dim │   │             │                  └──────────────────┘
│ vectors │   │ Append-only │
└────┬────┘   └──────┬──────┘
     │               │
     ▼               │
┌─────────┐          │
│ zerch-  │          │
│ core    │◄─────────┘
│         │
│ cosine_ │
│ similarity         │
│ euclidean_         │
│ distance│
└─────────┘

─────────────────────── ALTERNATIVE INTERFACES ──────────────────────

  zerch-engine (CLI)          zerch-mcp-server (AI Agents)
  ┌────────────────┐          ┌──────────────────────────────────┐
  │                │          │                                  │
  │ cargo run      │          │  stdio transport (Claude, etc.)  │
  │  -- <file>     │          │  HTTP transport  (:8787/mcp)     │
  │                │          │                                  │
  │ cargo run      │          │  MCP Tools:                      │
  │  -- --search   │          │  · embed_text { text }           │
  │     "query"    │          │  · search_logs { query, top_k }  │
  │                │          │                                  │
  └────────────────┘          └──────────────────────────────────┘
```

---

## 🗂️ Project Structure

```
zerch/
├── Cargo.toml                        # Workspace root (6 crates, resolver = "2")
├── Cargo.lock
├── .env                              # GROQ_API_KEY (gitignored)
├── docker-compose.yml                # Full-stack container orchestration
├── Dockerfile.api                    # Multi-stage Rust API image
├── setup.sh                          # One-shot Docker install + launch script
├── zerch_data.bin                    # Binary vector store (gitignored)
│
├── crates/
│   ├── zerch-core/                   # Pure math — cosine & euclidean distance
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── cosine.rs             # cosine_similarity(&[f32], &[f32]) → score
│   │       └── euclidean.rs          # euclidean_distance(&[f32], &[f32]) → dist
│   │
│   ├── zerch-embed/                  # Model download + ONNX inference
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── downloader.rs         # Downloads all-MiniLM-L6-v2 from HF Hub
│   │       └── model.rs              # LocalEmbedder: tokenize → infer → pool → L2norm
│   │
│   ├── zerch-storage/                # Append-only binary vector store
│   │   └── src/
│   │       ├── lib.rs
│   │       └── store.rs              # VectorStore: append_vector + clear
│   │
│   ├── zerch-engine/                 # CLI binary: index files + search
│   │   └── src/
│   │       └── main.rs
│   │
│   ├── zerch-api/                    # HTTP API server (Actix-Web 4)
│   │   └── src/
│   │       ├── main.rs               # Routes: /health, /api/upload, /api/search
│   │       └── summarize.rs          # POST /api/summarize → Groq API
│   │
│   └── zerch-mcp-server/             # MCP server for AI agent integration
│       └── src/
│           ├── main.rs
│           ├── config.rs             # McpServerConfig from env vars
│           ├── server.rs             # JSON-RPC 2.0 dispatcher
│           ├── tools.rs              # embed_text + search_logs tool implementations
│           └── transports.rs         # stdio (line-based) + HTTP (Axum) transports
│
├── ui/                               # React 19 + Vite 8 frontend
│   ├── src/
│   │   ├── main.jsx                  # App entry point
│   │   ├── App.jsx                   # Auth-gated router
│   │   ├── config.js / config/       # API endpoint config
│   │   ├── context/
│   │   │   └── AuthContext.jsx       # Auth state + localStorage persistence
│   │   ├── hooks/
│   │   │   ├── useLogs.js            # Log state management hook
│   │   │   └── useNavigate.js        # Custom SPA navigation
│   │   ├── pages/
│   │   │   ├── LandingPage.jsx/css   # Marketing homepage
│   │   │   ├── LoginPage.jsx/css     # Auth gate
│   │   │   ├── DashboardPage.jsx     # Main search & analysis UI
│   │   │   └── LiveLogsPage.jsx/css  # Live log stream + AI agent view
│   │   └── components/
│   │       ├── Topbar.jsx/css        # Header: stats, clock, AI indicator
│   │       ├── SearchPanel.jsx/css   # Query input + filters + semantic toggle
│   │       ├── LogPanel.jsx/css      # Uploaded log stream with badges
│   │       ├── SearchResultsPanel.jsx/css  # Ranked results with score bars
│   │       ├── RightPanel.jsx/css    # Upload zone + AI summary sidebar
│   │       ├── UploadSection.jsx/css # Drag-and-drop file uploader
│   │       └── IncidentDetails.jsx/css     # Modal overlay for incident details
│   ├── Dockerfile                    # Frontend container (Vite dev/preview)
│   ├── package.json
│   └── vite.config.js
│
├── log-server-demo/                  # Python FastAPI log generator for testing
│   ├── main.py                       # Generates heavy realistic logs continuously
│   └── requirements.txt
│
└── ProjectDemo /                     # UI screenshots
    ├── Home_Page.png
    ├── Auth_Page.png
    ├── Dashboard_Page.png
    └── Features_Page.png
```

---

## 🛠️ Tech Stack

### Backend — Rust Workspace

| Layer | Crate | Key Dependencies | Purpose |
|---|---|---|---|
| **Math Core** | `zerch-core` | *(zero external deps)* | Cosine similarity & Euclidean distance |
| **Embeddings** | `zerch-embed` | `ort 2.0` (ONNX Runtime + CUDA), `tokenizers`, `hf-hub`, `ndarray` | Load model, tokenize, run inference, mean pool, L2 normalize |
| **Storage** | `zerch-storage` | `anyhow` | Append-only binary vector store (`zerch_data.bin`) |
| **CLI Engine** | `zerch-engine` | `zerch-core`, `zerch-embed`, `zerch-storage` | Terminal-based indexing and search |
| **HTTP API** | `zerch-api` | `actix-web 4`, `actix-multipart`, `actix-cors`, `tokio`, `reqwest`, `serde_json`, `uuid`, `chrono`, `dotenv`, `env_logger` | Full REST API for the React frontend |
| **MCP Server** | `zerch-mcp-server` | `axum 0.7`, `tokio`, `serde_json`, `tracing`, `tracing-subscriber` | JSON-RPC 2.0 MCP server for AI agent integration |

### Frontend — React SPA

| Layer | Technology | Version | Purpose |
|---|---|---|---|
| **Framework** | React | 19.x | UI component framework |
| **Build Tool** | Vite | 8.x | Dev server, HMR, production bundler |
| **Styling** | Vanilla CSS + Custom Properties | — | Neobrutalism dark theme, animations |
| **Analytics** | `@vercel/analytics` | 2.x | Deployment analytics |
| **Routing** | Custom `useNavigate` hook | — | Lightweight SPA navigation (no router lib) |
| **Auth** | React Context + localStorage | — | Session persistence across reloads |

### AI & Machine Learning

| Component | Technology | Details |
|---|---|---|
| **Embedding Model** | `sentence-transformers/all-MiniLM-L6-v2` | 384-dimensional vectors, downloaded from HuggingFace Hub via `hf-hub` |
| **Inference Runtime** | ONNX Runtime 2.0 (`ort`) | Runs locally; auto-detects CUDA GPU, falls back to CPU |
| **Tokenizer** | HuggingFace `tokenizers` | Truncated to 512 tokens, WordPiece tokenization |
| **Similarity Metric** | Cosine Similarity | Dot product over L2 norms, custom zero-dep Rust impl |
| **LLM Summarization** | Groq API (`openai/gpt-oss-120b`) | Cloud-based, OpenAI-compatible endpoint, 512 max output tokens |

### Infrastructure & DevOps

| Tool | Purpose |
|---|---|
| **Docker** | Containerizes the Rust API (multi-stage build: `rust:latest` → `ubuntu:24.04`) |
| **Docker Compose** | Orchestrates `zerch-api` (`:8080`) + `zerch-ui` (`:5173`) with shared network |
| **`setup.sh`** | Auto-installs Docker/Compose + builds + launches in one command |
| **Log Demo Server** | Python FastAPI + threading generates realistic log streams for testing |

---

## 📦 Binary Storage Format

Every embedded log line is serialized into `zerch_data.bin` using a flat, append-only binary format with zero serialization overhead:

```
┌──────────────────┬──────────────────────────┬──────────────────┬──────────────────┐
│  vec_len (u32)   │  vector data             │  text_len (u32)  │  text (UTF-8)    │
│  4 bytes LE      │  vec_len × 4 bytes (f32) │  4 bytes LE      │  text_len bytes  │
└──────────────────┴──────────────────────────┴──────────────────┴──────────────────┘
```

For `all-MiniLM-L6-v2`, `vec_len` is always **384**, making each record:
- **Fixed:** `4 + (384 × 4) + 4` = **1,544 bytes** of header + vector
- **Variable:** `text_len` bytes of raw UTF-8 log text

Vectors are written using `unsafe { std::slice::from_raw_parts(...) }` for **zero-copy** byte reinterpretation — no serialization library, no overhead.

---

## 📖 How It Works

### 1. Indexing — Upload → Embed → Store

```
User uploads log.txt
        │
        ▼
  Actix-Web parses multipart
        │
        ▼
  For each non-empty line:
    ├── extract_log_level()   → ERROR / WARN / INFO / DEBUG
    ├── extract_service()     → text between first [...]
    ├── extract_timestamp()   → current datetime
    └── LocalEmbedder.embed(line)
            │
            ├── HF Tokenizer → input_ids, attention_mask, token_type_ids
            ├── Truncate to 512 tokens
            ├── ONNX Runtime inference → last_hidden_state [1, seq, 384]
            ├── Mean pooling over seq dimension → [384]
            └── L2 normalization → unit vector [384]
            │
            ▼
  VectorStore.append_vector(vector, text)
  → write to zerch_data.bin (binary, append-only)
        │
        ▼
  Return LogEntry[] JSON to React frontend
```

### 2. Searching — Query → Embed → Rank

```
User types: "why is the database failing?"
        │
        ▼
  GET /api/search?q=why+is+the+database+failing%3F&limit=5
        │
        ▼
  LocalEmbedder.embed(query) → query_vector [384]
        │
        ▼
  Open zerch_data.bin, scan linearly:
    For each stored record:
      ├── Read vec_len (u32 LE)
      ├── Read vec_len × f32 → log_vector
      ├── cosine_similarity(query_vector, log_vector).score
      └── Track top-N by score
        │
        ▼
  Sort descending, truncate to limit
  Return SearchResult[] with score + text
        │
        ▼
  React renders ranked results with score bars
```

### 3. AI Summarization — Click → Groq → Insight

```
User clicks a search result
        │
        ▼
  POST /api/summarize { "text": "<raw log line>" }
        │
        ▼
  Build Groq API payload:
    model: openai/gpt-oss-120b
    system: "Summarize this log. Suggest a fix."
    user: <log text>
        │
        ▼
  reqwest::Client → https://api.groq.com/openai/v1/chat/completions
        │
        ▼
  Extract choices[0].message.content
  Return { "summary": "..." }
        │
        ▼
  Displayed in AI LOG INSIGHTS sidebar
```

---

## 🚀 Quick Start

### Prerequisites

| Requirement | Version | Notes |
|---|---|---|
| **Rust** | stable (1.75+) | Install via [rustup](https://rustup.rs/) |
| **Node.js** | 18+ | For the React frontend |
| **npm** | 9+ | Bundled with Node.js |
| **GROQ_API_KEY** | — | Only needed for AI summarization — [get one free](https://console.groq.com/) |
| **CUDA** *(optional)* | 11.x+ | GPU acceleration; auto-falls-back to CPU |

---

### Option A — Manual Setup (Recommended for Development)

#### Step 1 — Clone the repo

```bash
git clone https://github.com/sagnik3788/zerch.git
cd zerch
```

#### Step 2 — Set up environment variables

Create a `.env` file in the project root:

```bash
# .env
GROQ_API_KEY=your_groq_api_key_here
```

> **Note:** The `GROQ_API_KEY` is only required for the `/api/summarize` endpoint.
> All other features (upload, search, embed) work without it.

#### Step 3 — Build & run the Rust API server

```bash
# Build the entire workspace
cargo build

# Run the API server (downloads the embedding model on first launch ~80MB)
cargo run -p zerch-api
```

The server will:
1. Download `sentence-transformers/all-MiniLM-L6-v2` from HuggingFace Hub (cached after first run)
2. Initialize the ONNX Runtime session (CUDA if available, otherwise CPU)
3. Start listening on `http://127.0.0.1:8080`

> ⏳ **First launch takes 30–90 seconds** for the model download. Subsequent starts are instant.

#### Step 4 — Run the React frontend

Open a second terminal:

```bash
cd ui
npm install
npm run dev
```

The Vite dev server starts at **`http://localhost:5173`**

#### Step 5 — Log in and start searching

Open your browser at `http://localhost:5173` and use the demo credentials:

```
Email:    admin@123.com
Password: admin123
```

---

### Option B — Docker Compose (Recommended for Production)

#### One-shot automated setup

```bash
git clone https://github.com/sagnik3788/zerch.git
cd zerch

# Set your Groq API key
export GROQ_API_KEY=your_groq_api_key_here

# Run the automated setup script (installs Docker if needed)
chmod +x setup.sh
./setup.sh
```

#### Or manually with Docker Compose

```bash
# Build and start both services
GROQ_API_KEY=your_key_here docker-compose up --build -d

# Check service health
docker-compose ps

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

| Service | URL | Notes |
|---|---|---|
| **Rust API** | `http://localhost:8080` | Actix-Web server + embedding model |
| **React UI** | `http://localhost:5173` | Vite preview server |

---

### CLI Usage — `zerch-engine`

Use the CLI to index and search logs directly from your terminal without the browser:

```bash
# Index a log file (embeds every line and stores in zerch_data.bin)
cargo run -p zerch-engine -- path/to/your/app.log

# Semantic search across indexed logs
cargo run -p zerch-engine -- --search "database connection timeout"
cargo run -p zerch-engine -- --search "authentication failed for user"
cargo run -p zerch-engine -- --search "out of memory error"
```

**Example output:**

```
Loading model... this might take a moment.
Searching for: "database connection timeout"
Query embedded in 12.3ms

Top 5 Results:
--------------------------------------------------
[1] Score: 0.9412 | [DB] ERROR: Connection to postgres:5432 refused after 3 retries
[2] Score: 0.8871 | [API] WARN: Request timed out waiting for database pool
[3] Score: 0.8543 | [Worker] ERROR: pg_connect() failed: timeout exceeded 30s
[4] Score: 0.7902 | [Auth] INFO: Falling back to cached session (db unavailable)
[5] Score: 0.7441 | [Monitor] WARN: Database health check failed — latency 2847ms
```

---

### Log Demo Server (Python)

The repo includes a FastAPI log generator for realistic testing:

```bash
cd log-server-demo
pip install -r requirements.txt
python main.py
```

This spins up a server on `http://0.0.0.0:8000` that:
- Continuously generates realistic INFO / WARN / ERROR logs to `app.log`
- Streams live logs via `GET /logs/stream` (Server-Sent Events)
- Provides `/logs`, `/logs/tail`, and `/stats` endpoints

Then upload the generated `app.log` to Zerch for semantic search!

---

## 📡 API Reference

All endpoints are served at `http://localhost:8080`. CORS is fully open (any origin).

### `GET /health`

Health check — confirms the Rust engine is running.

**Response:**
```json
{
  "status": "healthy",
  "engine": "Running (Rust)"
}
```

---

### `POST /api/upload`

Upload a log file for embedding and indexing.

**Request:** `multipart/form-data` with a `file` field containing the log file.

> ⚠️ Each upload **clears the previous vector store** before indexing the new file.

**Response:**
```json
{
  "success": true,
  "count": 1234,
  "message": "Successfully indexed 1234 logs",
  "logs": [
    {
      "id": "log-uuid-0",
      "ts": "2024-04-04 10:23:45",
      "level": "ERROR",
      "service": "Database",
      "msg": "[Database] ERROR: Connection refused",
      "similarity": 0.75
    }
  ]
}
```

**Log level detection** (keyword-based):

| Keyword in line | Level assigned |
|---|---|
| `error`, `err` | `ERROR` |
| `warn`, `warning` | `WARN` |
| `debug` | `DEBUG` |
| *(default)* | `INFO` |

---

### `GET /api/search`

Semantic search over all indexed log vectors.

**Query Parameters:**

| Parameter | Type | Default | Description |
|---|---|---|---|
| `q` | string | *(required)* | Natural language search query |
| `limit` | integer | `5` | Maximum number of results to return |

**Example:**
```
GET /api/search?q=memory+leak+in+worker+thread&limit=10
```

**Response:**
```json
{
  "success": true,
  "query": "memory leak in worker thread",
  "count": 5,
  "message": "Found 5 matching logs",
  "results": [
    {
      "id": "result-42",
      "score": 0.9234,
      "text": "[Worker] WARN: Heap size growing unbounded — possible memory leak"
    }
  ]
}
```

---

### `POST /api/summarize`

AI-powered log analysis via Groq.

**Request body:**
```json
{
  "text": "[Auth] ERROR: JWT token validation failed — signature mismatch for user-9821"
}
```

**Response:**
```json
{
  "summary": "This log indicates a JWT signature validation failure for user-9821. This typically happens when: (1) the signing key changed after the token was issued, (2) the token was tampered with, or (3) there's a mismatch between environments. Fix: verify your JWT_SECRET environment variable is consistent across all services and hasn't been rotated without re-issuing tokens."
}
```

> Requires `GROQ_API_KEY` in your `.env` file.

---

## 🤖 MCP Server — AI Agent Integration

Zerch ships a fully spec-compliant **Model Context Protocol (MCP) server** (`zerch-mcp-server`), enabling any MCP-compatible AI agent to embed text and search logs programmatically.

### Available Tools

| Tool | Input | Output | Description |
|---|---|---|---|
| `embed_text` | `{ "text": string }` | `{ "dimension": 384, "vector": [f32 × 384] }` | Embed text + store in `zerch_data.bin` |
| `search_logs` | `{ "query": string, "top_k"?: int }` | `{ "top_k": int, "hits": [{ score, text }] }` | Semantic search over indexed logs |

### Starting the MCP Server

**stdio transport** (for Claude Desktop, local agents):
```bash
# Uses stdin/stdout for JSON-RPC 2.0 communication
cargo run -p zerch-mcp-server
```

**HTTP transport** (for remote agents):
```bash
ZERCH_MCP_TRANSPORT=http \
ZERCH_MCP_HTTP_HOST=0.0.0.0 \
ZERCH_MCP_HTTP_PORT=8787 \
ZERCH_MCP_HTTP_PATH=/mcp \
cargo run -p zerch-mcp-server
```

### Configuration (Environment Variables)

| Variable | Default | Description |
|---|---|---|
| `ZERCH_MCP_TRANSPORT` | `stdio` | Transport mode: `stdio` or `http` |
| `ZERCH_MCP_HTTP_HOST` | `0.0.0.0` | HTTP bind host |
| `ZERCH_MCP_HTTP_PORT` | `8787` | HTTP bind port |
| `ZERCH_MCP_HTTP_PATH` | `/mcp` | MCP endpoint path |
| `ZERCH_MCP_NAME` | `zerch-mcp-server` | Server name reported to clients |
| `ZERCH_STORE_PATH` | `zerch_data.bin` | Path to the binary vector store |
| `RUST_LOG` | `zerch_mcp_server=info` | Log verbosity (stderr) |

### Example JSON-RPC Interaction

```json
// List available tools
→ {"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}
← {"jsonrpc":"2.0","id":1,"result":{"tools":[{"name":"embed_text",...},{"name":"search_logs",...}]}}

// Embed a log line
→ {"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"embed_text","arguments":{"text":"ERROR: disk full on /dev/sda1"}}}
← {"jsonrpc":"2.0","id":2,"result":{"content":[{"type":"text","text":"{\"dimension\":384,\"vector\":[...]}"}]}}

// Search across logs
→ {"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"search_logs","arguments":{"query":"disk space issues","top_k":3}}}
← {"jsonrpc":"2.0","id":3,"result":{"content":[{"type":"text","text":"{\"top_k\":3,\"hits\":[...]}"}]}}
```

---

## 🔧 Crate Deep Dive

### `zerch-core` — Pure Math, Zero Dependencies

The mathematical foundation of Zerch. No external crates — just clean Rust.

```rust
// Cosine similarity between two embedding vectors
let sim = cosine_similarity(&query_vector, &log_vector);
println!("Score: {:.4}", sim.score);  // Range: [-1.0, 1.0]

// Euclidean distance (alternative metric)
let dist = euclidean_distance(&vec_a, &vec_b);
println!("Distance: {:.4}", dist.distance);
```

Both functions return sentinel values (`f32::NEG_INFINITY` / `f32::INFINITY`) on dimension mismatch.

### `zerch-embed` — Local Inference Pipeline

```
Input text
  → HuggingFace Tokenizer (WordPiece, max 512 tokens)
  → ONNX Runtime Session (CUDAExecutionProvider → CPU fallback)
  → last_hidden_state: shape [1, seq_len, 384]
  → Mean pooling across seq dimension → [384]
  → L2 normalization → unit vector [384]
  → Vec<f32>
```

The model (`all-MiniLM-L6-v2`) is downloaded once from HuggingFace Hub and cached in `~/.cache/huggingface/`.

### `zerch-storage` — Zero-Copy Binary I/O

```rust
let store = VectorStore::new("zerch_data.bin");

// Zero-copy write: reinterprets &[f32] as &[u8] via unsafe ptr cast
store.append_vector(&vector, "ERROR: connection refused")?;

// Clear the store (truncates the file)
store.clear()?;
```

---

## 🌐 Environment Variables Reference

| Variable | Crate | Required | Default | Description |
|---|---|---|---|---|
| `GROQ_API_KEY` | `zerch-api` | For `/api/summarize` | — | Groq cloud LLM API key |
| `ZERCH_API_HOST` | `zerch-api` | No | `127.0.0.1` | API bind host |
| `ZERCH_API_PORT` | `zerch-api` | No | `8080` | API bind port |
| `RUST_LOG` | `zerch-api` | No | `info` | Log verbosity |
| `ZERCH_MCP_TRANSPORT` | `zerch-mcp-server` | No | `stdio` | MCP transport |
| `ZERCH_MCP_HTTP_HOST` | `zerch-mcp-server` | No | `0.0.0.0` | MCP HTTP host |
| `ZERCH_MCP_HTTP_PORT` | `zerch-mcp-server` | No | `8787` | MCP HTTP port |
| `ZERCH_MCP_HTTP_PATH` | `zerch-mcp-server` | No | `/mcp` | MCP HTTP path |
| `ZERCH_STORE_PATH` | `zerch-mcp-server` | No | `zerch_data.bin` | Vector store path |

---

## 🏷️ Known Limitations

| Limitation | Status |
|---|---|
| **Linear scan search** | No ANN index yet — scales linearly with store size |
| **512-token truncation** | Log lines longer than 512 tokens are silently truncated |
| **CUDA optional** | Compiled with `features = ["cuda"]` — may need adjustment on non-CUDA systems |
| **Timestamp parsing** | `extract_timestamp()` always returns current time, not the log's own timestamp |
| **Hardcoded API URL** | Frontend has `http://localhost:8080` hardcoded |
| **No auth on API** | All endpoints are publicly accessible — no token/key protection |
| **Single-threaded embedding** | `Mutex<LocalEmbedder>` serializes concurrent embedding requests |

---

## 📄 License

```
MIT License

Copyright (c) 2026 Sagnik Das

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
```

See the [LICENSE](./LICENSE) file for the full license text.

---

<div align="center">

**Built with ❤️ and Rust**

*If Zerch helped you debug something nasty, give it a ⭐*

<br/>

[![Rust](https://img.shields.io/badge/Made_with-Rust-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/UI_in-React_19-61DAFB?style=flat-square&logo=react&logoColor=black)](https://reactjs.org/)
[![ONNX](https://img.shields.io/badge/Embeddings-ONNX_Runtime-005CED?style=flat-square)](https://onnxruntime.ai/)

</div>