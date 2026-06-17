# Sagnir Hash Migration Plan

Sagnir v0.7.0 admits `sha256` and `sha3-256` object hash algorithm metadata.
Object IDs carry the algorithm name so hash changes do not reuse the same
collision domain.

## Rules

- Unknown hash algorithm names fail closed at every parse boundary.
- A digest is admitted only when its byte length matches the selected algorithm.
- Object type, hash algorithm, and digest are all part of object ID equality.
- `sha3-256` is the quantum-horizon parallel-admission path for sensitive
  profiles that do not want new object IDs minted with SHA-256.
- New hash algorithms require a release note entry, object ID parse tests,
  collision-domain tests, and a completed pentest report.
- Hash migration must be additive first: new objects may use the new algorithm
  only after old objects remain verifiable with their original algorithm tag.
- Any future default switch must preserve read support for every tagged
  algorithm that was previously released.

## Migration Steps

1. Add a new `HashAlgorithm` variant and raw wire value.
2. Add its canonical lowercase text name and digest length.
3. Add parser tests for valid, invalid, truncated, and wrong-case IDs.
4. Add mixed-algorithm equality tests using identical digest bytes.
5. Add documentation for the new algorithm admission policy.
6. Run the full release gate and pentest the exact implementation-stop commit.

Sagnir must never infer a hash algorithm from digest length alone. The algorithm
tag is authoritative, and length is only an admission check for that tag.
