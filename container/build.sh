#!/bin/bash -eu
# * mount this file to /build.sh
######## args ########
readonly workspace=$1
readonly output=$2
readonly build_dir=$3
######################

# Constants
readonly arch=$(uname -m)
readonly FILE=1
readonly NOFILE=0
readonly LINUX=0
readonly WIN=1

# Build command
build() {
  ###### args ########
  local os="$1"
  local flag="$2"
  local build_target="$3"
  local file="$4"
  local save_name="$5"
  #####################

  # Win or Linux
  local exec
  local pack
  local pack_cmd
  local pack_arg
  if [ $os -eq $WIN ];then
    exec=climg2base64.exe
    pack=climg2base64-${save_name}.zip
    pack_cmd=zip
    pack_arg=($pack $exec)
  else
    exec=climg2base64
    pack=climg2base64-${save_name}.tar.gz
    pack_cmd=tar
    pack_arg=(-zcf $pack $exec)
  fi

  # Cargo build variables
  local cargo_args=(build --release --target "$build_target")
  local tmpdir
  local outdir
  if [ "$file" -eq $FILE ];then
    cargo_args+=(--features file)
    if [ $os -eq $WIN ];then
      tmpdir=${build_dir}/${build_target}
      outdir=$output/${save_name}
    else
      tmpdir=${build_dir}/${build_target}/file
      outdir=$output/${save_name}/file
    fi
  else
    tmpdir=${build_dir}/${build_target}/no_file
    outdir=$output/${save_name}/no_file
  fi

  local cargo_env=("CARGO_TARGET_DIR=$tmpdir" "CARGO_HOME=${build_dir}/.cargo")
  if [ -n "$flag" ];then
    cargo_env+=("RUSTFLAGS=$flag")
  fi

  cd "$workspace"
  # Build
  env "${cargo_env[@]}" cargo "${cargo_args[@]}"
  # zip/tar
  mkdir -p $outdir
  cp $tmpdir/${build_target}/release/$exec $outdir
  cd $outdir
  $pack_cmd "${pack_arg[@]}"
  # MD5
  md5sum $pack > md5.txt
}

#  x86_64-unknown-linux-gnu
build $LINUX "-C linker=x86_64-linux-gnu-gcc" x86_64-unknown-linux-gnu $NOFILE linux-x86_64
if [ "$arch" = "x86_64" ];then
  # NOTE: --feature file can not cross compile
  build $LINUX "-C linker=x86_64-linux-gnu-gcc" x86_64-unknown-linux-gnu $FILE linux-x86_64
fi

# aarch64-unknown-linux-gnu
build $LINUX "-C linker=aarch64-linux-gnu-gcc" aarch64-unknown-linux-gnu $NOFILE linux-aarch64
if [ "$arch" = "aarch64" ];then
  # NOTE: --feature file can not cross compile
  build $LINUX "-C linker=aarch64-linux-gnu-gcc" aarch64-unknown-linux-gnu $FILE linux-aarch64
fi

# windows
build $WIN "" x86_64-pc-windows-gnu $FILE windows-x86_64
