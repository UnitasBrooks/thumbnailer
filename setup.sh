#!/usr/bin/env bash
set -e

echo "Updating apt repositories..."
sudo apt update

echo "Installing basic tools..."
sudo apt install -y vim build-essential clang pkg-config

echo "Installing FFmpeg runtime and development libraries..."
sudo apt install -y \
    ffmpeg \
    libavcodec-dev \
    libavformat-dev \
    libavfilter-dev \
    libavdevice-dev \
    libavutil-dev \
    libswscale-dev \
    libswresample-dev \
    curl

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

echo "Setting environment variables for C/C++ include paths (for bindgen)..."
export C_INCLUDE_PATH=/usr/include
export CPLUS_INCLUDE_PATH=/usr/include

echo "All dependencies installed."
echo "You can now build Rust projects using ffmpeg-next or ffmpeg-sys-next."
echo "Tip: run 'cargo clean && cargo build' in your Rust project to verify."