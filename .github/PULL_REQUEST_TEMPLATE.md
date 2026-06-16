# Pull Request

## Summary

Describe what changed and why.

## Type

- [ ] Bug fix
- [ ] Feature
- [ ] Documentation
- [ ] Refactor
- [ ] Dependency update
- [ ] Security hardening

## Checklist

- [ ] I kept the change scoped to Sagnir's existing architecture.
- [ ] I updated docs, examples, or roadmap entries when behavior changed.
- [ ] I updated `CHANGELOG.md` for release-relevant changes.
- [ ] I added or updated tests for behavior changes.
- [ ] I ran `cargo fmt --all --check`.
- [ ] I ran `scripts/validate-release-metadata.sh` when version, toolchain, or release docs changed.
- [ ] I ran `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] I ran `cargo test --workspace`.
- [ ] I checked dependency and license impact when adding or updating crates.
- [ ] I did not commit secrets, private keys, tokens, local runtime data, or generated artifacts.

## Security Notes

Describe any security-sensitive impact. Mention canonical encoding, object IDs,
local store recovery, proofs, policy, sync bundles, worktree materialization, or
dependency changes if they are touched.

## Follow-Up

List any known remaining work or intentionally deferred tasks.
