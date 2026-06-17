#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = sagnir_codec::read_byte_string(data, 4096);
});
