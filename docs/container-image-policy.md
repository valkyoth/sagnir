# Sagnir Container Image Policy

Status: policy

Container base images are supply-chain inputs.

Release images must pin base images by digest. Mutable tags such as
`debian:stable-slim` or `rust:1.96.1-bookworm` are acceptable only in local
development scaffolds before the release image gate exists.

Before any Sagnir container image is release-published:

- fetch the current base image digest from the registry;
- review the base image update as a dependency change;
- update the `Containerfile` digest deliberately;
- run the rootless Podman smoke;
- record the image digest in release notes.

The v0.1.0 scaffold does not publish release images.
