# Sagnir 0.10.1 Release Notes

Status: implementation stop

## Summary

`saga init`, local `sagad init`, and library callers share
`sagnir_store_fs::initialize`. Existing format and metadata bytes are
compatible with v0.10.0. Unix initialization now syncs the retained parent
directory after syncing the completed store, including when retrying.

## Verification

Run `scripts/release_0_10_1_gate.sh pentest-handoff` for local checks and
the rootless CLI smoke test. The Windows CI job exercises shared refusal
through both frontends. The final gate requires the maintainer's PASS report.

## Security Notes

Run pentest for this exact commit. Cover directory attachment, lock contention
between frontends, bounded metadata reads, stale temporary files, retry after
parent sync failure, and refusal before writing on unsupported platforms.
The daemon command operates only on the local working directory. Object and
WAL persistence are outside this release.
