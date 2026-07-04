//! Reference-counted byte blob for capture payloads.


use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ops::Deref;

#[derive(Clone)]
pub struct BlobMut {
    inner: Vec<u8>,
}

impl BlobMut {
    pub fn new() -> BlobMut {
        BlobMut { inner: Vec::new() }
    }
    pub fn with_capacity(cap: usize) -> BlobMut {
        BlobMut { inner: Vec::with_capacity(cap) }
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }
    pub fn extend_from_slice(&mut self, src: &[u8]) {
        self.inner.extend_from_slice(src);
    }
    pub fn push(&mut self, b: u8) {
        self.inner.push(b);
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    pub fn freeze(self) -> Blob {
        Blob::from_arc(Arc::new(self.inner))
    }
}

impl Default for BlobMut {
    fn default() -> Self {
        BlobMut::new()
    }
}

impl Deref for BlobMut {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.inner
    }
}

#[derive(Clone)]
pub struct Blob {
    store: Arc<Vec<u8>>,
    off: usize,
    len: usize,
}

impl Blob {
    fn from_arc(store: Arc<Vec<u8>>) -> Blob {
        let len = store.len();
        Blob { store, off: 0, len }
    }
    pub fn from_vec(v: Vec<u8>) -> Blob {
        Blob::from_arc(Arc::new(v))
    }
    pub fn copy_from_slice(s: &[u8]) -> Blob {
        Blob::from_vec(s.to_vec())
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.store[self.off..self.off + self.len]
    }
    pub fn slice(&self, start: usize, end: usize) -> Blob {
        assert!(start <= end && end <= self.len);
        Blob {
            store: Arc::clone(&self.store),
            off: self.off + start,
            len: end - start,
        }
    }
}

impl Deref for Blob {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl core::fmt::Debug for BlobMut {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlobMut")
            .field("len", &self.len())
            .field("cap", &self.capacity())
            .finish()
    }
}

impl core::fmt::Debug for Blob {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Blob").field("len", &self.len).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn slice_shares() {
        let b = Blob::copy_from_slice(b"capture");
        assert_eq!(&*b.slice(0, 3), b"cap");
    }
}
