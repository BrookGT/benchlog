//! Timestamp correction from sync pairs.


use crate::clock::sync::SyncPulse;
use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy)]
pub struct CorrectionModel {
    pub slope: f64,
    pub intercept: f64,
}

pub fn fit_model(pulses: &[SyncPulse]) -> Result<CorrectionModel> {
    if pulses.len() < 2 {
        return Err(Error::protocol("need >=2 sync pulses"));
    }
    let p0 = &pulses[0];
    let p1 = &pulses[pulses.len() - 1];
    let dt_host = (p1.host_mono_ns as f64) - (p0.host_mono_ns as f64);
    let dt_inst = (p1.instrument_tick as f64) - (p0.instrument_tick as f64);
    if dt_inst.abs() < f64::EPSILON {
        return Err(Error::OutOfRange { what: "instrument tick delta" });
    }
    Ok(CorrectionModel {
        slope: dt_host / dt_inst,
        intercept: (p0.host_mono_ns as f64) - (p0.instrument_tick as f64) * (dt_host / dt_inst),
    })
}

pub fn correct_timestamp(model: &CorrectionModel, instrument_tick: u64) -> u64 {
    let corrected = model.intercept + model.slope * (instrument_tick as f64);
    if corrected.is_finite() && corrected >= 0.0 {
        corrected as u64
    } else {
        instrument_tick
    }
}

pub fn apply_batch(model: &CorrectionModel, ticks: &[u64], out: &mut [u64]) -> Result<()> {
    if ticks.len() != out.len() {
        return Err(Error::protocol("correction batch"));
    }
    for (t, o) in ticks.iter().zip(out.iter_mut()) {
        *o = correct_timestamp(model, *t);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identity_slope() {
        let m = CorrectionModel { slope: 1.0, intercept: 0.0 };
        assert_eq!(correct_timestamp(&m, 100), 100);
    }
}
