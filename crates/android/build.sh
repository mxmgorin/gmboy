#!/usr/bin/env bash
# Build the Android native libraries and place them under app/src/main/jniLibs/<abi>/
# ready for Gradle to package:
#
#   1. libSDL2.so  — built from the pinned SDL source via the NDK + CMake
#   2. libmain.so  — the Rust cdylib, built with cargo-ndk
#
# Usage:
#   ./build.sh                                  # all ABIs
#   ABIS="arm64-v8a armeabi-v7a" ./build.sh     # a subset (e.g. one APK variant)
#   CLEAN=1 ./build.sh                          # force a clean Rust rebuild
#
# Requires: cargo-ndk (`cargo install cargo-ndk --locked`), the Android NDK
# (ANDROID_NDK_HOME) and cmake on PATH.
set -euo pipefail
cd "$(dirname "$0")"   # crates/android/

ABIS="${ABIS:-arm64-v8a armeabi-v7a x86_64 x86}"
API="${ANDROID_API:-21}"
export ANDROID_NDK_HOME="${ANDROID_NDK_HOME:-$HOME/Android/Sdk/ndk/26.3.11579264}"
SDL2_SRC="${SDL2_SRC:-$(pwd)/SDL2}"

# SDL 2.28's CMakeLists declares an old cmake_minimum_required that CMake 4.x
# rejects; this policy shim lets a modern cmake configure it anyway (no-op on 3.x).
export CMAKE_POLICY_VERSION_MINIMUM="${CMAKE_POLICY_VERSION_MINIMUM:-3.5}"

command -v cargo-ndk >/dev/null 2>&1 || {
    echo "cargo-ndk not found. Install it with:  cargo install cargo-ndk --locked" >&2
    exit 1
}

# Force a clean Rust rebuild with CLEAN=1 (cargo tracks dependencies correctly
# otherwise, so a clean is only ever needed to recover from a broken state).
if [ "${CLEAN:-0}" = "1" ]; then
    echo ">>> CLEAN=1 set, running cargo clean"
    ( cd ../.. && cargo clean )
fi

# Map an Android ABI to its Rust target triple.
rust_target() {
    case "$1" in
        arm64-v8a)   echo aarch64-linux-android ;;
        armeabi-v7a) echo armv7-linux-androideabi ;;
        x86_64)      echo x86_64-linux-android ;;
        x86)         echo i686-linux-android ;;
        *) echo "unknown ABI: $1" >&2; exit 1 ;;
    esac
}

# Clone the pinned SDL2 source once.
if [ ! -d "$SDL2_SRC" ]; then
    echo ">>> cloning SDL 2.28.x into $SDL2_SRC"
    git clone --branch release-2.28.x --depth=1 https://github.com/libsdl-org/SDL.git "$SDL2_SRC"
fi

# Build libSDL2.so per ABI. Skipped when already present (e.g. restored from cache).
for abi in $ABIS; do
    jni="app/src/main/jniLibs/$abi"
    if [ -f "$jni/libSDL2.so" ]; then
        echo ">>> SDL2 for $abi already present, skipping"
        continue
    fi
    echo ">>> building SDL2 for $abi"
    build="build/build-sdl2-$abi"
    cmake -S "$SDL2_SRC" -B "$build" \
        -DCMAKE_TOOLCHAIN_FILE="$ANDROID_NDK_HOME/build/cmake/android.toolchain.cmake" \
        -DANDROID_ABI="$abi" -DANDROID_PLATFORM="android-$API" \
        -DCMAKE_BUILD_TYPE=Release -DSDL_STATIC=OFF -DSDL_SHARED=ON
    cmake --build "$build" --config Release -j"$(nproc)"
    mkdir -p "$jni"
    cp "$build/libSDL2.so" "$jni/"
done

# Ensure the Rust targets for the requested ABIs are installed.
for abi in $ABIS; do
    t="$(rust_target "$abi")"
    rustup target list --installed | grep -qx "$t" || rustup target add "$t"
done

# Build libmain.so for every ABI with cargo-ndk, straight into jniLibs. cargo-ndk
# wires up the NDK toolchain; the per-ABI `-L` path to libSDL2.so comes from the
# workspace-root .cargo/config.toml. Run from the workspace root so that applies.
ndk_targets=()
for abi in $ABIS; do ndk_targets+=(-t "$abi"); done
( cd ../.. && cargo ndk "${ndk_targets[@]}" --platform "$API" \
    -o crates/android/app/src/main/jniLibs build --release --package android )

echo "=== ✅ Android build complete for: $ABIS ==="
