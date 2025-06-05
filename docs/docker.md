# Docker Setup for Inception MCP Server

This project supports two Docker modes optimized for different workflows:

---

## Development Mode (`docker-compose.dev.yml`)

Minimal setup — runs only ChromaDB in Docker.  
You run MCP server locally via `cargo run`.

### Setup

```bash
docker compose -f docker-compose.dev.yml up -d
```

Then, in another terminal:

```bash
cargo run
```

### Why use this?

- Less CPU/RAM usage (ideal for low-power machines)
- Fast rebuilds, instant feedback
- You can edit Rust code freely without rebuilding Docker

---

## Production Mode (`docker-compose.prod.yml`)

Runs both MCP server and ChromaDB inside Docker containers.
Use this for end-to-end testing or deployment.

### Setup

```bash
docker compose -f docker-compose.prod.yml up --build
```

### Why use this?

- Reproducible, portable environment
- No need for local Rust/toolchain setup
- Includes persistent ChromaDB volume

---

## Switching Modes

- For everyday dev: use **dev mode**
- For integration tests or demo builds: use **prod mode**

---

## File Overview

```
docker-compose.dev.yml      # Lightweight ChromaDB only
docker-compose.prod.yml     # Full stack: MCP + ChromaDB
Dockerfile                  # Rust MCP server build definition
```

---

## Notes

- `.env` must contain `CHROMADB_URL=http://localhost:8000`
- Persistent ChromaDB storage is mapped to `./chromadb/`
- MCP server listens on port `8080`, ChromaDB on `8000`

---
