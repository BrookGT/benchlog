//! Parse → validate → report ingest pipeline.


use crate::check::crossref::{crossref_events, crossref_samples};
use crate::check::integrity::verify_file;
use crate::check::rules::{ValidationRules, apply_rules};
use crate::error::Result;
use crate::frame::sections::scan_sections;
use crate::ingest::parse::{parse_capture, CaptureDoc};
use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngestReport {
    pub instrument_label: String,
    pub channels: u32,
    pub samples: u32,
    pub events: u32,
    pub sync_pulses: u32,
    pub verified: bool,
}

pub fn ingest(data: &[u8]) -> Result<IngestReport> {
    verify_file(data)?;
    let doc = parse_capture(data)?;
    drive_validate(&doc)?;
    Ok(report_from_doc(&doc))
}

fn drive_validate(doc: &CaptureDoc) -> Result<()> {
    let version = doc.header.as_ref().map(|h| h.version).unwrap_or(1);
    let payload_len = doc.samples.len(); // placeholder for section scan
    let _ = payload_len;
    let sections = if doc.header.is_some() {
        scan_sections(&[]).unwrap_or_default()
    } else {
        alloc::vec::Vec::new()
    };
    let rules = ValidationRules::default();
    apply_rules(&rules, version, &sections, &doc.channels, &doc.samples, &doc.events)?;
    crossref_samples(&doc.channels, &doc.samples)?;
    crossref_events(&doc.channels, &doc.events)?;
    Ok(())
}

fn report_from_doc(doc: &CaptureDoc) -> IngestReport {
    IngestReport {
        instrument_label: alloc::format!("inst-{}", doc.header.as_ref().map(|h| h.instrument_id).unwrap_or(0)),
        channels: doc.channels.len() as u32,
        samples: doc.samples.len() as u32,
        events: doc.events.len() as u32,
        sync_pulses: doc.sync_pulses.len() as u32,
        verified: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn short_fails() {
        assert!(ingest(&[]).is_err());
    }
}
