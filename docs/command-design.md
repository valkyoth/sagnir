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

The early scaffold supports:

```bash
saga help
saga version
```

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
