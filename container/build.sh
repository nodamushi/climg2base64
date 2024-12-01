#!/bin/bash -eu
# * mount this file to /build.sh
workspace=$1
output=$2
build_dir=$3
arch=$(uname -m)

cd "$workspace"

FILE=1
NOFILE=0

LINUX=0
WIN=1

build() {
  os=$1
  flag=$2
  build_target=$3
  file=$4
  save_dirname=$5
  cd "$workspace"

  if [ $os -eq $WIN ];then
    exec=climg2base64.exe
    pack=climg2base64-${save_dirname}.zip
    pack_cmd=zip
    pack_arg=($pack $exec)
  else
    exec=climg2base64
    pack=climg2base64-${save_dirname}.tar.gz
    pack_cmd=tar
    pack_arg=(-zcf $pack $exec)
  fi

  args=(build --release --target "$build_target")

  if [ "$file" -eq $FILE ];then
    args+=(--features file)
    if [ $os -eq $WIN ];then
      tmpdir=${build_dir}/${build_target}
      outdir=$output/${save_dirname}
    else
      tmpdir=${build_dir}/${build_target}/no_file
      outdir=$output/${save_dirname}/no_file
    fi
  else
    tmpdir=${build_dir}/${build_target}/file
    outdir=$output/${save_dirname}/file
  fi

  envvar=("CARGO_TARGET_DIR=$tmpdir" "CARGO_HOME=${build_dir}/.cargo")
  if [ -n "$flag" ];then
    envvar+=("RUSTFLAGS=-C $flag")
  fi

  env "${envvar[@]}" cargo "${args[@]}"

  mkdir -p $outdir
  cp $tmpdir/${build_target}/release/$exec $outdir
  cd $outdir
  $pack_cmd "${pack_arg[@]}"
  md5sum $pack > md5.txt
}

#  x86_64-unknown-linux-gnu
build $LINUX "linker=x86_64-linux-gnu-gcc" x86_64-unknown-linux-gnu $NOFILE linux-x86_64
if [ "$arch" = "x86_64" ];then
  # NOTE: --feature file can not cross compile
  build $LINUX "linker=x86_64-linux-gnu-gcc" x86_64-unknown-linux-gnu $FILE linux-x86_64
fi

# aarch64-unknown-linux-gnu
build $LINUX "linker=aarch64-linux-gnu-gcc" aarch64-unknown-linux-gnu $NOFILE linux-aarch64
if [ "$arch" = "aarch64" ];then
  # NOTE: --feature file can not cross compile
  build $LINUX "linker=aarch64-linux-gnu-gcc" aarch64-unknown-linux-gnu $FILE linux-aarch64
fi

# windows
build $WIN "" x86_64-pc-windows-gnu $FILE windows-x86_64
