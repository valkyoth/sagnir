#!/usr/bin/env sh
set -eu

version="${1:?usage: scripts/validate-release-notes.sh <version>}"
file="release-notes/RELEASE_NOTES_$version.md"

if [ ! -f "$file" ]; then
    echo "missing release notes: $file" >&2
    exit 1
fi

grep -q "^# Sagnir $version Release Notes$" "$file" || {
    echo "$file must start with the Sagnir $version release-note heading" >&2
    exit 1
}

grep -Eq '^Status: (planned|implementation stop|released)$' "$file" || {
    echo "$file must have Status: planned, implementation stop, or released" >&2
    exit 1
}

for heading in '## Summary' '## Verification' '## Security Notes'; do
    grep -q "^$heading$" "$file" || {
        echo "$file missing heading: $heading" >&2
        exit 1
    }
done

grep -q 'Run pentest for this exact commit' "$file" || {
    echo "$file must include the pentest stop text" >&2
    exit 1
}
