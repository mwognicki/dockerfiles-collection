FROM rockylinux/rockylinux:10-minimal

LABEL org.opencontainers.image.authors="Marek Kapusta-Ognicki <marek@ognicki.ooo>" \
      org.opencontainers.image.vendor="Marek Kapusta-Ognicki" \
      org.opencontainers.image.title="ghcr.io/mwognicki/python" \
      org.opencontainers.image.description="Python 3.12 image on Rocky Linux 10 with pip preinstalled (red-quartz variant)." \
      org.opencontainers.image.url="https://ognicki.ooo" \
      org.opencontainers.image.documentation="https://ognicki.ooo"

# Install minimal packages
RUN microdnf install -y \
    curl \
    python312 \
    unzip \
    wget \
    && microdnf clean all

RUN alternatives --install /usr/bin/python python /usr/bin/python3.12 1

RUN curl -O https://bootstrap.pypa.io/get-pip.py && \
    python get-pip.py && \
    python -m pip install --upgrade pip && \
    rm -f get-pip.py
