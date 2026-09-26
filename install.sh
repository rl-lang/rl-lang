#!/usr/bin/env bash
set -euo pipefail

REPO="rl-lang/rl-lang"
BINARY="rlm"
INSTALL_DIR="${RL_INSTALL_DIR:-$HOME/.local/bin}"

# --- Bootstrap note ---
# Thin bootstrapper: downloads and SHA256-verifies only `rlm`,
# the rl-lang toolchain manager, then hands off to `rlm install`
# for the rest (rl, rlc, rlt, rlrepl, rlsp, rldocs).
#   curl -fsSL https://raw.githubusercontent.com/rl-lang/rl-lang/main/install.sh | bash
# Or install rlm directly from GitHub Releases and use:
#   rlm install

# --- Colors (disabled when not a terminal) ---

if [ -t 1 ] && [ -t 2 ]; then
  C_RESET=$'\e[0m'
  C_BOLD=$'\e[1m'
  C_DIM=$'\e[2m'
  C_CYAN=$'\e[36m'
  C_GREEN=$'\e[32m'
  C_RED=$'\e[31m'
  C_YELLOW=$'\e[33m'
else
  C_RESET=""
  C_BOLD=""
  C_DIM=""
  C_CYAN=""
  C_GREEN=""
  C_RED=""
  C_YELLOW=""
fi

# --- Output helpers ---

msg() { printf '%s\n' "$*"; }
info() { printf '  %s::%s %s\n' "${C_DIM}" "${C_RESET}" "$*"; }
ok() { printf '  %s[ OK ]%s %s\n' "${C_GREEN}" "${C_RESET}" "$*"; }
warn() { printf '  %s[WARN]%s %s\n' "${C_YELLOW}" "${C_RESET}" "$*"; }
err() { printf '  %s[FAIL]%s %s\n' "${C_RED}" "${C_RESET}" "$*" >&2; }

# --- Usage ---

usage() {
  cat <<EOF
Usage: install.sh [OPTIONS] [VERSION]

Thin bootstrapper: installs only the rlm toolchain manager from
GitHub Releases (SHA256-verified), then hands off to rlm itself:

  rlm install              # interactive binary picker
  rlm install latest       # latest stable, no picker

Arguments:
  VERSION    Version of rlm to install (default: interactive picker,
             forwarded to 'rlm install' as well)
             Use "latest", "nightly", or a specific version like "v2.0.0"

Options:
  -h, --help              Show this help message
  -p, --prefix DIR        Install directory (default: ~/.local/bin,
                          forwarded to 'rlm install')
  -f, --force             Overwrite existing binaries (forwarded to
                          'rlm install')
  -b, --binaries BINS     Comma-separated list for 'rlm install'
                          (e.g. rl,rlc,rlt) or "all"
  --variant BINS          Same as --binaries
  --no-tui                Forwarded to 'rlm install' (CLI-only mode)
  --bootstrapper-only     Only install rlm, skip the 'rlm install' handoff
  --uninstall             Remove rlm (use 'rlm uninstall' for the rest)

Environment variables:
  RL_INSTALL_DIR          Same as --prefix
  RL_VERSION              Same as VERSION argument

Examples:
  install.sh                          # interactive install
  install.sh latest                   # install latest stable
  install.sh nightly                  # install nightly build
  install.sh v2.0.0                   # install specific version
  install.sh -b rl,rlc,rlt latest     # rlm + specific binaries
  install.sh --bootstrapper-only      # only rlm, no handoff
  install.sh -p /usr/local/bin -f v2.0.0
  install.sh --uninstall              # remove rlm
EOF
}

# --- Platform detection ---

detect_arch() {
  case "$(uname -m)" in
    x86_64 | amd64) echo "x86_64" ;;
    aarch64 | arm64) echo "aarch64" ;;
    *)
      echo "Unsupported arch: $(uname -m)" >&2
      exit 1
      ;;
  esac
}

detect_termux() {
  [ -n "${TERMUX_VERSION:-}" ] && return 0
  [ -n "${PREFIX:-}" ] && return 0
  [ -d /data/data/com.termux ] && return 0
  return 1
}

detect_platform() {
  case "$(uname -s)" in
    Linux)
      if detect_termux; then
        echo "android"
      else
        echo "linux"
      fi
      ;;
    Darwin) echo "macos" ;;
    *)
      err "Unsupported platform: $(uname -s)"
      err "Use install.ps1 on Windows."
      exit 1
      ;;
  esac
}

