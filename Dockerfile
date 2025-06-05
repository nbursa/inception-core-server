FROM rust:latest AS builder
WORKDIR /usr/src/inception-ICORE-server
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/inception-ICORE-server/target/release/inception-ICORE-server /usr/local/bin/inception-ICORE-server
CMD ["inception-ICORE-server"]
