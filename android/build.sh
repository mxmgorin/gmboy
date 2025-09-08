#!/bin/bash
set -e

export ANDROID_NDK_HOME=${ANDROID_NDK_HOME:-$HOME/Android/Sdk/ndk/26.3.11579264}
SDL2_SRC=${SDL2_SRC:-$(pwd)/SDL2}   # SDL2 source folder (default inside project)

# ✅ Auto-clone SDL2 (stable 2.28.x) if missing
if [ ! -d "$SDL2_SRC" ]; then
    echo ">>> SDL2 source not found, cloning SDL2.28..."
    git clone --branch release-2.28.x --depth=1 https://github.com/libsdl-org/SDL.git "$SDL2_SRC"
    echo ">>> SDL2.28 cloned into $SDL2_SRC"
fi


# Get crate name dynamically
LIB_NAME="libmain.so"

# Android targets (TRIPLE:API:JNILIBS_DIR)
TARGETS=(
  "aarch64-linux-android:33:arm64-v8a"
  "armv7a-linux-androideabi:33:armeabi-v7a"
  "x86_64-linux-android:33:x86_64"
  "i686-linux-android:33:x86"
)

# ✅ Ensure Rust targets are installed
for t in aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android; do
    if ! rustup target list | grep -q "^$t (installed)"; then
        echo ">>> Installing missing Rust target: $t"
        rustup target add $t
    fi
done

# ✅ Function to build SDL2 for a given arch
build_sdl2() {
    TRIPLE=$1
    API=$2
    JNI_DIR=$3

    echo ">>> Building SDL2 for $JNI_DIR"
    TOOLCHAIN="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64"

    BUILD_DIR="build/build-sdl2-$JNI_DIR"
    mkdir -p "$BUILD_DIR"
    pushd "$BUILD_DIR" >/dev/null

    cmake "$SDL2_SRC" \
        -DCMAKE_TOOLCHAIN_FILE="$ANDROID_NDK_HOME/build/cmake/android.toolchain.cmake" \
        -DANDROID_ABI="$JNI_DIR" \
        -DANDROID_PLATFORM=android-$API \
        -DCMAKE_BUILD_TYPE=Release \
        -DSDL_STATIC=OFF -DSDL_SHARED=ON

    cmake --build . --config Release -j$(nproc)

    popd >/dev/null
    mkdir -p "app/src/main/jniLibs/$JNI_DIR"
    cp "$BUILD_DIR/libSDL2.so" "app/src/main/jniLibs/$JNI_DIR/"
}

# ✅ Clean old builds
cargo clean

# ✅ Loop through each arch, build SDL2 and Rust .so
for target in "${TARGETS[@]}"; do
    IFS=":" read -r TRIPLE API JNI_DIR <<< "$target"

    # Build SDL2 first
    build_sdl2 $TRIPLE $API $JNI_DIR

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
    cargo build --release --target "$RUST_TARGET"

    # Copy Rust .so
    cp "../target/$RUST_TARGET/release/$LIB_NAME" "$SDL2_LIB_PATH/"
done

echo "=== ✅ Build completed (SDL2 + Rust) for all architectures ==="
