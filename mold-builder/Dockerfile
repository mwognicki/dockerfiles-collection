FROM rockylinux/rockylinux:10-minimal AS rust-builder

LABEL org.opencontainers.image.authors="Marek Kapusta-Ognicki <marek@ognicki.ooo>" \
      org.opencontainers.image.vendor="Marek Kapusta-Ognicki" \
      org.opencontainers.image.title="ghcr.io/mwognicki/mold-builder" \
      org.opencontainers.image.description="Rust/C++ builder image with mold and clang configured for faster linking." \
      org.opencontainers.image.url="https://ognicki.ooo" \
      org.opencontainers.image.documentation="https://ognicki.ooo"

RUN microdnf install -y dnf dnf-plugins-core && \
    dnf -y install epel-release || \
    dnf -y install https://dl.fedoraproject.org/pub/epel/epel-release-latest-10.noarch.rpm && \
    dnf config-manager --set-enabled crb && \
    dnf -y install curl git cmake make gcc gcc-c++ clang && \
    dnf clean all && \
    rm -rf /var/cache/dnf

RUN git clone --depth 1 --branch stable https://github.com/rui314/mold.git /tmp/mold && \
    cd /tmp/mold && \
    ./install-build-deps.sh && \
    cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=clang++ -B build && \
    cmake --build build -j"$(nproc)" && \
    cmake --build build --target install && \
    mold --version

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:$PATH"

# Tell cargo to use clang + mold for linking
RUN mkdir -p /root/.cargo && printf \
    '[target.x86_64-unknown-linux-gnu]\nlinker = "clang"\nrustflags = ["-C", "link-arg=-fuse-ld=mold"]\n' \
    > /root/.cargo/config.toml
