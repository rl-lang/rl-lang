#!/usr/bin/env bash
set -euo pipefail

TARGET="$1"
PLATFORM="$2"
ARCH="$3"
OUT_DIR="$4"

mkdir -p "$OUT_DIR"
OUT_DIR="$(cd "$OUT_DIR" && pwd)"

# Build profile: "release", "nightly", or "dev-release"
BUILD_PROFILE="${RL_BUILD_PROFILE:-release}"

# All 7 binaries
BINARIES="rl rlc rlt rlrepl rlsp rldocs rlm"

exe_ext() {
  if [ "$PLATFORM" = "windows" ]; then
    echo ".exe"
  fi
}

profile_flag() {
  case "$1" in
    release)     echo "--release" ;;
    nightly)     echo "--profile nightly" ;;
    dev-release) echo "--profile dev-release" ;;
    *)           echo "--release" ;;
  esac
}

profile_dir() {
  case "$1" in
    release)     echo "release" ;;
    nightly)     echo "nightly" ;;
    dev-release) echo "dev-release" ;;
    *)           echo "release" ;;
  esac
}

package_elf() {
  local actual="$1" bin_path="$2" label="$3"
  local stage
  stage=$(mktemp -d)
  cp "$bin_path" "$stage/${actual}"
  chmod +x "$stage/${actual}"
  tar -czf "$OUT_DIR/${actual}-${label}-${ARCH}.tar.gz" -C "$stage" "${actual}"
  rm -rf "$stage"
}

package_windows() {
  local actual="$1" bin_path="$2"
  local stage
  stage=$(mktemp -d)
  cp "$bin_path" "$stage/${actual}.exe"
  (cd "$stage" && zip -q "$OUT_DIR/${actual}-windows-${ARCH}.zip" "${actual}.exe")
  rm -rf "$stage"
}

package_one() {
  local name="$1" bin_path="$2"
  if [ "$PLATFORM" = "windows" ]; then
    package_windows "$name" "$bin_path"
  elif [ "$PLATFORM" = "android" ]; then
    package_elf "$name" "$bin_path" "android"
  else
    package_elf "$name" "$bin_path" "$PLATFORM"
  fi
}

pflag="$(profile_flag "$BUILD_PROFILE")"
pdir="$(profile_dir "$BUILD_PROFILE")"
EXE="$(exe_ext)"

echo "=== Building all binaries [profile: ${BUILD_PROFILE}] ==="

# rl (core)
echo "--- rl ---"
cargo build $pflag --no-default-features --features "vm,docs,docs-tui,repl,pm" \
  --target "$TARGET" -p rl-cli
package_one "rl" "target/${TARGET}/${pdir}/rl${EXE}"

# rlc (compiler, VM backend)
echo "--- rlc ---"
cargo build $pflag --no-default-features --features vm --target "$TARGET" -p rl-cli --bin rlc
package_one "rlc" "target/${TARGET}/${pdir}/rlc${EXE}"

# rlt (transpiler)
echo "--- rlt ---"
cargo build $pflag --target "$TARGET" -p rl-cli --bin rlt --features cc
package_one "rlt" "target/${TARGET}/${pdir}/rlt${EXE}"

# rlrepl
echo "--- rlrepl ---"
cargo build $pflag --target "$TARGET" -p rl-repl --bin rlrepl
package_one "rlrepl" "target/${TARGET}/${pdir}/rlrepl${EXE}"

# rlsp
echo "--- rlsp ---"
cargo build $pflag --target "$TARGET" -p rl-lsp --bin rlsp
package_one "rlsp" "target/${TARGET}/${pdir}/rlsp${EXE}"

# rldocs
echo "--- rldocs ---"
cargo build $pflag --target "$TARGET" -p rl-docs --bin rldocs
package_one "rldocs" "target/${TARGET}/${pdir}/rldocs${EXE}"

# rlm (toolchain manager)
echo "--- rlm ---"
cargo build $pflag --target "$TARGET" -p rl-manager --bin rlm
package_one "rlm" "target/${TARGET}/${pdir}/rlm${EXE}"

echo "=== Done: all binaries ==="
