# Sagnir Container Image Policy

Status: policy

Container base images are supply-chain inputs.

Release images must pin base images by digest. Mutable tags such as
`debian:stable-slim` or `rust:<version>-bookworm` are acceptable only in local
development scaffolds before the release image gate exists. When an official
Rust container patch tag lags the current stable toolchain, a digest-pinned
older bootstrap image may be used only if the container explicitly installs and
verifies the exact `rust-toolchain.toml` version before building Sagnir.

Before any Sagnir container image is release-published:

- fetch the current base image digest from the registry;
- review the base image update as a dependency change;
- update the `Containerfile` digest deliberately;
- run the rootless Podman smoke;
- record the image digest in release notes.

The v0.1.0 scaffold does not publish release images.
