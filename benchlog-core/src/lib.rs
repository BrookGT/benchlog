//! benchlog-core — BLF (Bench Log Format) parsing and validation.


#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(missing_debug_implementations)]

extern crate alloc;

pub mod error;
pub mod limits;
pub mod mem;
pub mod frame;
pub mod channel;
pub mod sample;
pub mod event;
pub mod clock;
pub mod codec;
pub mod stream;
pub mod stitch;
pub mod export;
pub mod check;
pub mod ingest;

pub use error::{Error, Result};
pub use ingest::{CaptureDoc, IngestReport, ingest, parse_capture, write_capture};

pub mod pipeline {
    use super::*;
    pub fn run(wire: &[u8]) -> Result<IngestReport> {
        ingest(wire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn limits_sane() {
        assert!(limits::MAX_CHANNELS >= 8);
    }
}
