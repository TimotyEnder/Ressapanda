#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

APP_ID="io.github.timotyender.ressapanda"

DATA_HOME="${XDG_DATA_HOME:-${HOME}/.local/share}"
BIN_HOME="${XDG_BIN_HOME:-${HOME}/.local/bin}"
APPS_DIR="${DATA_HOME}/applications"
ICON_DIR="${DATA_HOME}/icons/hicolor/256x256/apps"
DESKTOP_FILE="${APPS_DIR}/${APP_ID}.desktop"
ICON_FILE="${ICON_DIR}/${APP_ID}.png"

# Stable, version-independent location. The desktop entry always points here,
# so rebuilding (and even bumping the version, which renames the artifact in
# dist/) never breaks the launcher.
INSTALLED_APPIMAGE="${BIN_HOME}/Ressapanda.AppImage"
LAUNCHER_LINK="${BIN_HOME}/ressapanda"

say() { printf '\033[1;34m==>\033[0m %s\n' "$*"; }

usage() {
    cat <<EOF
Usage: $(basename "$0") [options] [path/to/Ressapanda-*.AppImage]

Installs a desktop entry + icon into your user data dirs, and copies the
AppImage to a stable path (${INSTALLED_APPIMAGE}) so the launcher survives
rebuilds and version bumps.

Without an explicit path the latest AppImage in ${SCRIPT_DIR}/dist is used.

Options:
  --link        Symlink to the given AppImage instead of copying it.
                (Convenient while developing; breaks if you later delete the
                build output or bump the version.)
  -u, --uninstall   Remove the entry, icon, launcher and stable copy.
  -h, --help        Show this help.
EOF
}

uninstall() {
    say "Removing desktop entry, icon, launcher and stable copy..."
    rm -f "${DESKTOP_FILE}" "${ICON_FILE}" "${LAUNCHER_LINK}"
    # Only remove the stable copy if it is not the build output we were pointed at.
    rm -f "${INSTALLED_APPIMAGE}"
    command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "${APPS_DIR}" >/dev/null 2>&1 || true
    for k in kbuildsycoca6 kbuildsycoca5; do
        command -v "${k}" >/dev/null 2>&1 && "${k}" >/dev/null 2>&1 || true
    done
    say "Done."
}

UNINSTALL=0
LINK=0
APPIMAGE=""
for arg in "$@"; do
    case "${arg}" in
        --uninstall|-u) UNINSTALL=1 ;;
        --link) LINK=1 ;;
        -h|--help) usage; exit 0 ;;
        *) APPIMAGE="${arg}" ;;
    esac
done

if [ "${UNINSTALL}" -eq 1 ]; then
    uninstall
    exit 0
fi

if [ -z "${APPIMAGE}" ]; then
    APPIMAGE="$(ls -1 "${SCRIPT_DIR}"/dist/Ressapanda-*-x86_64.AppImage 2>/dev/null | sort -V | tail -1 || true)"
fi
if [ -z "${APPIMAGE}" ] || [ ! -f "${APPIMAGE}" ]; then
    echo "error: no AppImage found (pass one explicitly, or build it first)" >&2
    exit 1
fi
APPIMAGE="$(readlink -f "${APPIMAGE}")"

ICON_SRC="${REPO_ROOT}/Assets/application_icon.png"
if [ ! -f "${ICON_SRC}" ]; then
    say "Extracting icon from ${APPIMAGE}"
    _icontmp="$(mktemp -d)"
    trap 'rm -rf "${_icontmp}"' EXIT
    ( cd "${_icontmp}" && "${APPIMAGE}" --appimage-extract "${APP_ID}.png" >/dev/null 2>&1 )
    ICON_SRC="${_icontmp}/squashfs-root/${APP_ID}.png"
fi
[ -f "${ICON_SRC}" ] || { echo "error: no icon source found" >&2; exit 1; }

# ---------------------------------------------------------------------------
# 1. Place the AppImage at a stable, version-independent path
# ---------------------------------------------------------------------------
install -d "${BIN_HOME}"

if [ "${LINK}" -eq 1 ]; then
    say "Linking ${INSTALLED_APPIMAGE} -> ${APPIMAGE}"
    ln -sfn "${APPIMAGE}" "${INSTALLED_APPIMAGE}"
elif [ "${APPIMAGE}" = "${INSTALLED_APPIMAGE}" ]; then
    say "AppImage already at stable path: ${INSTALLED_APPIMAGE}"
else
    say "Installing AppImage -> ${INSTALLED_APPIMAGE}"
    install -m 0755 "${APPIMAGE}" "${INSTALLED_APPIMAGE}"
fi

# Friendly command name for launching from a terminal.
ln -sfn "Ressapanda.AppImage" "${LAUNCHER_LINK}"

# ---------------------------------------------------------------------------
# 2. Icon
# ---------------------------------------------------------------------------
say "Installing icon -> ${ICON_FILE}"
install -d "${ICON_DIR}" "${APPS_DIR}"
install -m 0644 "${ICON_SRC}" "${ICON_FILE}"

# ---------------------------------------------------------------------------
# 3. Desktop entry (points at the stable path)
# ---------------------------------------------------------------------------
say "Installing desktop entry -> ${DESKTOP_FILE}"
cat > "${DESKTOP_FILE}" <<EOF
[Desktop Entry]
Type=Application
Name=Ressapanda
GenericName=Voxel Editor
Comment=A 3D voxel editor built with wgpu and egui
Exec="${INSTALLED_APPIMAGE}" %U
TryExec="${INSTALLED_APPIMAGE}"
Icon=${APP_ID}
Terminal=false
Categories=Graphics;3DGraphics;
Keywords=voxel;3d;model;editor;game;gamedev;modelling;
StartupNotify=true
StartupWMClass=${APP_ID}
EOF
chmod 0644 "${DESKTOP_FILE}"

# ---------------------------------------------------------------------------
# 4. Refresh caches
# ---------------------------------------------------------------------------
say "Refreshing desktop/icon caches..."
command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "${APPS_DIR}" >/dev/null 2>&1 || true
command -v gtk-update-icon-cache >/dev/null 2>&1 && gtk-update-icon-cache -f -t "${DATA_HOME}/icons/hicolor" >/dev/null 2>&1 || true
for k in kbuildsycoca6 kbuildsycoca5; do
    if command -v "${k}" >/dev/null 2>&1; then
        "${k}" >/dev/null 2>&1 || true
    fi
done

say "Done."
echo "    Launcher:  ${LAUNCHER_LINK}"
echo "    AppImage:  ${INSTALLED_APPIMAGE}  (from ${APPIMAGE})"
echo "    Entry:     ${DESKTOP_FILE}"
echo "    Icon:      ${ICON_FILE}"

if [ "${LINK}" -eq 0 ]; then
    echo
    echo "NOTE: this is a copy. Re-run this script after every rebuild to update it."
fi

case ":${PATH}:" in
    *":${BIN_HOME}:"*) ;;
    *)
        echo
        echo "NOTE: ${BIN_HOME} is not in your PATH; add it to use the 'ressapanda' command."
        ;;
esac

echo
echo "If a running window still shows the old icon, close and relaunch the app."
