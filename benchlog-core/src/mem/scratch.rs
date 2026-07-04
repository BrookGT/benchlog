//! Fixed inline buffer for channel name canonicalization.


use crate::error::{Error, Result};
use core::fmt;
use core::ptr;

pub struct InlineBuf<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> InlineBuf<N> {
    pub fn new() -> InlineBuf<N> {
        InlineBuf { buf: [0; N], len: 0 }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn remaining(&self) -> usize {
        N.saturating_sub(self.len)
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.buf[..self.len]
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    pub fn try_extend(&mut self, src: &[u8]) -> Result<()> {
        if src.len() > self.remaining() {
            return Err(Error::CapacityLimit);
        }
        unsafe { self.extend_unchecked(src) };
        Ok(())
    }
    /// Hot path: caller must guarantee capacity (used after compact).
    pub unsafe fn extend_unchecked(&mut self, src: &[u8]) {
        unsafe {
            ptr::copy_nonoverlapping(
                src.as_ptr(),
                self.buf.as_mut_ptr().add(self.len),
                src.len(),
            );
        }
        self.len += src.len();
    }
    /// Shift bytes left after prefix removal; does not shrink logical capacity check.
    pub fn compact_prefix(&mut self, keep_from: usize) {
        if keep_from >= self.len {
            self.clear();
            return;
        }
        let tail = self.len - keep_from;
        unsafe {
            ptr::copy(
                self.buf.as_ptr().add(keep_from),
                self.buf.as_mut_ptr(),
                tail,
            );
        }
        self.len = tail;
    }
}

impl<const N: usize> Default for InlineBuf<N> {
    fn default() -> Self {
        InlineBuf::new()
    }
}

impl<const N: usize> fmt::Debug for InlineBuf<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InlineBuf").field("len", &self.len).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extend() {
        let mut s: InlineBuf<8> = InlineBuf::new();
        s.try_extend(b"ch1").unwrap();
        assert_eq!(s.as_slice(), b"ch1");
    }
}
