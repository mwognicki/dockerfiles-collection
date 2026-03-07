FROM ghcr.io/mwognicki/nodejs-pnpm:22-10

LABEL org.opencontainers.image.authors="Marek Kapusta-Ognicki <marek@ognicki.ooo>" \
      org.opencontainers.image.vendor="Marek Kapusta-Ognicki" \
      org.opencontainers.image.title="ghcr.io/mwognicki/nodejs-pnpm-chrome" \
      org.opencontainers.image.description="Node.js + pnpm image with Google Chrome dependencies tailored for Puppeteer workloads." \
      org.opencontainers.image.url="https://ognicki.ooo" \
      org.opencontainers.image.documentation="https://ognicki.ooo"

# Install rpm (needed to install local RPM files) and Chrome dependencies
RUN microdnf install -y \
    rpm \
    wget \
    fontconfig \
    freetype \
    libX11 \
    libXcomposite \
    libXcursor \
    libXdamage \
    libXext \
    libXi \
    libXrandr \
    libXrender \
    libXtst \
    cups-libs \
    dbus-libs \
    libxcb \
    libdrm \
    mesa-libgbm \
    libxshmfence \
    nss \
    nspr \
    atk \
    at-spi2-atk \
    cairo \
    pango \
    alsa-lib \
    libxkbcommon \
    && microdnf clean all

# Download and install Chrome using rpm directly
RUN wget -q https://dl.google.com/linux/direct/google-chrome-stable_current_x86_64.rpm && \
    rpm -ivh --nodeps google-chrome-stable_current_x86_64.rpm && \
    rm google-chrome-stable_current_x86_64.rpm
