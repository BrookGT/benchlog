
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = benchlog_core::event::markers::parse_events(data);
});
