//! Canonical limits shared across BLF parsers.


pub const MAX_SECTIONS: usize = 4096;
pub const MAX_CHANNELS: usize = 256;
pub const MAX_SAMPLE_BATCH: usize = 65536;
pub const MAX_EVENT_NOTE: usize = 4096;
pub const MAX_RUN_LABEL: usize = 256;
pub const MAX_CHANNEL_NAME: usize = 128;
pub const MAX_CHECKPOINTS: usize = 128;
pub const MAX_STITCH_OVERLAP_NS: u64 = 60_000_000_000;
pub const HEADER_SIZE: usize = 32;
pub const TRAILER_SIZE: usize = 16;

pub fn clamp_batch(count: usize) -> usize {
    count.min(MAX_SAMPLE_BATCH)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sane() {
        assert!(MAX_CHANNELS >= 8);
        assert_eq!(clamp_batch(999_999), MAX_SAMPLE_BATCH);
    }
}
