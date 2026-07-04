//! Typed section envelope wrappers.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SectionKind {
    RunInfo = 1,
    ChannelMap = 2,
    SampleBlock = 3,
    EventBlock = 4,
    ClockSync = 5,
    Checkpoint = 6,
}

impl SectionKind {
    pub fn from_u8(v: u8) -> Option<SectionKind> {
        match v {
            1 => Some(SectionKind::RunInfo),
            2 => Some(SectionKind::ChannelMap),
            3 => Some(SectionKind::SampleBlock),
            4 => Some(SectionKind::EventBlock),
            5 => Some(SectionKind::ClockSync),
            6 => Some(SectionKind::Checkpoint),
            _ => None,
        }
    }
}

use crate::codec::leb128;
use crate::error::{Error, Result};
use crate::mem::reader::Reader;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionEnvelope {
    pub kind: SectionKind,
    pub flags: u8,
    pub payload: Vec<u8>,
}

impl SectionEnvelope {
    pub fn parse(r: &mut Reader<'_>) -> Result<SectionEnvelope> {
        let kind_raw = r.read_u8()?;
        let kind = SectionKind::from_u8(kind_raw)
            .ok_or(Error::protocol("unknown section kind"))?;
        let flags = r.read_u8()?;
        let len = leb128::read_u64(r)? as usize;
        if len > crate::limits::MAX_SAMPLE_BATCH.saturating_mul(16) {
            return Err(Error::LengthOverflow { field: "section payload" });
        }
        let payload = r.read_slice(len)?.to_vec();
        Ok(SectionEnvelope { kind, flags, payload })
    }
    pub fn write(&self, out: &mut Vec<u8>) {
        out.push(self.kind as u8);
        out.push(self.flags);
        leb128::write_u64(out, self.payload.len() as u64);
        out.extend_from_slice(&self.payload);
    }
    pub fn body(&self) -> &[u8] {
        &self.payload
    }
}

pub fn scan_sections(data: &[u8]) -> Result<Vec<SectionEnvelope>> {
    let mut r = Reader::new(data);
    let mut out = Vec::new();
    while r.remaining() > 0 {
        out.push(SectionEnvelope::parse(&mut r)?);
        if out.len() > crate::limits::MAX_SECTIONS {
            return Err(Error::DepthLimit);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip_empty_payload() {
        let env = SectionEnvelope {
            kind: SectionKind::RunInfo,
            flags: 0,
            payload: Vec::new(),
        };
        let mut buf = Vec::new();
        env.write(&mut buf);
        let mut r = Reader::new(&buf);
        let back = SectionEnvelope::parse(&mut r).unwrap();
        assert_eq!(back.kind, SectionKind::RunInfo);
    }
}
