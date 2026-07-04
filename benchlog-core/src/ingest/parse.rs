//! Full capture parse into document model.


use crate::channel::descriptor::{ChannelMap, parse_map};
use crate::clock::sync::parse_sync_block;
use crate::error::{Error, Result};
use crate::event::markers::parse_events;
use crate::frame::header::BlfHeader;
use crate::frame::run_info::RunInfo;
use crate::frame::sections::{SectionEnvelope, SectionKind, scan_sections};
use crate::frame::trailer::BlfTrailer;
use crate::limits::{HEADER_SIZE, TRAILER_SIZE};
use crate::sample::decode::{SampleBlock, decode_block};
use alloc::vec::Vec;

#[derive(Debug, Clone, Default)]
pub struct CaptureDoc {
    pub header: Option<BlfHeader>,
    pub run_info: Option<RunInfo>,
    pub channels: ChannelMap,
    pub samples: Vec<SampleBlock>,
    pub events: alloc::vec::Vec<crate::event::markers::EventRecord>,
    pub sync_pulses: alloc::vec::Vec<crate::clock::sync::SyncPulse>,
}

pub fn parse_capture(data: &[u8]) -> Result<CaptureDoc> {
    if data.len() < HEADER_SIZE + TRAILER_SIZE {
        return Err(Error::UnexpectedEof);
    }
    let (header, off) = BlfHeader::parse(data)?;
    let payload = &data[off..data.len() - TRAILER_SIZE];
    let _trailer = BlfTrailer::parse(data)?;
    let sections = scan_sections(payload)?;
    let mut doc = CaptureDoc {
        header: Some(header),
        ..CaptureDoc::default()
    };
    for sec in sections {
        match sec.kind {
            SectionKind::RunInfo => {
                doc.run_info = Some(RunInfo::parse(sec.body())?);
            }
            SectionKind::ChannelMap => {
                doc.channels = parse_map(sec.body())?;
            }
            SectionKind::SampleBlock => {
                doc.samples.push(decode_block(sec.body())?);
            }
            SectionKind::EventBlock => {
                doc.events = parse_events(sec.body())?;
            }
            SectionKind::ClockSync => {
                doc.sync_pulses = parse_sync_block(sec.body())?;
            }
            SectionKind::Checkpoint => {}
        }
    }
    Ok(doc)
}

pub fn probe_header(data: &[u8]) -> Result<BlfHeader> {
    crate::frame::header::probe_header(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn short_input() {
        assert!(parse_capture(&[]).is_err());
    }
}
