#!/usr/bin/env bash
set -euo pipefail

REPO_NAME="hoshi"
BIN_NAME="hoshi"

# Default install prefix; can be overridden with PREFIX=...
PREFIX="${PREFIX:-/usr/local}"
INSTALL_DIR="$PREFIX/bin"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log()  { printf "%b%s%b\n" "$GREEN" "$1" "$NC"; }
warn() { printf "%b%s%b\n" "$YELLOW" "$1" "$NC"; }
err()  { printf "%b%s%b\n" "$RED" "$1" "$NC"; }

if [[ "$OSTYPE" != linux* ]]; then
    err "hoshi is currently only supported on Linux."
    exit 1
fi

# Detect distro family
if command -v apt-get >/dev/null 2>&1; then
    DISTRO="debian"
    SQLITE_PKG="libsqlite3-dev"
    PKGS="pkg-config $SQLITE_PKG"
elif command -v dnf >/dev/null 2>&1 || command -v yum >/dev/null 2>&1; then
    DISTRO="redhat"
    SQLITE_PKG="sqlite-devel"
    PKGS="pkgconfig $SQLITE_PKG"
elif command -v pacman >/dev/null 2>&1; then
    DISTRO="arch"
    SQLITE_PKG="sqlite"
    PKGS="pkgconf $SQLITE_PKG"
elif command -v zypper >/dev/null 2>&1; then
    DISTRO="suse"
    SQLITE_PKG="sqlite3-devel"
    PKGS="pkg-config $SQLITE_PKG"
elif command -v apk >/dev/null 2>&1; then
    DISTRO="alpine"
    SQLITE_PKG="sqlite-dev"
    PKGS="pkgconf $SQLITE_PKG"
else
    warn "Could not detect your distro. Make sure 'pkg-config' and sqlite3 dev files are installed."
    DISTRO="unknown"
    PKGS=""
fi

# Install build dependencies if we can
install_build_deps() {
    [[ -z "$PKGS" ]] && return

    log "Detected distro family: $DISTRO"
    log "Installing build dependencies: $PKGS"

    case "$DISTRO" in
        debian)
            sudo apt-get update
            sudo apt-get install -y $PKGS
            ;;
        redhat)
            if command -v dnf >/dev/null 2>&1; then
                sudo dnf install -y $PKGS
            else
                sudo yum install -y $PKGS
            fi
            ;;
        arch)
            sudo pacman -Sy --needed --noconfirm $PKGS
            ;;
        suse)
            sudo zypper install -y $PKGS
            ;;
        alpine)
            sudo apk add $PKGS
            ;;
    esac
}

# Check for cargo
if ! command -v cargo >/dev/null 2>&1; then
    warn "Rust/Cargo not found."
    if command -v rustup >/dev/null 2>&1; then
        log "rustup is installed; ensuring stable toolchain is active..."
        rustup default stable
    else
        log "Installing Rust via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        # shellcheck source=/dev/null
        source "$HOME/.cargo/env"
    fi
fi

# Make sure pkg-config can find sqlite3 before building
if ! pkg-config --exists sqlite3 2>/dev/null; then
    install_build_deps
fi

if ! pkg-config --exists sqlite3 2>/dev/null; then
    err "sqlite3 development files still not found."
    err "Please install them manually (e.g. $SQLITE_PKG on $DISTRO) and rerun."
    exit 1
fi

# Build release binary
log "Building $REPO_NAME in release mode..."
cargo build --release

# Pick install location
if [[ ! -w "$INSTALL_DIR" ]] && [[ "$INSTALL_DIR" == "/usr/local/bin" ]]; then
    if command -v sudo >/dev/null 2>&1; then
        SUDO="sudo"
    else
        warn "No write access to $INSTALL_DIR and sudo not found."
        warn "Falling back to ~/.local/bin"
        INSTALL_DIR="$HOME/.local/bin"
        SUDO=""
    fi
else
    SUDO=""
fi

if [[ ! -d "$INSTALL_DIR" ]]; then
    $SUDO mkdir -p "$INSTALL_DIR"
fi

log "Installing $BIN_NAME to $INSTALL_DIR..."
$SUDO cp "target/release/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
$SUDO chmod +x "$INSTALL_DIR/$BIN_NAME"

if command -v "$BIN_NAME" >/dev/null 2>&1; then
    log "Installation complete! Run '$BIN_NAME' to try it."
else
    warn "$INSTALL_DIR is not in your PATH."
    warn "Add the following to your shell profile:"
    warn "  export PATH=\"$INSTALL_DIR:\$PATH\""
fi
