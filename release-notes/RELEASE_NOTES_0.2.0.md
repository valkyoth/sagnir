# Sagnir 0.2.0 Release Notes

Status: released

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
- First pentest findings closed before tag: case-folded `.saga` control-path
  detection, constant-time equality APIs for sensitive identifiers, broader
  credential scanning, pinned CI action and security-tool versions, bounded
  bundle counts, validated obligation bitmasks, symlink policy documentation,
  CodeQL default setup documentation, and container build-context cleanup.
- Second pentest findings closed before tag: timing-helper documentation and
  `black_box` hardening, Windows trailing-dot canary documentation, and
  `scanner:allow` support for intentional non-secret placeholders.
