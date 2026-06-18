# Sagnir Command Design

Status: planning document

The CLI command is `saga`.

Core command set:

```text
saga init
saga status
saga diff
saga world
saga change
saga save
saga seal
saga prove
saga promote
saga undo
saga log
saga why
saga explain
saga trace
saga impact
saga context
saga ask
saga encrypt
saga unlock
saga lock
saga vault
saga bundle
saga sync
```

Developer model:

- a realm contains worlds;
- a world contains source state;
- a change modifies a world;
- a seal makes a change revision immutable;
- a proof says whether policy and evidence are satisfied;
- a promotion moves proven state between worlds;
- an event records bounded command behavior;
- a fact records stable admitted evidence;
- an explanation cites facts, objects, policy decisions, missing evidence, and
  redactions;
- a context pack carries bounded evidence for diagnostics and optional AI use;
- a vault protects encrypted realm storage and lock/unlock materialization;
- a sync shares objects, facts, and worlds.

## Workflow Layers

Sagnir exposes both high-level workflows and explicit primitives.

High-level commands are for daily local work:

```text
saga init
saga status
saga diff
saga save "message"
saga log
saga undo
saga why <path>
```

Explicit primitive commands remain available for users and environments that
need direct control:

```text
saga change begin
saga seal
saga prove
saga promote
saga evidence
saga review
```

`saga save "message"` is planned as a secure workflow command. It may create or
reuse local intent, build the source-state transition, seal an immutable
revision, record the operation, evaluate local proof/policy, and update the
current world only when the same transition is allowed by policy.

It must not be implemented as Git-style commit compatibility, and it must not
bypass configured requirements. In a `standard` or `solo` realm it can be a
short daily command. In a `team` or `regulated` realm it must produce clear
denial output when signatures, reviews, tests, or other evidence are missing.

Planned profiles:

- `standard`: strict integrity by default with simple workflow commands;
- `solo`: explicit opt-in for fewer evidence and review requirements;
- `team`: signatures, reviews, and protected worlds;
- `regulated`: strict signatures, evidence, audit, and promotion policy.

The early scaffold supports:

```bash
saga help
saga version
```

It also has stable command-line usage behavior:

- successful commands return exit code `0`;
- unknown commands return exit code `2`;
- unexpected extra arguments return exit code `2`;
- help and version output are pinned by golden tests.

Planned vault commands:

```text
saga encrypt project
saga unlock
saga lock
saga vault status
saga vault scan-leaks
saga vault recipient add
saga vault rekey
saga bundle create --encrypted
```

Planned memory and explanation commands:

```text
saga why src/storage/wal.rs
saga explain change <id>
saga explain decision <id>
saga explain world <name>
saga op explain last
saga trace change <id>
saga trace world <left>..<right>
saga impact change <id>
saga facts list
saga facts show <id>
saga context build --question "why did this change happen?"
saga ask "why did this change happen?"
```

`saga ask` is a bounded query layer over deterministic facts. It may summarize
selected evidence, but it must cite fact IDs, show missing evidence, and never
approve changes, override policy, or promote worlds.
