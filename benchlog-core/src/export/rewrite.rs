//! Binary rewrite preserving section order.


use crate::error::Result;
use crate::frame::header::{BlfHeader, crc32};
use crate::frame::sections::{SectionEnvelope, SectionKind};
use crate::frame::trailer::BlfTrailer;
use crate::limits::{HEADER_SIZE, TRAILER_SIZE};
use alloc::vec::Vec;

pub fn rewrite_binary(header: &BlfHeader, sections: &[SectionEnvelope]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    let mut h = header.clone();
    h.section_count = sections.len() as u32;
    h.write(&mut out);
    let payload_start = out.len();
    for sec in sections {
        sec.write(&mut out);
    }
    let payload = &out[payload_start..];
    let trailer = BlfTrailer {
        payload_bytes: payload.len() as u64,
        rolling_crc: crc32(payload),
        reserved: 0,
    };
    trailer.write(&mut out);
    let _ = HEADER_SIZE;
    let _ = TRAILER_SIZE;
    Ok(out)
}

pub fn filter_sections(sections: &[SectionEnvelope], kind: SectionKind) -> Vec<SectionEnvelope> {
    sections.iter().filter(|s| s.kind == kind).cloned().collect()
}

pub fn reorder_by_kind(sections: &mut [SectionEnvelope]) {
    sections.sort_by_key(|s| s.kind as u8);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::header::BLF_MAGIC;
    #[test]
    fn empty_sections_trailer() {
        let h = BlfHeader {
            flags: 0,
            version: 1,
            run_start_ns: 0,
            instrument_id: 1,
            section_count: 0,
            header_crc: 0,
        };
        let out = rewrite_binary(&h, &[]).unwrap();
        assert!(out.len() >= HEADER_SIZE + TRAILER_SIZE);
        assert_eq!(&out[..5], &BLF_MAGIC);
    }
}
