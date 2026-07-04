//! Integer sample batch utilities.


use crate::error::{Error, Result};
use crate::sample::gaps::{GapReport, find_gaps};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntBatch {
    pub base_ts_ns: u64,
    pub values: Vec<i32>,
}

impl IntBatch {
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn min(&self) -> Option<i32> {
        self.values.iter().copied().min()
    }
    pub fn max(&self) -> Option<i32> {
        self.values.iter().copied().max()
    }
    pub fn mean(&self) -> Option<f64> {
        if self.values.is_empty() {
            return None;
        }
        let sum: i64 = self.values.iter().map(|v| *v as i64).sum();
        Some(sum as f64 / self.values.len() as f64)
    }
    pub fn validate_range(&self, lo: i32, hi: i32) -> Result<()> {
        for &v in &self.values {
            if v < lo || v > hi {
                return Err(Error::OutOfRange { what: "sample value" });
            }
        }
        Ok(())
    }
    pub fn detect_gaps(&self, step_ns: u64) -> GapReport {
        find_gaps(self.base_ts_ns, self.len(), step_ns)
    }
    pub fn slice(&self, start: usize, end: usize) -> Result<IntBatch> {
        if start > end || end > self.values.len() {
            return Err(Error::protocol("int batch slice"));
        }
        Ok(IntBatch {
            base_ts_ns: self.base_ts_ns.saturating_add(start as u64 * 1000),
            values: self.values[start..end].to_vec(),
        })
    }
}

pub fn merge_batches(a: &IntBatch, b: &IntBatch) -> IntBatch {
    let mut values = a.values.clone();
    values.extend_from_slice(&b.values);
    IntBatch { base_ts_ns: a.base_ts_ns, values }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stats() {
        let b = IntBatch { base_ts_ns: 0, values: vec![1, 2, 3] };
        assert_eq!(b.mean().unwrap(), 2.0);
    }
}
