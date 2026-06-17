# Sagnir Object Format

Status: planning document

Sagnir objects use domain-separated identity.

Object identity must not be:

```text
hash(bytes)
```

Object identity must be:

```text
hash("sagnir.object.v1" || object_type || canonical_bytes)
```

The initial admitted hash algorithm has a 32-byte digest. New hash algorithms
must be admitted through explicit parser support and must not silently truncate
or pad digest bytes to fit an existing object ID shape.

Required object kinds:

- blob;
- tree;
- state root;
- change;
- change revision;
- world;
- fact;
- operation;
- bundle.

Encrypted realms add encrypted object envelopes:

- public storage ID over ciphertext;
- private keyed object ID over canonical plaintext;
- object type in authenticated associated data;
- realm ID in authenticated associated data;
- crypto epoch in authenticated associated data;
- nonce and AEAD algorithm metadata;
- bounded ciphertext length;
- authentication tag.

Format rules:

- field order is canonical;
- integer encoding is fixed-width little-endian through `sagnir-codec`;
- byte strings are encoded as `u64` length followed by exact payload bytes;
- lists use bounded `u64` item-count encoding before element bytes;
- duplicate fields are rejected;
- unknown critical fields are rejected;
- object body limits are checked before allocation;
- hashes and type tags are verified before indexing.

Object header v1 is fixed-width:

- magic: `SAGNOBJ\0`;
- object type: `u16`;
- format version: `u16`;
- body length: `u64`;
- flags: `u32`.

No flags are admitted in v0.6.0. Unknown flags fail closed and are treated as
critical until explicitly admitted.

Zero-length bodies are admitted only for blob objects in v0.6.0. Structured
object types must carry a body until their canonical empty encodings are
explicitly admitted.

Object header parsing validates that the input slice contains at least the
declared body length before returning post-header bytes. Callers may still
split body bytes from trailing transport data according to their own framing,
but they must not accept a header whose declared body is unavailable.

Decode rule: length-prefixed decoding must use `read_len_prefixed` or an
equivalent bounded API that validates the declared length against a
caller-provided maximum before returning a payload slice. Malicious object and
bundle input must never be able to request allocation from an unchecked length
prefix.

In sealed private mode, public plaintext hashes must not be used as externally
visible storage identifiers.
