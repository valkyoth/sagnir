# Sagnir 0.8.0 Release Notes

Status: released

## Summary

Sagnir v0.8.0 adds the in-memory object graph baseline before durable object
storage is introduced.

This release gives `sagnir-object` a fixed-capacity graph verifier for
pre-persistence object relationships. The graph stores object IDs, typed
references, missing-reference reports, an explicit acyclic policy, and traversal
checks without requiring allocation or a local database.

The first v0.8.0 pentest findings are closed in this stop. Graph verification
and path checks now use iterative worklists instead of recursion, the public
graph capacity constants document the admitted budget, graph fuzz targets are
wired into the release gate, WAL CRC-32C metadata now binds frame kind and
transaction ID in addition to payload bytes, and crypto/object identity
documentation now calls out non-verification and hashing boundaries explicitly.
The second v0.8.0 pentest findings are also closed: defensive graph invariant
failures now report `InvalidEntry` instead of presenting impossible structural
breakage as a cycle.

## Verification

Required local verification for this implementation stop:

```bash
cargo test -p sagnir-object
scripts/checks.sh
cargo check --manifest-path fuzz/Cargo.toml --bins
scripts/release_0_8_gate.sh
```

`scripts/release_0_8_gate.sh` requires
`security/pentest/v0.8.0.md` to be completed with `Status: PASS` before
tagging.

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
- review iterative traversal and graph capacity constants;
- review graph fuzz target coverage;
- review WAL checksum metadata binding and the remaining MAC requirement;
- review that graph invariant guards report invalid entries, not false cycles;
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
- Production callers should stay at or below `OBJECT_GRAPH_ENTRIES_MAX` entries
  and `OBJECT_GRAPH_REFS_MAX` references unless a future release gate admits a
  larger budget.
- Graph verification and path checks are iterative to avoid recursive stack
  growth on hostile graph shapes.
- References bind the expected target object kind and reject mismatched target
  IDs.
- Missing references report the exact unresolved typed reference.
- Cycles fail verification before durable storage can rely on graph
  relationships.
- Blob objects are leaves and cannot be used as reference sources.
- WAL CRC-32C checks are crash-corruption detection only. v0.8.0 binds the CRC
  input to frame kind, transaction ID, and payload, but adversarial tamper
  detection still requires the planned keyed MAC.
- `HybridSignatureEnvelope::parse()` remains length admission only and is not a
  verifier.