# --- Version resolution ---

release_exists() {
  local tag="$1" json
  json="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/tags/${tag}" 2>/dev/null)" || true
  case "$json" in
    *'"tag_name"'*) return 0 ;;
    *) return 1 ;;
  esac
}

latest_tag() {
  local json tag
  json="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null)" || true
  tag="$(printf '%s' "$json" | sed -nE 's/.*"tag_name"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/p' | head -n1)"
  printf '%s' "$tag"
}

resolve_version() {
  local requested="$1" normalized
  case "$requested" in
    latest)
      local tag
      tag="$(latest_tag)"
      if [ -z "$tag" ]; then
        err "Could not resolve the latest release from GitHub."
        exit 1
      fi
      echo "$tag"
      ;;
    nightly)
      if ! release_exists "nightly"; then
        err "Release 'nightly' not found on GitHub."
        exit 1
      fi
      echo "nightly"
      ;;
    v[0-9]*)
      if ! release_exists "$requested"; then
        err "Release '$requested' not found on GitHub."
        err "Check the tag (e.g. 'v1.0.0') or use 'latest'/'nightly'."
        exit 1
      fi
      echo "$requested"
      ;;
    [0-9]*)
      normalized="v${requested}"
      if ! release_exists "$normalized"; then
        err "Release '$normalized' not found on GitHub."
        err "Check the tag (e.g. 'v1.0.0') or use 'latest'/'nightly'."
        exit 1
      fi
      echo "$normalized"
      ;;
    *)
      err "Unknown version '$requested'."
      err "Use 'latest', 'nightly', or a specific version like 'v1.0.0'."
      exit 1
      ;;
  esac
}

select_version_picker() {
  msg "" >&2
  msg "  Select a version to install:" >&2
  msg "" >&2
  printf '    %s1) latest%s   - newest stable release\n' "${C_BOLD}" "${C_RESET}" >&2
  printf '    %s2) nightly%s  - latest build from the dev branch\n' "${C_BOLD}" "${C_RESET}" >&2
  printf '    %s3) custom%s   - pin a specific version (e.g. v2.0.0)\n' "${C_BOLD}" "${C_RESET}" >&2
  msg "" >&2
  local choice custom
  read -rp "  Choose [1]: " choice >&2
  choice="${choice:-1}"
  case "$choice" in
    2)  echo "nightly" ;;
    3)
      read -rp "  Enter version (e.g. v2.0.0): " custom >&2
      echo "${custom:-latest}"
      ;;
    *) echo "latest" ;;
  esac
}

# --- Checksum helpers ---

# sha256sum (Linux) or shasum -a 256 (macOS fallback).
sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

# --- Install rlm ---

install_rlm() {
  local arch="$1" version="$2" platform="$3" force="$4"
  local url tmpdir asset sha_url

  if [ -f "$INSTALL_DIR/${BINARY}" ] && [ "$force" != "1" ]; then
    warn "${BINARY} already exists at $INSTALL_DIR/${BINARY}. Use --force to overwrite."
    return 0
  fi

  info "Installing ${BINARY} ${version} (${platform}-${arch})..."

  asset="${BINARY}-${platform}-${arch}.tar.gz"
  url="https://github.com/${REPO}/releases/download/${version}/${asset}"

  tmpdir=$(mktemp -d)
  # shellcheck disable=SC2064
  trap "rm -rf '$tmpdir'" EXIT

  if ! curl -fsSL "$url" -o "$tmpdir/${asset}"; then
    trap - EXIT
    rm -rf "$tmpdir"
    err "Failed to download $url"
    err "Check that rlm was published for version '${version}'."
    return 1
  fi

  # SHA256 verification is mandatory: abort if the checksum file is
  # missing or the hash does not match.
  sha_url="${url}.sha256"
  if ! curl -fsSL "$sha_url" -o "$tmpdir/${asset}.sha256" 2>/dev/null; then
    trap - EXIT
    rm -rf "$tmpdir"
    err "Failed to download checksum file ${sha_url}"
    err "Aborting installation: rlm cannot be verified."
    return 1
  fi

  local expected actual
  expected="$(awk '{print $1}' "$tmpdir/${asset}.sha256")"
  actual="$(sha256_of "$tmpdir/${asset}")"

  if [ "$expected" != "$actual" ]; then
    trap - EXIT
    rm -rf "$tmpdir"
    err "Checksum mismatch for ${asset}!"
    err "  Expected: $expected"
    err "  Got:      $actual"
    return 1
  fi
  info "SHA256 verified: ${asset}"

  tar -xzf "$tmpdir/${asset}" -C "$tmpdir"

  mkdir -p "$INSTALL_DIR"
  cp "$tmpdir/${BINARY}" "$INSTALL_DIR/${BINARY}"
  chmod +x "$INSTALL_DIR/${BINARY}"

  trap - EXIT
  rm -rf "$tmpdir"

  ok "Installed: $INSTALL_DIR/${BINARY}"
}

