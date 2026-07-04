//! File-level integrity checks.


use crate::error::{Error, Result};
use crate::frame::header::{BlfHeader, crc32};
use crate::frame::trailer::BlfTrailer;
use crate::limits::{HEADER_SIZE, TRAILER_SIZE};

pub fn verify_file(data: &[u8]) -> Result<()> {
    if data.len() < HEADER_SIZE + TRAILER_SIZE {
        return Err(Error::UnexpectedEof);
    }
    let (header, _) = BlfHeader::parse(data)?;
    let trailer = BlfTrailer::parse(data)?;
    let payload = &data[HEADER_SIZE..data.len() - TRAILER_SIZE];
    trailer.verify_payload(payload)?;
    if header.section_count as usize > crate::limits::MAX_SECTIONS {
        return Err(Error::DepthLimit);
    }
    let _ = crc32(payload);
    Ok(())
}

pub fn quick_probe(data: &[u8]) -> bool {
    verify_file(data).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn short_fails() {
        assert!(verify_file(&[]).is_err());
    }
}
