#!/usr/bin/env bash
set -euo pipefail

# scripts/install-local.sh - install/uninstall the locally built rl binary
#
# Usage:
#   ./scripts/install-local.sh
#   ./scripts/install-local.sh --uninstall
#
# Searches target-bins/ for rl regardless of build profile
# and installs it to ~/.local/bin/rl.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd -- "$SCRIPT_DIR/.." && pwd)"

INSTALL_DIR="$HOME/.local/bin"
DEST="$INSTALL_DIR/rl"

# --- Colors ---

if [ -t 1 ] && [ -t 2 ]; then
  C_RESET=$'\e[0m'
  C_BOLD=$'\e[1m'
  C_DIM=$'\e[2m'
  C_CYAN=$'\e[36m'
  C_GREEN=$'\e[32m'
  C_RED=$'\e[31m'
else
  C_RESET=""
  C_BOLD=""
  C_DIM=""
  C_CYAN=""
  C_GREEN=""
  C_RED=""
fi

info() {
  printf '%s  ::%s %s\n' "$C_DIM" "$C_RESET" "$*"
}

ok() {
  printf '%s  [ OK ]%s %s\n' "$C_GREEN" "$C_RESET" "$*"
}

fail() {
  printf '%s  [FAIL]%s %s\n' "$C_RED" "$C_RESET" "$*"
}

header() {
  printf '\n%s%s:: %s%s\n' "$C_BOLD" "$C_CYAN" "$*" "$C_RESET"
}

# --- Find binary ---

find_rl() {
  local profile
  local path

  for profile in release nightly dev; do
    path="$ROOT_DIR/target-bins/$profile/rl"

    if [ -f "$path" ]; then
      printf '%s\n' "$path"
      return 0
    fi
  done

  return 1
}

# --- Uninstall ---

uninstall() {
  header "Uninstalling rl"

  if [ -f "$DEST" ]; then
    rm -f -- "$DEST"
    ok "Removed $DEST"
  else
    info "rl is not installed at $DEST"
  fi
}

# --- Install ---

install() {
  local src

  header "Installing rl"

  if ! src="$(find_rl)"; then
    fail "rl binary not found."
    fail "Looked in:"
    fail "  $ROOT_DIR/target-bins/release/rl"
    fail "  $ROOT_DIR/target-bins/nightly/rl"
    fail "  $ROOT_DIR/target-bins/dev/rl"
    exit 1
  fi

  info "source: $src"
  info "target: $DEST"

  mkdir -p "$INSTALL_DIR"
  cp -- "$src" "$DEST"
  chmod +x "$DEST"

  ok "Installed rl to $DEST"

  if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    printf "\n"
    info "Add this to your shell profile:"
    info "  ${C_BOLD}export PATH=\"$INSTALL_DIR:\$PATH\"${C_RESET}"
  fi
}

# --- Main ---

main() {
  case "${1:-}" in
    "")
      install
      ;;

    --uninstall)
      uninstall
      ;;

    -h|--help)
      printf '%s\n' \
        "Usage: ./scripts/install-local.sh [--uninstall]" \
        "" \
        "Install the locally built rl binary to ~/.local/bin/rl." \
        "" \
        "Options:" \
        "  --uninstall    Remove ~/.local/bin/rl" \
        "  -h, --help     Show this help"
      ;;

    *)
      fail "Unknown option: $1"
      printf 'Run with --help for usage.\n'
      exit 1
      ;;
  esac
}

main "$@"
