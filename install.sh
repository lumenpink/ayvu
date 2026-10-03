#!/bin/sh
set -e

# Ayvu standalone installer script
# Standard POSIX sh compatible

PREFIX="${PREFIX:-}"
DESTDIR="${DESTDIR:-}"

usage() {
    cat <<EOF
Usage: ./install.sh [OPTIONS]

Options:
  --prefix <DIR>     Set installation prefix (default: /usr/local or ~/.local)
  --destdir <DIR>    Set staging directory for package managers
  -h, --help         Show this help message
EOF
    exit 0
}

while [ $# -gt 0 ]; do
    case "$1" in
        --prefix)
            PREFIX="$2"
            shift 2
            ;;
        --prefix=*)
            PREFIX="${1#*=}"
            shift
            ;;
        --destdir)
            DESTDIR="$2"
            shift 2
            ;;
        --destdir=*)
            DESTDIR="${1#*=}"
            shift
            ;;
        -h|--help)
            usage
            ;;
        *)
            echo "Unknown option: $1" >&2
            echo "Run './install.sh --help' for usage." >&2
            exit 1
            ;;
    esac
done

if [ -z "$PREFIX" ]; then
    if [ "$(id -u)" -eq 0 ]; then
        PREFIX="/usr/local"
    else
        PREFIX="$HOME/.local"
    fi
fi

BINDIR="${DESTDIR}${PREFIX}/bin"

echo "==> Installing Ayvu..."
echo "    Target directory: ${BINDIR}"

if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: cargo (Rust toolchain) is not installed or not in PATH." >&2
    echo "Please install Rust from https://rustup.rs/ before running this installer." >&2
    exit 1
fi

echo "==> Building optimized release binary..."
cargo build --release

TARGET_BIN="target/release/ayvu"
if [ ! -f "$TARGET_BIN" ]; then
    # In case cargo config points to musl target
    TARGET_BIN=$(find target -type f -name ayvu -path "*/release/ayvu" | head -n 1)
fi

if [ -z "$TARGET_BIN" ] || [ ! -f "$TARGET_BIN" ]; then
    echo "Error: Release binary could not be found after compilation." >&2
    exit 1
fi

echo "==> Installing binary to ${BINDIR}/ayvu..."
mkdir -p "${BINDIR}"
cp "$TARGET_BIN" "${BINDIR}/ayvu"
chmod 0755 "${BINDIR}/ayvu"

echo "✓ Ayvu successfully installed to ${BINDIR}/ayvu"
if ! echo "$PATH" | grep -q "${PREFIX}/bin"; then
    echo "Notice: Make sure ${PREFIX}/bin is included in your PATH environment variable."
fi
