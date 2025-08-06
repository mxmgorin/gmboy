#!/bin/bash

export ANDROID_NDK_HOME=${ANDROID_NDK_HOME:-$HOME/Android/Sdk/ndk/27.0.12077973}
export TARGET_CC="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android33-clang"
export TARGET_AR="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"

SDL2_LIB_PATH="$(pwd)/app/src/main/jniLibs/arm64-v8a"

RUSTFLAGS="-C linker=$TARGET_CC -L $SDL2_LIB_PATH" \
CC=$TARGET_CC \
AR=$TARGET_AR \
cargo build --release --target aarch64-linux-android "$@"
