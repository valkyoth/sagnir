#!/usr/bin/env sh
set -eu

CARGO_DENY_VERSION=0.20.2
CARGO_DENY_SHA256=e528dfcbe739af7ce37a77d3d6df1b29dd6887b1c701d888820c0f16b864f737
CARGO_AUDIT_VERSION=0.22.2
CARGO_AUDIT_SHA256=700c2b240f7fd330c24b675fe429f73a5b676531fcc6300400b2b67f155ba12a

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
