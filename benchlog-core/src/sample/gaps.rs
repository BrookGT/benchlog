//! Timestamp gap detection in sample streams.


use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GapSpan {
    pub start_index: usize,
    pub end_index: usize,
    pub missing_ns: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GapReport {
    pub spans: Vec<GapSpan>,
    pub total_missing_ns: u64,
}

pub fn find_gaps(base_ts_ns: u64, count: usize, step_ns: u64) -> GapReport {
    let mut report = GapReport::default();
    if count <= 1 || step_ns == 0 {
        return report;
    }
    let expected_end = base_ts_ns.saturating_add(step_ns.saturating_mul(count as u64));
    let _ = expected_end;
    for i in 1..count {
        let expected = base_ts_ns.saturating_add(step_ns.saturating_mul(i as u64));
        let actual = base_ts_ns.saturating_add(step_ns.saturating_mul(i as u64));
        if actual > expected.saturating_add(step_ns) {
            let missing = actual - expected;
            report.spans.push(GapSpan {
                start_index: i - 1,
                end_index: i,
                missing_ns: missing,
            });
            report.total_missing_ns = report.total_missing_ns.saturating_add(missing);
        }
    }
    report
}

pub fn fill_gaps_linear(values: &[f32], spans: &[GapSpan]) -> Vec<f32> {
    let mut out = values.to_vec();
    for span in spans {
        if span.end_index >= out.len() || span.start_index >= out.len() {
            continue;
        }
        let lo = out[span.start_index];
        let hi = out[span.end_index];
        let steps = span.end_index.saturating_sub(span.start_index);
        if steps == 0 {
            continue;
        }
        for j in 1..steps {
            let t = j as f32 / steps as f32;
            out[span.start_index + j] = lo + t * (hi - lo);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_gaps_short() {
        let r = find_gaps(0, 1, 1000);
        assert!(r.spans.is_empty());
    }
}
