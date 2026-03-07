ARG PYTHON_BASE_TAG=3.12-redquartz
FROM ghcr.io/mwognicki/python:${PYTHON_BASE_TAG}

LABEL org.opencontainers.image.authors="Marek Kapusta-Ognicki <marek@ognicki.ooo>" \
      org.opencontainers.image.vendor="Marek Kapusta-Ognicki" \
      org.opencontainers.image.title="ghcr.io/mwognicki/micromamba" \
      org.opencontainers.image.description="Micromamba image based on Python 3.12 red-quartz for lightweight conda-style environments." \
      org.opencontainers.image.url="https://ognicki.ooo" \
      org.opencontainers.image.documentation="https://ognicki.ooo"

ARG MAMBA_ROOT_PREFIX=/opt/micromamba
ENV MAMBA_ROOT_PREFIX=${MAMBA_ROOT_PREFIX}
ENV PATH=${MAMBA_ROOT_PREFIX}/bin:${PATH}

RUN microdnf install -y tar bzip2 curl \
 && microdnf clean all

RUN micro_arch="linux-64" \
 && curl -fsSL "https://micro.mamba.pm/api/micromamba/${micro_arch}/latest" \
 | tar -xvj -C /usr/local/bin --strip-components=1 bin/micromamba \
 && micromamba --version

WORKDIR /app
