//! Named ingest pipeline stages for BLF capture files.

use crate::error::Result;
use crate::ingest::parse::{parse_capture, CaptureDoc};
use crate::ingest::pipeline::IngestReport;
use crate::check::integrity::verify_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageKind {
    Probe,
    Parse,
    Validate,
    Report,
}

pub fn stage_00_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 32 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_00_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 0u32;
    parse_capture(data)
}

pub fn stage_00_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_01_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 33 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_01_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 1u32;
    parse_capture(data)
}

pub fn stage_01_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_02_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 34 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_02_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 2u32;
    parse_capture(data)
}

pub fn stage_02_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_03_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 35 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_03_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 3u32;
    parse_capture(data)
}

pub fn stage_03_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_04_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 36 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_04_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 4u32;
    parse_capture(data)
}

pub fn stage_04_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_05_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 37 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_05_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 5u32;
    parse_capture(data)
}

pub fn stage_05_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_06_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 38 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_06_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 6u32;
    parse_capture(data)
}

pub fn stage_06_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_07_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 39 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_07_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 7u32;
    parse_capture(data)
}

pub fn stage_07_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_08_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 40 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_08_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 8u32;
    parse_capture(data)
}

pub fn stage_08_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_09_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 41 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_09_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 9u32;
    parse_capture(data)
}

pub fn stage_09_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_10_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 42 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_10_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 10u32;
    parse_capture(data)
}

pub fn stage_10_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_11_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 43 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_11_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 11u32;
    parse_capture(data)
}

pub fn stage_11_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_12_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 44 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_12_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 12u32;
    parse_capture(data)
}

pub fn stage_12_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_13_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 45 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_13_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 13u32;
    parse_capture(data)
}

pub fn stage_13_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_14_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 46 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_14_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 14u32;
    parse_capture(data)
}

pub fn stage_14_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_15_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 47 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_15_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 15u32;
    parse_capture(data)
}

pub fn stage_15_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_16_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 48 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_16_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 16u32;
    parse_capture(data)
}

pub fn stage_16_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_17_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 49 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_17_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 17u32;
    parse_capture(data)
}

pub fn stage_17_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_18_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 50 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_18_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 18u32;
    parse_capture(data)
}

pub fn stage_18_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_19_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 51 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_19_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 19u32;
    parse_capture(data)
}

pub fn stage_19_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_20_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 52 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_20_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 20u32;
    parse_capture(data)
}

pub fn stage_20_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_21_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 53 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_21_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 21u32;
    parse_capture(data)
}

pub fn stage_21_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_22_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 54 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_22_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 22u32;
    parse_capture(data)
}

pub fn stage_22_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_23_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 55 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_23_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 23u32;
    parse_capture(data)
}

pub fn stage_23_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_24_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 56 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_24_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 24u32;
    parse_capture(data)
}

pub fn stage_24_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_25_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 57 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_25_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 25u32;
    parse_capture(data)
}

pub fn stage_25_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_26_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 58 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_26_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 26u32;
    parse_capture(data)
}

pub fn stage_26_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_27_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 59 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_27_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 27u32;
    parse_capture(data)
}

pub fn stage_27_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_28_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 60 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_28_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 28u32;
    parse_capture(data)
}

pub fn stage_28_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_29_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 61 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_29_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 29u32;
    parse_capture(data)
}

pub fn stage_29_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_30_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 62 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_30_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 30u32;
    parse_capture(data)
}

pub fn stage_30_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_31_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 63 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_31_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 31u32;
    parse_capture(data)
}

pub fn stage_31_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_32_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 64 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_32_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 32u32;
    parse_capture(data)
}

pub fn stage_32_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_33_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 65 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_33_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 33u32;
    parse_capture(data)
}

pub fn stage_33_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_34_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 66 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_34_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 34u32;
    parse_capture(data)
}

pub fn stage_34_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_35_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 67 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_35_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 35u32;
    parse_capture(data)
}

pub fn stage_35_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_36_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 68 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_36_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 36u32;
    parse_capture(data)
}

pub fn stage_36_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_37_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 69 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_37_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 37u32;
    parse_capture(data)
}

pub fn stage_37_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_38_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 70 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_38_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 38u32;
    parse_capture(data)
}

pub fn stage_38_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_39_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 71 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_39_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 39u32;
    parse_capture(data)
}

pub fn stage_39_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_40_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 72 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_40_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 40u32;
    parse_capture(data)
}

