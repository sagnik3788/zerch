# zerch-api

HTTP API server for the Zerch semantic log search engine.

## Endpoints

- `GET /health` - Health check
- `POST /api/upload` - Upload log files
- `GET /api/search` - Semantic search
- `POST /api/summarize` - AI-powered log summarization

## Usage

```bash
zerch-api
```

Requires `GROQ_API_KEY` in environment or `.env` file for AI summarization.

## License

MIT
