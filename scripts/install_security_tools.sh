#!/usr/bin/env sh
set -eu

CARGO_DENY_VERSION=0.22.0
CARGO_DENY_SHA256=8372c38eace236d1fa34511ae7eb2ce28595cc50be63430d0bea8a833ad51446
CARGO_AUDIT_VERSION=0.23.0
CARGO_AUDIT_SHA256=956f85aedfb64db887ec38466f982fc2b1ae55de726d4a12f2fddd5b812c6446

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/sagnir-security-tools.XXXXXX")"
trap 'rm -rf "$tmp_dir"' EXIT INT TERM

install_verified_crate() {
    crate="$1"
    version="$2"
    expected_sha="$3"
    archive="$tmp_dir/$crate-$version.crate"
    expected_root="$crate-$version"

    curl -fsSL -A sagnir-release-gate \
        "https://crates.io/api/v1/crates/$crate/$version/download" \
        -o "$archive"

    printf '%s  %s\n' "$expected_sha" "$archive" | sha256sum --check
    tar -tzf "$archive" | while IFS= read -r entry; do
        case "$entry" in
            "" | /* | ../* | */../* | .. | */..)
                echo "unsafe archive path in $archive: $entry" >&2
                exit 1
                ;;
            "$expected_root" | "$expected_root"/*)
                ;;
            *)
                echo "unexpected archive path in $archive: $entry" >&2
                exit 1
                ;;
        esac
    done
    tar -xzf "$archive" -C "$tmp_dir"
    cargo install --locked --path "$tmp_dir/$crate-$version"
}

install_verified_crate cargo-deny "$CARGO_DENY_VERSION" "$CARGO_DENY_SHA256"
install_verified_crate cargo-audit "$CARGO_AUDIT_VERSION" "$CARGO_AUDIT_SHA256"
