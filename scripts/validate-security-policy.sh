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

if rg -U '#\[derive\([^\]]*Copy[^\]]*\)\][[:space:]]*(pub[[:space:]]+)?struct[[:alnum:]_]*(Secret|Key|Passphrase|Token|Vault|Credential|Master|Material|Derived|Wrapped|Session|Epoch)' crates tools --glob '*.rs' >/dev/null 2>&1; then
    echo "secret or key material structs must not derive Copy" >&2
    exit 1
fi

if rg 'Sagaheim|Urdstack|Mimirroot|Nornvault|Wyrdgraph|Runeward' README.md SECURITY.md CHANGELOG.md ROADMAP.md docs .github release-notes >/dev/null; then
    echo "documentation contains non-Sagnir project wording" >&2
    exit 1
fi

if rg 'scanner:allow' crates tools scripts .github README.md SECURITY.md CHANGELOG.md ROADMAP.md Cargo.toml deny.toml rust-toolchain.toml Containerfile containers --glob '!scripts/validate-security-policy.sh' >/dev/null 2>&1; then
    echo "scanner:allow is only permitted in docs, release notes, or reviewed fixtures" >&2
    exit 1
fi

hardcoded_patterns='(password|passphrase|api_key|secret_key|private_key|signing_key|master_key|realm_key|encryption_key|auth_token|access_token|refresh_token|id_token|jwt_token|hmac_key|aes_key|rsa_key|dsa_key|ecdsa_key|wrap_key|kek|token|secret|bearer|private|credential)\s*[=:]\s*"[^"]+'
credential_paths='crates tools scripts docs .github release-notes README.md SECURITY.md CHANGELOG.md ROADMAP.md Cargo.toml deny.toml rust-toolchain.toml Containerfile containers'
if rg --multiline --glob '*.rs' --glob '*.sh' --glob '*.toml' --glob '*.md' --glob '*.yml' --glob '*.yaml' --glob 'Containerfile*' "$hardcoded_patterns" $credential_paths | grep -v 'scanner:allow' >/dev/null 2>&1; then
    echo "possible hardcoded credential detected" >&2
    exit 1
fi

if rg --multiline --glob '*.rs' --glob '*.sh' --glob '*.toml' --glob '*.md' --glob '*.yml' --glob '*.yaml' --glob 'Containerfile*' -- '-----BEGIN [A-Z ]+PRIVATE KEY-----|eyJ[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{20,}' $credential_paths | grep -v 'scanner:allow' >/dev/null 2>&1; then
    echo "possible PEM private key or JWT literal detected" >&2
    exit 1
fi

if rg --multiline 'Authorization:\s*Bearer\s+[A-Za-z0-9._~+/=-]+' scripts docs .github --glob '*.sh' --glob '*.md' --glob '*.yml' --glob '*.yaml' | grep -v 'scanner:allow' >/dev/null 2>&1; then
    echo "possible hardcoded bearer token detected" >&2
    exit 1
fi

if rg -n '^[[:space:]]*uses: [^[:space:]]+@' .github/workflows --glob '*.yml' --glob '*.yaml' |
    grep -vE '@[0-9a-f]{40}([[:space:]]*(#.*)?)?$' >/dev/null 2>&1; then
    echo "GitHub Actions must be pinned to immutable 40-character SHAs" >&2
    exit 1
fi

if [ -f Cargo.lock ]; then
    crypto_crate_pattern='name = "(aes-gcm|argon2|chacha20poly1305|ed25519-dalek|hmac|hkdf|ml-dsa|pbkdf2|password-hash|ring|scrypt|sha2)"'
    if rg "$crypto_crate_pattern" Cargo.lock >/dev/null 2>&1; then
        rg 'name = "subtle"' Cargo.lock >/dev/null 2>&1 || {
            echo "crypto provider crates require subtle admission before release" >&2
            exit 1
        }
        rg 'name = "zeroize"' Cargo.lock >/dev/null 2>&1 || {
            echo "crypto provider crates require zeroize admission before release" >&2
            exit 1
        }
    fi
fi
