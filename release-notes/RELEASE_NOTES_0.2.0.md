# Sagnir 0.2.0 Release Notes

Status: implementation stop

## Summary

Sagnir 0.2.0 establishes the reusable release metadata baseline for future
version stops. It adds release-note validation, pentest-report validation, a
dedicated v0.2.0 release gate, and a permanent pentest placeholder for the next
tag decision.

## Verification

Required before tag:

```bash
scripts/checks.sh
scripts/release_0_2_gate.sh
```

Tag stop:

```text
v0.2.0 implementation stop reached. Run pentest for this exact commit.
```

Pentest task:

- complete `security/pentest/v0.2.0.md`;
- record the exact commit under review;
- set `Status: PASS` only after findings are resolved;
- do not tag until tagging is explicitly requested.

## Security Notes

- Root `PENTEST.md` remains scratch input and is rejected by the local gate.
- `scripts/checks.sh` runs `cargo deny check` and `cargo audit` through the
  security tool gate.
- `scripts/release_0_2_gate.sh` refuses to pass until the v0.2.0 pentest report
  is marked `Status: PASS`.
