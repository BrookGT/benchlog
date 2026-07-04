//! Detect temporal overlap between two runs.


use crate::error::{Error, Result};
use crate::limits::MAX_STITCH_OVERLAP_NS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverlapRegion {
    pub a_start_ns: u64,
    pub a_end_ns: u64,
    pub b_start_ns: u64,
    pub b_end_ns: u64,
    pub overlap_ns: u64,
}

pub fn find_overlap(a_start: u64, a_end: u64, b_start: u64, b_end: u64) -> Result<Option<OverlapRegion>> {
    if a_end < a_start || b_end < b_start {
        return Err(Error::protocol("invalid run span"));
    }
    let start = a_start.max(b_start);
    let end = a_end.min(b_end);
    if start >= end {
        return Ok(None);
    }
    let overlap_ns = end - start;
    if overlap_ns > MAX_STITCH_OVERLAP_NS {
        return Err(Error::CapacityLimit);
    }
    Ok(Some(OverlapRegion {
        a_start_ns: a_start,
        a_end_ns: a_end,
        b_start_ns: b_start,
        b_end_ns: b_end,
        overlap_ns,
    }))
}

pub fn merge_weight(overlap: &OverlapRegion, pos_ns: u64) -> f64 {
    if overlap.overlap_ns == 0 {
        return 0.5;
    }
    let t = (pos_ns.saturating_sub(overlap.a_start_ns)) as f64 / overlap.overlap_ns as f64;
    t.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn disjoint() {
        assert!(find_overlap(0, 10, 20, 30).unwrap().is_none());
    }
}
