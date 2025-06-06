FROM rust:latest AS builder
WORKDIR /usr/src/inception-icore-server
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/inception-icore-server/target/release/inception-icore-server /usr/local/bin/inception-icore-server
CMD ["inception-icore-server"]
