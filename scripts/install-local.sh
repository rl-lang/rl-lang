#!/usr/bin/env bash
set -euo pipefail

# scripts/install-local.sh - install locally built binaries
#
# Usage:
#   ./scripts/install-local.sh                    # install release binaries
#   ./scripts/install-local.sh --nightly          # install nightly binaries
#   ./scripts/install-local.sh --dev              # install debug binaries
#   ./scripts/install-local.sh -p /usr/local/bin  # install to custom prefix
#   ./scripts/install-local.sh --force            # overwrite existing binaries
#   ./scripts/install-local.sh --uninstall        # remove installed binaries
#   ./scripts/install-local.sh -b rl,rlc          # install specific binaries

INSTALL_DIR="${RL_INSTALL_DIR:-$HOME/.local/bin}"
PROFILE="release"
FORCE=0
BINARIES_ARG=""

BINARIES=(rl rlc rlt rlrepl rlsp rldocs rlm)

# --- Colors ---
if [ -t 1 ] && [ -t 2 ]; then
  C_RESET=$'\e[0m'
  C_BOLD=$'\e[1m'
  C_DIM=$'\e[2m'
  C_CYAN=$'\e[36m'
  C_GREEN=$'\e[32m'
  C_YELLOW=$'\e[33m'
  C_RED=$'\e[31m'
  C_BLUE=$'\e[34m'
else
  C_RESET="" C_BOLD="" C_DIM="" C_CYAN="" C_GREEN="" C_YELLOW="" C_RED="" C_BLUE=""
fi

info()  { printf "${C_DIM}  ::${C_RESET} %s\n" "$*"; }
ok()    { printf "${C_GREEN}  [ OK ]${C_RESET} %s\n" "$*"; }
fail()  { printf "${C_RED}  [FAIL]${C_RESET} %s\n" "$*"; }
warn()  { printf "${C_YELLOW}  [WARN]${C_RESET} %s\n" "$*"; }
header(){ printf "\n${C_BOLD}${C_CYAN}:: %s${C_RESET}\n" "$*"; }

usage() {
  printf "\n"
  printf "${C_BOLD}Usage:${C_RESET} ${C_CYAN}install-local.sh${C_RESET} [OPTIONS]\n"
  printf "\n"
  printf "Install locally built binaries from ${C_BOLD}target-bins/${C_RESET}.\n"
  printf "\n"
  printf "${C_BOLD}Options:${C_RESET}\n"
  printf "  ${C_GREEN}--release${C_RESET}          Install release binaries ${C_DIM}(default)${C_RESET}\n"
  printf "  ${C_GREEN}--nightly${C_RESET}          Install nightly binaries\n"
  printf "  ${C_GREEN}--dev${C_RESET}              Install debug binaries\n"
  printf "  ${C_YELLOW}-p, --prefix${C_RESET} ${C_CYAN}DIR${C_RESET}    Install directory ${C_DIM}(default: ~/.local/bin)${C_RESET}\n"
  printf "  ${C_RED}-f, --force${C_RESET}          Overwrite existing binaries\n"
  printf "  ${C_BLUE}-b, --binaries${C_RESET} ${C_CYAN}BINS${C_RESET}  Comma-separated list of binaries to install\n"
  printf "                           ${C_DIM}Use 'all' for every binary${C_RESET}\n"
  printf "  ${C_RED}--uninstall${C_RESET}         Remove installed binaries\n"
  printf "  ${C_BLUE}-h, --help${C_RESET}          Show this help\n"
  printf "\n"
  printf "${C_BOLD}Examples:${C_RESET}\n"
  printf "  ${C_CYAN}install-local.sh${C_RESET}                      # install all release binaries\n"
  printf "  ${C_CYAN}install-local.sh${C_RESET} ${C_GREEN}--dev${C_RESET}              # install all debug binaries\n"
  printf "  ${C_CYAN}install-local.sh${C_RESET} ${C_GREEN}-b rl,rlc,rlm${C_RESET}    # install specific binaries\n"
  printf "  ${C_CYAN}install-local.sh${C_RESET} ${C_GREEN}-p /usr/local/bin${C_RESET} # install to custom directory\n"
  printf "\n"
}

# --- Binary selection ---

print_menu() {
  printf "\n"
  printf "  ${C_BOLD}Select binaries to install:${C_RESET}\n"
  printf "\n"
  printf '    %s1) rl%s        - core (run, check, new, dev, format, pm)\n' "${C_BOLD}" "${C_RESET}"
  printf '    %s2) rlc%s       - compiler (VM backend)\n' "${C_BOLD}" "${C_RESET}"
  printf '    %s3) rlt%s       - transpiler (to C99)\n' "${C_BOLD}" "${C_RESET}"
  printf '    %s4) rlrepl%s    - interactive TUI REPL\n' "${C_BOLD}" "${C_RESET}"
  printf '    %s5) rlsp%s      - LSP server\n' "${C_BOLD}" "${C_RESET}"
  printf '    %s6) rldocs%s    - documentation viewer\n' "${C_BOLD}" "${C_RESET}"
  printf '    %s7) rlm%s       - toolchain manager\n' "${C_BOLD}" "${C_RESET}"
  printf "\n"
  printf "  ${C_DIM}Enter number(s), comma-separated (e.g. 1,3,7), or 'all'.${C_RESET}\n"
}

