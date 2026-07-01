#!/usr/bin/env sh
set -eu

if [ -f PENTEST.md ]; then
    echo "root PENTEST.md is scratch input and must not be committed" >&2
    exit 1
fi

test -f CHANGELOG.md
test -f release-notes/RELEASE_NOTES_0.1.0.md
test -f release-notes/RELEASE_NOTES_0.2.0.md
test -f release-notes/RELEASE_NOTES_0.3.0.md
test -f release-notes/RELEASE_NOTES_0.4.0.md
test -f release-notes/RELEASE_NOTES_0.5.0.md
test -f release-notes/RELEASE_NOTES_0.6.0.md
test -f release-notes/RELEASE_NOTES_0.7.0.md
test -f release-notes/RELEASE_NOTES_0.8.0.md
test -f release-notes/RELEASE_NOTES_0.9.0.md
test -f SECURITY.md
test -f LICENSE
test -f security/pentest/v0.1.0.md
test -f security/pentest/v0.2.0.md
test -f security/pentest/v0.3.0.md
test -f security/pentest/v0.4.0.md
test -f security/pentest/v0.5.0.md
test -f security/pentest/v0.6.0.md
test -f security/pentest/v0.7.0.md
test -f security/pentest/v0.8.0.md
test -f security/pentest/v0.9.0.md

scripts/validate-release-notes.sh 0.1.0
scripts/validate-release-notes.sh 0.2.0
scripts/validate-release-notes.sh 0.3.0
scripts/validate-release-notes.sh 0.4.0
scripts/validate-release-notes.sh 0.5.0
scripts/validate-release-notes.sh 0.6.0
scripts/validate-release-notes.sh 0.7.0
scripts/validate-release-notes.sh 0.8.0
scripts/validate-release-notes.sh 0.9.0
scripts/validate-pentest-report.sh v0.1.0
scripts/validate-pentest-report.sh v0.2.0
scripts/validate-pentest-report.sh v0.3.0
scripts/validate-pentest-report.sh v0.4.0
scripts/validate-pentest-report.sh v0.5.0
scripts/validate-pentest-report.sh v0.6.0
scripts/validate-pentest-report.sh v0.7.0
scripts/validate-pentest-report.sh v0.8.0
scripts/validate-pentest-report.sh v0.9.0

grep -q '1.96.1' rust-toolchain.toml
grep -q 'EUPL-1.2' Cargo.toml
grep -q 'Sagnir' README.md

workspace_version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)

for manifest in crates/*/Cargo.toml tools/*/Cargo.toml; do
    grep -q '^version.workspace = true$' "$manifest" || {
        echo "$manifest must inherit workspace package version $workspace_version" >&2
        exit 1
    }
done

if grep -R 'path = "../sagnir' crates tools --include Cargo.toml |
    grep -v "version = \"$workspace_version\"" >/dev/null 2>&1; then
    echo "internal path dependencies must include explicit version = \"$workspace_version\"" >&2
    exit 1
fi
