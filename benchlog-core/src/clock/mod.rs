//! Clock sync, timestamp correction, monotonic pairing.


pub mod sync;
pub mod correction;
pub mod pairing;
pub mod timeline;
pub mod resample;

pub use sync::{SyncPulse, parse_sync_block};
pub use correction::correct_timestamp;
pub use pairing::{MonoPair, pair_monotonic};
