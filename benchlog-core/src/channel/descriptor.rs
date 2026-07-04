//! Channel map parse and serialization.


use crate::channel::scaling::LinearScale;
use crate::channel::units::UnitCode;
use crate::error::{Error, Result};
use crate::limits::{MAX_CHANNEL_NAME, MAX_CHANNELS};
use crate::mem::reader::Reader;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SampleKind {
    Int32 = 0,
    Float32 = 1,
    BoolPack = 2,
}

impl SampleKind {
    pub fn from_u8(v: u8) -> Result<SampleKind> {
        match v {
            0 => Ok(SampleKind::Int32),
            1 => Ok(SampleKind::Float32),
            2 => Ok(SampleKind::BoolPack),
            _ => Err(Error::protocol("sample kind")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelDesc {
    pub id: u32,
    pub kind: SampleKind,
    pub unit: UnitCode,
    pub scale: LinearScale,
    pub name: String,
}

#[derive(Debug, Clone, Default)]
pub struct ChannelMap {
    pub channels: Vec<ChannelDesc>,
}

impl ChannelMap {
    pub fn new() -> ChannelMap {
        ChannelMap { channels: Vec::new() }
    }
    pub fn len(&self) -> usize {
        self.channels.len()
    }
    pub fn get(&self, id: u32) -> Option<&ChannelDesc> {
        self.channels.iter().find(|c| c.id == id)
    }
    pub fn push(&mut self, desc: ChannelDesc) -> Result<()> {
        if self.channels.len() >= MAX_CHANNELS {
            return Err(Error::CapacityLimit);
        }
        if self.get(desc.id).is_some() {
            return Err(Error::protocol("duplicate channel id"));
        }
        self.channels.push(desc);
        Ok(())
    }
}

pub fn parse_one(r: &mut Reader<'_>) -> Result<ChannelDesc> {
    let id = r.read_u32_le()?;
    let kind = SampleKind::from_u8(r.read_u8()?)?;
    let unit = UnitCode::from_u16(r.read_u16_le()?)?;
    let factor = r.read_f64_le()?;
    let offset = r.read_f64_le()?;
    let name_len = r.read_u16_le()? as usize;
    if name_len > MAX_CHANNEL_NAME {
        return Err(Error::LengthOverflow { field: "channel name" });
    }
    let name_bytes = r.read_slice(name_len)?;
    let name = core::str::from_utf8(name_bytes)?.into();
    Ok(ChannelDesc {
        id,
        kind,
        unit,
        scale: LinearScale { factor, offset },
        name,
    })
}

pub fn parse_map(body: &[u8]) -> Result<ChannelMap> {
    let mut r = Reader::new(body);
    let count = r.read_u32_le()? as usize;
    if count > MAX_CHANNELS {
        return Err(Error::CapacityLimit);
    }
    let mut map = ChannelMap::new();
    for _ in 0..count {
        map.push(parse_one(&mut r)?)?;
    }
    Ok(map)
}

pub fn write_map(map: &ChannelMap, out: &mut Vec<u8>) -> Result<()> {
    out.extend_from_slice(&(map.len() as u32).to_le_bytes());
    for ch in &map.channels {
        out.extend_from_slice(&ch.id.to_le_bytes());
        out.push(ch.kind as u8);
        out.extend_from_slice(&(ch.unit as u16).to_le_bytes());
        out.extend_from_slice(&ch.scale.factor.to_le_bytes());
        out.extend_from_slice(&ch.scale.offset.to_le_bytes());
        out.extend_from_slice(&(ch.name.len() as u16).to_le_bytes());
        out.extend_from_slice(ch.name.as_bytes());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_map() {
        let m = ChannelMap::new();
        let mut buf = Vec::new();
        write_map(&m, &mut buf).unwrap();
        let back = parse_map(&buf).unwrap();
        assert_eq!(back.len(), 0);
    }
}