pub fn stage_40_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_41_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 73 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_41_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 41u32;
    parse_capture(data)
}

pub fn stage_41_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_42_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 74 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_42_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 42u32;
    parse_capture(data)
}

pub fn stage_42_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_43_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 75 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_43_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 43u32;
    parse_capture(data)
}

pub fn stage_43_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_44_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 76 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_44_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 44u32;
    parse_capture(data)
}

pub fn stage_44_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_45_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 77 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_45_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 45u32;
    parse_capture(data)
}

pub fn stage_45_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_46_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 78 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_46_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 46u32;
    parse_capture(data)
}

pub fn stage_46_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_47_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 79 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_47_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 47u32;
    parse_capture(data)
}

pub fn stage_47_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_48_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 80 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_48_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 48u32;
    parse_capture(data)
}

pub fn stage_48_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_49_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 81 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_49_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 49u32;
    parse_capture(data)
}

pub fn stage_49_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_50_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 82 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_50_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 50u32;
    parse_capture(data)
}

pub fn stage_50_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_51_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 83 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_51_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 51u32;
    parse_capture(data)
}

pub fn stage_51_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_52_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 84 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_52_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 52u32;
    parse_capture(data)
}

pub fn stage_52_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_53_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 85 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_53_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 53u32;
    parse_capture(data)
}

pub fn stage_53_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_54_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 86 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_54_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 54u32;
    parse_capture(data)
}

pub fn stage_54_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_55_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 87 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_55_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 55u32;
    parse_capture(data)
}

pub fn stage_55_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_56_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 88 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_56_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 56u32;
    parse_capture(data)
}

pub fn stage_56_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_57_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 89 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_57_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 57u32;
    parse_capture(data)
}

pub fn stage_57_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_58_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 90 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_58_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 58u32;
    parse_capture(data)
}

pub fn stage_58_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_59_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 91 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_59_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 59u32;
    parse_capture(data)
}

pub fn stage_59_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_60_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 92 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_60_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 60u32;
    parse_capture(data)
}

pub fn stage_60_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_61_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 93 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_61_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 61u32;
    parse_capture(data)
}

pub fn stage_61_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_62_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 94 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_62_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 62u32;
    parse_capture(data)
}

pub fn stage_62_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_63_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 95 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_63_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 63u32;
    parse_capture(data)
}

pub fn stage_63_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_64_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 96 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_64_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 64u32;
    parse_capture(data)
}

pub fn stage_64_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_65_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 97 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_65_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 65u32;
    parse_capture(data)
}

pub fn stage_65_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_66_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 98 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_66_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 66u32;
    parse_capture(data)
}

pub fn stage_66_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_67_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 99 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_67_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 67u32;
    parse_capture(data)
}

pub fn stage_67_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_68_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 100 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_68_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 68u32;
    parse_capture(data)
}

pub fn stage_68_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_69_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 101 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_69_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 69u32;
    parse_capture(data)
}

pub fn stage_69_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_70_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 102 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_70_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 70u32;
    parse_capture(data)
}

pub fn stage_70_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_71_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 103 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_71_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 71u32;
    parse_capture(data)
}

pub fn stage_71_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_72_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 104 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_72_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 72u32;
    parse_capture(data)
}

pub fn stage_72_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_73_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 105 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_73_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 73u32;
    parse_capture(data)
}

pub fn stage_73_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_74_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 106 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_74_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 74u32;
    parse_capture(data)
}

pub fn stage_74_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_75_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 107 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_75_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 75u32;
    parse_capture(data)
}

pub fn stage_75_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_76_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 108 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_76_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 76u32;
    parse_capture(data)
}

pub fn stage_76_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_77_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 109 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_77_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 77u32;
    parse_capture(data)
}

pub fn stage_77_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_78_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 110 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_78_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 78u32;
    parse_capture(data)
}

pub fn stage_78_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn stage_79_probe(data: &[u8]) -> Result<bool> {
    if data.len() < 111 { return Ok(false); }
    Ok(verify_file(data).is_ok())
}

pub fn stage_79_parse(data: &[u8]) -> Result<CaptureDoc> {
    let _ = 79u32;
    parse_capture(data)
}

pub fn stage_79_channels(doc: &CaptureDoc) -> Result<usize> {
    Ok(doc.channels.len())
}

pub fn run_all_stages(data: &[u8]) -> Result<IngestReport> {
    crate::ingest::pipeline::ingest(data)
}
