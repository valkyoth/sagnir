# Sagnir Causal Memory

Status: planning document

Sagnir should be self-describing, self-explaining, and evidence-aware. That
does not mean model output is trusted as truth. It means Sagnir records enough
bounded local evidence to answer why source state changed, what proved it, who
trusted it, and what depends on it.

## Core Rule

Evidence comes first. Optional AI summaries come second.

Sagnir may use AI to summarize selected facts, draft query plans, or make a
causal chain easier to read. AI must not create authoritative facts, approve
changes, override policy, hide missing evidence, promote worlds, or mark a
release safe.

## Data Layers

Canonical truth:

- immutable objects;
- committed WAL frames;
- admitted command events;
- canonical facts;
- signed policy, proof, and explanation objects.

Rebuildable projections:

- path indexes;
- symbol indexes;
- operation indexes;
- policy-decision indexes;
- causal graph indexes;
- search indexes;
- cached explanations;
- cached context packs.

If projections are deleted, Sagnir must be able to rebuild local memory from
canonical truth.

## Event To Fact Flow

Commands emit bounded operation events. Events describe what happened during
command execution, but they are not automatically authoritative facts.

The fact compiler admits stable facts from objects, policy decisions, proofs,
reviews, tests, and accepted events. Derived facts must keep source references
so Sagnir can explain how they were produced.

Example flow:

```text
operation event -> fact compiler -> canonical fact -> causal index -> explanation
```

## First-Class Questions

Sagnir should answer these local questions without hosted infrastructure:

```text
saga why <path|change|world|decision>
saga explain <change|decision|world|operation>
saga trace <change|world|fact|operation>
saga impact <key|dependency|change|fact|model>
saga context build --question "..."
saga ask "..."
```

The answer must separate known facts, inferred relationships, missing evidence,
and redactions.

## Explanation Objects

Explanations are objects so answers can be audited later.

An explanation records:

- the question;
- deterministic query plan;
- evidence edges;
- missing evidence;
- redaction notices;
- whether AI was used;
- confidence for derived analysis;
- creation metadata and signatures.

`saga explain explanation <id>` should show what facts, objects, policy
decisions, and redactions were used to produce an answer.

## Context Packs

Context packs are bounded evidence bundles for diagnostics and optional AI use.
They contain selected facts, object references, causal paths, snippets,
redaction notices, and missing-evidence markers.

Context packs must not silently include unrelated source, private facts,
protected metadata, secret material, or local keys.

## Honest Missing Knowledge

When Sagnir cannot prove an answer, the output should say so directly.

Example:

```text
Sagnir cannot prove why this path changed.

Known:
  revision rev_A91 modified src/payments/retry.rs
  world: draft/payment-retry

Missing:
  no intent fact
  no linked issue or incident
  no review fact
  no policy decision
```

This honesty is part of the security model. Unknown reasons must not be filled
with invented explanations.
