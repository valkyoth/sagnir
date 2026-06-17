#![no_main]

// Scaffold target for future cargo-fuzz/libFuzzer admission.
//
// Intended harness body:
//
// fuzz_target!(|data: &[u8]| {
//     let _ = sagnir_object::parse_object_header(data);
// });
