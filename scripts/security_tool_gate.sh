#!/usr/bin/env sh
set -eu

export PATH="$HOME/.cargo/bin:$PATH"

cargo deny --version >/dev/null 2>&1 || {
    echo "cargo deny is required for release gates" >&2
    exit 2
}

cargo audit --version >/dev/null 2>&1 || {
    echo "cargo audit is required for release gates" >&2
    exit 2
}

cargo deny check
cargo audit
