#!/usr/bin/env sh
set -eu

scripts/checks.sh
test -f docs/IMPLEMENTATION_PLAN.md
test -f docs/VERSION_PLAN.md
test -f release-notes/RELEASE_NOTES_0.1.0.md
test -d security/pentest
