# Latent Memory (ChromaDB v0.6.3 Integration)

This document outlines the current implementation of the Latent Memory system in the `inception-ICORE-server` project, based on integration with **ChromaDB v0.6.3** using REST API v2.

---

## Status

- **ChromaDB** running via Docker (version `0.6.3`)
- `latent.rs` implements manual vector embedding and search
- `/api/mem/latent/embed` stores vectorized entries
- `/api/mem/latent/query` performs similarity search over embeddings

---

## Setup

### Docker Service

```yaml
# docker-compose.dev.yml
services:
  chromadb:
    image: chromadb/chroma:0.6.3
    ports:
      - "8000:8000"
    environment:
      - IS_PERSISTENT=FALSE
      - ANONYMIZED_TELEMETRY=FALSE
    volumes:
      - /tmp/chroma:/tmp/chroma
```

Start dev instance:

```bash
docker compose -f docker-compose.dev.yml up -d chromadb
```

---

## Collection

Collection must be created manually via:

```bash
curl -X POST http://localhost:8000/api/v2/tenants/default_tenant/databases/default_database/collections \
  -H "Content-Type: application/json" \
  -d '{
        "name": "mem",
        "embedding_function": null,
        "dimension": 1536
      }'
```

Store the returned UUID and hardcode it in `latent.rs` (temporary workaround).

---

## Embed

```bash
curl -X POST http://localhost:8080/api/mem/latent/embed \
  -H "Content-Type: application/json" \
  -d '{"id": "abc", "content": "rust memory"}'
```

This uses a dummy 1536-dim vector (all zeros). Replace with real embeddings later.

---

## Query

```bash
curl -X POST http://localhost:8080/api/mem/latent/query \
  -H "Content-Type: application/json" \
  -d '{"content": "rust memory"}'
```

Returns most similar IDs.

---

## Cleanup

To reset:

```bash
docker compose -f docker-compose.dev.yml down -v
```

---

## Next

- Replace dummy vectors with actual embedding generator (OpenAI or local)
- Use `.env` for dynamic collection UUID injection
- Support multiple collections (optional)
