#!/bin/bash

set -e

# Setup
BUILD_DIR=platform-build
mkdir $BUILD_DIR
cd $BUILD_DIR

# Create the jniLibs build directory
JNI_DIR=jniLibs
mkdir -p $JNI_DIR

# Set up cargo-ndk
cargo install --version 3.5.4 cargo-ndk
rustup target add \
        aarch64-linux-android \
        armv7-linux-androideabi \
        x86_64-linux-android \
        i686-linux-android

# Google Play (Nov 2024) requires 16 KiB aligned LOAD segments.
# Pass the linker flags through RUSTFLAGS so every target inherits them.
export RUSTFLAGS="${RUSTFLAGS:-} -C link-arg=-Wl,-z,max-page-size=0x4000 -C link-arg=-Wl,-z,common-page-size=0x4000"

# Build the android libraries in the jniLibs directory
cargo ndk -o $JNI_DIR \
        --manifest-path ../Cargo.toml \
        -t armeabi-v7a \
        -t arm64-v8a \
        -t x86 \
        -t x86_64 \
        build --release

# Archive the dynamic libs
cd $JNI_DIR
tar -czvf ../android.tar.gz *
cd -

# Cleanup
rm -rf $JNI_DIR
