#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

OUT_DIR="${SCRIPT_DIR}/out"
DIST_DIR="${SCRIPT_DIR}/dist"
CACHE_DIR="${SCRIPT_DIR}/.cache"

APP_ID="io.github.timotyender.ressapanda"
DESKTOP_FILE="${APP_ID}.desktop"
IMAGE_NAME="ressapanda-build"
RUST_VERSION="1.98.1"
TARGET_VOLUME="ressapanda-target"
REGISTRY_VOLUME="ressapanda-cargo-registry"

# glibc version the produced binary is allowed to require. AlmaLinux 8 = 2.28.
MAX_GLIBC="2.28"

APPIMAGE_TOOL_URL="https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage"

say()  { printf '\n\033[1;34m==>\033[0m %s\n' "$*"; }
info() { printf '    %s\n' "$*"; }
die()  { printf '\n\033[1;31mERROR:\033[0m %s\n' "$*" >&2; exit 1; }

command -v docker >/dev/null || die "docker is required"
command -v objdump >/dev/null || die "objdump (binutils) is required"

# Version comes straight from Cargo.toml so the artifact name never drifts.
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' "${REPO_ROOT}/Cargo.toml" | head -1)"
[ -n "${VERSION}" ] || die "could not read version from Cargo.toml"
APPIMAGE_NAME="Ressapanda-${VERSION}-x86_64.AppImage"
APPDIR="${DIST_DIR}/Ressapanda.AppDir"

mkdir -p "${OUT_DIR}" "${DIST_DIR}" "${CACHE_DIR}"

# ---------------------------------------------------------------------------
# 1. Build the toolchain image (cached; only rebuilds if the Dockerfile changes)
# ---------------------------------------------------------------------------
say "Ensuring build image is present (Rust ${RUST_VERSION}, glibc ${MAX_GLIBC})..."
docker build \
    --build-arg "RUST_VERSION=${RUST_VERSION}" \
    -t "${IMAGE_NAME}" \
    -f "${SCRIPT_DIR}/Dockerfile" \
    "${SCRIPT_DIR}"

# ---------------------------------------------------------------------------
# 2. Compile inside a container with the host repo mounted
# ---------------------------------------------------------------------------
say "Compiling (incremental; first run will take a while due to LTO)..."
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
        cargo build --release --locked
        strip --strip-unneeded /build/target/release/ressapanda
        install -m 0755 /build/target/release/ressapanda /out/ressapanda
        # The container runs as root, so hand the artifact back to the host user.
        chown "${HOST_UID}:${HOST_GID}" /out/ressapanda
    '

BIN="${OUT_DIR}/ressapanda"
[ -f "${BIN}" ] || die "container did not produce a binary"
chmod +x "${BIN}"

# ---------------------------------------------------------------------------
# 3. Hard gate on the glibc floor
# ---------------------------------------------------------------------------
say "Verifying glibc floor..."

highest_glibc="$(objdump -T "${BIN}" \
    | grep -oE 'GLIBC_[0-9]+\.[0-9]+' \
    | sed 's/^GLIBC_//' \
    | sort -V \
    | tail -1)"

[ -n "${highest_glibc}" ] || die "could not determine glibc symbol versions from ${BIN}"

newest_allowed="$(printf '%s\n%s\n' "${MAX_GLIBC}" "${highest_glibc}" | sort -V | tail -1)"
if [ "${newest_allowed}" != "${MAX_GLIBC}" ]; then
    die "binary requires glibc ${highest_glibc}, newer than the ${MAX_GLIBC} limit. The AppImage would fail on older distros. Did the host toolchain leak into the build?"
fi
info "highest required glibc: ${highest_glibc} (limit ${MAX_GLIBC}) -- OK"

# ---------------------------------------------------------------------------
# 4. Assemble the AppDir
# ---------------------------------------------------------------------------
say "Assembling AppDir..."

rm -rf "${APPDIR}"
install -d \
    "${APPDIR}/usr/bin" \
    "${APPDIR}/usr/share/applications" \
    "${APPDIR}/usr/share/icons/hicolor/256x256/apps" \
    "${APPDIR}/usr/share/metainfo"

install -m 0755 "${BIN}"                                          "${APPDIR}/usr/bin/ressapanda"
install -m 0755 "${SCRIPT_DIR}/AppRun"                            "${APPDIR}/AppRun"
install -m 0644 "${SCRIPT_DIR}/${DESKTOP_FILE}"                    "${APPDIR}/${DESKTOP_FILE}"
install -m 0644 "${SCRIPT_DIR}/${DESKTOP_FILE}"                    "${APPDIR}/usr/share/applications/${DESKTOP_FILE}"
install -m 0644 "${SCRIPT_DIR}/ressapanda.appdata.xml"            "${APPDIR}/usr/share/metainfo/${APP_ID}.appdata.xml"
install -m 0644 "${REPO_ROOT}/Assets/application_icon.png"        "${APPDIR}/${APP_ID}.png"
install -m 0644 "${REPO_ROOT}/Assets/application_icon.png"        "${APPDIR}/usr/share/icons/hicolor/256x256/apps/${APP_ID}.png"

# The AppImage spec expects the desktop file and icon at the AppDir root too.
ln -sf "${APP_ID}.png" "${APPDIR}/.DirIcon"

# ---------------------------------------------------------------------------
# 5. Validate the desktop entry
# ---------------------------------------------------------------------------
if command -v desktop-file-validate >/dev/null; then
    say "Validating desktop entry..."
    desktop-file-validate "${APPDIR}/${DESKTOP_FILE}"
    info "desktop entry OK"
else
    echo "warning: desktop-file-validate not found; skipping validation"
fi

# ---------------------------------------------------------------------------
# 6. Fetch appimagetool and produce the AppImage
# ---------------------------------------------------------------------------
APPIMAGETOOL="${CACHE_DIR}/appimagetool-x86_64.AppImage"
if [ ! -x "${APPIMAGETOOL}" ]; then
    say "Downloading appimagetool..."
    curl -fsSL -o "${APPIMAGETOOL}" "${APPIMAGE_TOOL_URL}"
    chmod +x "${APPIMAGETOOL}"
fi

say "Generating ${APPIMAGE_NAME}..."
rm -f "${DIST_DIR}/${APPIMAGE_NAME}"
# --appimage-extract-and-run avoids requiring FUSE on the build host.
ARCH=x86_64 "${APPIMAGETOOL}" --appimage-extract-and-run \
    "${APPDIR}" "${DIST_DIR}/${APPIMAGE_NAME}"

chmod +x "${DIST_DIR}/${APPIMAGE_NAME}"

say "Done: ${DIST_DIR}/${APPIMAGE_NAME}"
ls -lh "${DIST_DIR}/${APPIMAGE_NAME}"
