//! Unified timeline construction from sections.


use crate::clock::sync::SyncPulse;
use crate::event::markers::EventRecord;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineEntryKind {
    Sample,
    Event,
    Sync,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimelineEntry {
    pub ts_ns: u64,
    pub kind: TimelineEntryKind,
    pub index: u32,
}

pub fn build_timeline(events: &[EventRecord], syncs: &[SyncPulse]) -> Vec<TimelineEntry> {
    let mut out = Vec::new();
    for (i, ev) in events.iter().enumerate() {
        out.push(TimelineEntry {
            ts_ns: ev.ts_ns,
            kind: TimelineEntryKind::Event,
            index: i as u32,
        });
    }
    for (i, s) in syncs.iter().enumerate() {
        out.push(TimelineEntry {
            ts_ns: s.host_mono_ns,
            kind: TimelineEntryKind::Sync,
            index: i as u32,
        });
    }
    out.sort_by_key(|e| e.ts_ns);
    out
}

pub fn span_ns(entries: &[TimelineEntry]) -> u64 {
    if entries.len() < 2 {
        return 0;
    }
    entries.last().unwrap().ts_ns.saturating_sub(entries[0].ts_ns)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_span() {
        assert_eq!(span_ns(&[]), 0);
    }
}
