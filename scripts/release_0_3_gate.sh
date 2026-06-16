#!/usr/bin/env sh
set -eu

scripts/checks.sh
scripts/security_tool_gate.sh
scripts/validate-release-notes.sh 0.3.0
scripts/validate-pentest-report.sh v0.3.0
scripts/validate-pentest-pass.sh v0.3.0
