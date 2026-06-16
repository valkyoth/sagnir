#!/usr/bin/env sh
set -eu

mode="${1:-check}"

if [ "$mode" != "check" ]; then
    echo "usage: scripts/validate-modularity-policy.sh check" >&2
    exit 2
fi

if [ ! -s docs/modularity-policy.md ]; then
    echo "modularity policy: docs/modularity-policy.md missing" >&2
    exit 1
fi

if [ -d src ]; then
    echo "modularity policy: root src/ is not allowed; use focused crates/" >&2
    exit 1
fi

find crates tools tests -type f -name '*.rs' 2>/dev/null | while IFS= read -r file; do
    case "$file" in
        */target/* | */generated/* | */vendor/*)
            continue
            ;;
    esac

    lines="$(wc -l < "$file" | tr -d ' ')"
    if [ "$lines" -gt 500 ]; then
        echo "modularity policy: $file has $lines lines; split before release" >&2
        exit 1
    fi
done

if [ -f Cargo.toml ] && ! grep -q '^resolver = "3"$' Cargo.toml; then
    echo "modularity policy: workspace resolver must be 3 for edition 2024 workspaces" >&2
    exit 1
fi

echo "modularity policy: ok"
