//! Delta and delta-of-deltas integer packing.


use crate::error::{Error, Result};
use crate::mem::reader::Reader;
use alloc::vec::Vec;

pub fn encode_i32(values: &[i32]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    let mut prev = 0i32;
    for &v in values {
        let delta = v.wrapping_sub(prev);
        out.extend_from_slice(&delta.to_le_bytes());
        prev = v;
    }
    Ok(out)
}

pub fn decode_i32(data: &[u8], count: usize) -> Result<Vec<i32>> {
    if data.len() < count * 4 {
        return Err(Error::UnexpectedEof);
    }
    let mut r = Reader::new(data);
    let mut out = Vec::with_capacity(count);
    let mut prev = 0i32;
    for _ in 0..count {
        let delta = i32::from_le_bytes(r.read_array::<4>()?);
        let v = prev.wrapping_add(delta);
        out.push(v);
        prev = v;
    }
    Ok(out)
}

pub fn encode_dod(values: &[i32]) -> Result<Vec<u8>> {
    if values.is_empty() {
        return Ok(Vec::new());
    }
    let mut deltas = Vec::with_capacity(values.len());
    let mut prev = values[0];
    deltas.push(prev);
    for &v in &values[1..] {
        deltas.push(v.wrapping_sub(prev));
        prev = v;
    }
    let mut dod = Vec::with_capacity(deltas.len());
    let mut prev_d = deltas[0];
    dod.push(prev_d);
    for &d in &deltas[1..] {
        dod.push(d.wrapping_sub(prev_d));
        prev_d = d;
    }
    let mut out = Vec::new();
    for v in dod {
        out.extend_from_slice(&v.to_le_bytes());
    }
    Ok(out)
}

pub fn decode_dod(data: &[u8], count: usize) -> Result<Vec<i32>> {
    if count == 0 {
        return Ok(Vec::new());
    }
    if data.len() < count * 4 {
        return Err(Error::UnexpectedEof);
    }
    let mut r = Reader::new(data);
    let mut dod = Vec::with_capacity(count);
    for _ in 0..count {
        dod.push(i32::from_le_bytes(r.read_array::<4>()?));
    }
    let mut deltas = Vec::with_capacity(count);
    deltas.push(dod[0]);
    for i in 1..count {
        deltas.push(dod[i].wrapping_add(deltas[i - 1]));
    }
    let mut values = Vec::with_capacity(count);
    values.push(deltas[0]);
    for i in 1..count {
        values.push(deltas[i].wrapping_add(values[i - 1]));
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn delta_roundtrip() {
        let v = vec![10, 12, 15];
        let enc = encode_i32(&v).unwrap();
        assert_eq!(decode_i32(&enc, 3).unwrap(), v);
    }
}
