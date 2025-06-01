# 🧠 ChromaDB Setup & Usage (for Inception MCP Server)

ChromaDB is used for latent (vector-based) memory in the Inception MCP system.  
It provides fast similarity search over embedded representations.

---

## ✅ Requirements

- Docker installed
- Port `8000` must be free

---

## 🐳 Run ChromaDB (Dev Mode)

```bash
docker compose -f docker-compose.dev.yml up -d chromadb
```

Or manually:

```bash
docker run -p 8000:8000 chromadb/chroma:0.6.3
```

---

## 📦 Default API Endpoint

```
http://localhost:8000/api/v1
```

---

## 🔧 API Overview

### ➕ Create Collection

```bash
curl -X POST http://localhost:8000/api/v1/collections \
  -H "Content-Type: application/json" \
  -d '{"name": "mem"}'
```

### 📥 Add Vectors (embed manually)

```bash
curl -X POST http://localhost:8000/api/v1/collections/mem/add \
  -H "Content-Type: application/json" \
  -d '{
        "ids": ["abc"],
        "embeddings": [[0.1, 0.2, 0.3]],
        "metadatas": [{"source": "test"}]
      }'
```

### 🔎 Query

```bash
curl -X POST http://localhost:8000/api/v1/collections/mem/query \
  -H "Content-Type: application/json" \
  -d '{
        "query_embeddings": [[0.1, 0.2, 0.3]],
        "n_results": 3
      }'
```

---

## 🧠 Notes

- ChromaDB **no longer accepts raw `documents`**. You must provide `embeddings`.
- You can generate embeddings using:

  - OpenAI API (text-embedding-3-small)
  - HuggingFace models (sentence-transformers)
  - Local LLMs

- All vector dimensions must be the same (e.g. `768` or `1536`)

---

## 📁 Storage

ChromaDB can persist data if started with:

```yaml
environment:
  - IS_PERSISTENT=TRUE
  - PERSIST_DIRECTORY=/chroma/chroma
```

Volume mount example:

```yaml
volumes:
  - ./chromadb:/chroma/chroma
```

---

## 🔐 Telemetry

Disable telemetry (recommended for local dev):

```yaml
environment:
  - ANONYMIZED_TELEMETRY=FALSE
```

---

## 🧪 Swagger UI

Visit:

```
http://localhost:8000/docs
```

For interactive API reference.
