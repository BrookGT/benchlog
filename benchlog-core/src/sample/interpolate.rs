//! Interpolation helpers for missing samples.


use crate::error::{Error, Result};
use alloc::vec::Vec;

pub fn linear_at(xs: &[f64], ys: &[f32], x: f64) -> Result<f32> {
    if xs.len() != ys.len() || xs.is_empty() {
        return Err(Error::protocol("interpolate input"));
    }
    if x <= xs[0] {
        return Ok(ys[0]);
    }
    if x >= xs[xs.len() - 1] {
        return Ok(ys[ys.len() - 1]);
    }
    for i in 1..xs.len() {
        if x <= xs[i] {
            let x0 = xs[i - 1];
            let x1 = xs[i];
            let t = ((x - x0) / (x1 - x0)) as f32;
            return Ok(ys[i - 1] + t * (ys[i] - ys[i - 1]));
        }
    }
    Ok(ys[ys.len() - 1])
}

pub fn resample_linear(times: &[u64], values: &[f32], out_times: &[u64]) -> Result<Vec<f32>> {
    if times.len() != values.len() {
        return Err(Error::protocol("resample length"));
    }
    let xs: Vec<f64> = times.iter().map(|t| *t as f64).collect();
    let mut out = Vec::with_capacity(out_times.len());
    for &t in out_times {
        out.push(linear_at(&xs, values, t as f64)?);
    }
    Ok(out)
}

pub fn moving_average(values: &[f32], window: usize) -> Vec<f32> {
    if window == 0 || values.is_empty() {
        return values.to_vec();
    }
    let mut out = Vec::with_capacity(values.len());
    for i in 0..values.len() {
        let start = i.saturating_sub(window - 1);
        let slice = &values[start..=i];
        let sum: f32 = slice.iter().sum();
        out.push(sum / slice.len() as f32);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn endpoints() {
        let xs = [0.0, 1.0];
        let ys = [10.0, 20.0];
        assert_eq!(linear_at(&xs, &ys, 0.0).unwrap(), 10.0);
    }
}
