FROM rockylinux/rockylinux:10-minimal AS workspace

ARG PNPM_VERSION="10.26.0"

RUN microdnf install -y curl && \
    curl -fsSL https://rpm.nodesource.com/setup_22.x | bash - && \
    microdnf install -y nodejs && \
    microdnf clean all

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

RUN curl -fsSL https://get.pnpm.io/install.sh | env PNPM_VERSION=${PNPM_VERSION} PNPM_HOME=/pnpm sh -