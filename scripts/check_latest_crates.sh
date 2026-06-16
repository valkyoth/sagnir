#!/usr/bin/env sh
set -eu

command -v cargo >/dev/null 2>&1 || {
    echo "tooling freshness: cargo is required" >&2
    exit 2
}

command -v curl >/dev/null 2>&1 || {
    echo "tooling freshness: curl is required" >&2
    exit 2
}

rust_manifest_tmp="${TMPDIR:-/tmp}/sagnir-rust-stable.$$"
cargo_update_tmp="${TMPDIR:-/tmp}/sagnir-cargo-update-dry-run.$$"
trap 'rm -f "$rust_manifest_tmp" "$cargo_update_tmp"' EXIT INT TERM

first_match() {
    sed -n "$1" "$2" | sed -n '1p'
}

workspace_rust_version="$(
    first_match 's/^rust-version = "\([^"]*\)"/\1/p' Cargo.toml
)"
toolchain_version="$(
    first_match 's/^channel = "\([^"]*\)"/\1/p' rust-toolchain.toml
)"
container_rust_version="$(
    first_match 's/^ARG RUST_VERSION=\([^[:space:]]*\)$/\1/p' Containerfile
)"
cli_container_rust_version="$(
    first_match 's/^ARG RUST_VERSION=\([^[:space:]]*\)$/\1/p' containers/Containerfile.cli
)"

if [ -z "$workspace_rust_version" ]; then
    echo "tooling freshness: Cargo.toml rust-version is missing" >&2
    exit 1
fi

if [ "$toolchain_version" != "$workspace_rust_version" ]; then
    echo "tooling freshness: rust-toolchain.toml $toolchain_version does not match Cargo.toml rust-version $workspace_rust_version" >&2
    exit 1
fi

if [ "$container_rust_version" != "$workspace_rust_version" ]; then
    echo "tooling freshness: Containerfile Rust $container_rust_version does not match $workspace_rust_version" >&2
    exit 1
fi

if [ "$cli_container_rust_version" != "$workspace_rust_version" ]; then
    echo "tooling freshness: containers/Containerfile.cli Rust $cli_container_rust_version does not match $workspace_rust_version" >&2
    exit 1
fi

stable_manifest="${SAGNIR_RUST_STABLE_MANIFEST:-https://static.rust-lang.org/dist/channel-rust-stable.toml}"
curl -fsSL "$stable_manifest" >"$rust_manifest_tmp"
latest_rust="$(
    awk '
            /^\[pkg.rust\]$/ { in_rust = 1; next }
            /^\[/ && in_rust { exit }
            in_rust && /^version = / {
                version = $3
                gsub(/"/, "", version)
                print version
                exit
            }
        ' "$rust_manifest_tmp"
)"

if [ -z "$latest_rust" ]; then
    echo "tooling freshness: could not determine latest stable Rust" >&2
    exit 1
fi

if [ "$workspace_rust_version" != "$latest_rust" ]; then
    echo "tooling freshness: Rust $workspace_rust_version is not latest stable $latest_rust" >&2
    exit 1
fi

cargo update --workspace --dry-run >"$cargo_update_tmp" 2>&1

stale_crates="$(
    awk '/^[[:space:]]+Updating / { print }' "$cargo_update_tmp"
)"

if [ -n "$stale_crates" ]; then
    echo "tooling freshness: compatible Cargo updates are available:" >&2
    echo "$stale_crates" >&2
    echo "tooling freshness: run cargo update before release" >&2
    exit 1
fi

check_ci_cargo_tool() {
    crate="$1"
    pinned="$(
        sed -n "s/.*cargo install --locked $crate --version \\([0-9][0-9.]*\\).*/\\1/p" .github/workflows/ci.yml |
            sed -n '1p'
    )"
    latest="$(
        cargo info "$crate" |
            sed -n 's/^version: \([0-9][0-9.]*\).*/\1/p' |
            sed -n '1p'
    )"

    if [ -z "$pinned" ]; then
        echo "tooling freshness: .github/workflows/ci.yml does not pin $crate" >&2
        exit 1
    fi

    if [ -z "$latest" ]; then
        echo "tooling freshness: could not determine latest $crate version" >&2
        exit 1
    fi

    if [ "$pinned" != "$latest" ]; then
        echo "tooling freshness: CI pins $crate $pinned but latest is $latest" >&2
        exit 1
    fi
}

check_ci_cargo_tool cargo-deny
check_ci_cargo_tool cargo-audit

checkout_pin="$(
    sed -n 's/^[[:space:]]*uses: actions\/checkout@\([0-9a-f]\{40\}\)$/\1/p' .github/workflows/ci.yml |
        sed -n '1p'
)"
checkout_comment="$(
    sed -n 's/^[[:space:]]*# actions\/checkout \(v[0-9][^[:space:]]*\)$/\1/p' .github/workflows/ci.yml |
        sed -n '1p'
)"
checkout_release_json="$(
    curl -fsSL https://api.github.com/repos/actions/checkout/releases/latest
)"
checkout_latest_tag="$(
    printf '%s\n' "$checkout_release_json" |
        sed -n 's/.*"tag_name": "\(v[^"]*\)".*/\1/p' |
        sed -n '1p'
)"

if [ -z "$checkout_pin" ]; then
    echo "tooling freshness: actions/checkout must be pinned to a 40-character SHA" >&2
    exit 1
fi

if [ -z "$checkout_latest_tag" ]; then
    echo "tooling freshness: could not determine latest actions/checkout release" >&2
    exit 1
fi

if [ "$checkout_comment" != "$checkout_latest_tag" ]; then
    echo "tooling freshness: actions/checkout comment $checkout_comment does not match latest $checkout_latest_tag" >&2
    exit 1
fi

checkout_ref_json="$(
    curl -fsSL "https://api.github.com/repos/actions/checkout/git/ref/tags/$checkout_latest_tag"
)"
checkout_latest_sha="$(
    printf '%s\n' "$checkout_ref_json" |
        sed -n 's/.*"sha": "\([0-9a-f]\{40\}\)".*/\1/p' |
        sed -n '1p'
)"

if [ -z "$checkout_latest_sha" ]; then
    echo "tooling freshness: could not determine actions/checkout $checkout_latest_tag SHA" >&2
    exit 1
fi

if [ "$checkout_pin" != "$checkout_latest_sha" ]; then
    echo "tooling freshness: actions/checkout pin $checkout_pin does not match $checkout_latest_tag SHA $checkout_latest_sha" >&2
    exit 1
fi

echo "tooling freshness: ok"
