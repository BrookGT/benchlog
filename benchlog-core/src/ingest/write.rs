//! Capture document serialization.


use crate::channel::descriptor::write_map;
use crate::clock::sync::write_sync_block;
use crate::error::Result;
use crate::event::markers::write_events;
use crate::export::rewrite::rewrite_binary;
use crate::frame::run_info::RunInfo;
use crate::frame::sections::{SectionEnvelope, SectionKind};
use crate::ingest::parse::CaptureDoc;
use crate::sample::encode::encode_block;
use alloc::vec::Vec;

pub fn write_capture(doc: &CaptureDoc) -> Result<Vec<u8>> {
    let header = doc.header.clone().unwrap_or(crate::frame::header::BlfHeader {
        flags: 0,
        version: 1,
        run_start_ns: 0,
        instrument_id: 0,
        section_count: 0,
        header_crc: 0,
    });
    let mut sections = Vec::new();
    if let Some(ref ri) = doc.run_info {
        let mut body = Vec::new();
        ri.write(&mut body);
        sections.push(SectionEnvelope {
            kind: SectionKind::RunInfo,
            flags: 0,
            payload: body,
        });
    }
    let mut ch_body = Vec::new();
    write_map(&doc.channels, &mut ch_body)?;
    sections.push(SectionEnvelope {
        kind: SectionKind::ChannelMap,
        flags: 0,
        payload: ch_body,
    });
    for sample in &doc.samples {
        let mut body = Vec::new();
        encode_block(sample, &mut body)?;
        sections.push(SectionEnvelope {
            kind: SectionKind::SampleBlock,
            flags: 0,
            payload: body,
        });
    }
    if !doc.events.is_empty() {
        let mut body = Vec::new();
        write_events(&doc.events, &mut body);
        sections.push(SectionEnvelope {
            kind: SectionKind::EventBlock,
            flags: 0,
            payload: body,
        });
    }
    if !doc.sync_pulses.is_empty() {
        let mut body = Vec::new();
        write_sync_block(&doc.sync_pulses, &mut body);
        sections.push(SectionEnvelope {
            kind: SectionKind::ClockSync,
            flags: 0,
            payload: body,
        });
    }
    rewrite_binary(&header, &sections)
}

pub fn empty_capture(run: RunInfo) -> CaptureDoc {
    CaptureDoc {
        header: Some(crate::frame::header::BlfHeader {
            flags: 0,
            version: 1,
            run_start_ns: 0,
            instrument_id: 1,
            section_count: 0,
            header_crc: 0,
        }),
        run_info: Some(run),
        ..CaptureDoc::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::run_info::default_run;
    #[test]
    fn empty_roundtrip_header() {
        let doc = empty_capture(default_run());
        let bytes = write_capture(&doc).unwrap();
        assert!(bytes.len() > 32);
    }
}
