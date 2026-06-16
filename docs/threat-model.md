# Sagnir Threat Model

Status: baseline

Sagnir assumes hostile networks, malicious bundles, copied repositories,
compromised developer devices, stolen signing keys, poisoned dependencies,
corrupt object packs, replayed sync messages, stale remotes, malicious
automation, local plaintext leakage while unlocked, and partial local disk
exposure.

## Protected Assets

- source objects;
- world history;
- sealed revisions;
- local facts;
- policy epochs;
- crypto epochs;
- signatures;
- evidence references;
- local operation log;
- local event log;
- causal graph indexes;
- explanation objects;
- context packs;
- sync bundles;
- key metadata;
- encrypted realm keys;
- recipient slots;
- compartment metadata;
- encrypted bundle manifests.

## Initial Threats

- forged object type or digest confusion;
- malicious canonical encoding;
- WAL corruption;
- alias rollback;
- path traversal during worktree materialization;
- symlink traversal outside the worktree or into `.saga/` control data;
- `.saga/` control data accidentally tracked as source;
- forged facts or reviews;
- forged or replayed command events;
- event-to-fact compiler confusion;
- explanation output that hides missing evidence;
- context packs that leak unrelated source, facts, keys, or protected metadata;
- AI summaries being mistaken for authoritative evidence;
- oversized signature or fact payloads;
- unauthorized promotion;
- poisoned automation output;
- bundle replay;
- local key disclosure through logs or debug output.
- plaintext left in editor caches, build outputs, language-server indexes,
  shell history, backups, or OS search indexes;
- public object IDs leaking known plaintext membership in encrypted realms;
- recipient removal being mistaken for retroactive access revocation.

## Design Responses

- domain-separated object IDs;
- strict canonical decoding;
- append-only operation and fact logs;
- separation between command events and authoritative facts;
- deterministic fact compiler rules;
- auditable explanation objects;
- bounded context packs with redaction notices;
- AI output cannot create authority, override policy, or promote worlds;
- immutable objects;
- rebuildable indexes;
- deterministic promotion preflight;
- case-folded `.saga` control-path detection;
- symlink resolution required before filesystem I/O accepts tracked candidates;
- future filesystem APIs must use a verified worktree-path type so symlink
  boundary checks are enforced before source-state I/O;
- bounded signature envelopes;
- crypto-agile metadata;
- encrypted realm storage;
- private keyed object IDs for sealed private mode;
- lock/unlock materialization;
- recipient key wrapping;
- key epochs and rekey operations;
- plaintext leak scanner;
- local proof verification;
- causal impact traversal;
- quarantine state;
- no unsafe code in trusted crates.
