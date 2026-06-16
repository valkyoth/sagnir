# Sagnir Unsafe Policy

Status: policy

Trusted Sagnir crates forbid unsafe Rust.

Default crate policy:

```rust
#![forbid(unsafe_code)]
#![deny(unused_must_use)]
```

Core library crates use `#![no_std]` where practical.

If unsafe code ever becomes unavoidable:

- it must be isolated in a dedicated boundary crate;
- the boundary crate must have a documented admission decision;
- every unsafe block must have a `SAFETY:` comment;
- tests must cover the boundary behavior;
- release notes must call out the new unsafe boundary.

No unsafe boundary is admitted in the v0.1.0 scaffold.

## Sensitive Debug Policy

Types that hold key material, plaintext secrets, signatures, ciphertext keys,
recipient wrapping data, or decrypted object bytes must not derive `Debug`
unless the derived output is proven not to expose sensitive bytes.

Use manual `Debug` implementations that report lengths, algorithm identifiers,
and redacted markers instead of byte contents.
