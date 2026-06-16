# Sagnir 0.1.0 Release Notes

Status: planned

## Summary

Sagnir 0.1.0 establishes the repository foundation, Rust workspace, security
baseline, documentation baseline, and `saga` CLI scaffold.

## Verification

Required before tag:

```bash
scripts/checks.sh
cargo deny check
cargo audit
```

Tag stop:

```text
v0.1.0 implementation stop reached. Run pentest for this exact commit.
```

Pentest task:

- complete `security/pentest/v0.1.0.md`;
- record the exact commit under review;
- set `Status: PASS` only after findings are resolved;
- do not tag until tagging is explicitly requested.

## Security Notes

- No external Rust dependencies are required by the initial code scaffold.
- Trusted library crates forbid unsafe code.
- Release profile overflow checks are enabled.
- The first release is not a production-ready source-state engine.
