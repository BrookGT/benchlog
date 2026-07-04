//! Capture buffer with zero-copy borrow tracking.


use crate::error::{Error, Result};
use core::slice;

pub struct CaptureBuffer {
    owned: alloc::vec::Vec<u8>,
    borrowed_ptr: *const u8,
    borrowed_len: usize,
    use_borrow: bool,
}

impl CaptureBuffer {
    pub fn from_slice(data: &[u8]) -> CaptureBuffer {
        CaptureBuffer {
            owned: alloc::vec::Vec::new(),
            borrowed_ptr: data.as_ptr(),
            borrowed_len: data.len(),
            use_borrow: true,
        }
    }
    pub fn from_owned(data: alloc::vec::Vec<u8>) -> CaptureBuffer {
        let len = data.len();
        CaptureBuffer {
            borrowed_ptr: data.as_ptr(),
            borrowed_len: len,
            owned: data,
            use_borrow: false,
        }
    }
    pub fn len(&self) -> usize {
        if self.use_borrow {
            self.borrowed_len
        } else {
            self.owned.len()
        }
    }
    pub fn as_slice(&self) -> &[u8] {
        if self.use_borrow {
            unsafe { slice::from_raw_parts(self.borrowed_ptr, self.borrowed_len) }
        } else {
            &self.owned
        }
    }
    pub fn materialize(&mut self) {
        if !self.use_borrow {
            return;
        }
        let copy = self.as_slice().to_vec();
        self.owned = copy;
        self.borrowed_ptr = self.owned.as_ptr();
        self.borrowed_len = self.owned.len();
        self.use_borrow = false;
    }
    pub fn invalidate_borrow(&mut self) {
        self.use_borrow = false;
        self.borrowed_len = 0;
    }
}

impl core::fmt::Debug for CaptureBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CaptureBuffer").field("len", &self.len()).finish()
    }
}

unsafe impl Send for CaptureBuffer {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn borrow_len() {
        let b = CaptureBuffer::from_slice(b"abc");
        assert_eq!(b.len(), 3);
    }
}
