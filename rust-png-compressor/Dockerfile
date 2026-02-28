FROM rust:1.93-bookworm AS builder

RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && apt-get install -y --no-install-recommends musl-tools

WORKDIR /src

COPY src/png-compressor/Cargo.toml src/png-compressor/Cargo.lock ./

RUN mkdir src && \
    echo 'fn main() {}' > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    rm -rf src

COPY src/png-compressor/src ./src
RUN touch src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

FROM rockylinux/rockylinux:10-minimal

LABEL org.opencontainers.image.authors="Marek Kapusta-Ognicki <marek@ognicki.ooo>" \
      org.opencontainers.image.vendor="Marek Kapusta-Ognicki" \
      org.opencontainers.image.title="ghcr.io/mwognicki/png-compressor" \
      org.opencontainers.image.description="Minimal Rust-based PNG compression CLI image using oxipng." \
      org.opencontainers.image.url="https://ognicki.ooo" \
      org.opencontainers.image.documentation="https://ognicki.ooo"

COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/png-compressor /usr/local/bin/png-compressor
