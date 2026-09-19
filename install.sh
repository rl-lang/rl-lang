#!/usr/bin/env bash
set -euo pipefail

REPO="rl-lang/rl-lang"
INSTALL_DIR="${RL_INSTALL_DIR:-$HOME/.local/bin}"

# --- Bootstrap note ---
# This script is a one-time bootstrapper. Once installed, use `rlm` to manage
# your rl-lang toolchain (install, update, uninstall).
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

# --- Binary definitions ---
# 7 binaries: rl, rlc, rlt, rlrepl, rlsp, rldocs, rlm

BINARIES=(rl rlc rlt rlrepl rlsp rldocs rlm)

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

Install prebuilt rl-lang binaries from GitHub Releases.

Arguments:
  VERSION    Version to install (default: interactive picker)
             Use "latest", "nightly", or a specific version like "v2.0.0"

Options:
  -h, --help              Show this help message
  -p, --prefix DIR        Install directory (default: ~/.local/bin)
  -f, --force             Overwrite existing binaries without prompting
  -b, --binaries BINS     Comma-separated list of binaries to install
                          (default: interactive picker)
                          Use "all" to install all binaries
  --uninstall             Remove installed binaries

Environment variables:
  RL_INSTALL_DIR          Same as --prefix
  RL_VERSION              Same as VERSION argument
  RL_BINARIES             Same as --binaries

Examples:
  install.sh                          # interactive install
  install.sh latest                   # install latest stable
  install.sh nightly                  # install nightly build
  install.sh v2.0.0                   # install specific version
  install.sh -b rl,rlc,rlm latest     # install specific binaries
  install.sh -p /usr/local/bin -f v2.0.0  # force install to /usr/local/bin
  install.sh --uninstall              # remove all installed binaries
EOF
}

# --- Binary selection ---

print_menu() {
  msg "  Select binaries to install:"
  msg ""
  printf '    %s1) rl%s        - core (run, check, new, dev, format, pm)\n' "${C_BOLD}" "${C_RESET}"
  printf '    %s2) rlc%s       - compiler (VM backend)\n' "${C_BOLD}" "${C_RESET}"
  printf '    %s3) rlt%s       - transpiler (to C99)\n' "${C_BOLD}" "${C_RESET}"
  printf '    %s4) rlrepl%s    - interactive TUI REPL\n' "${C_BOLD}" "${C_RESET}"
  printf '    %s5) rlsp%s      - LSP server\n' "${C_BOLD}" "${C_RESET}"
  printf '    %s6) rldocs%s    - documentation viewer\n' "${C_BOLD}" "${C_RESET}"
  printf '    %s7) rlm%s       - toolchain manager\n' "${C_BOLD}" "${C_RESET}"
  msg ""
  msg "  ${C_DIM}Enter number(s), comma-separated (e.g. 1,3,7), or 'all'.${C_RESET}"
}

select_binaries() {
  if [ -n "${BINARIES_ARG:-}" ]; then
    if [ "$BINARIES_ARG" = "all" ]; then
      echo "${BINARIES[*]}"
      return
    fi
    echo "$BINARIES_ARG" | tr ',' '\n' | sed 's/^ *//; s/ *$//'
    return
  fi

  if [ -n "${RL_BINARIES:-}" ]; then
    if [ "$RL_BINARIES" = "all" ]; then
      echo "${BINARIES[*]}"
      return
    fi
    echo "$RL_BINARIES" | tr ',' '\n' | sed 's/^ *//; s/ *$//'
    return
  fi

  if [ ! -t 0 ]; then
    err "No TTY detected and RL_BINARIES is not set."
    err "Non-interactive use requires: RL_BINARIES=rl,rlc,rlm ./install.sh [version]"
    exit 1
  fi

  print_menu >&2
  local choices
  read -rp "  Enter number(s), comma-separated (e.g. 1,3,7), or 'all': " choices >&2

  if [ "$(echo "$choices" | tr -d '[:space:]')" = "all" ]; then
    echo "${BINARIES[*]}"
    return
  fi

  local IFS=','
  local part

  for part in $choices; do
    part="$(echo "$part" | tr -d '[:space:]')"
    [ -z "$part" ] && continue

    case "$part" in
      1) echo "rl" ;;
      2) echo "rlc" ;;
      3) echo "rlt" ;;
      4) echo "rlrepl" ;;
      5) echo "rlsp" ;;
      6) echo "rldocs" ;;
      7) echo "rlm" ;;
      *)
        echo "Invalid selection: $part" >&2
        exit 1
        ;;
    esac
  done
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

# --- Checksum verification ---

verify_checksum() {
  local file="$1" sha_file="${1}.sha256"

  if [ ! -f "$sha_file" ]; then
    warn "No checksum file found for $(basename "$file"). Skipping verification."
    return 0
  fi

  local expected actual
  expected="$(awk '{print $1}' "$sha_file")"
  actual="$(sha256sum "$file" | awk '{print $1}')"

  if [ "$expected" = "$actual" ]; then
    return 0
  else
    err "Checksum mismatch for $(basename "$file")!"
    err "  Expected: $expected"
    err "  Got:      $actual"
    return 1
  fi
}

# --- Install ---

