
FROM docker.io/library/almalinux:8

ARG RUST_VERSION=1.98.1

# gcc-toolset-12 provides a modern gcc; Rust invokes `cc` as its linker driver.
RUN dnf install -y --setopt=install_weak_deps=False \
        gcc-toolset-12 \
        ca-certificates \
        curl \
        git \
        tar \
        xz \
    && dnf clean all \
    && rm -rf /var/cache/dnf

# Rustup: pin the toolchain rather than tracking `stable`, so rebuilds are
# reproducible.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --no-modify-path --profile minimal --default-toolchain "$RUST_VERSION"

ENV PATH="/opt/rh/gcc-toolset-12/root/usr/bin:/root/.cargo/bin:${PATH}"
ENV CARGO_TARGET_DIR="/build/target"

WORKDIR /src

CMD ["bash"]
