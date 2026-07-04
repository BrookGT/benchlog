//! BLF file header parse and validation.


use crate::error::{Error, Result};
use crate::limits::HEADER_SIZE;
use crate::mem::reader::Reader;

pub const BLF_MAGIC: [u8; 5] = [0x42, 0x4c, 0x46, 1, 0];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlfHeader {
    pub flags: u8,
    pub version: u16,
    pub run_start_ns: u64,
    pub instrument_id: u64,
    pub section_count: u32,
    pub header_crc: u32,
}

pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

impl BlfHeader {
    pub fn parse(data: &[u8]) -> Result<(BlfHeader, usize)> {
        if data.len() < HEADER_SIZE {
            return Err(Error::UnexpectedEof);
        }
        let mut r = Reader::new(&data[..HEADER_SIZE]);
        let magic = r.read_array::<5>()?;
        if &magic != &BLF_MAGIC {
            return Err(Error::protocol("bad magic"));
        }
        let flags = r.read_u8()?;
        let version = r.read_u16_le()?;
        if version == 0 || version > 3 {
            return Err(Error::protocol("unsupported version"));
        }
        let run_start_ns = r.read_u64_le()?;
        let instrument_id = r.read_u64_le()?;
        let section_count = r.read_u32_le()?;
        let header_crc = r.read_u32_le()?;
        let body = &data[..28];
        let expect = crc32(body);
        if expect != header_crc {
            return Err(Error::integrity("header crc"));
        }
        Ok((
            BlfHeader {
                flags,
                version,
                run_start_ns,
                instrument_id,
                section_count,
                header_crc,
            },
            HEADER_SIZE,
        ))
    }
    pub fn write(&self, out: &mut alloc::vec::Vec<u8>) {
        out.extend_from_slice(&BLF_MAGIC);
        out.push(self.flags);
        out.extend_from_slice(&self.version.to_le_bytes());
        out.extend_from_slice(&self.run_start_ns.to_le_bytes());
        out.extend_from_slice(&self.instrument_id.to_le_bytes());
        out.extend_from_slice(&self.section_count.to_le_bytes());
        let crc = crc32(&out[..28]);
        out.extend_from_slice(&crc.to_le_bytes());
    }
}

pub fn probe_header(data: &[u8]) -> Result<BlfHeader> {
    BlfHeader::parse(data).map(|(h, _)| h)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn magic_required() {
        assert!(BlfHeader::parse(&[0; 32]).is_err());
    }
}
