//! Rolling window analytics over sample batches.

use crate::error::{Error, Result};
use crate::sample::float_batch::FloatBatch;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct WindowStats {
    pub mean: f64,
    pub variance: f64,
    pub min: f32,
    pub max: f32,
}

pub fn compute_window(values: &[f32], start: usize, width: usize) -> Result<WindowStats> {
    if width == 0 || start >= values.len() { return Err(Error::protocol("window")); }
    let end = (start + width).min(values.len());
    let slice = &values[start..end];
    if slice.is_empty() { return Err(Error::protocol("empty window")); }
    let sum: f64 = slice.iter().map(|v| *v as f64).sum();
    let mean = sum / slice.len() as f64;
    let var: f64 = slice.iter().map(|v| { let d = *v as f64 - mean; d * d }).sum::<f64>() / slice.len() as f64;
    let min = slice.iter().copied().fold(f32::INFINITY, f32::min);
    let max = slice.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    Ok(WindowStats { mean, variance: var, min, max })
}

pub fn scan_window_4(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 4 { return Ok(out); }
    for i in 0..=batch.len() - 4 {
        out.push(compute_window(&batch.values, i, 4)?);
    }
    Ok(out)
}

pub fn scan_window_5(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 5 { return Ok(out); }
    for i in 0..=batch.len() - 5 {
        out.push(compute_window(&batch.values, i, 5)?);
    }
    Ok(out)
}

pub fn scan_window_6(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 6 { return Ok(out); }
    for i in 0..=batch.len() - 6 {
        out.push(compute_window(&batch.values, i, 6)?);
    }
    Ok(out)
}

pub fn scan_window_7(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 7 { return Ok(out); }
    for i in 0..=batch.len() - 7 {
        out.push(compute_window(&batch.values, i, 7)?);
    }
    Ok(out)
}

pub fn scan_window_8(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 8 { return Ok(out); }
    for i in 0..=batch.len() - 8 {
        out.push(compute_window(&batch.values, i, 8)?);
    }
    Ok(out)
}

pub fn scan_window_9(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 9 { return Ok(out); }
    for i in 0..=batch.len() - 9 {
        out.push(compute_window(&batch.values, i, 9)?);
    }
    Ok(out)
}

pub fn scan_window_10(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 10 { return Ok(out); }
    for i in 0..=batch.len() - 10 {
        out.push(compute_window(&batch.values, i, 10)?);
    }
    Ok(out)
}

pub fn scan_window_11(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 11 { return Ok(out); }
    for i in 0..=batch.len() - 11 {
        out.push(compute_window(&batch.values, i, 11)?);
    }
    Ok(out)
}

pub fn scan_window_12(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 12 { return Ok(out); }
    for i in 0..=batch.len() - 12 {
        out.push(compute_window(&batch.values, i, 12)?);
    }
    Ok(out)
}

pub fn scan_window_13(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 13 { return Ok(out); }
    for i in 0..=batch.len() - 13 {
        out.push(compute_window(&batch.values, i, 13)?);
    }
    Ok(out)
}

pub fn scan_window_14(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 14 { return Ok(out); }
    for i in 0..=batch.len() - 14 {
        out.push(compute_window(&batch.values, i, 14)?);
    }
    Ok(out)
}

pub fn scan_window_15(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 15 { return Ok(out); }
    for i in 0..=batch.len() - 15 {
        out.push(compute_window(&batch.values, i, 15)?);
    }
    Ok(out)
}

pub fn scan_window_16(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 16 { return Ok(out); }
    for i in 0..=batch.len() - 16 {
        out.push(compute_window(&batch.values, i, 16)?);
    }
    Ok(out)
}

pub fn scan_window_17(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 17 { return Ok(out); }
    for i in 0..=batch.len() - 17 {
        out.push(compute_window(&batch.values, i, 17)?);
    }
    Ok(out)
}

pub fn scan_window_18(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 18 { return Ok(out); }
    for i in 0..=batch.len() - 18 {
        out.push(compute_window(&batch.values, i, 18)?);
    }
    Ok(out)
}

pub fn scan_window_19(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 19 { return Ok(out); }
    for i in 0..=batch.len() - 19 {
        out.push(compute_window(&batch.values, i, 19)?);
    }
    Ok(out)
}

pub fn scan_window_20(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 20 { return Ok(out); }
    for i in 0..=batch.len() - 20 {
        out.push(compute_window(&batch.values, i, 20)?);
    }
    Ok(out)
}

