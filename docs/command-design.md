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
- a sync shares objects, facts, and worlds.

The early scaffold supports:

```bash
saga help
saga version
```
