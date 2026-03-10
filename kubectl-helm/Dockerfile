FROM rockylinux/rockylinux:10-minimal

ARG KUBECTL_VERSION=v1.34.3
ARG HELM_VERSION=v3.19.1

LABEL org.opencontainers.image.authors="Marek Kapusta-Ognicki <marek@ognicki.ooo>" \
      org.opencontainers.image.vendor="Marek Kapusta-Ognicki" \
      org.opencontainers.image.title="ghcr.io/mwognicki/kubectl-helm" \
      org.opencontainers.image.description="Rocky Linux image with kubectl and Helm for CI/CD and Kubernetes automation workflows." \
      org.opencontainers.image.url="https://ognicki.ooo" \
      org.opencontainers.image.documentation="https://ognicki.ooo"

RUN microdnf install -y \
      curl \
      ca-certificates \
      tar \
      gzip \
      git \
    && microdnf clean all

# Install kubectl
RUN curl -fsSL "https://dl.k8s.io/release/${KUBECTL_VERSION}/bin/linux/amd64/kubectl" \
      -o /usr/local/bin/kubectl \
    && chmod +x /usr/local/bin/kubectl \
    && kubectl version --client

# Install helm
RUN curl -fsSL "https://get.helm.sh/helm-${HELM_VERSION}-linux-amd64.tar.gz" \
      | tar -xz --strip-components=1 -C /usr/local/bin linux-amd64/helm \
    && chmod +x /usr/local/bin/helm \
    && helm version

# Create a non-root user for pipeline execution
RUN useradd -m -u 1001 -s /bin/sh ciuser

# Helm cache/config dirs owned by ciuser
RUN mkdir -p /home/ciuser/.kube /home/ciuser/.config/helm /home/ciuser/.cache/helm \
    && chown -R ciuser:ciuser /home/ciuser

USER ciuser
WORKDIR /workspace

# Smoke-test both tools at build time
RUN kubectl version --client && helm version

ENTRYPOINT ["helm"]
CMD ["--help"]
