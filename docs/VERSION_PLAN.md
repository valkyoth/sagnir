# Sagnir Version Plan

Status: planning document

Tags use:

```text
v0.N.0      milestone release
v0.N.P      patch or fix release
v1.0.0      first serious production-ready CLI
```

The list below is not a maximum. Add versions whenever smaller release stops
make review and testing clearer.

## Release Principles

Every release must have:

- definition of done;
- a clean stop point before tag work;
- local verification command;
- rootless Podman status;
- security review notes;
- pentest handoff and resolution pass;
- known limitations;
- release notes;
- no hidden dependency on one developer machine.

## Clean Stop And Pentest Rule

Each version has a deliberate clean stop. When implementation criteria are done,
work stops before tagging and the maintainer is told:

```text
vX.Y.Z implementation stop reached. Run pentest for this exact commit.
```

No tag is created at that point.

Pentest flow:

1. Implementation reaches the version stop point.
2. Local gates pass: `scripts/checks.sh`, `cargo deny check`, and `cargo audit`.
3. The maintainer runs pentest and writes findings to root `PENTEST.md`.
4. Findings are reviewed and fixed.
5. `PENTEST.md` is removed after findings are handled.
6. Local gates are run again.
7. A permanent report is written at `security/pentest/<tag>.md` only when the
   exact commit is ready to tag and the result is `Status: PASS`.
8. Tagging and pushing tags happen only when explicitly requested.

Root `PENTEST.md` is temporary scratch input. It must not be committed, and the
release metadata validator fails while it exists.

## Mandatory Per-Version Stop

Every version section below inherits this tag stop:

```text
vX.Y.Z implementation stop reached. Run pentest for this exact commit.
```

Every version also inherits this pentest task:

- run the local gates for the exact commit;
- review security-sensitive changes in scope;
- write temporary findings to root `PENTEST.md`;
- fix or document every release-blocking finding;
- remove root `PENTEST.md`;
- create or update `security/pentest/<tag>.md` with `Status: PASS`, exact
  `Commit:`, non-blank `Tester:`, non-blank `Scope:`, and `Date: YYYY-MM-DD`;
- tag only after the maintainer explicitly requests tagging.

## v0.1.0 - Repository Foundation

Goal: initialize the serious Rust workspace and policy baseline.

Deliverables:

