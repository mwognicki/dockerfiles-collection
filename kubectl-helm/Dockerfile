# ── Stage 1: download & verify binaries ──────────────────────────────────────
FROM almalinux/10-kitten-minimal AS builder

ARG KUBECTL_VERSION=v1.32.3
ARG HELM_VERSION=v3.17.2

RUN microdnf install -y curl ca-certificates tar gzip \
    && microdnf clean all

# kubectl — download + checksum verify
RUN curl -fsSL "https://dl.k8s.io/release/${KUBECTL_VERSION}/bin/linux/amd64/kubectl" \
      -o /usr/local/bin/kubectl \
    && curl -fsSL "https://dl.k8s.io/release/${KUBECTL_VERSION}/bin/linux/amd64/kubectl.sha256" \
      -o /tmp/kubectl.sha256 \
    && echo "$(cat /tmp/kubectl.sha256)  /usr/local/bin/kubectl" | sha256sum -c - \
    && chmod +x /usr/local/bin/kubectl

# helm — download + checksum verify
RUN curl -fsSL "https://get.helm.sh/helm-${HELM_VERSION}-linux-amd64.tar.gz" \
      -o /tmp/helm.tar.gz \
    && curl -fsSL "https://get.helm.sh/helm-${HELM_VERSION}-linux-amd64.tar.gz.sha256sum" \
      -o /tmp/helm.sha256 \
    && sha256sum -c /tmp/helm.sha256 \
    && tar -xz --strip-components=1 -C /usr/local/bin -f /tmp/helm.tar.gz linux-amd64/helm \
    && chmod +x /usr/local/bin/helm

# ── Stage 2: lean runtime image ───────────────────────────────────────────────
FROM almalinux/10-kitten-micro

LABEL org.opencontainers.image.authors="Marek Kapusta-Ognicki <marek@ognicki.ooo>" \
      org.opencontainers.image.vendor="Marek Kapusta-Ognicki" \
      org.opencontainers.image.title="ghcr.io/mwognicki/kubectl-helm" \
      org.opencontainers.image.description="Lean Alma Linux image with kubectl and Helm for CI/CD and Kubernetes automation workflows." \
      org.opencontainers.image.url="https://ognicki.ooo" \
      org.opencontainers.image.documentation="https://ognicki.ooo"

COPY --from=builder /usr/local/bin/kubectl /usr/local/bin/kubectl
COPY --from=builder /usr/local/bin/helm    /usr/local/bin/helm
COPY --from=builder /etc/ssl/certs         /etc/ssl/certs

RUN useradd -m -u 1001 -s /bin/sh ciuser \
    && mkdir -p /home/ciuser/.kube /home/ciuser/.config/helm /home/ciuser/.cache/helm \
    && chown -R ciuser:ciuser /home/ciuser

USER ciuser
WORKDIR /workspace

ENTRYPOINT ["helm"]
CMD ["--help"]
