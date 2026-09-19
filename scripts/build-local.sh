#!/usr/bin/env bash
set -euo pipefail

# scripts/build-local.sh - build all rl-lang binaries locally
#
# Usage:
#   ./scripts/build-local.sh                    # release build
#   ./scripts/build-local.sh --debug            # debug build
#   ./scripts/build-local.sh --nightly          # nightly profile
#   ./scripts/build-local.sh -j 8               # 8 parallel jobs
#   ./scripts/build-local.sh --clean            # clean target/ before building
#   ./scripts/build-local.sh --clean --release  # clean + release

PROFILE="release"
CLEAN=0
CLEAN_BINS=0
JOBS=""

# --- Colors ---
if [ -t 1 ] && [ -t 2 ]; then
  C_RESET=$'\e[0m'
  C_BOLD=$'\e[1m'
  C_DIM=$'\e[2m'
  C_CYAN=$'\e[36m'
  C_GREEN=$'\e[32m'
  C_YELLOW=$'\e[33m'
  C_RED=$'\e[31m'
  C_MAGENTA=$'\e[35m'
  C_BLUE=$'\e[34m'
else
  C_RESET="" C_BOLD="" C_DIM="" C_CYAN="" C_GREEN="" C_YELLOW="" C_RED="" C_MAGENTA="" C_BLUE=""
fi

info()  { printf "${C_DIM}  ::${C_RESET} %s\n" "$*"; }
ok()    { printf "${C_GREEN}  [ OK ]${C_RESET} %s\n" "$*"; }
fail()  { printf "${C_RED}  [FAIL]${C_RESET} %s\n" "$*"; }
warn()  { printf "${C_YELLOW}  [WARN]${C_RESET} %s\n" "$*"; }
header(){ printf "\n${C_BOLD}${C_CYAN}:: %s${C_RESET}\n" "$*"; }

usage() {
  printf "\n"
  printf "${C_BOLD}Usage:${C_RESET} ${C_CYAN}build-local.sh${C_RESET} [OPTIONS]\n"
  printf "\n"
  printf "Build all rl-lang binaries and copy them to ${C_BOLD}target-bins/${C_RESET}.\n"
  printf "\n"
  printf "${C_BOLD}Options:${C_RESET}\n"
  printf "  ${C_GREEN}--release${C_RESET}          Build with --release ${C_DIM}(default)${C_RESET}\n"
  printf "  ${C_GREEN}--nightly${C_RESET}          Build with --profile nightly\n"
  printf "  ${C_GREEN}--dev${C_RESET}              Build without --release ${C_DIM}(debug)${C_RESET}\n"
  printf "  ${C_YELLOW}-j, --jobs${C_RESET} ${C_CYAN}N${C_RESET}       Number of parallel cargo jobs\n"
  printf "  ${C_RED}--clean${C_RESET}            Remove target/ before building\n"
  printf "  ${C_RED}--clean-bins${C_RESET}       Remove target-bins/ before building\n"
  printf "  ${C_RED}--clean-all${C_RESET}        Remove target/ and target-bins/ before building\n"
  printf "  ${C_BLUE}-h, --help${C_RESET}         Show this help\n"
  printf "\n"
}

while [ $# -gt 0 ]; do
  case "$1" in
    --release)  PROFILE="release"; shift ;;
    --nightly)  PROFILE="nightly"; shift ;;
    --dev)      PROFILE="dev"; shift ;;
    -j|--jobs)  JOBS="$2"; shift 2 ;;
    --clean)    CLEAN=1; shift ;;
    --clean-bins) CLEAN_BINS=1; shift ;;
    --clean-all) CLEAN=1; CLEAN_BINS=1; shift ;;
    -h|--help)  usage; exit 0 ;;
    *)          fail "unknown option: $1"; exit 1 ;;
  esac
done

OUT_DIR="target-bins/${PROFILE}"

