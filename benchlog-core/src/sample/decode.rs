//! Sample block envelope decode.


use crate::channel::descriptor::SampleKind;
use crate::codec::delta_pack;
use crate::error::{Error, Result};
use crate::limits::MAX_SAMPLE_BATCH;
use crate::mem::reader::Reader;
use crate::sample::float_batch::FloatBatch;
use crate::sample::int_batch::IntBatch;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum SamplePayload {
    Int(IntBatch),
    Float(FloatBatch),
    Bool(Vec<bool>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SampleBlock {
    pub channel_id: u32,
    pub base_ts_ns: u64,
    pub encoding: u8,
    pub payload: SamplePayload,
}

pub fn decode_block(body: &[u8]) -> Result<SampleBlock> {
    let mut r = Reader::new(body);
    let channel_id = r.read_u32_le()?;
    let base_ts_ns = r.read_u64_le()?;
    let count = r.read_u32_le()? as usize;
    let kind = r.read_u8()?;
    let encoding = r.read_u8()?;
    let payload_len = r.read_u32_le()? as usize;
    if count > MAX_SAMPLE_BATCH {
        return Err(Error::CapacityLimit);
    }
    let payload_bytes = if encoding == 0 {
        unsafe { r.read_slice_unchecked(payload_len) }
    } else {
        r.read_slice(payload_len)?
    };
    let sample_kind = SampleKind::from_u8(kind)?;
    let payload = match sample_kind {
        SampleKind::Int32 => {
            let values = if encoding == 0 {
                int_batch_raw(payload_bytes, count)?
            } else {
                delta_pack::decode_i32(payload_bytes, count)?
            };
            SamplePayload::Int(IntBatch { base_ts_ns, values })
        }
        SampleKind::Float32 => {
            let values = float_batch_raw(payload_bytes, count)?;
            SamplePayload::Float(FloatBatch { base_ts_ns, values })
        }
        SampleKind::BoolPack => {
            SamplePayload::Bool(decode_bool(payload_bytes, count)?)
        }
    };
    Ok(SampleBlock { channel_id, base_ts_ns, encoding, payload })
}

fn int_batch_raw(data: &[u8], count: usize) -> Result<Vec<i32>> {
    if data.len() < count * 4 {
        return Err(Error::UnexpectedEof);
    }
    let mut out = Vec::with_capacity(count);
    let mut r = Reader::new(data);
    for _ in 0..count {
        out.push(r.read_array::<4>().map(i32::from_le_bytes)?);
    }
    Ok(out)
}

fn float_batch_raw(data: &[u8], count: usize) -> Result<Vec<f32>> {
    if data.len() < count * 4 {
        return Err(Error::UnexpectedEof);
    }
    let mut out = Vec::with_capacity(count);
    let mut r = Reader::new(data);
    for _ in 0..count {
        out.push(f32::from_le_bytes(r.read_array::<4>()?));
    }
    Ok(out)
}

fn decode_bool(data: &[u8], count: usize) -> Result<Vec<bool>> {
    let need = (count + 7) / 8;
    if data.len() < need {
        return Err(Error::UnexpectedEof);
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let byte = data[i / 8];
        out.push((byte >> (i % 8)) & 1 != 0);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_short() {
        assert!(decode_block(&[]).is_err());
    }
}