install_one() {
  local binary="$1" arch="$2" version="$3" platform="$4" force="$5"
  local url tmpdir asset sha_url

  # Check if already installed
  if [ -f "$INSTALL_DIR/${binary}" ] && [ "$force" != "1" ]; then
    warn "${binary} already exists at $INSTALL_DIR/${binary}. Use --force to overwrite."
    return 0
  fi

  info "Installing ${binary} ${version} (${platform}-${arch})..."

  if [ "$platform" = "windows" ]; then
    asset="${binary}-windows-${arch}.zip"
  else
    asset="${binary}-${platform}-${arch}.tar.gz"
  fi
  url="https://github.com/${REPO}/releases/download/${version}/${asset}"

  tmpdir=$(mktemp -d)

  # Download asset
  if ! curl -fsSL "$url" -o "$tmpdir/${asset}"; then
    rm -rf "$tmpdir"
    err "Failed to download $url"
    err "Check that this binary/version combination was published."
    return 1
  fi

  # Download checksum if available
  sha_url="${url}.sha256"
  curl -fsSL "$sha_url" -o "$tmpdir/${asset}.sha256" 2>/dev/null || true

  # Verify checksum
  if ! verify_checksum "$tmpdir/${asset}"; then
    rm -rf "$tmpdir"
    err "Aborting installation due to checksum failure."
    return 1
  fi

  # Extract
  if [ "$platform" = "windows" ]; then
    unzip -qo "$tmpdir/${asset}" -d "$tmpdir"
  else
    tar -xzf "$tmpdir/${asset}" -C "$tmpdir"
  fi

  mkdir -p "$INSTALL_DIR"

  if [ "$platform" = "windows" ]; then
    cp "$tmpdir/${binary}.exe" "$INSTALL_DIR/${binary}.exe"
  else
    cp "$tmpdir/${binary}" "$INSTALL_DIR/${binary}"
    chmod +x "$INSTALL_DIR/${binary}"
  fi

  rm -rf "$tmpdir"

  ok "Installed: $INSTALL_DIR/${binary}"
}

# --- Uninstall ---

do_uninstall() {
  local removed=0

  msg ""
  msg "  ${C_BOLD}Uninstalling rl-lang binaries from ${INSTALL_DIR}...${C_RESET}"
  msg ""

  for binary in "${BINARIES[@]}"; do
    for ext in "" ".exe"; do
      local path="$INSTALL_DIR/${binary}${ext}"
      if [ -f "$path" ]; then
        rm -f "$path"
        ok "Removed: $path"
        removed=$((removed + 1))
      fi
    done
  done

  msg ""
  if [ "$removed" -gt 0 ]; then
    printf '  %sRemoved %s binary(ies).%s\n' "${C_GREEN}" "$removed" "${C_RESET}"
  else
    msg "  ${C_DIM}No rl-lang binaries found in ${INSTALL_DIR}.${C_RESET}"
  fi
}

# --- Main ---

main() {
  local force=0 version requested binaries platform arch

  # Parse arguments
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
      -b|--binaries)
        BINARIES_ARG="$2"
        shift 2
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

  platform="$(detect_platform)"
  arch="$(detect_arch)"

  if [ -z "${requested:-}" ]; then
    if [ -n "${RL_VERSION:-}" ]; then
      requested="$RL_VERSION"
    elif [ -t 0 ]; then
      requested="$(select_version_picker)"
    else
      requested="latest"
    fi
  fi

  version="$(resolve_version "$requested")"

  msg ""
  printf '  %srl-lang installer%s\n' "${C_BOLD}" "${C_RESET}"
  msg "  ${C_DIM}repo:    ${C_RESET}${REPO}"
  msg "  ${C_DIM}arch:    ${C_RESET}${arch}"
  msg "  ${C_DIM}platform:${C_RESET} ${platform}"
  msg "  ${C_DIM}version: ${C_RESET}${version}"
  msg "  ${C_DIM}install: ${C_RESET}${INSTALL_DIR}"
  msg "  ${C_DIM}----------------------------------------${C_RESET}"
  msg ""

  binaries="$(select_binaries)"

  local failed=0 installed=0 total=0
  while IFS= read -r binary; do
    [ -z "$binary" ] && continue
    total=$((total + 1))
    if install_one "$binary" "$arch" "$version" "$platform" "$force"; then
      installed=$((installed + 1))
    else
      failed=1
    fi
  done <<<"$binaries"

  msg ""
  if [ "$failed" = "1" ]; then
    printf '  %sSummary:%s %s%s%s/%s installed, some failed.\n' "${C_BOLD}" "${C_RESET}" "${C_GREEN}" "$installed" "${C_RESET}" "$total"
  else
    printf '  %sSummary:%s %s%s%s/%s installed.\n' "${C_BOLD}" "${C_RESET}" "${C_GREEN}" "$installed" "${C_RESET}" "$total"
  fi

  if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    msg ""
    msg "  ${C_DIM}Add this to your shell profile:${C_RESET}"
    msg "  ${C_BOLD}  export PATH=\"$INSTALL_DIR:\$PATH\"${C_RESET}"
  fi

  if [ "$failed" = "1" ]; then
    exit 1
  fi
}

main "$@"
