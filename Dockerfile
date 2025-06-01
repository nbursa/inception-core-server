FROM rust:latest AS builder
WORKDIR /usr/src/inception-mcp-server
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/inception-mcp-server/target/release/inception-mcp-server /usr/local/bin/inception-mcp-server
CMD ["inception-mcp-server"]
