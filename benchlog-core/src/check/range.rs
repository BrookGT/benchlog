//! Sample and timestamp range validation.


use crate::error::{Error, Result};
use crate::sample::decode::{SampleBlock, SamplePayload};

pub fn validate_sample_ranges(block: &SampleBlock, lo: f64, hi: f64) -> Result<()> {
    match &block.payload {
        SamplePayload::Int(b) => {
            for &v in &b.values {
                let fv = v as f64;
                if fv < lo || fv > hi {
                    return Err(Error::OutOfRange { what: "int sample" });
                }
            }
        }
        SamplePayload::Float(b) => {
            for &v in &b.values {
                let fv = v as f64;
                if fv < lo || fv > hi {
                    return Err(Error::OutOfRange { what: "float sample" });
                }
            }
        }
        SamplePayload::Bool(_) => {}
    }
    Ok(())
}

pub fn validate_timestamp_monotonic(times: &[u64]) -> Result<()> {
    for w in times.windows(2) {
        if w[1] < w[0] {
            return Err(Error::TimestampGap { expected: w[0], found: w[1] });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn monotonic_ok() {
        assert!(validate_timestamp_monotonic(&[1, 2, 3]).is_ok());
    }
}
