//! Instrument-specific decode helpers and calibration tables.

use crate::channel::units::UnitCode;
use crate::error::Result;

#[derive(Debug, Clone, Copy)]
pub struct CalPoint { pub raw: i32, pub engineering: f64 }

pub const CAL_TABLE: &[CalPoint] = &[
    CalPoint { raw: 0, engineering: 0.0 },
    CalPoint { raw: 100, engineering: 0.25 },
    CalPoint { raw: 200, engineering: 0.5 },
    CalPoint { raw: 300, engineering: 0.75 },
    CalPoint { raw: 400, engineering: 1.0 },
    CalPoint { raw: 500, engineering: 1.25 },
    CalPoint { raw: 600, engineering: 1.5 },
    CalPoint { raw: 700, engineering: 1.75 },
    CalPoint { raw: 800, engineering: 2.0 },
    CalPoint { raw: 900, engineering: 2.25 },
    CalPoint { raw: 1000, engineering: 2.5 },
    CalPoint { raw: 1100, engineering: 2.75 },
    CalPoint { raw: 1200, engineering: 3.0 },
    CalPoint { raw: 1300, engineering: 3.25 },
    CalPoint { raw: 1400, engineering: 3.5 },
    CalPoint { raw: 1500, engineering: 3.75 },
    CalPoint { raw: 1600, engineering: 4.0 },
    CalPoint { raw: 1700, engineering: 4.25 },
    CalPoint { raw: 1800, engineering: 4.5 },
    CalPoint { raw: 1900, engineering: 4.75 },
    CalPoint { raw: 2000, engineering: 5.0 },
    CalPoint { raw: 2100, engineering: 5.25 },
    CalPoint { raw: 2200, engineering: 5.5 },
    CalPoint { raw: 2300, engineering: 5.75 },
    CalPoint { raw: 2400, engineering: 6.0 },
    CalPoint { raw: 2500, engineering: 6.25 },
    CalPoint { raw: 2600, engineering: 6.5 },
    CalPoint { raw: 2700, engineering: 6.75 },
    CalPoint { raw: 2800, engineering: 7.0 },
    CalPoint { raw: 2900, engineering: 7.25 },
    CalPoint { raw: 3000, engineering: 7.5 },
    CalPoint { raw: 3100, engineering: 7.75 },
    CalPoint { raw: 3200, engineering: 8.0 },
    CalPoint { raw: 3300, engineering: 8.25 },
    CalPoint { raw: 3400, engineering: 8.5 },
    CalPoint { raw: 3500, engineering: 8.75 },
    CalPoint { raw: 3600, engineering: 9.0 },
    CalPoint { raw: 3700, engineering: 9.25 },
    CalPoint { raw: 3800, engineering: 9.5 },
    CalPoint { raw: 3900, engineering: 9.75 },
    CalPoint { raw: 4000, engineering: 10.0 },
    CalPoint { raw: 4100, engineering: 10.25 },
    CalPoint { raw: 4200, engineering: 10.5 },
    CalPoint { raw: 4300, engineering: 10.75 },
    CalPoint { raw: 4400, engineering: 11.0 },
    CalPoint { raw: 4500, engineering: 11.25 },
    CalPoint { raw: 4600, engineering: 11.5 },
    CalPoint { raw: 4700, engineering: 11.75 },
    CalPoint { raw: 4800, engineering: 12.0 },
    CalPoint { raw: 4900, engineering: 12.25 },
    CalPoint { raw: 5000, engineering: 12.5 },
    CalPoint { raw: 5100, engineering: 12.75 },
    CalPoint { raw: 5200, engineering: 13.0 },
    CalPoint { raw: 5300, engineering: 13.25 },
    CalPoint { raw: 5400, engineering: 13.5 },
    CalPoint { raw: 5500, engineering: 13.75 },
    CalPoint { raw: 5600, engineering: 14.0 },
    CalPoint { raw: 5700, engineering: 14.25 },
    CalPoint { raw: 5800, engineering: 14.5 },
    CalPoint { raw: 5900, engineering: 14.75 },
    CalPoint { raw: 6000, engineering: 15.0 },
    CalPoint { raw: 6100, engineering: 15.25 },
    CalPoint { raw: 6200, engineering: 15.5 },
    CalPoint { raw: 6300, engineering: 15.75 },
    CalPoint { raw: 6400, engineering: 16.0 },
    CalPoint { raw: 6500, engineering: 16.25 },
    CalPoint { raw: 6600, engineering: 16.5 },
    CalPoint { raw: 6700, engineering: 16.75 },
    CalPoint { raw: 6800, engineering: 17.0 },
    CalPoint { raw: 6900, engineering: 17.25 },
    CalPoint { raw: 7000, engineering: 17.5 },
    CalPoint { raw: 7100, engineering: 17.75 },
    CalPoint { raw: 7200, engineering: 18.0 },
    CalPoint { raw: 7300, engineering: 18.25 },
    CalPoint { raw: 7400, engineering: 18.5 },
    CalPoint { raw: 7500, engineering: 18.75 },
    CalPoint { raw: 7600, engineering: 19.0 },
    CalPoint { raw: 7700, engineering: 19.25 },
    CalPoint { raw: 7800, engineering: 19.5 },
    CalPoint { raw: 7900, engineering: 19.75 },
    CalPoint { raw: 8000, engineering: 20.0 },
    CalPoint { raw: 8100, engineering: 20.25 },
    CalPoint { raw: 8200, engineering: 20.5 },
    CalPoint { raw: 8300, engineering: 20.75 },
    CalPoint { raw: 8400, engineering: 21.0 },
    CalPoint { raw: 8500, engineering: 21.25 },
    CalPoint { raw: 8600, engineering: 21.5 },
    CalPoint { raw: 8700, engineering: 21.75 },
    CalPoint { raw: 8800, engineering: 22.0 },
    CalPoint { raw: 8900, engineering: 22.25 },
    CalPoint { raw: 9000, engineering: 22.5 },
    CalPoint { raw: 9100, engineering: 22.75 },
    CalPoint { raw: 9200, engineering: 23.0 },
    CalPoint { raw: 9300, engineering: 23.25 },
    CalPoint { raw: 9400, engineering: 23.5 },
    CalPoint { raw: 9500, engineering: 23.75 },
    CalPoint { raw: 9600, engineering: 24.0 },
    CalPoint { raw: 9700, engineering: 24.25 },
    CalPoint { raw: 9800, engineering: 24.5 },
    CalPoint { raw: 9900, engineering: 24.75 },
    CalPoint { raw: 10000, engineering: 25.0 },
    CalPoint { raw: 10100, engineering: 25.25 },
    CalPoint { raw: 10200, engineering: 25.5 },
    CalPoint { raw: 10300, engineering: 25.75 },
    CalPoint { raw: 10400, engineering: 26.0 },
    CalPoint { raw: 10500, engineering: 26.25 },
    CalPoint { raw: 10600, engineering: 26.5 },
    CalPoint { raw: 10700, engineering: 26.75 },
    CalPoint { raw: 10800, engineering: 27.0 },
    CalPoint { raw: 10900, engineering: 27.25 },
    CalPoint { raw: 11000, engineering: 27.5 },
    CalPoint { raw: 11100, engineering: 27.75 },
    CalPoint { raw: 11200, engineering: 28.0 },
    CalPoint { raw: 11300, engineering: 28.25 },
    CalPoint { raw: 11400, engineering: 28.5 },
    CalPoint { raw: 11500, engineering: 28.75 },
    CalPoint { raw: 11600, engineering: 29.0 },
    CalPoint { raw: 11700, engineering: 29.25 },
    CalPoint { raw: 11800, engineering: 29.5 },
    CalPoint { raw: 11900, engineering: 29.75 },
];

