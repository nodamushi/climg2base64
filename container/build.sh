#!/bin/bash -eu
workspace=$1
output=$2
arch=$(uname -m)

cd "$workspace"

#  x86_64-unknown-linux-gnu
RUSTFLAGS="-C linker=x86_64-linux-gnu-gcc" \
CARGO_TARGET_DIR=/tmp/linux-x86_64/target_no_file \
  cargo build --release --target x86_64-unknown-linux-gnu

mkdir -p "$output/linux-x86_64/no_file"
cp /tmp/linux-x86_64/target_no_file/x86_64-unknown-linux-gnu/release/climg2base64 "$output/linux-x86_64/no_file"
cd "$output/linux-x86_64/no_file"
tar -zcf climg2base64-linux-x86_64.tar.gz ./climg2base64

if [ "$arch" = "x86_64" ];then
  # NOTE: --feature file can not cross compile
  cd "$workspace"
  RUSTFLAGS="-C linker=x86_64-linux-gnu-gcc" \
  CARGO_TARGET_DIR=/tmp/linux-x86_64/target_file \
    cargo build --release --target x86_64-unknown-linux-gnu --features file

  mkdir -p "$output/linux-x86_64/file"
  cp /tmp/linux-x86_64/target_file/x86_64-unknown-linux-gnu/release/climg2base64 "$output/linux-x86_64/file"
  cd "$output/linux-x86_64/file"
  tar -zcf climg2base64-linux-x86_64.tar.gz ./climg2base64
fi

# aarch64-unknown-linux-gnu
cd "$workspace"
RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc" \
CARGO_TARGET_DIR=/tmp/linux-aarch64/target_no_file \
  cargo build --release --target aarch64-unknown-linux-gnu

mkdir -p "$output/linux-aarch64/no_file"
cp /tmp/linux-aarch64/target_no_file/aarch64-unknown-linux-gnu/release/climg2base64 "$output/linux-aarch64/no_file"
cd "$output/linux-aarch64/no_file"
tar -zcf climg2base64-linux-aarch64.tar.gz ./climg2base64


if [ "$arch" = "aarch64" ];then
  # NOTE: --feature file can not cross compile
  cd "$workspace"
  RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc" \
  CARGO_TARGET_DIR=/tmp/linux-aarch64/target_file \
    cargo build --release --target aarch64-unknown-linux-gnu --features file

  mkdir -p "$output/linux-aarch64/file"
  cp /tmp/linux-aarch64/target_file/aarch64-unknown-linux-gnu/release/climg2base64 "$output/linux-aarch64/file"
  cd "$output/linux-aarch64/file"
  tar -zcf climg2base64-linux-aarch64.tar.gz ./climg2base64
fi

# windows
cd "$workspace"
CARGO_TARGET_DIR=/tmp/windows/target \
    cargo build --release --features "file" \
    --target x86_64-pc-windows-gnu

mkdir -p "$output/windows-x86_64"
cp /tmp/windows/target/x86_64-pc-windows-gnu/release/climg2base64.exe "$output/windows-x86_64"
cd "$output/windows-x86_64"
zip climg2base64-windows-x86_64.zip ./climg2base64.exe
