# Sagnir Release Runbook

Status: policy

Release flow:

1. Finish the version criteria from [Version Plan](VERSION_PLAN.md).
2. Run `scripts/checks.sh`.
3. Run `cargo deny check`.
4. Run `cargo audit`.
5. Generate SBOM evidence with `scripts/generate-sbom.sh`.
6. Stop and request pentest for the exact commit.
7. Fix findings from root `PENTEST.md`.
8. Remove root `PENTEST.md`.
9. Record permanent pentest result under `security/pentest/`.
10. Update release notes.
11. Tag only when explicitly requested.
