//! LEB128 unsigned integer codec.


use crate::error::{Error, Result};
use crate::mem::reader::Reader;
use alloc::vec::Vec;

pub fn read_u64(r: &mut Reader<'_>) -> Result<u64> {
    let mut out = 0u64;
    let mut shift = 0u32;
    for _ in 0..10 {
        let b = r.read_u8()?;
        out |= ((b & 0x7f) as u64) << shift;
        if b & 0x80 == 0 {
            return Ok(out);
        }
        shift += 7;
        if shift >= 64 {
            return Err(Error::OutOfRange { what: "leb128" });
        }
    }
    Err(Error::OutOfRange { what: "leb128" })
}

pub fn write_u64(out: &mut Vec<u8>, mut v: u64) {
    loop {
        let mut b = (v & 0x7f) as u8;
        v >>= 7;
        if v != 0 {
            b |= 0x80;
        }
        out.push(b);
        if v == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        let mut buf = Vec::new();
        write_u64(&mut buf, 300);
        let mut r = Reader::new(&buf);
        assert_eq!(read_u64(&mut r).unwrap(), 300);
    }
}
