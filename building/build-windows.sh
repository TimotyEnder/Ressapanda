#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

OUT_DIR="${SCRIPT_DIR}/out"
DIST_DIR="${SCRIPT_DIR}/dist"
CACHE_DIR="${SCRIPT_DIR}/.cache"

IMAGE_NAME="ressapanda-windows-build"
RUST_VERSION="1.98.1"
TARGET_VOLUME="ressapanda-windows-target"
REGISTRY_VOLUME="ressapanda-cargo-registry"

say()  { printf '\n\033[1;34m==>\033[0m %s\n' "$*"; }
info() { printf '    %s\n' "$*"; }
die()  { printf '\n\033[1;31mERROR:\033[0m %s\n' "$*" >&2; exit 1; }

command -v docker >/dev/null || die "docker is required"

# Version comes straight from Cargo.toml so the artifact name never drifts.
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' "${REPO_ROOT}/Cargo.toml" | head -1)"
[ -n "${VERSION}" ] || die "could not read version from Cargo.toml"
EXE_NAME="Ressapanda-${VERSION}-windows-x64.exe"

mkdir -p "${OUT_DIR}" "${DIST_DIR}" "${CACHE_DIR}"

# ---------------------------------------------------------------------------
# 1. Build the toolchain image (cached; only rebuilds if the Dockerfile changes)
# ---------------------------------------------------------------------------
say "Ensuring Windows build image is present (Rust ${RUST_VERSION})..."
docker build \
    --build-arg "RUST_VERSION=${RUST_VERSION}" \
    -t "${IMAGE_NAME}" \
    -f "${SCRIPT_DIR}/Dockerfile.windows" \
    "${SCRIPT_DIR}"

# ---------------------------------------------------------------------------
# 2. Compile inside a container with the host repo mounted
# ---------------------------------------------------------------------------
say "Cross-compiling to x86_64-pc-windows-gnu (incremental; first run will take a while due to LTO)..."
docker run --rm \
    --user root \
    -v "${REPO_ROOT}:/src:ro" \
    -v "${OUT_DIR}:/out" \
    -v "${TARGET_VOLUME}:/build/target" \
    -v "${REGISTRY_VOLUME}:/root/.cargo/registry" \
    -e CARGO_TARGET_DIR=/build/target \
    -e "HOST_UID=$(id -u)" \
    -e "HOST_GID=$(id -g)" \
    -w /src \
    "${IMAGE_NAME}" \
    bash -c '
        set -euo pipefail
        x86_64-w64-mingw32-windres /src/building/appicon.rc /tmp/appicon.o
        RUSTFLAGS="-C link-arg=/tmp/appicon.o" cargo build --release --locked --target x86_64-pc-windows-gnu
        x86_64-w64-mingw32-strip --strip-unneeded /build/target/x86_64-pc-windows-gnu/release/ressapanda.exe
        install -m 0755 /build/target/x86_64-pc-windows-gnu/release/ressapanda.exe /out/ressapanda.exe
        chown "${HOST_UID}:${HOST_GID}" /out/ressapanda.exe
    '

BIN="${OUT_DIR}/ressapanda.exe"
[ -f "${BIN}" ] || die "container did not produce a Windows binary"
chmod +x "${BIN}"

# ---------------------------------------------------------------------------
# 3. Copy to dist with versioned name
# ---------------------------------------------------------------------------
say "Copying to dist..."
cp -f "${BIN}" "${DIST_DIR}/${EXE_NAME}"

say "Done: ${DIST_DIR}/${EXE_NAME}"
ls -lh "${DIST_DIR}/${EXE_NAME}"