# --- Uninstall ---

do_uninstall() {
  msg ""
  msg "  ${C_BOLD}Uninstalling rlm from ${INSTALL_DIR}...${C_RESET}"
  msg "  ${C_DIM}Use 'rlm uninstall' to remove the rest of the toolchain.${C_RESET}"
  msg ""

  local path="$INSTALL_DIR/${BINARY}"
  if [ -f "$path" ]; then
    rm -f "$path"
    ok "Removed: $path"
  else
    msg "  ${C_DIM}No rlm binary found in ${INSTALL_DIR}.${C_RESET}"
  fi
}

# --- Main ---

main() {
  local force=0 bootstrapper_only=0 requested=""
  local binaries_arg="" no_tui=0

  while [ $# -gt 0 ]; do
    case "$1" in
      -h|--help)
        usage
        exit 0
        ;;
      -p|--prefix)
        INSTALL_DIR="$2"
        shift 2
        ;;
      -f|--force)
        force=1
        shift
        ;;
      -b|--binaries|--variant)
        binaries_arg="$2"
        shift 2
        ;;
      --no-tui)
        no_tui=1
        shift
        ;;
      --bootstrapper-only)
        bootstrapper_only=1
        shift
        ;;
      --uninstall)
        do_uninstall
        exit 0
        ;;
      -*)
        err "Unknown option: $1"
        usage >&2
        exit 1
        ;;
      *)
        requested="$1"
        shift
        ;;
    esac
  done

  local platform arch
  platform="$(detect_platform)"
  arch="$(detect_arch)"

  if [ -z "$requested" ]; then
    if [ -n "${RL_VERSION:-}" ]; then
      requested="$RL_VERSION"
    elif [ -t 0 ]; then
      requested="$(select_version_picker)"
    else
      requested="latest"
    fi
  fi

  local version
  version="$(resolve_version "$requested")"

  msg ""
  printf '  %srlm bootstrapper%s\n' "${C_BOLD}" "${C_RESET}"
  msg "  ${C_DIM}repo:    ${C_RESET}${REPO}"
  msg "  ${C_DIM}arch:    ${C_RESET}${arch}"
  msg "  ${C_DIM}platform:${C_RESET} ${platform}"
  msg "  ${C_DIM}version: ${C_RESET}${version}"
  msg "  ${C_DIM}install: ${C_RESET}${INSTALL_DIR}"
  msg "  ${C_DIM}----------------------------------------${C_RESET}"
  msg ""

  if ! install_rlm "$arch" "$version" "$platform" "$force"; then
    exit 1
  fi

  if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    msg ""
    msg "  ${C_DIM}Add this to your shell profile:${C_RESET}"
    msg "  ${C_BOLD}  export PATH=\"$INSTALL_DIR:\$PATH\"${C_RESET}"
  fi

  if [ "$bootstrapper_only" = "1" ]; then
    msg ""
    msg "  Next: rlm install   # pick the rest of the toolchain (rl, rlc, rlt, rlrepl, rlsp, rldocs)"
    exit 0
  fi

  # Hand off to rlm for the rest of the toolchain. The version this
  # script resolved, the install dir, and the force/binaries flags are
  # forwarded so one invocation installs everything.
  local rlm_bin="$INSTALL_DIR/${BINARY}"
  if [ ! -x "$rlm_bin" ]; then
    err "rlm binary not found at $rlm_bin after install."
    exit 1
  fi

  msg ""
  info "Handing off to rlm..."
  local rlm_args=("$version" --prefix "$INSTALL_DIR")
  if [ -n "$binaries_arg" ]; then
    rlm_args+=(--variant "$binaries_arg")
  fi
  if [ "$force" = "1" ]; then
    rlm_args+=(--force)
  fi
  if [ "$no_tui" = "1" ]; then
    rlm_args+=(--no-tui)
  fi
  exec "$rlm_bin" install "${rlm_args[@]}"
}

main "$@"
