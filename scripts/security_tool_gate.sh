#!/usr/bin/env sh
set -eu

find_tool() {
    tool="$1"
    if command -v "$tool" >/dev/null 2>&1; then
        command -v "$tool"
        return 0
    fi
    if [ -x "$HOME/.cargo/bin/$tool" ]; then
        printf '%s\n' "$HOME/.cargo/bin/$tool"
        return 0
    fi
    return 1
}

cargo_deny="$(find_tool cargo-deny)" || {
    echo "cargo-deny is required for release gates" >&2
    exit 2
}

cargo_audit="$(find_tool cargo-audit)" || {
    echo "cargo-audit is required for release gates" >&2
    exit 2
}

"$cargo_deny" check
"$cargo_audit" audit