if [ "$CLEAN" = "1" ] || [ "$CLEAN_BINS" = "1" ]; then
  header "Cleaning"
  if [ "$CLEAN" = "1" ]; then
    info "removing target/"
    rm -rf target/
  fi
  if [ "$CLEAN_BINS" = "1" ]; then
    info "removing target-bins/"
    rm -rf target-bins/
  fi
  ok "cleaned"
fi

mkdir -p "$OUT_DIR"

# Build profile flag
case "$PROFILE" in
  release) PROFILE_FLAG="--release";  PROFILE_DIR="target/release" ;;
  nightly) PROFILE_FLAG="--profile nightly"; PROFILE_DIR="target/nightly" ;;
  dev)     PROFILE_FLAG="";           PROFILE_DIR="target/debug" ;;
esac

JOBS_FLAG=""
if [ -n "$JOBS" ]; then
  JOBS_FLAG="-j $JOBS"
fi

header "Build config"
info "profile: ${C_BOLD}${PROFILE}${C_RESET}"
info "output:  ${C_BOLD}${OUT_DIR}/${C_RESET}"
info "jobs:    ${C_BOLD}${JOBS:-auto}${C_RESET}"

# Track failures
FAILED=0
BUILT=0

build_one() {
  local name="$1" pkg="$2" bin_flag="${3:-}" features="${4:-}"

  local cmd="cargo build $PROFILE_FLAG $JOBS_FLAG --target-dir target $features $bin_flag -p $pkg"

  printf "\n${C_CYAN}  ::${C_RESET} ${C_BOLD}building %-10s${C_RESET}\n" "$name"
  printf "${C_DIM}     %s${C_RESET}\n" "$cmd"

  if eval "$cmd"; then
    printf "${C_GREEN}  [ OK ]${C_RESET} %s\n" "$name"
  else
    printf "${C_RED}  [FAIL]${C_RESET} %s\n" "$name"
    FAILED=1
    return
  fi

  # Find the binary in the profile-specific target dir
  local src="$PROFILE_DIR/$name"
  local src_exe="$PROFILE_DIR/$name.exe"
  if [ -f "$src" ]; then
    cp "$src" "$OUT_DIR/$name"
    chmod +x "$OUT_DIR/$name"
    BUILT=$((BUILT + 1))
  elif [ -f "$src_exe" ]; then
    cp "$src_exe" "$OUT_DIR/$name.exe"
    BUILT=$((BUILT + 1))
  else
    warn "could not find $name binary in $PROFILE_DIR/"
    return
  fi

  # Strip release and nightly binaries
  if [ "$PROFILE" != "dev" ]; then
    local dst="$OUT_DIR/$name"
    local dst_exe="$OUT_DIR/$name.exe"
    if [ -f "$dst" ]; then
      strip "$dst" 2>/dev/null || true
    elif [ -f "$dst_exe" ]; then
      strip "$dst_exe" 2>/dev/null || true
    fi
  fi
}

header "Building binaries"

# rl (core)
build_one "rl" "rl-cli" "--no-default-features" "--features vm,docs,docs-tui,repl,pm"

# rlc (compiler + runner)
build_one "rlc" "rl-cli" "--no-default-features --bin rlc" "--features vm"

# rlt (transpiler)
build_one "rlt" "rl-cli" "--no-default-features --bin rlt" "--features cc"

# rlrepl
build_one "rlrepl" "rl-repl" "--bin rlrepl" ""

# rlsp
build_one "rlsp" "rl-lsp" "--bin rlsp" ""

# rldocs
build_one "rldocs" "rl-docs" "--bin rldocs" ""

# rlm
build_one "rlm" "rl-manager" "--bin rlm" ""

header "Results"

if [ "$FAILED" = "1" ]; then
  fail "some builds failed"
  exit 1
else
  ok "${BUILT}/7 built"
  info "binaries in ${C_BOLD}${OUT_DIR}/${C_RESET}:"
  ls -lh "$OUT_DIR/" | tail -n +2
fi
