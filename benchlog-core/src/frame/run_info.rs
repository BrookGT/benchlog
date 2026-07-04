//! Run metadata section body.


use crate::error::{Error, Result};
use crate::limits::MAX_RUN_LABEL;
use crate::mem::reader::Reader;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunInfo {
    pub operator_id: u32,
    pub protocol_id: u32,
    pub sample_rate_hz: u32,
    pub label: String,
    pub notes: String,
}

impl RunInfo {
    pub fn parse(body: &[u8]) -> Result<RunInfo> {
        let mut r = Reader::new(body);
        let operator_id = r.read_u32_le()?;
        let protocol_id = r.read_u32_le()?;
        let sample_rate_hz = r.read_u32_le()?;
        let label_len = r.read_u16_le()? as usize;
        if label_len > MAX_RUN_LABEL {
            return Err(Error::LengthOverflow { field: "run label" });
        }
        let label_bytes = r.read_slice(label_len)?;
        let label = core::str::from_utf8(label_bytes)?.into();
        let notes_len = r.read_u16_le()? as usize;
        if notes_len > MAX_RUN_LABEL * 4 {
            return Err(Error::LengthOverflow { field: "run notes" });
        }
        let notes_bytes = r.read_slice(notes_len)?;
        let notes = core::str::from_utf8(notes_bytes)?.into();
        Ok(RunInfo {
            operator_id,
            protocol_id,
            sample_rate_hz,
            label,
            notes,
        })
    }
    pub fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.operator_id.to_le_bytes());
        out.extend_from_slice(&self.protocol_id.to_le_bytes());
        out.extend_from_slice(&self.sample_rate_hz.to_le_bytes());
        out.extend_from_slice(&(self.label.len() as u16).to_le_bytes());
        out.extend_from_slice(self.label.as_bytes());
        out.extend_from_slice(&(self.notes.len() as u16).to_le_bytes());
        out.extend_from_slice(self.notes.as_bytes());
    }
}

pub fn default_run() -> RunInfo {
    RunInfo {
        operator_id: 0,
        protocol_id: 1,
        sample_rate_hz: 1000,
        label: String::from("default-run"),
        notes: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        let ri = default_run();
        let mut buf = Vec::new();
        ri.write(&mut buf);
        let back = RunInfo::parse(&buf).unwrap();
        assert_eq!(back.label, ri.label);
    }
}
