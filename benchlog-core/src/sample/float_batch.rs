//! Float sample batch utilities.


use crate::error::{Error, Result};
use crate::sample::gaps::{GapReport, find_gaps};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct FloatBatch {
    pub base_ts_ns: u64,
    pub values: Vec<f32>,
}

impl FloatBatch {
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn min(&self) -> Option<f32> {
        self.values.iter().copied().fold(None, |acc, v| {
            Some(match acc {
                None => v,
                Some(m) => if v < m { v } else { m },
            })
        })
    }
    pub fn max(&self) -> Option<f32> {
        self.values.iter().copied().fold(None, |acc, v| {
            Some(match acc {
                None => v,
                Some(m) => if v > m { v } else { m },
            })
        })
    }
    pub fn rms(&self) -> Option<f64> {
        if self.values.is_empty() {
            return None;
        }
        let sum: f64 = self.values.iter().map(|v| (*v as f64) * (*v as f64)).sum();
        Some((sum / self.values.len() as f64).sqrt())
    }
    pub fn validate_finite(&self) -> Result<()> {
        for &v in &self.values {
            if !v.is_finite() {
                return Err(Error::OutOfRange { what: "float sample" });
            }
        }
        Ok(())
    }
    pub fn detect_gaps(&self, step_ns: u64) -> GapReport {
        find_gaps(self.base_ts_ns, self.len(), step_ns)
    }
}

pub fn lowpass_simple(batch: &FloatBatch, alpha: f32) -> FloatBatch {
    let mut out = Vec::with_capacity(batch.len());
    let mut prev = 0.0f32;
    for &v in &batch.values {
        prev = alpha * v + (1.0 - alpha) * prev;
        out.push(prev);
    }
    FloatBatch { base_ts_ns: batch.base_ts_ns, values: out }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rms() {
        let b = FloatBatch { base_ts_ns: 0, values: vec![3.0, 4.0] };
        assert!((b.rms().unwrap() - 3.535).abs() < 0.01);
    }
}
