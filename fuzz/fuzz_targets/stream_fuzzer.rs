
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = benchlog_core::stream::incremental::drive_stream(data);
});
