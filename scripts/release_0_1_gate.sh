#!/usr/bin/env sh
set -eu

scripts/checks.sh
scripts/security_tool_gate.sh
test -f docs/IMPLEMENTATION_PLAN.md
test -f docs/VERSION_PLAN.md
test -f release-notes/RELEASE_NOTES_0.1.0.md
test -d security/pentest
scripts/validate-pentest-pass.sh v0.1.0
