//! Sample block encode for rewrite export.


use crate::codec::delta_pack;
use crate::error::Result;
use crate::sample::decode::{SampleBlock, SamplePayload};
use alloc::vec::Vec;

pub fn encode_block(block: &SampleBlock, out: &mut Vec<u8>) -> Result<()> {
    out.extend_from_slice(&block.channel_id.to_le_bytes());
    out.extend_from_slice(&block.base_ts_ns.to_le_bytes());
    let (count, kind, payload) = match &block.payload {
        SamplePayload::Int(b) => (b.values.len(), 0u8, encode_int(&b.values, block.encoding)?),
        SamplePayload::Float(b) => (b.values.len(), 1u8, encode_float(&b.values)?),
        SamplePayload::Bool(v) => (v.len(), 2u8, encode_bool(v)),
    };
    out.extend_from_slice(&(count as u32).to_le_bytes());
    out.push(kind);
    out.push(block.encoding);
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(&payload);
    Ok(())
}

fn encode_int(values: &[i32], encoding: u8) -> Result<Vec<u8>> {
    if encoding == 0 {
        let mut out = Vec::with_capacity(values.len() * 4);
        for v in values {
            out.extend_from_slice(&v.to_le_bytes());
        }
        Ok(out)
    } else {
        delta_pack::encode_i32(values)
    }
}

fn encode_float(values: &[f32]) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(values.len() * 4);
    for v in values {
        out.extend_from_slice(&v.to_le_bytes());
    }
    Ok(out)
}

fn encode_bool(values: &[bool]) -> Vec<u8> {
    let need = (values.len() + 7) / 8;
    let mut out = vec![0u8; need];
    for (i, &b) in values.iter().enumerate() {
        if b {
            out[i / 8] |= 1 << (i % 8);
        }
    }
    out
}
