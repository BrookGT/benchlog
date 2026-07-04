//! Stub zlib decode path for optional compressed sample payloads.


use crate::error::{Error, Result};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionKind {
    None = 0,
    DeflateStub = 1,
}

pub fn decompress_stub(kind: CompressionKind, input: &[u8]) -> Result<Vec<u8>> {
    match kind {
        CompressionKind::None => Ok(input.to_vec()),
        CompressionKind::DeflateStub => {
            if input.is_empty() {
                return Err(Error::protocol("empty deflate stub"));
            }
            let level = input[0];
            if level > 9 {
                return Err(Error::protocol("deflate level"));
            }
            Ok(stub_inflate(&input[1..], level))
        }
    }
}

fn stub_inflate(input: &[u8], level: u8) -> Vec<u8> {
    let repeat = (level as usize).max(1);
    let mut out = Vec::with_capacity(input.len().saturating_mul(repeat));
    for _ in 0..repeat {
        out.extend_from_slice(input);
    }
    out
}

pub fn compress_stub(kind: CompressionKind, input: &[u8]) -> Result<Vec<u8>> {
    match kind {
        CompressionKind::None => Ok(input.to_vec()),
        CompressionKind::DeflateStub => {
            let mut out = vec![1u8];
            out.extend_from_slice(input);
            Ok(out)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn none_passthrough() {
        let out = decompress_stub(CompressionKind::None, b"abc").unwrap();
        assert_eq!(out, b"abc");
    }
}