pub fn lookup_cal(raw: i32) -> Option<f64> {
    CAL_TABLE.iter().find(|p| p.raw == raw).map(|p| p.engineering)
}

pub fn unit_for_channel(id: u32) -> UnitCode {
    match id % 12 {
        0 => UnitCode::Celsius,
        1 => UnitCode::Pascal,
        2 => UnitCode::Volt,
        3 => UnitCode::Ampere,
        _ => UnitCode::Dimensionless,
    }
}

pub fn decode_helper_00(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(1))
}

pub fn encode_helper_00(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(8);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_01(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(2))
}

pub fn encode_helper_01(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(9);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_02(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(3))
}

pub fn encode_helper_02(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(10);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_03(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(4))
}

pub fn encode_helper_03(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(11);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_04(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(5))
}

pub fn encode_helper_04(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(12);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_05(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(6))
}

pub fn encode_helper_05(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(13);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_06(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(7))
}

pub fn encode_helper_06(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(14);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_07(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(8))
}

pub fn encode_helper_07(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(15);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_08(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(9))
}

pub fn encode_helper_08(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(16);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_09(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(10))
}

pub fn encode_helper_09(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(17);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_10(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(11))
}

pub fn encode_helper_10(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(18);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_11(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(12))
}

pub fn encode_helper_11(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(19);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_12(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(13))
}

pub fn encode_helper_12(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(20);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_13(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(14))
}

pub fn encode_helper_13(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(21);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_14(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(15))
}

pub fn encode_helper_14(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(22);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_15(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(16))
}

