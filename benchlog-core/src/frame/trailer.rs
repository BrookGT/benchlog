//! BLF trailer with payload byte count and rolling CRC.


use crate::error::{Error, Result};
use crate::frame::header::crc32;
use crate::limits::TRAILER_SIZE;
use crate::mem::reader::Reader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlfTrailer {
    pub payload_bytes: u64,
    pub rolling_crc: u32,
    pub reserved: u32,
}

impl BlfTrailer {
    pub fn parse(data: &[u8]) -> Result<BlfTrailer> {
        if data.len() < TRAILER_SIZE {
            return Err(Error::UnexpectedEof);
        }
        let mut r = Reader::new(&data[data.len() - TRAILER_SIZE..]);
        Ok(BlfTrailer {
            payload_bytes: r.read_u64_le()?,
            rolling_crc: r.read_u32_le()?,
            reserved: r.read_u32_le()?,
        })
    }
    pub fn write(&self, out: &mut alloc::vec::Vec<u8>) {
        out.extend_from_slice(&self.payload_bytes.to_le_bytes());
        out.extend_from_slice(&self.rolling_crc.to_le_bytes());
        out.extend_from_slice(&self.reserved.to_le_bytes());
    }
    pub fn verify_payload(&self, payload: &[u8]) -> Result<()> {
        if payload.len() as u64 != self.payload_bytes {
            return Err(Error::integrity("payload length"));
        }
        let crc = crc32(payload);
        if crc != self.rolling_crc {
            return Err(Error::integrity("rolling crc"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_min() {
        let mut buf = [0u8; 16];
        buf[0..8].copy_from_slice(&5u64.to_le_bytes());
        let t = BlfTrailer::parse(&buf).unwrap();
        assert_eq!(t.payload_bytes, 5);
    }
}