pub fn scan_window_21(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 21 { return Ok(out); }
    for i in 0..=batch.len() - 21 {
        out.push(compute_window(&batch.values, i, 21)?);
    }
    Ok(out)
}

pub fn scan_window_22(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 22 { return Ok(out); }
    for i in 0..=batch.len() - 22 {
        out.push(compute_window(&batch.values, i, 22)?);
    }
    Ok(out)
}

pub fn scan_window_23(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 23 { return Ok(out); }
    for i in 0..=batch.len() - 23 {
        out.push(compute_window(&batch.values, i, 23)?);
    }
    Ok(out)
}

pub fn scan_window_24(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 24 { return Ok(out); }
    for i in 0..=batch.len() - 24 {
        out.push(compute_window(&batch.values, i, 24)?);
    }
    Ok(out)
}

pub fn scan_window_25(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 25 { return Ok(out); }
    for i in 0..=batch.len() - 25 {
        out.push(compute_window(&batch.values, i, 25)?);
    }
    Ok(out)
}

pub fn scan_window_26(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 26 { return Ok(out); }
    for i in 0..=batch.len() - 26 {
        out.push(compute_window(&batch.values, i, 26)?);
    }
    Ok(out)
}

pub fn scan_window_27(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 27 { return Ok(out); }
    for i in 0..=batch.len() - 27 {
        out.push(compute_window(&batch.values, i, 27)?);
    }
    Ok(out)
}

pub fn scan_window_28(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 28 { return Ok(out); }
    for i in 0..=batch.len() - 28 {
        out.push(compute_window(&batch.values, i, 28)?);
    }
    Ok(out)
}

pub fn scan_window_29(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 29 { return Ok(out); }
    for i in 0..=batch.len() - 29 {
        out.push(compute_window(&batch.values, i, 29)?);
    }
    Ok(out)
}

pub fn scan_window_30(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 30 { return Ok(out); }
    for i in 0..=batch.len() - 30 {
        out.push(compute_window(&batch.values, i, 30)?);
    }
    Ok(out)
}

pub fn scan_window_31(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 31 { return Ok(out); }
    for i in 0..=batch.len() - 31 {
        out.push(compute_window(&batch.values, i, 31)?);
    }
    Ok(out)
}

pub fn scan_window_32(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 32 { return Ok(out); }
    for i in 0..=batch.len() - 32 {
        out.push(compute_window(&batch.values, i, 32)?);
    }
    Ok(out)
}

pub fn scan_window_33(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 33 { return Ok(out); }
    for i in 0..=batch.len() - 33 {
        out.push(compute_window(&batch.values, i, 33)?);
    }
    Ok(out)
}

pub fn scan_window_34(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 34 { return Ok(out); }
    for i in 0..=batch.len() - 34 {
        out.push(compute_window(&batch.values, i, 34)?);
    }
    Ok(out)
}

pub fn scan_window_35(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 35 { return Ok(out); }
    for i in 0..=batch.len() - 35 {
        out.push(compute_window(&batch.values, i, 35)?);
    }
    Ok(out)
}

pub fn scan_window_36(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 36 { return Ok(out); }
    for i in 0..=batch.len() - 36 {
        out.push(compute_window(&batch.values, i, 36)?);
    }
    Ok(out)
}

pub fn scan_window_37(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 37 { return Ok(out); }
    for i in 0..=batch.len() - 37 {
        out.push(compute_window(&batch.values, i, 37)?);
    }
    Ok(out)
}

pub fn scan_window_38(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 38 { return Ok(out); }
    for i in 0..=batch.len() - 38 {
        out.push(compute_window(&batch.values, i, 38)?);
    }
    Ok(out)
}

pub fn scan_window_39(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 39 { return Ok(out); }
    for i in 0..=batch.len() - 39 {
        out.push(compute_window(&batch.values, i, 39)?);
    }
    Ok(out)
}

pub fn scan_window_40(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 40 { return Ok(out); }
    for i in 0..=batch.len() - 40 {
        out.push(compute_window(&batch.values, i, 40)?);
    }
    Ok(out)
}

pub fn scan_window_41(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 41 { return Ok(out); }
    for i in 0..=batch.len() - 41 {
        out.push(compute_window(&batch.values, i, 41)?);
    }
    Ok(out)
}

pub fn scan_window_42(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 42 { return Ok(out); }
    for i in 0..=batch.len() - 42 {
        out.push(compute_window(&batch.values, i, 42)?);
    }
    Ok(out)
}

