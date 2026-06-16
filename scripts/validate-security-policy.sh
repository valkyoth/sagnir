#!/usr/bin/env sh
set -eu

mode="${1:-check}"

if [ "$mode" != "check" ]; then
    echo "usage: scripts/validate-security-policy.sh check" >&2
    exit 2
fi

command -v rg >/dev/null 2>&1 || {
    echo "ripgrep (rg) is required for security policy validation" >&2
    exit 2
}

unsafe_patterns='unsafe\s*(fn|impl|trait|extern|\{)'
if rg "$unsafe_patterns" crates tools --glob '*.rs' >/dev/null 2>&1; then
    echo "unsafe Rust block or declaration found in trusted Sagnir crates" >&2
    exit 1
fi

if rg 'Sagaheim|Urdstack|Mimirroot|Nornvault|Wyrdgraph|Runeward' README.md SECURITY.md CHANGELOG.md ROADMAP.md docs .github release-notes >/dev/null; then
    echo "documentation contains non-Sagnir project wording" >&2
    exit 1
fi

hardcoded_patterns='(password|passphrase|api_key|secret_key)\s*=\s*"[^"]+'
if rg --multiline "$hardcoded_patterns" crates tools --glob '*.rs' >/dev/null 2>&1; then
    echo "possible hardcoded credential detected" >&2
    exit 1
fi
