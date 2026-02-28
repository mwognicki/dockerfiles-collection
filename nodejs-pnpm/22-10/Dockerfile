FROM rockylinux/rockylinux:10-minimal AS workspace

LABEL org.opencontainers.image.authors="Marek Kapusta-Ognicki <marek@ognicki.ooo>" \
      org.opencontainers.image.vendor="Marek Kapusta-Ognicki" \
      org.opencontainers.image.title="ghcr.io/mwognicki/nodejs-pnpm" \
      org.opencontainers.image.description="Node.js 22 and pnpm 10 workspace image for reproducible JavaScript package builds." \
      org.opencontainers.image.url="https://ognicki.ooo" \
      org.opencontainers.image.documentation="https://ognicki.ooo"

ARG PNPM_VERSION="10.26.0"

RUN microdnf install -y curl && \
    curl -fsSL https://rpm.nodesource.com/setup_22.x | bash - && \
    microdnf install -y nodejs && \
    microdnf clean all

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

RUN curl -fsSL https://get.pnpm.io/install.sh | env PNPM_VERSION=${PNPM_VERSION} PNPM_HOME=/pnpm sh -
