#!/usr/bin/env sh
set -eu

if [ -f PENTEST.md ]; then
    echo "root PENTEST.md is scratch input and must not be committed" >&2
    exit 1
fi

test -f CHANGELOG.md
test -f release-notes/RELEASE_NOTES_0.1.0.md
test -f SECURITY.md
test -f LICENSE

grep -q '1.96.0' rust-toolchain.toml
grep -q 'EUPL-1.2' Cargo.toml
grep -q 'Sagnir' README.md
