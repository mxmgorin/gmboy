#!/bin/bash
set -e

# Android NDK path
export ANDROID_NDK_HOME=${ANDROID_NDK_HOME:-$HOME/Android/Sdk/ndk/27.0.12077973}

# Target definitions: ARCH=TRIPLE:API:JNILIBS_DIR
TARGETS=(
  "aarch64-linux-android:33:arm64-v8a"
  "armv7a-linux-androideabi:33:armeabi-v7a"
  "x86_64-linux-android:33:x86_64"
  "i686-linux-android:33:x86"
)

# Output library name (from your Cargo project)
LIB_NAME="libmain.so"

# Clean before building (optional)
cargo clean

for target in "${TARGETS[@]}"; do
    IFS=":" read -r TRIPLE API JNI_DIR <<< "$target"

    echo "=== Building for $JNI_DIR ==="

    TOOLCHAIN="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin"
    CC="$TOOLCHAIN/${TRIPLE}${API}-clang"
    AR="$TOOLCHAIN/llvm-ar"

    # Create jniLibs subdirectory if not exists
    SDL2_LIB_PATH="$(pwd)/app/src/main/jniLibs/$JNI_DIR"
    mkdir -p "$SDL2_LIB_PATH"

    # Set Rust target triple
    case $TRIPLE in
        aarch64-linux-android) RUST_TARGET="aarch64-linux-android" ;;
        armv7a-linux-androideabi) RUST_TARGET="armv7-linux-androideabi" ;;
        x86_64-linux-android) RUST_TARGET="x86_64-linux-android" ;;
        i686-linux-android) RUST_TARGET="i686-linux-android" ;;
    esac

    # Build
    RUSTFLAGS="-C linker=$CC -L $SDL2_LIB_PATH" \
    CC=$CC \
    AR=$AR \
    cargo build --release --target "$RUST_TARGET"

    # Copy .so to the correct jniLibs folder
    cp "../target/$RUST_TARGET/release/$LIB_NAME" "$SDL2_LIB_PATH/"
done

echo "=== Build completed for all architectures ==="
