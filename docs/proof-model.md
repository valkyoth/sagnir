# Sagnir Proof Model

Status: planning document

A Sagnir proof explains whether a change, world, bundle, or release satisfies
integrity, evidence, signature, and policy requirements.

Initial proof checks:

- object graph integrity;
- object type separation;
- format version;
- policy epoch;
- crypto epoch;
- required facts;
- signature envelopes;
- causal links.

Proof output must be deterministic and explain missing requirements without
leaking sensitive internal details.
