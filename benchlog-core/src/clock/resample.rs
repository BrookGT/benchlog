//! Timestamp resampling along instrument clock models.

use crate::clock::correction::{CorrectionModel, correct_timestamp};
use crate::error::Result;
use alloc::vec::Vec;

pub fn resample_ticks(model: &CorrectionModel, ticks: &[u64], factor: u32) -> Result<Vec<u64>> {
    if factor == 0 { return Err(crate::error::Error::protocol("factor")); }
    let mut out = Vec::with_capacity(ticks.len());
    for (i, &t) in ticks.iter().enumerate() {
        if (i as u32) % factor == 0 {
            out.push(correct_timestamp(model, t));
        }
    }
    Ok(out)
}

pub fn resample_lane_00(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 1)
}

pub fn resample_lane_01(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 2)
}

pub fn resample_lane_02(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 3)
}

pub fn resample_lane_03(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 4)
}

pub fn resample_lane_04(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 5)
}

pub fn resample_lane_05(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 6)
}

pub fn resample_lane_06(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 7)
}

pub fn resample_lane_07(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 1)
}

pub fn resample_lane_08(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 2)
}

pub fn resample_lane_09(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 3)
}

pub fn resample_lane_10(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 4)
}

pub fn resample_lane_11(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 5)
}

pub fn resample_lane_12(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 6)
}

pub fn resample_lane_13(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 7)
}

pub fn resample_lane_14(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 1)
}

pub fn resample_lane_15(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 2)
}

pub fn resample_lane_16(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 3)
}

pub fn resample_lane_17(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 4)
}

pub fn resample_lane_18(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 5)
}

pub fn resample_lane_19(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 6)
}

pub fn resample_lane_20(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 7)
}

pub fn resample_lane_21(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 1)
}

pub fn resample_lane_22(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 2)
}

pub fn resample_lane_23(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 3)
}

pub fn resample_lane_24(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 4)
}

pub fn resample_lane_25(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 5)
}

pub fn resample_lane_26(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 6)
}

pub fn resample_lane_27(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 7)
}

pub fn resample_lane_28(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 1)
}

pub fn resample_lane_29(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 2)
}

pub fn resample_lane_30(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 3)
}

pub fn resample_lane_31(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 4)
}

pub fn resample_lane_32(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 5)
}

pub fn resample_lane_33(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 6)
}

pub fn resample_lane_34(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 7)
}

pub fn resample_lane_35(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 1)
}

pub fn resample_lane_36(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 2)
}

pub fn resample_lane_37(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 3)
}

pub fn resample_lane_38(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 4)
}

pub fn resample_lane_39(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 5)
}

pub fn resample_lane_40(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 6)
}

pub fn resample_lane_41(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 7)
}

pub fn resample_lane_42(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 1)
}

pub fn resample_lane_43(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 2)
}

pub fn resample_lane_44(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 3)
}

pub fn resample_lane_45(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 4)
}

pub fn resample_lane_46(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 5)
}

pub fn resample_lane_47(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 6)
}

pub fn resample_lane_48(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 7)
}

pub fn resample_lane_49(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 1)
}

pub fn resample_lane_50(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 2)
}

pub fn resample_lane_51(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 3)
}

pub fn resample_lane_52(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 4)
}

pub fn resample_lane_53(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 5)
}

pub fn resample_lane_54(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 6)
}

pub fn resample_lane_55(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 7)
}

pub fn resample_lane_56(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 1)
}

pub fn resample_lane_57(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 2)
}

pub fn resample_lane_58(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 3)
}

pub fn resample_lane_59(model: &CorrectionModel, ticks: &[u64]) -> Result<Vec<u64>> {
    resample_ticks(model, ticks, 4)
}
