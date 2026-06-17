# Sagnir 0.7.0 Release Notes

Status: implementation stop

## Summary

Sagnir v0.7.0 makes object identity type-separated, algorithm-agile, and
round-trippable as text before durable object storage is introduced.

This release adds a canonical object ID display and parse format:

```text
sagnir-object-v1:<object-type>:<hash-algorithm>:<lowercase-hex-digest>
```

Parsing fails closed for unknown prefixes, object types, hash algorithms,
wrong digest lengths, uppercase hex, non-hex input, and malformed field counts.
Digest slice admission also checks the selected algorithm's expected digest
length before an `ObjectId` can be built from external bytes.

Object ID equality remains type-separated and uses the admitted `subtle`-backed
constant-time byte comparison path. New collision-domain tests prove equal raw
digests cannot confuse blob, tree, state root, change, change revision, world,
fact, operation, or bundle identities.

The implementation stop also folds in the first v0.7.0 pentest findings:
verified proof reports now require an opaque verification token, world/change
and state-root references use typed ID wrappers, parsed object headers expose
named `header`, `body`, and `rest` fields, and the hash migration path is
documented before any second hash algorithm is admitted.

The first v0.7.0 pentest pass also tightened constant-time comparison
composition, sanitized signature envelope invariants, hybrid signature component
framing, worktree path proof types, policy obligation checks, WAL frame checksum
metadata, redacted object ID display, parser fuzz coverage, and supply-chain
tooling notes.

## Verification

Required local verification for this implementation stop:

```bash
cargo test -p sagnir-object
scripts/checks.sh
scripts/release_0_7_gate.sh
```

`scripts/release_0_7_gate.sh` must fail until
`security/pentest/v0.7.0.md` is completed with `Status: PASS`.

Tag stop:

```text
v0.7.0 implementation stop reached. Run pentest for this exact commit.
```

Pentest task:

- run the local gates for the exact commit;
- review object ID display and parse behavior;
- review object type and hash algorithm name admission;
- review digest length checks;
- review lowercase hex rejection policy for malformed IDs;
- review collision-domain tests across object kinds;
- review timing-safe object ID equality strategy;
- write temporary findings to root `PENTEST.md`;
- fix or document every release-blocking finding;
- remove root `PENTEST.md`;
- update `security/pentest/v0.7.0.md` with `Status: PASS`, exact commit,
  tester, date, scope, and notes before tagging;
- wait for GitHub CI to go green before tagging;
- tag only after explicit maintainer instruction.

## Security Notes

- Object ID parsing is exact and fail-closed.
- Object ID text uses lowercase hex only to avoid mixed canonical forms.
- Digest slice admission checks the selected hash algorithm length before
  construction.
- Object type is part of equality, display, parsing, and hashing.
- Equal raw digests in different object domains are distinct object IDs.
- Object ID equality uses the admitted `subtle`-backed comparison path.
- `ProofStatus::Verified` cannot be admitted through generic report
  construction.
- Security tool archive extraction rejects absolute names and directory
  overwrite behavior.
- Composed equality paths keep `subtle::Choice` until the final boolean
  boundary.
- `OwnedSignature::as_envelope` rejects sanitized signatures.
- `HybridSignatureEnvelope` separates classical and post-quantum components.
- `HybridSignatureEnvelope` debug output redacts component bytes and is not
  implicitly `Copy`.
- `ObjectId::redacted()` hides plaintext digests for sealed private contexts.
- WAL frame metadata includes CRC-32c integrity scaffolding.
- Empty-payload WAL structural frames are constructable and verifiable.
- `scripts/validate-pentest-pass.sh` strictly requires `Status: PASS`, exact
  commit, tester, scope, and date before tagging.
