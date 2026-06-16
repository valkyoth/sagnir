#!/usr/bin/env sh
set -eu

mode="${1:-check}"

if [ "$mode" != "check" ]; then
    echo "usage: scripts/validate-security-policy.sh check" >&2
    exit 2
fi

if rg '\bunsafe\b' crates tools --glob '*.rs' | rg -v 'forbid\(unsafe_code\)' >/dev/null; then
    echo "unsafe Rust is not admitted in trusted Sagnir crates" >&2
    exit 1
fi

if rg 'Sagaheim|Urdstack|Mimirroot|Nornvault|Wyrdgraph|Runeward' README.md SECURITY.md CHANGELOG.md ROADMAP.md docs .github release-notes >/dev/null; then
    echo "documentation contains non-Sagnir project wording" >&2
    exit 1
fi

if rg 'private key|token|secret' crates --glob '*.rs' >/dev/null; then
    echo "review possible secret-handling code before admitting it" >&2
    exit 1
fi
