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

Format rules:

- field order is canonical;
- integer encoding is specified;
- lists are length-prefixed;
- duplicate fields are rejected;
- unknown critical fields are rejected;
- object body limits are checked before allocation;
- hashes and type tags are verified before indexing.
