<!-- docker compose -f docker-compose.dev.yml up -d -->

<!-- docker compose -f docker-compose.dev.yml up --build -d -->

# llm server

```bash
llama.cpp/build/bin/llama-server \
 --model models/mistral-7b-q4.gguf \
 --ctx-size 256 \
 --threads 1 \
 --port 11434
```
