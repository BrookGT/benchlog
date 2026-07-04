//! Clock synchronization pulse blocks.


use crate::error::{Error, Result};
use crate::mem::reader::Reader;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyncPulse {
    pub host_mono_ns: u64,
    pub instrument_tick: u64,
    pub sequence: u32,
}

pub fn parse_one(r: &mut Reader<'_>) -> Result<SyncPulse> {
    Ok(SyncPulse {
        host_mono_ns: r.read_u64_le()?,
        instrument_tick: r.read_u64_le()?,
        sequence: r.read_u32_le()?,
    })
}

pub fn parse_sync_block(body: &[u8]) -> Result<Vec<SyncPulse>> {
    let mut r = Reader::new(body);
    let count = r.read_u32_le()? as usize;
    if count > crate::limits::MAX_CHECKPOINTS * 4 {
        return Err(Error::CapacityLimit);
    }
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        out.push(parse_one(&mut r)?);
    }
    Ok(out)
}

pub fn write_sync_block(pulses: &[SyncPulse], out: &mut Vec<u8>) {
    out.extend_from_slice(&(pulses.len() as u32).to_le_bytes());
    for p in pulses {
        out.extend_from_slice(&p.host_mono_ns.to_le_bytes());
        out.extend_from_slice(&p.instrument_tick.to_le_bytes());
        out.extend_from_slice(&p.sequence.to_le_bytes());
    }
}

pub fn latest_sequence(pulses: &[SyncPulse]) -> Option<u32> {
    pulses.iter().map(|p| p.sequence).max()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        let p = [SyncPulse { host_mono_ns: 1, instrument_tick: 2, sequence: 3 }];
        let mut buf = Vec::new();
        write_sync_block(&p, &mut buf);
        let back = parse_sync_block(&buf).unwrap();
        assert_eq!(back[0].sequence, 3);
    }
}
