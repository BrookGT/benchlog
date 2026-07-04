//! Forward read cursor over capture wire bytes.


use crate::error::{Error, Result};
use core::fmt;

#[derive(Clone)]
pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Reader<'a> {
        Reader { data, pos: 0 }
    }
    pub fn position(&self) -> usize {
        self.pos
    }
    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }
    pub fn seek(&mut self, pos: usize) -> Result<()> {
        if pos > self.data.len() {
            return Err(Error::unexpected("seek", pos));
        }
        self.pos = pos;
        Ok(())
    }
    pub fn read_u8(&mut self) -> Result<u8> {
        self.data.get(self.pos).copied().ok_or(Error::UnexpectedEof).map(|b| {
            self.pos += 1;
            b
        })
    }
    pub fn read_u16_le(&mut self) -> Result<u16> {
        let s = self.read_array::<2>()?;
        Ok(u16::from_le_bytes(s))
    }
    pub fn read_u32_le(&mut self) -> Result<u32> {
        let s = self.read_array::<4>()?;
        Ok(u32::from_le_bytes(s))
    }
    pub fn read_u64_le(&mut self) -> Result<u64> {
        let s = self.read_array::<8>()?;
        Ok(u64::from_le_bytes(s))
    }
    pub fn read_f64_le(&mut self) -> Result<f64> {
        Ok(f64::from_le_bytes(self.read_array::<8>()?))
    }
    pub fn read_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        if self.remaining() < N {
            return Err(Error::UnexpectedEof);
        }
        let mut out = [0u8; N];
        out.copy_from_slice(&self.data[self.pos..self.pos + N]);
        self.pos += N;
        Ok(out)
    }
    pub fn read_slice(&mut self, n: usize) -> Result<&'a [u8]> {
        if n > self.remaining() {
            return Err(Error::UnexpectedEof);
        }
        let s = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }
    pub unsafe fn read_slice_unchecked(&mut self, n: usize) -> &'a [u8] {
        let s = unsafe { self.data.get_unchecked(self.pos..self.pos + n) };
        self.pos += n;
        s
    }
}

impl<'a> fmt::Debug for Reader<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Reader")
            .field("pos", &self.pos)
            .field("len", &self.data.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reads() {
        let mut r = Reader::new(&[1, 2, 3, 4]);
        assert_eq!(r.read_u16_le().unwrap(), 0x0201);
    }
}
