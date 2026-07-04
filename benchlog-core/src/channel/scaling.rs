//! Linear scale and offset for raw ADC codes.


use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearScale {
    pub factor: f64,
    pub offset: f64,
}

impl Default for LinearScale {
    fn default() -> LinearScale {
        LinearScale { factor: 1.0, offset: 0.0 }
    }
}

impl LinearScale {
    pub fn identity() -> LinearScale {
        LinearScale::default()
    }
    pub fn validate(&self) -> Result<()> {
        if !self.factor.is_finite() || !self.offset.is_finite() {
            return Err(Error::OutOfRange { what: "scale coefficients" });
        }
        Ok(())
    }
    pub fn apply_i32(&self, raw: i32) -> f64 {
        self.factor * (raw as f64) + self.offset
    }
    pub fn apply_f32(&self, raw: f32) -> f64 {
        self.factor * (raw as f64) + self.offset
    }
    pub fn invert(&self, engineering: f64) -> Result<f64> {
        if self.factor.abs() < f64::EPSILON {
            return Err(Error::OutOfRange { what: "scale factor" });
        }
        Ok((engineering - self.offset) / self.factor)
    }
}

pub fn apply_scale(scale: &LinearScale, raw: i32) -> f64 {
    scale.apply_i32(raw)
}

pub fn batch_apply(scale: &LinearScale, raw: &[i32], out: &mut [f64]) -> Result<()> {
    if raw.len() != out.len() {
        return Err(Error::protocol("batch length mismatch"));
    }
    scale.validate()?;
    for (r, o) in raw.iter().zip(out.iter_mut()) {
        *o = scale.apply_i32(*r);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identity() {
        let s = LinearScale::identity();
        assert_eq!(s.apply_i32(42), 42.0);
    }
}
