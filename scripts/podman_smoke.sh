#!/usr/bin/env sh
set -eu

image="${SAGNIR_PODMAN_IMAGE:-localhost/sagnir-cli:dev}"
podman build -f Containerfile -t "$image" .
podman run --rm --userns=keep-id "$image" version
