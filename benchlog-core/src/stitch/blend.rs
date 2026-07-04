//! Overlap blend strategies for stitched runs.

use crate::sample::float_batch::FloatBatch;
use alloc::vec::Vec;

pub fn blend_linear_00(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.01f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_00(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_00(&a.values, &b.values) }
}

pub fn blend_linear_01(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.02f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_01(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_01(&a.values, &b.values) }
}

pub fn blend_linear_02(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.03f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_02(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_02(&a.values, &b.values) }
}

pub fn blend_linear_03(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.04f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_03(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_03(&a.values, &b.values) }
}

pub fn blend_linear_04(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.05f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_04(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_04(&a.values, &b.values) }
}

pub fn blend_linear_05(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.06f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_05(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_05(&a.values, &b.values) }
}

pub fn blend_linear_06(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.07f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_06(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_06(&a.values, &b.values) }
}

pub fn blend_linear_07(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.08f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_07(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_07(&a.values, &b.values) }
}

pub fn blend_linear_08(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.09f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_08(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_08(&a.values, &b.values) }
}

pub fn blend_linear_09(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.1f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_09(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_09(&a.values, &b.values) }
}

pub fn blend_linear_10(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.11f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_10(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_10(&a.values, &b.values) }
}

pub fn blend_linear_11(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.12f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_11(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_11(&a.values, &b.values) }
}

pub fn blend_linear_12(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.13f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_12(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_12(&a.values, &b.values) }
}

pub fn blend_linear_13(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.14f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_13(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_13(&a.values, &b.values) }
}

pub fn blend_linear_14(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.15f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_14(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_14(&a.values, &b.values) }
}

pub fn blend_linear_15(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.16f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_15(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_15(&a.values, &b.values) }
}

pub fn blend_linear_16(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.17f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_16(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_16(&a.values, &b.values) }
}

pub fn blend_linear_17(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.18f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_17(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_17(&a.values, &b.values) }
}

pub fn blend_linear_18(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.19f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_18(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_18(&a.values, &b.values) }
}

pub fn blend_linear_19(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.2f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_19(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_19(&a.values, &b.values) }
}

pub fn blend_linear_20(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.21f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_20(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_20(&a.values, &b.values) }
}

pub fn blend_linear_21(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.22f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_21(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_21(&a.values, &b.values) }
}

pub fn blend_linear_22(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.23f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_22(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_22(&a.values, &b.values) }
}

pub fn blend_linear_23(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.24f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_23(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_23(&a.values, &b.values) }
}

pub fn blend_linear_24(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.25f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_24(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_24(&a.values, &b.values) }
}

pub fn blend_linear_25(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.26f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_25(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_25(&a.values, &b.values) }
}

pub fn blend_linear_26(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.27f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_26(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_26(&a.values, &b.values) }
}

pub fn blend_linear_27(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.28f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_27(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_27(&a.values, &b.values) }
}

pub fn blend_linear_28(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.29f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_28(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_28(&a.values, &b.values) }
}

pub fn blend_linear_29(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.3f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_29(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_29(&a.values, &b.values) }
}

pub fn blend_linear_30(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.31f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_30(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_30(&a.values, &b.values) }
}

pub fn blend_linear_31(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.32f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_31(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_31(&a.values, &b.values) }
}

pub fn blend_linear_32(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.33f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_32(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_32(&a.values, &b.values) }
}

pub fn blend_linear_33(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.34f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_33(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_33(&a.values, &b.values) }
}

pub fn blend_linear_34(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.35f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_34(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_34(&a.values, &b.values) }
}

pub fn blend_linear_35(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.36f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_35(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_35(&a.values, &b.values) }
}

pub fn blend_linear_36(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.37f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_36(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_36(&a.values, &b.values) }
}

pub fn blend_linear_37(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.38f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_37(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_37(&a.values, &b.values) }
}

pub fn blend_linear_38(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.39f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_38(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_38(&a.values, &b.values) }
}

pub fn blend_linear_39(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.4f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_39(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_39(&a.values, &b.values) }
}

pub fn blend_linear_40(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.41f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_40(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_40(&a.values, &b.values) }
}

pub fn blend_linear_41(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.42f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_41(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_41(&a.values, &b.values) }
}

pub fn blend_linear_42(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.43f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_42(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_42(&a.values, &b.values) }
}

pub fn blend_linear_43(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.44f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_43(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_43(&a.values, &b.values) }
}

pub fn blend_linear_44(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.45f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_44(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_44(&a.values, &b.values) }
}

pub fn blend_linear_45(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.46f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_45(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_45(&a.values, &b.values) }
}

pub fn blend_linear_46(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.47f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_46(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_46(&a.values, &b.values) }
}

pub fn blend_linear_47(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.48f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_47(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_47(&a.values, &b.values) }
}

pub fn blend_linear_48(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.49f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_48(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_48(&a.values, &b.values) }
}

pub fn blend_linear_49(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.5f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_49(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_49(&a.values, &b.values) }
}

pub fn blend_linear_50(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.51f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_50(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_50(&a.values, &b.values) }
}

pub fn blend_linear_51(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.52f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_51(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_51(&a.values, &b.values) }
}

pub fn blend_linear_52(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.53f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_52(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_52(&a.values, &b.values) }
}

pub fn blend_linear_53(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.54f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_53(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_53(&a.values, &b.values) }
}

pub fn blend_linear_54(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.55f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_54(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_54(&a.values, &b.values) }
}

pub fn blend_linear_55(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.56f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_55(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_55(&a.values, &b.values) }
}

pub fn blend_linear_56(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.57f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_56(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_56(&a.values, &b.values) }
}

pub fn blend_linear_57(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.58f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_57(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_57(&a.values, &b.values) }
}

pub fn blend_linear_58(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.59f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_58(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_58(&a.values, &b.values) }
}

pub fn blend_linear_59(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.6f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_59(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_59(&a.values, &b.values) }
}

pub fn blend_linear_60(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.61f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_60(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_60(&a.values, &b.values) }
}

pub fn blend_linear_61(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.62f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_61(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_61(&a.values, &b.values) }
}

pub fn blend_linear_62(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.63f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_62(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_62(&a.values, &b.values) }
}

pub fn blend_linear_63(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.64f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_63(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_63(&a.values, &b.values) }
}

pub fn blend_linear_64(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.65f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_64(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_64(&a.values, &b.values) }
}

pub fn blend_linear_65(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.66f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_65(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_65(&a.values, &b.values) }
}

pub fn blend_linear_66(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.67f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_66(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_66(&a.values, &b.values) }
}

pub fn blend_linear_67(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.68f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_67(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_67(&a.values, &b.values) }
}

pub fn blend_linear_68(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.69f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_68(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_68(&a.values, &b.values) }
}

pub fn blend_linear_69(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().min(b.len());
    let mut out = Vec::with_capacity(len);
    for j in 0..len {
        let t = 0.7f32;
        out.push(a[j] * (1.0 - t) + b[j] * t);
    }
    out
}

pub fn blend_batch_69(a: &FloatBatch, b: &FloatBatch) -> FloatBatch {
    FloatBatch { base_ts_ns: a.base_ts_ns, values: blend_linear_69(&a.values, &b.values) }
}
