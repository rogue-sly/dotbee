#!/usr/bin/env bash

info()  { echo -e "  \e[36m->\e[0m $*"; }
step()  { echo -e "\n\e[34m══ \e[1m$*\e[0m"; }
ok()    { echo -e "  \e[32m\xe2\x9c\x93\e[0m $*"; }
err()   { echo -e "  \e[31m\xe2\x9c\x97\e[0m $*"; }

init() {
  VERSION=$(grep -m 1 '^version = ' Cargo.toml | cut -d '"' -f 2)
  TAG="v$VERSION"
  ROOT="$(cd "$(dirname "$0")" && pwd)"
  while [ ! -f "$ROOT/Cargo.toml" ] && [ "$ROOT" != "/" ]; do
    ROOT="$(dirname "$ROOT")"
  done
  cd "$ROOT"
}

validate_version() {
  if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Usage: <version>"
    echo "  version must be semver (e.g., 0.8.0)"
    exit 1
  fi
}

detect_ndk() {
  if [ -n "${ANDROID_NDK_HOME:-}" ] && [ -d "$ANDROID_NDK_HOME" ]; then
    echo "$ANDROID_NDK_HOME"
    return 0
  fi
  if [ -n "${ANDROID_HOME:-}" ]; then
    local ndk
    ndk=$(ls -d "$ANDROID_HOME/ndk/"* 2>/dev/null | sort -V | tail -1)
    [ -n "$ndk" ] && echo "$ndk" && return 0
  fi
  shopt -s nullglob
  local candidates=(
    "$HOME/Android/Sdk/ndk/"*/
    "/usr/local/lib/android/sdk/ndk/"*/
    "/opt/android-sdk/ndk/"*/
  )
  shopt -u nullglob
  for dir in "${candidates[@]}"; do
    [ -d "$dir" ] && echo "${dir%/}" && return 0
  done
  echo ""
}

setup_ndk_env() {
  local ndk
  ndk=$(detect_ndk)
  if [ -z "$ndk" ]; then
    err "Android NDK not found. Set ANDROID_NDK_HOME or install it."
    err "See: https://developer.android.com/ndk/downloads"
    exit 1
  fi
  export ANDROID_NDK_HOME="$ndk"
  export PATH="$ndk/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"
  export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="aarch64-linux-android21-clang"
  ok "Android NDK: $ndk"
}
