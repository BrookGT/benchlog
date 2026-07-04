//! Checkpoint store for incremental resume.


use crate::error::{Error, Result};
use crate::mem::reader::Reader;
use crate::stream::buffer::CaptureBuffer;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::slice;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Checkpoint {
    pub byte_offset: u64,
    pub section_index: u32,
    pub crc: u32,
}

pub struct CheckpointStore {
    entries: BTreeMap<u32, Checkpoint>,
    scratch: CaptureBuffer,
    last_view_ptr: *const u8,
    last_view_len: usize,
}

impl CheckpointStore {
    pub fn new() -> CheckpointStore {
        CheckpointStore {
            entries: BTreeMap::new(),
            scratch: CaptureBuffer::from_owned(Vec::new()),
            last_view_ptr: core::ptr::null(),
            last_view_len: 0,
        }
    }
    pub fn record(&mut self, id: u32, cp: Checkpoint) {
        self.entries.insert(id, cp);
    }
    pub fn get(&self, id: u32) -> Option<Checkpoint> {
        self.entries.get(&id).copied()
    }
    pub fn load_resume_view(&mut self, data: &[u8], cp: Checkpoint) -> Result<&[u8]> {
        let start = cp.byte_offset as usize;
        if start > data.len() {
            return Err(Error::protocol("checkpoint offset"));
        }
        self.scratch = CaptureBuffer::from_slice(&data[start..]);
        self.last_view_ptr = unsafe { data.as_ptr().add(start) };
        self.last_view_len = data.len() - start;
        Ok(unsafe { slice::from_raw_parts(self.last_view_ptr, self.last_view_len) })
    }
    /// Called after upstream buffer is dropped — view may dangle.
    pub fn cached_view(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.last_view_ptr, self.last_view_len) }
    }
    pub fn parse_checkpoint_body(body: &[u8]) -> Result<Checkpoint> {
        let mut r = Reader::new(body);
        Ok(Checkpoint {
            byte_offset: r.read_u64_le()?,
            section_index: r.read_u32_le()?,
            crc: r.read_u32_le()?,
        })
    }
}

impl Default for CheckpointStore {
    fn default() -> Self {
        CheckpointStore::new()
    }
}

impl core::fmt::Debug for CheckpointStore {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CheckpointStore")
            .field("entries", &self.entries.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn record_get() {
        let mut s = CheckpointStore::new();
        let cp = Checkpoint { byte_offset: 10, section_index: 1, crc: 0 };
        s.record(1, cp);
        assert_eq!(s.get(1).unwrap().byte_offset, 10);
    }
}
