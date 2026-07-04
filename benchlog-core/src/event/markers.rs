//! Event block parse and classification.


use crate::error::{Error, Result};
use crate::limits::MAX_EVENT_NOTE;
use crate::mem::reader::Reader;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventSeverity {
    Info = 0,
    Notice = 1,
    Warning = 2,
    Alarm = 3,
    Critical = 4,
}

impl EventSeverity {
    pub fn from_u8(v: u8) -> Result<EventSeverity> {
        match v {
            0 => Ok(EventSeverity::Info),
            1 => Ok(EventSeverity::Notice),
            2 => Ok(EventSeverity::Warning),
            3 => Ok(EventSeverity::Alarm),
            4 => Ok(EventSeverity::Critical),
            _ => Err(Error::protocol("event severity")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventKind {
    Marker = 0,
    OperatorNote = 1,
    ThresholdCross = 2,
    SyncPulse = 3,
    RunBoundary = 4,
}

impl EventKind {
    pub fn from_u8(v: u8) -> Result<EventKind> {
        match v {
            0 => Ok(EventKind::Marker),
            1 => Ok(EventKind::OperatorNote),
            2 => Ok(EventKind::ThresholdCross),
            3 => Ok(EventKind::SyncPulse),
            4 => Ok(EventKind::RunBoundary),
            _ => Err(Error::protocol("event kind")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventRecord {
    pub ts_ns: u64,
    pub severity: EventSeverity,
    pub kind: EventKind,
    pub channel_id: u32,
    pub note: String,
}

pub fn parse_one(r: &mut Reader<'_>) -> Result<EventRecord> {
    let ts_ns = r.read_u64_le()?;
    let severity = EventSeverity::from_u8(r.read_u8()?)?;
    let kind = EventKind::from_u8(r.read_u8()?)?;
    let channel_id = r.read_u32_le()?;
    let note_len = r.read_u16_le()? as usize;
    if note_len > MAX_EVENT_NOTE {
        return Err(Error::LengthOverflow { field: "event note" });
    }
    let note_bytes = r.read_slice(note_len)?;
    let note = core::str::from_utf8(note_bytes)?.into();
    Ok(EventRecord { ts_ns, severity, kind, channel_id, note })
}

pub fn parse_events(body: &[u8]) -> Result<Vec<EventRecord>> {
    let mut r = Reader::new(body);
    let count = r.read_u32_le()? as usize;
    if count > crate::limits::MAX_SECTIONS {
        return Err(Error::CapacityLimit);
    }
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        out.push(parse_one(&mut r)?);
    }
    Ok(out)
}

pub fn write_events(events: &[EventRecord], out: &mut Vec<u8>) {
    out.extend_from_slice(&(events.len() as u32).to_le_bytes());
    for ev in events {
        out.extend_from_slice(&ev.ts_ns.to_le_bytes());
        out.push(ev.severity as u8);
        out.push(ev.kind as u8);
        out.extend_from_slice(&ev.channel_id.to_le_bytes());
        out.extend_from_slice(&(ev.note.len() as u16).to_le_bytes());
        out.extend_from_slice(ev.note.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&0u32.to_le_bytes());
        assert!(parse_events(&buf).unwrap().is_empty());
    }
}
