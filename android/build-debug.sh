#!/bin/bash
set -e

export ANDROID_NDK_HOME=${ANDROID_NDK_HOME:-$HOME/Android/Sdk/ndk/26.3.11579264}

LIB_NAME="libmain.so"

# Android targets (TRIPLE:API:JNILIBS_DIR)
TARGETS=(
  "x86_64-linux-android:33:x86_64"
)

cargo clean

for target in "${TARGETS[@]}"; do
    IFS=":" read -r TRIPLE API JNI_DIR <<< "$target"

    # Setup toolchain for Rust
    TOOLCHAIN="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin"
    CC="$TOOLCHAIN/${TRIPLE}${API}-clang"
    AR="$TOOLCHAIN/llvm-ar"

    case $TRIPLE in
        aarch64-linux-android) RUST_TARGET="aarch64-linux-android" ;;
        armv7a-linux-androideabi) RUST_TARGET="armv7-linux-androideabi" ;;
        x86_64-linux-android) RUST_TARGET="x86_64-linux-android" ;;
        i686-linux-android) RUST_TARGET="i686-linux-android" ;;
    esac

    # Build Rust library
    SDL2_LIB_PATH="$(pwd)/app/src/main/jniLibs/$JNI_DIR"
    RUSTFLAGS="-C linker=$CC -L $SDL2_LIB_PATH" \
    CC=$CC \
    AR=$AR \
    cargo build --target "$RUST_TARGET"

    # Copy Rust .so
    cp "../target/$RUST_TARGET/debug/$LIB_NAME" "$SDL2_LIB_PATH/"
done