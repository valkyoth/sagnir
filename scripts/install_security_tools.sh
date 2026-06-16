#!/usr/bin/env sh
set -eu

CARGO_DENY_VERSION=0.19.9
CARGO_DENY_SHA256=24bb0f6e6660ac1585169cb9a24ebd6ad77944561604e27f7e87ce2a6c70b6b7
CARGO_AUDIT_VERSION=0.22.2
CARGO_AUDIT_SHA256=700c2b240f7fd330c24b675fe429f73a5b676531fcc6300400b2b67f155ba12a

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/sagnir-security-tools.XXXXXX")"
trap 'rm -rf "$tmp_dir"' EXIT INT TERM

install_verified_crate() {
    crate="$1"
    version="$2"
    expected_sha="$3"
    archive="$tmp_dir/$crate-$version.crate"

    curl -fsSL -A sagnir-release-gate \
        "https://crates.io/api/v1/crates/$crate/$version/download" \
        -o "$archive"

    printf '%s  %s\n' "$expected_sha" "$archive" | sha256sum --check
    tar -xzf "$archive" -C "$tmp_dir"
    cargo install --locked --path "$tmp_dir/$crate-$version"
}

install_verified_crate cargo-deny "$CARGO_DENY_VERSION" "$CARGO_DENY_SHA256"
install_verified_crate cargo-audit "$CARGO_AUDIT_VERSION" "$CARGO_AUDIT_SHA256"