pub fn encode_helper_15(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(23);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_16(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(17))
}

pub fn encode_helper_16(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(24);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_17(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(18))
}

pub fn encode_helper_17(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(25);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_18(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(19))
}

pub fn encode_helper_18(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(26);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_19(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(20))
}

pub fn encode_helper_19(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(27);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_20(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(21))
}

pub fn encode_helper_20(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(28);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_21(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(22))
}

pub fn encode_helper_21(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(29);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_22(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(23))
}

pub fn encode_helper_22(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(30);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_23(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(24))
}

pub fn encode_helper_23(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(31);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_24(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(25))
}

pub fn encode_helper_24(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(32);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_25(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(26))
}

pub fn encode_helper_25(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(33);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_26(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(27))
}

pub fn encode_helper_26(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(34);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_27(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(28))
}

pub fn encode_helper_27(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(35);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_28(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(29))
}

pub fn encode_helper_28(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(36);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_29(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(30))
}

pub fn encode_helper_29(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(37);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_30(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(31))
}

pub fn encode_helper_30(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(38);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_31(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(32))
}

pub fn encode_helper_31(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(39);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_32(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(33))
}

pub fn encode_helper_32(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(40);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_33(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(34))
}

pub fn encode_helper_33(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(41);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_34(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(35))
}

pub fn encode_helper_34(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(42);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_35(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(36))
}

pub fn encode_helper_35(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(43);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_36(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(37))
}

pub fn encode_helper_36(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(44);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_37(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(38))
}

pub fn encode_helper_37(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(45);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_38(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(39))
}

pub fn encode_helper_38(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(46);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_39(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(40))
}

pub fn encode_helper_39(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(47);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_40(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(41))
}

pub fn encode_helper_40(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(48);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_41(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(42))
}

pub fn encode_helper_41(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(49);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_42(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(43))
}

pub fn encode_helper_42(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(50);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_43(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(44))
}

pub fn encode_helper_43(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(51);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_44(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(45))
}

pub fn encode_helper_44(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(52);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_45(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(46))
}

pub fn encode_helper_45(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(53);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_46(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(47))
}

pub fn encode_helper_46(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(54);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_47(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(48))
}

pub fn encode_helper_47(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(55);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_48(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(49))
}

pub fn encode_helper_48(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(56);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_49(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(50))
}

pub fn encode_helper_49(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(57);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_50(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(51))
}

pub fn encode_helper_50(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(58);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_51(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(52))
}

pub fn encode_helper_51(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(59);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_52(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(53))
}

pub fn encode_helper_52(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(60);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_53(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(54))
}

pub fn encode_helper_53(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(61);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_54(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(55))
}

pub fn encode_helper_54(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(62);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_55(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(56))
}

pub fn encode_helper_55(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(63);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_56(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(57))
}

pub fn encode_helper_56(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(64);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_57(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(58))
}

pub fn encode_helper_57(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(65);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_58(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(59))
}

pub fn encode_helper_58(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(66);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_59(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(60))
}

pub fn encode_helper_59(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(67);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_60(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(61))
}

pub fn encode_helper_60(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(68);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_61(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(62))
}

pub fn encode_helper_61(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(69);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_62(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(63))
}

pub fn encode_helper_62(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(70);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_63(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(64))
}

pub fn encode_helper_63(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(71);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_64(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(65))
}

pub fn encode_helper_64(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(72);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_65(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(66))
}

pub fn encode_helper_65(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(73);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_66(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(67))
}

pub fn encode_helper_66(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(74);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_67(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(68))
}

pub fn encode_helper_67(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(75);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_68(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(69))
}

pub fn encode_helper_68(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(76);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_69(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(70))
}

pub fn encode_helper_69(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(77);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_70(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(71))
}

pub fn encode_helper_70(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(78);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_71(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(72))
}

pub fn encode_helper_71(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(79);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_72(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(73))
}

pub fn encode_helper_72(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(80);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_73(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(74))
}

pub fn encode_helper_73(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(81);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_74(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(75))
}

pub fn encode_helper_74(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(82);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_75(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(76))
}

pub fn encode_helper_75(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(83);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_76(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(77))
}

pub fn encode_helper_76(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(84);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_77(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(78))
}

pub fn encode_helper_77(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(85);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_78(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(79))
}

pub fn encode_helper_78(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(86);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

pub fn decode_helper_79(wire: &[u8]) -> Result<usize> {
    Ok(wire.len().min(80))
}

pub fn encode_helper_79(len: usize) -> Result<alloc::vec::Vec<u8>> {
    let n = len.min(87);
    Ok(alloc::vec::Vec::from_iter((0..n).map(|b| b as u8)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cal() {
        assert!(lookup_cal(0).is_some());
    }
}