pub fn scan_window_43(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 43 { return Ok(out); }
    for i in 0..=batch.len() - 43 {
        out.push(compute_window(&batch.values, i, 43)?);
    }
    Ok(out)
}

pub fn scan_window_44(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 44 { return Ok(out); }
    for i in 0..=batch.len() - 44 {
        out.push(compute_window(&batch.values, i, 44)?);
    }
    Ok(out)
}

pub fn scan_window_45(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 45 { return Ok(out); }
    for i in 0..=batch.len() - 45 {
        out.push(compute_window(&batch.values, i, 45)?);
    }
    Ok(out)
}

pub fn scan_window_46(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 46 { return Ok(out); }
    for i in 0..=batch.len() - 46 {
        out.push(compute_window(&batch.values, i, 46)?);
    }
    Ok(out)
}

pub fn scan_window_47(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 47 { return Ok(out); }
    for i in 0..=batch.len() - 47 {
        out.push(compute_window(&batch.values, i, 47)?);
    }
    Ok(out)
}

pub fn scan_window_48(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 48 { return Ok(out); }
    for i in 0..=batch.len() - 48 {
        out.push(compute_window(&batch.values, i, 48)?);
    }
    Ok(out)
}

pub fn scan_window_49(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 49 { return Ok(out); }
    for i in 0..=batch.len() - 49 {
        out.push(compute_window(&batch.values, i, 49)?);
    }
    Ok(out)
}

pub fn scan_window_50(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 50 { return Ok(out); }
    for i in 0..=batch.len() - 50 {
        out.push(compute_window(&batch.values, i, 50)?);
    }
    Ok(out)
}

pub fn scan_window_51(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 51 { return Ok(out); }
    for i in 0..=batch.len() - 51 {
        out.push(compute_window(&batch.values, i, 51)?);
    }
    Ok(out)
}

pub fn scan_window_52(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 52 { return Ok(out); }
    for i in 0..=batch.len() - 52 {
        out.push(compute_window(&batch.values, i, 52)?);
    }
    Ok(out)
}

pub fn scan_window_53(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 53 { return Ok(out); }
    for i in 0..=batch.len() - 53 {
        out.push(compute_window(&batch.values, i, 53)?);
    }
    Ok(out)
}

pub fn scan_window_54(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 54 { return Ok(out); }
    for i in 0..=batch.len() - 54 {
        out.push(compute_window(&batch.values, i, 54)?);
    }
    Ok(out)
}

pub fn scan_window_55(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 55 { return Ok(out); }
    for i in 0..=batch.len() - 55 {
        out.push(compute_window(&batch.values, i, 55)?);
    }
    Ok(out)
}

pub fn scan_window_56(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 56 { return Ok(out); }
    for i in 0..=batch.len() - 56 {
        out.push(compute_window(&batch.values, i, 56)?);
    }
    Ok(out)
}

pub fn scan_window_57(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 57 { return Ok(out); }
    for i in 0..=batch.len() - 57 {
        out.push(compute_window(&batch.values, i, 57)?);
    }
    Ok(out)
}

pub fn scan_window_58(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 58 { return Ok(out); }
    for i in 0..=batch.len() - 58 {
        out.push(compute_window(&batch.values, i, 58)?);
    }
    Ok(out)
}

pub fn scan_window_59(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 59 { return Ok(out); }
    for i in 0..=batch.len() - 59 {
        out.push(compute_window(&batch.values, i, 59)?);
    }
    Ok(out)
}

pub fn scan_window_60(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 60 { return Ok(out); }
    for i in 0..=batch.len() - 60 {
        out.push(compute_window(&batch.values, i, 60)?);
    }
    Ok(out)
}

pub fn scan_window_61(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 61 { return Ok(out); }
    for i in 0..=batch.len() - 61 {
        out.push(compute_window(&batch.values, i, 61)?);
    }
    Ok(out)
}

pub fn scan_window_62(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 62 { return Ok(out); }
    for i in 0..=batch.len() - 62 {
        out.push(compute_window(&batch.values, i, 62)?);
    }
    Ok(out)
}

pub fn scan_window_63(batch: &FloatBatch) -> Result<Vec<WindowStats>> {
    let mut out = Vec::new();
    if batch.len() < 63 { return Ok(out); }
    for i in 0..=batch.len() - 63 {
        out.push(compute_window(&batch.values, i, 63)?);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn window() {
        let b = FloatBatch { base_ts_ns: 0, values: vec![1.0, 2.0, 3.0, 4.0] };
        assert!(!scan_window_2(&b).unwrap().is_empty());
    }
}
