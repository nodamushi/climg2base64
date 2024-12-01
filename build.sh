#!/bin/bash -eu
repository=$(dirname "$0")
repository=$(readlink -f "$repository")

mkdir -p "$repository/output/.tmp"
echo "*" > "$repository/output/.gitignore"
podman run --rm -it \
  -v "$repository/container/build.sh:/build.sh:ro" \
  -v "$repository:/climg2base64:ro" \
  -v "$repository/output:/output" \
  -v "$repository/output/.tmp:/build_dir" \
  climg2base64_build /climg2base64 /output /build_dir
