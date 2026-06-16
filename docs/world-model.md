# Sagnir World Model

Status: planning document

A world is first-class source state with policy and proof context.

World kinds:

- local;
- draft;
- review;
- staging;
- production;
- audit;
- simulation;
- agent.

A world is not just a mutable branch pointer. A world points to signed state and
carries policy, accepted changes, quarantined changes, parent links, and
promotion requirements.

Promotion moves proven state between worlds. Promotion preflight must expose
deterministic categories before mutating aliases.
