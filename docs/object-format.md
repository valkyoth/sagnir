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
- integer encoding is specified;
- lists are length-prefixed;
- duplicate fields are rejected;
- unknown critical fields are rejected;
- object body limits are checked before allocation;
- hashes and type tags are verified before indexing.

Future decode rule: the first decoder matching `write_len_prefixed` must expose
a bounded read API that validates the declared length against a caller-provided
maximum before allocation. Malicious object and bundle input must never be able
to request allocation from an unchecked length prefix.

In sealed private mode, public plaintext hashes must not be used as externally
visible storage identifiers.
