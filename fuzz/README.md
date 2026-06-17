# Sagnir Fuzz Targets

Status: active target

This directory contains Sagnir parser fuzz targets.

The v0.6.0 object-header target is:

```text
fuzz/fuzz_targets/object_header_parse.rs
```

Run it with:

```sh
cargo fuzz run object_header_parse
```

The fuzz package is intentionally separate from the stable Rust workspace so
normal release gates do not require nightly tooling. Parser release gates still
validate the target path and unit tests; fuzz campaigns are pentest and
hardening evidence until a dedicated fuzz CI budget is admitted.
