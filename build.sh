#!/bin/bash -eu
repository=$(dirname "$0")
repository=$(readlink -f "$repository")

mkdir -p "$repository/output"

podman run --rm -it \
  -v "$repository:/climg2base64:ro" \
  -v "$repository/output:/output" \
  climg2base64_build /climg2base64 /output
