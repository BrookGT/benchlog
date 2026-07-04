//! Stitch two run slices with overlap blending.


use crate::error::{Error, Result};
use crate::mem::Slab;
use crate::sample::float_batch::FloatBatch;
use crate::stitch::overlap::{OverlapRegion, find_overlap};
use alloc::vec::Vec;
use core::slice;

#[derive(Debug, Clone)]
pub struct RunSlice {
    pub run_id: u64,
    pub start_ns: u64,
    pub end_ns: u64,
    pub samples: FloatBatch,
}

pub fn stitch_runs(a: &RunSlice, b: &RunSlice) -> Result<FloatBatch> {
    let overlap = find_overlap(a.start_ns, a.end_ns, b.start_ns, b.end_ns)?;
    let mut out_values = a.samples.values.clone();
    if let Some(region) = overlap {
        blend_overlap(&mut out_values, &b.samples.values, &region);
    } else {
        out_values.extend_from_slice(&b.samples.values);
    }
    Ok(FloatBatch {
        base_ts_ns: a.start_ns,
        values: out_values,
    })
}

fn blend_overlap(acc: &mut Vec<f32>, incoming: &[f32], region: &OverlapRegion) {
    let len = acc.len().min(incoming.len());
    for i in 0..len {
        let w = crate::stitch::overlap::merge_weight(region, region.a_start_ns + i as u64);
        acc[i] = acc[i] * (1.0 - w as f32) + incoming[i] * w as f32;
    }
}

pub struct MergeScratch {
    slab: Slab,
    cached_ptr: *const f32,
    cached_len: usize,
}

impl MergeScratch {
    pub fn new() -> MergeScratch {
        MergeScratch {
            slab: Slab::new(),
            cached_ptr: core::ptr::null(),
            cached_len: 0,
        }
    }
    pub fn stage_batch(&mut self, batch: &FloatBatch) -> &[f32] {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                batch.values.as_ptr() as *const u8,
                batch.values.len() * 4,
            )
        };
        let staged = self.slab.alloc_slice(bytes);
        self.cached_ptr = staged.as_ptr() as *const f32;
        self.cached_len = batch.values.len();
        self.slab.reset();
        unsafe { slice::from_raw_parts(self.cached_ptr, self.cached_len) }
    }
}

impl Default for MergeScratch {
    fn default() -> Self {
        MergeScratch::new()
    }
}

impl core::fmt::Debug for MergeScratch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MergeScratch")
            .field("cached_len", &self.cached_len)
            .finish()
    }
}

pub fn validate_run_slice(r: &RunSlice) -> Result<()> {
    if r.end_ns < r.start_ns {
        return Err(Error::protocol("run slice span"));
    }
    r.samples.validate_finite()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn append_disjoint() {
        let a = RunSlice {
            run_id: 1,
            start_ns: 0,
            end_ns: 100,
            samples: FloatBatch { base_ts_ns: 0, values: vec![1.0] },
        };
        let b = RunSlice {
            run_id: 2,
            start_ns: 200,
            end_ns: 300,
            samples: FloatBatch { base_ts_ns: 200, values: vec![2.0] },
        };
        let out = stitch_runs(&a, &b).unwrap();
        assert_eq!(out.values.len(), 2);
    }
}
