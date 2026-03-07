ARG ROCKY_VERSION=10-minimal
ARG RUST_TOOLCHAIN=stable
ARG ZIG_VERSION=0.13.0

# ─────────────────────────────────────────
# Stage 1: base system dependencies
# ─────────────────────────────────────────
FROM rockylinux/rockylinux:${ROCKY_VERSION} AS base

RUN microdnf -y update && \
    microdnf -y install \
        ca-certificates \
        curl \
        gcc \
        gcc-c++ \
        git \
        make \
        openssl-devel \
        pkgconf-pkg-config \
        tar \
        xz \
        gzip \
        which \
        findutils \
        shadow-utils && \
    microdnf clean all

# ─────────────────────────────────────────
# Stage 2: Rust toolchain
# ─────────────────────────────────────────
FROM base AS rust

ARG RUST_TOOLCHAIN

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:${PATH} \
    RUST_TOOLCHAIN=${RUST_TOOLCHAIN}

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --no-modify-path --profile default --default-toolchain ${RUST_TOOLCHAIN} && \
    rustc --version && \
    cargo --version

RUN rustup component add rustfmt clippy

# ─────────────────────────────────────────
# Stage 3: Zig toolchain
# ─────────────────────────────────────────
FROM rust AS workspace

ARG ZIG_VERSION
ARG TARGETARCH

LABEL org.opencontainers.image.authors="Marek Kapusta-Ognicki <marek@ognicki.ooo>" \
      org.opencontainers.image.vendor="Marek Kapusta-Ognicki" \
      org.opencontainers.image.title="ghcr.io/mwognicki/rust-zigbuild" \
      org.opencontainers.image.description="Rust + Zig cross-compilation image with cargo-zigbuild and preinstalled MUSL/GNU targets." \
      org.opencontainers.image.url="https://ognicki.ooo" \
      org.opencontainers.image.documentation="https://ognicki.ooo"

ENV ZIG_VERSION=${ZIG_VERSION}

# Map Docker's TARGETARCH to Zig's arch naming
RUN case "${TARGETARCH:-amd64}" in \
        amd64) ZIG_ARCH=x86_64 ;; \
        arm64) ZIG_ARCH=aarch64 ;; \
        *) echo "Unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
    esac && \
    ZIG_DIR="zig-linux-${ZIG_ARCH}-${ZIG_VERSION}" && \
    curl -sSL "https://ziglang.org/download/${ZIG_VERSION}/${ZIG_DIR}.tar.xz" \
        -o /tmp/zig.tar.xz && \
    tar -xf /tmp/zig.tar.xz -C /usr/local && \
    ln -s "/usr/local/${ZIG_DIR}/zig" /usr/local/bin/zig && \
    rm /tmp/zig.tar.xz && \
    zig version

RUN cargo install cargo-zigbuild

# Pre-install common cross-compilation targets
RUN rustup target add \
    x86_64-unknown-linux-musl \
    x86_64-unknown-linux-gnu \
    aarch64-unknown-linux-musl \
    aarch64-unknown-linux-gnu
