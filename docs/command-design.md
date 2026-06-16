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
saga impact
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