- Rust stable `1.96.0` pinned.
- Focused workspace crates.
- `saga` CLI scaffold.
- `scripts/checks.sh`.
- CI, dependency policy, security policy, release notes.
- Implementation, version, modularity, threat-model, toolchain, and security
  control docs.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_1_gate.sh`
- `cargo test --workspace`

## v0.2.0 - Canonical IDs And Encoding

Goal: make canonical object identity and encoding rules testable.

Deliverables:

- domain-separated object IDs;
- object type registry;
- format version checks;
- canonical integer and byte encoding;
- malformed encoding tests.

## v0.3.0 - Local Store Layout

Goal: make `.saga/` initialization and recovery scaffolds real.

Deliverables:

- `saga init`;
- `.saga/FORMAT`;
- config and realm files;
- required store directories;
- WAL frame model;
- `saga fsck`;
- interrupted-init tests.

## v0.4.0 - Worktree Scanner And Status

Goal: compare a worktree against an empty or initialized Sagnir state.

Deliverables:

- portable path scanner;
- control-path exclusion;
- ignore rule scaffold;
- blob hashing boundary;
- tree builder scaffold;
- `saga status`;
- tests for Linux, Windows-style separators, BSD, MacOS, Android, and iOS path
  assumptions.

## v0.5.0 - Basic Diff

Goal: produce deterministic path and text diffs.

Deliverables:

- path-level diff;
- text diff;
- binary detection scaffold;
- deterministic ordering;
- `saga diff`;
- tests for rename-neutral add, modify, delete, and binary cases.

## v0.6.0 - Worlds

Goal: model worlds as first-class source states.

Deliverables:

- world metadata;
- world aliases;
- `saga world open`;
- `saga world switch`;
- dirty-worktree protection;
- world list;
- tests for world isolation.

## v0.7.0 - Changes And Seal

Goal: make source changes immutable and inspectable.

Deliverables:

- change metadata;
- sealed revision metadata;
- state root metadata;
- `saga change begin`;
- `saga seal`;
- `saga change amend`;
- `saga log`;
- tests for sealed revision immutability.

## v0.8.0 - Operation Ledger And Undo

Goal: make local operations recoverable and undoable.

Deliverables:

- operation objects;
- committed operation log;
- operation replay;
- `saga undo`;
- startup recovery tests;
- interrupted-operation tests.

## v0.9.0 - Proof Skeleton

Goal: verify integrity and explain missing evidence.

Deliverables:

- object graph verification;
- proof report type;
- policy epoch metadata;
- `saga prove`;
- tests for missing object, wrong type, wrong format, and missing evidence.

## v0.10.0 - Local Policy And Promotion

Goal: promote state between worlds through explicit policy decisions.

Deliverables:

- local policy file;
- promotion preflight;
- deterministic conflict categories;
- `saga promote`;
- denial output with required facts;
- tests for allowed, denied, conflict, and missing-proof paths.

## v0.11.0 - Identity And Signature Metadata

Goal: make local signing metadata explicit.

Deliverables:

- actor identity;
- key registry;
- signature envelope;
- signature-set bounds;
- crypto epoch metadata;
- `saga actor init`;
- `saga sign`;
- `saga verify`;
- tests for oversized signatures and unknown algorithms.

## v0.12.0 - Local Facts

Goal: record evidence as local facts.

Deliverables:

- fact envelope;
- fact log;
- test facts;
- review facts;
- promotion facts;
- causal links;
- `saga test record`;
- `saga review approve`;
- tests for invalid fact bounds and missing evidence.

## v0.13.0 - Why And Impact

Goal: answer local provenance and blast-radius questions.

Deliverables:

- causal graph traversal;
- taint facts;
- quarantine facts;
- `saga why`;
- `saga impact`;
- tests for key, dependency, change, and fact taint propagation.

## v0.14.0 - Bundle Format

Goal: move Sagnir state offline as proof-carrying bundles.

Deliverables:

- bundle manifest;
- object pack;
- fact range;
- policy references;
- signature footer;
- `saga bundle create`;
- `saga bundle verify`;
- `saga bundle import`;
- malicious bundle tests.

## v0.15.0 - Minimal Sync Protocol

Goal: sync bundles with a minimal Sagnir remote.

Deliverables:

- remote head exchange;
- missing object negotiation;
- remote verification result;
- accepted, denied, and quarantined responses;
- `saga sync`;
- protocol tests.

## v0.16.0 - Minimal Daemon

Goal: provide optional local and remote daemon support.

Deliverables:

- `sagad serve`;
- remote object store;
- remote fact store;
- policy-light acceptance;
- rootless Podman smoke test;
- daemon shutdown and restart tests.

## v0.17.0 - Parser Fuzz And Malicious Corpus

Goal: harden untrusted bytes before 1.0.

Deliverables:

- canonical codec corpus;
- object corpus;
- pack corpus;
- bundle corpus;
- fuzz target scaffolds;
- crash-free parser tests.

## v0.18.0 - Release Evidence

Goal: make release outputs auditable.

Deliverables:

- SBOM generation;
- reproducible local release build check;
- release notes validator;
- signed tag checklist;
- release runbook.

## v0.19.0 - Cross-Platform Gate

Goal: keep Sagnir portable from day one.

Deliverables:

- Linux check;
- Windows check;
- BSD check instructions;
- MacOS check;
- Android check scaffold;
- iOS check scaffold;
- documented unsupported behavior.

## v1.0.0 - Production-Ready CLI

Goal: first serious production-ready `saga` CLI.

Deliverables:

- local realm initialization;
- world and change workflow;
- seal and amend;
- status and diff;
- proofs and promotion;
- operation undo;
- local facts;
- why and impact;
- bundles;
- minimal sync;
- optional daemon;
- complete release notes;
- completed pentest report;
- passing release gate.
