//! CSV export of channel samples and events.


use crate::channel::descriptor::ChannelMap;
use crate::error::Result;
use crate::event::markers::EventRecord;
use crate::sample::decode::{SampleBlock, SamplePayload};
use alloc::string::String;
use alloc::vec::Vec;

pub fn export_csv(
    channels: &ChannelMap,
    samples: &[SampleBlock],
    events: &[EventRecord],
) -> Result<String> {
    let mut out = String::from("ts_ns,channel,value,note\n");
    for block in samples {
        let name = channels
            .get(block.channel_id)
            .map(|c| c.name.as_str())
            .unwrap_or("unknown");
        match &block.payload {
            SamplePayload::Int(b) => {
                for (i, v) in b.values.iter().enumerate() {
                    let ts = block.base_ts_ns.saturating_add(i as u64 * 1000);
                    out.push_str(&format!("{ts},{name},{v},\n"));
                }
            }
            SamplePayload::Float(b) => {
                for (i, v) in b.values.iter().enumerate() {
                    let ts = block.base_ts_ns.saturating_add(i as u64 * 1000);
                    out.push_str(&format!("{ts},{name},{v},\n"));
                }
            }
            SamplePayload::Bool(v) => {
                for (i, b) in v.iter().enumerate() {
                    let ts = block.base_ts_ns.saturating_add(i as u64 * 1000);
                    out.push_str(&format!("{ts},{name},{b},\n"));
                }
            }
        }
    }
    for ev in events {
        out.push_str(&format!("{},event,{},{}\n", ev.ts_ns, ev.channel_id, ev.note));
    }
    Ok(out)
}

pub fn escape_field(s: &str) -> String {
    if s.contains(',') || s.contains('"') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        String::from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn escape_quote() {
        assert_eq!(escape_field("a,b"), "\"a,b\"");
    }
}
