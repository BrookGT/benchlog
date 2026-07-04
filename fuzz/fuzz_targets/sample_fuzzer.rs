
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = benchlog_core::sample::decode::decode_block(data);
});
