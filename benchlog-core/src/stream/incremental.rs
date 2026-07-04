//! Incremental BLF stream driver.


use crate::error::Result;
use crate::frame::header::BlfHeader;
use crate::frame::sections::{SectionEnvelope, SectionKind, scan_sections};
use crate::frame::trailer::BlfTrailer;
use crate::limits::{HEADER_SIZE, TRAILER_SIZE};
use crate::stream::checkpoint::{Checkpoint, CheckpointStore};

#[derive(Debug, Default)]
pub struct StreamReader {
    pub header: Option<BlfHeader>,
    pub sections: alloc::vec::Vec<SectionEnvelope>,
    pub checkpoints: CheckpointStore,
}

impl StreamReader {
    pub fn new() -> StreamReader {
        StreamReader::default()
    }
    pub fn feed(&mut self, data: &[u8]) -> Result<usize> {
        if self.header.is_none() {
            if data.len() < HEADER_SIZE {
                return Ok(0);
            }
            let (h, _) = BlfHeader::parse(data)?;
            self.header = Some(h);
        }
        let payload_end = data.len().saturating_sub(TRAILER_SIZE);
        if payload_end <= HEADER_SIZE {
            return Ok(HEADER_SIZE);
        }
        let payload = &data[HEADER_SIZE..payload_end];
        self.sections = scan_sections(payload)?;
        if data.len() >= HEADER_SIZE + TRAILER_SIZE {
            let _trailer = BlfTrailer::parse(data)?;
        }
        Ok(data.len())
    }
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }
    pub fn sample_sections(&self) -> usize {
        self.sections
            .iter()
            .filter(|s| s.kind == SectionKind::SampleBlock)
            .count()
    }
}

pub fn drive_stream(data: &[u8]) -> Result<StreamReader> {
    let mut reader = StreamReader::new();
    let _ = reader.feed(data)?;
    if !data.is_empty() && data.len() > 16 {
        let cp = Checkpoint {
            byte_offset: (data.len() / 2) as u64,
            section_index: 0,
            crc: 0,
        };
        let _ = reader.checkpoints.load_resume_view(data, cp);
    }
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_no_header() {
        let mut s = StreamReader::new();
        assert_eq!(s.feed(&[]).unwrap(), 0);
    }
}
