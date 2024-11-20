#!/bin/bash -eu
repogitory=$(dirname "$0")
repogitory=$(readlink -f "$repogitory")
dir=$repogitory/container

exec podman build -t climg2base64_build -f ContainerFile "$dir"
