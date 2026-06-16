# Sagnir Release Runbook

Status: policy

Release flow:

1. Finish the version criteria from [Version Plan](VERSION_PLAN.md).
2. Run `scripts/checks.sh`.
3. Run `scripts/security_tool_gate.sh`.
4. Generate SBOM evidence with `scripts/generate-sbom.sh`.
5. Stop and request pentest for the exact commit.
6. Fix findings from root `PENTEST.md`.
7. Remove root `PENTEST.md`.
8. Record permanent pentest result under `security/pentest/`.
9. Update release notes.
10. Run the release gate for the target version.
11. Tag only when explicitly requested.