select_binaries() {
  if [ -n "$BINARIES_ARG" ]; then
    if [ "$BINARIES_ARG" = "all" ]; then
      printf '%s\n' "${BINARIES[@]}"
      return
    fi
    echo "$BINARIES_ARG" | tr ',' '\n' | sed 's/^ *//; s/ *$//'
    return
  fi

  if [ -n "${RL_BINARIES:-}" ]; then
    if [ "$RL_BINARIES" = "all" ]; then
      printf '%s\n' "${BINARIES[@]}"
      return
    fi
    echo "$RL_BINARIES" | tr ',' '\n' | sed 's/^ *//; s/ *$//'
    return
  fi

  if [ ! -t 0 ]; then
    fail "No TTY detected and --binaries is not set."
    fail "Non-interactive use requires: install-local.sh -b rl,rlc,rlm"
    exit 1
  fi

  print_menu >&2
  local choices
  read -rp "  Enter number(s), comma-separated (e.g. 1,3,7), or 'all': " choices >&2

  choices="$(echo "$choices" | tr -d '[:space:]')"
  if [ -z "$choices" ] || [ "$choices" = "all" ]; then
    printf '%s\n' "${BINARIES[@]}"
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
        fail "Invalid selection: $part"
        exit 1
        ;;
    esac
  done
}

# --- Install ---

install_one() {
  local binary="$1"
  local src_dir="target-bins/${PROFILE}"
  local src="$src_dir/$binary"
  local dst="$INSTALL_DIR/$binary"

  if [ ! -f "$src" ]; then
    fail "${binary} not found in ${src_dir}/"
    fail "Did you run ./scripts/build-local.sh --${PROFILE}?"
    return 1
  fi

  if [ -f "$dst" ] && [ "$FORCE" != "1" ]; then
    warn "${binary} already exists at ${dst}. Use --force to overwrite."
    return 0
  fi

  mkdir -p "$INSTALL_DIR"
  cp "$src" "$dst"
  chmod +x "$dst"

  ok "Installed: ${dst}"
}

# --- Uninstall ---

do_uninstall() {
  local removed=0

  header "Uninstalling rl-lang binaries"
  info "prefix: ${C_BOLD}${INSTALL_DIR}${C_RESET}"

  for binary in "${BINARIES[@]}"; do
    for ext in "" ".exe"; do
      local path="$INSTALL_DIR/${binary}${ext}"
      if [ -f "$path" ]; then
        rm -f "$path"
        ok "Removed: ${path}"
        removed=$((removed + 1))
      fi
    done
  done

  printf "\n"
  if [ "$removed" -gt 0 ]; then
    ok "${C_BOLD}${removed}${C_RESET} binary(ies) removed."
  else
    info "No rl-lang binaries found in ${INSTALL_DIR}."
  fi
}

# --- Main ---

main() {
  while [ $# -gt 0 ]; do
    case "$1" in
      -h|--help)      usage; exit 0 ;;
      --release)      PROFILE="release"; shift ;;
      --nightly)      PROFILE="nightly"; shift ;;
      --dev)          PROFILE="dev"; shift ;;
      -p|--prefix)    INSTALL_DIR="$2"; shift 2 ;;
      -f|--force)     FORCE=1; shift ;;
      -b|--binaries)  BINARIES_ARG="$2"; shift 2 ;;
      --uninstall)    do_uninstall; exit 0 ;;
      *)              fail "unknown option: $1"; usage >&2; exit 1 ;;
    esac
  done

  local src_dir="target-bins/${PROFILE}"
  if [ ! -d "$src_dir" ]; then
    fail "${src_dir}/ does not exist."
    fail "Run ./scripts/build-local.sh --${PROFILE} first."
    exit 1
  fi

  header "Install config"
  info "profile:  ${C_BOLD}${PROFILE}${C_RESET}"
  info "source:   ${C_BOLD}${src_dir}/${C_RESET}"
  info "prefix:   ${C_BOLD}${INSTALL_DIR}${C_RESET}"

  local binaries
  binaries="$(select_binaries)"

  header "Installing binaries"

  local failed=0 installed=0 total=0
  while IFS= read -r binary; do
    [ -z "$binary" ] && continue
    total=$((total + 1))
    if install_one "$binary"; then
      installed=$((installed + 1))
    else
      failed=1
    fi
  done <<<"$binaries"

  header "Results"

  if [ "$failed" = "1" ]; then
    fail "${installed}/${total} installed, some failed."
    exit 1
  else
    ok "${C_BOLD}${installed}/${total}${C_RESET} installed."
  fi

  if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    printf "\n"
    info "Add this to your shell profile:"
    info "  ${C_BOLD}export PATH=\"${INSTALL_DIR}:\$PATH\"${C_RESET}"
  fi

  printf "\n"
}

main "$@"
