# Sagnir 0.8.0 Release Notes

Status: implementation stop

## Summary

Sagnir v0.8.0 adds the in-memory object graph baseline before durable object
storage is introduced.

This release gives `sagnir-object` a fixed-capacity graph verifier for
pre-persistence object relationships. The graph stores object IDs, typed
references, missing-reference reports, an explicit acyclic policy, and traversal
checks without requiring allocation or a local database.

## Verification

Required local verification for this implementation stop:

```bash
cargo test -p sagnir-object
scripts/checks.sh
scripts/release_0_8_gate.sh
```

`scripts/release_0_8_gate.sh` must fail until
`security/pentest/v0.8.0.md` is completed with `Status: PASS`.

Tag stop:

```text
v0.8.0 implementation stop reached. Run pentest for this exact commit.
```

Pentest task:

- run the local gates for the exact commit;
- review fixed-capacity graph admission;
- review typed object reference construction;
- review missing-reference reports;
- review cycle policy behavior;
- review traversal behavior on complete and incomplete graphs;
- review blob leaf policy;
- write temporary findings to root `PENTEST.md`;
- fix or document every release-blocking finding;
- remove root `PENTEST.md`;
- update `security/pentest/v0.8.0.md` with `Status: PASS`, exact commit,
  tester, date, scope, and notes before tagging;
- wait for GitHub CI to go green before tagging;
- tag only after explicit maintainer instruction.

## Security Notes

- Object graph verification is pre-persistence only in v0.8.0.
- The graph is fixed-capacity and `no_std`; it does not allocate based on
  untrusted object data.
- References bind the expected target object kind and reject mismatched target
  IDs.
- Missing references report the exact unresolved typed reference.
- Cycles fail verification before durable storage can rely on graph
  relationships.
- Blob objects are leaves and cannot be used as reference sources.
