//! Operator annotation merge and deduplication.


use crate::event::markers::{EventKind, EventRecord};
use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;

pub fn filter_operator_notes(events: &[EventRecord]) -> Vec<&EventRecord> {
    events.iter().filter(|e| e.kind == EventKind::OperatorNote).collect()
}

pub fn dedupe_notes(events: &[EventRecord]) -> Vec<EventRecord> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for ev in events {
        if ev.kind != EventKind::OperatorNote {
            out.push(ev.clone());
            continue;
        }
        if seen.insert(ev.note.clone()) {
            out.push(ev.clone());
        }
    }
    out
}

pub fn merge_notes(a: &[EventRecord], b: &[EventRecord]) -> Vec<EventRecord> {
    let mut out = a.to_vec();
    out.extend_from_slice(b);
    out.sort_by_key(|e| e.ts_ns);
    dedupe_notes(&out)
}

pub fn annotate_run(label: &str, ts_ns: u64) -> EventRecord {
    EventRecord {
        ts_ns,
        severity: crate::event::markers::EventSeverity::Info,
        kind: EventKind::OperatorNote,
        channel_id: 0,
        note: String::from(label),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::markers::EventSeverity;
    #[test]
    fn dedupe() {
        let ev = annotate_run("note", 0);
        let d = dedupe_notes(&[ev.clone(), ev]);
        assert_eq!(d.len(), 1);
    }
}
