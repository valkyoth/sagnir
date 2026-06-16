#!/usr/bin/env sh
set -eu

mode="${1:-check}"

if [ "$mode" != "check" ]; then
    echo "usage: scripts/validate-modularity-policy.sh check" >&2
    exit 2
fi

find crates tools -type f -name '*.rs' | while IFS= read -r file; do
    lines="$(wc -l < "$file")"
    if [ "$lines" -gt 500 ]; then
        echo "$file exceeds 500 lines; split it or document an exception" >&2
        exit 1
    fi
done
