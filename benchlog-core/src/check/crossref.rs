//! Cross-reference channel ids across sections.


use crate::channel::descriptor::ChannelMap;
use crate::error::{Error, Result};
use crate::event::markers::EventRecord;
use crate::sample::decode::SampleBlock;

pub fn crossref_samples(map: &ChannelMap, samples: &[SampleBlock]) -> Result<()> {
    for block in samples {
        if map.get(block.channel_id).is_none() {
            return Err(Error::ChannelUnknown { id: block.channel_id });
        }
    }
    Ok(())
}

pub fn crossref_events(map: &ChannelMap, events: &[EventRecord]) -> Result<()> {
    for ev in events {
        if ev.channel_id != 0 && map.get(ev.channel_id).is_none() {
            return Err(Error::ChannelUnknown { id: ev.channel_id });
        }
    }
    Ok(())
}

pub fn orphan_channels(map: &ChannelMap, samples: &[SampleBlock]) -> alloc::vec::Vec<u32> {
    let mut used = alloc::collections::BTreeSet::new();
    for b in samples {
        used.insert(b.channel_id);
    }
    map.channels
        .iter()
        .filter(|c| !used.contains(&c.id))
        .map(|c| c.id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_map_sample_fails() {
        let m = ChannelMap::new();
        let s = SampleBlock {
            channel_id: 1,
            base_ts_ns: 0,
            encoding: 0,
            payload: crate::sample::decode::SamplePayload::Bool(alloc::vec::Vec::new()),
        };
        assert!(crossref_samples(&m, &[s]).is_err());
    }
}
