//! Alarm threshold evaluation over sample windows.


use crate::event::markers::{EventKind, EventRecord, EventSeverity};
use crate::sample::float_batch::FloatBatch;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub struct AlarmRule {
    pub channel_id: u32,
    pub hi: f32,
    pub lo: f32,
}

pub fn evaluate_float(batch: &FloatBatch, channel_id: u32, rule: &AlarmRule) -> Vec<EventRecord> {
    let mut out = Vec::new();
    for (i, &v) in batch.values.iter().enumerate() {
        if v > rule.hi {
            out.push(EventRecord {
                ts_ns: batch.base_ts_ns.saturating_add(i as u64 * 1000),
                severity: EventSeverity::Alarm,
                kind: EventKind::ThresholdCross,
                channel_id,
                note: String::from("hi"),
            });
        } else if v < rule.lo {
            out.push(EventRecord {
                ts_ns: batch.base_ts_ns.saturating_add(i as u64 * 1000),
                severity: EventSeverity::Warning,
                kind: EventKind::ThresholdCross,
                channel_id,
                note: String::from("lo"),
            });
        }
    }
    out
}

pub fn count_by_severity(events: &[EventRecord]) -> [usize; 5] {
    let mut counts = [0usize; 5];
    for ev in events {
        let idx = ev.severity as usize;
        if idx < counts.len() {
            counts[idx] += 1;
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hi_trip() {
        let b = FloatBatch { base_ts_ns: 0, values: vec![100.0] };
        let rule = AlarmRule { channel_id: 1, hi: 50.0, lo: -50.0 };
        assert_eq!(evaluate_float(&b, 1, &rule).len(), 1);
    }
}
