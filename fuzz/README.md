# Sagnir Fuzz Targets

Status: scaffold

This directory reserves stable locations for parser fuzz targets.

The v0.6.0 object-header target is:

```text
fuzz/fuzz_targets/object_header_parse.rs
```

The target is intentionally dependency-free at this milestone. Before fuzzing
is wired into CI or release gates, admit the chosen fuzz harness and document
its toolchain, corpus storage, and crash minimization workflow.
