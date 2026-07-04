//! SI and lab-specific unit codes.


use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum UnitCode {
    Dimensionless = 0,
    Celsius = 1,
    Pascal = 2,
    LitersPerMin = 3,
    Ph = 4,
    SiemensPerM = 5,
    Volt = 6,
    Ampere = 7,
    Count = 8,
    Newton = 9,
    Percent = 10,
    Rpm = 11,
}

impl UnitCode {
    pub fn from_u16(v: u16) -> Result<UnitCode> {
        match v {
            0 => Ok(UnitCode::Dimensionless),
            1 => Ok(UnitCode::Celsius),
            2 => Ok(UnitCode::Pascal),
            3 => Ok(UnitCode::LitersPerMin),
            4 => Ok(UnitCode::Ph),
            5 => Ok(UnitCode::SiemensPerM),
            6 => Ok(UnitCode::Volt),
            7 => Ok(UnitCode::Ampere),
            8 => Ok(UnitCode::Count),
            9 => Ok(UnitCode::Newton),
            10 => Ok(UnitCode::Percent),
            11 => Ok(UnitCode::Rpm),
            _ => Err(Error::OutOfRange { what: "unit code" }),
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            UnitCode::Dimensionless => "1",
            UnitCode::Celsius => "°C",
            UnitCode::Pascal => "Pa",
            UnitCode::LitersPerMin => "L/min",
            UnitCode::Ph => "pH",
            UnitCode::SiemensPerM => "S/m",
            UnitCode::Volt => "V",
            UnitCode::Ampere => "A",
            UnitCode::Count => "cnt",
            UnitCode::Newton => "N",
            UnitCode::Percent => "%",
            UnitCode::Rpm => "rpm",
        }
    }
}

pub fn conversion_factor(from: UnitCode, to: UnitCode) -> Option<f64> {
    use UnitCode::*;
    if from == to {
        return Some(1.0);
    }
    match (from, to) {
        (Celsius, Dimensionless) => Some(1.0),
        (Pascal, Dimensionless) => Some(1e-5),
        (Percent, Dimensionless) => Some(0.01),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn labels() {
        assert_eq!(UnitCode::Volt.label(), "V");
    }
}
