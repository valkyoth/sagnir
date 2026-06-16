# Sagnir Protocol

Status: planning document

The Sagnir protocol moves proof-carrying bundles.

Sync flow:

1. Client announces local heads, worlds, and fact roots.
2. Remote answers with missing objects and facts.
3. Client sends a bundle.
4. Remote verifies hashes, object types, signatures, and facts.
5. Remote evaluates policy.
6. Remote accepts, denies, quarantines, or asks for more evidence.
7. Client records the remote decision locally.

Local work must never require network access.
