//! Host/instrument monotonic counter pairing.


use crate::clock::sync::SyncPulse;
use crate::error::{Error, Result};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonoPair {
    pub host_ns: u64,
    pub instrument_tick: u64,
}

pub fn pair_monotonic(pulses: &[SyncPulse]) -> Vec<MonoPair> {
    pulses
        .iter()
        .map(|p| MonoPair {
            host_ns: p.host_mono_ns,
            instrument_tick: p.instrument_tick,
        })
        .collect()
}

pub fn find_nearest(pairs: &[MonoPair], host_ns: u64) -> Option<&MonoPair> {
    pairs.iter().min_by_key(|p| p.host_ns.abs_diff(host_ns))
}

pub fn interpolate_tick(pairs: &[MonoPair], host_ns: u64) -> Result<u64> {
    if pairs.is_empty() {
        return Err(Error::protocol("no pairs"));
    }
    if pairs.len() == 1 {
        return Ok(pairs[0].instrument_tick);
    }
    for w in pairs.windows(2) {
        if host_ns >= w[0].host_ns && host_ns <= w[1].host_ns {
            let span = w[1].host_ns.saturating_sub(w[0].host_ns);
            if span == 0 {
                return Ok(w[0].instrument_tick);
            }
            let t = (host_ns - w[0].host_ns) as f64 / span as f64;
            let tick = w[0].instrument_tick as f64
                + t * ((w[1].instrument_tick as f64) - (w[0].instrument_tick as f64));
            return Ok(tick as u64);
        }
    }
    Ok(pairs[pairs.len() - 1].instrument_tick)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nearest() {
        let pairs = [
            MonoPair { host_ns: 0, instrument_tick: 0 },
            MonoPair { host_ns: 100, instrument_tick: 10 },
        ];
        assert_eq!(find_nearest(&pairs, 90).unwrap().instrument_tick, 10);
    }
}
