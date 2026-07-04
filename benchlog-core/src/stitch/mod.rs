//! Merge overlapping instrument capture runs.


pub mod merge;
pub mod overlap;
pub mod blend;

pub use merge::{stitch_runs, RunSlice};
pub use overlap::{OverlapRegion, find_overlap};
