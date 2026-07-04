//! Consolidated validation rules for BLF ingest.

use crate::channel::descriptor::ChannelMap;
use crate::check::range::{validate_sample_ranges, validate_timestamp_monotonic};
use crate::check::schema::{SchemaExpectation, validate_schema};
use crate::error::Result;
use crate::event::markers::EventRecord;
use crate::frame::sections::SectionEnvelope;
use crate::sample::decode::SampleBlock;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct ValidationRules {
    pub schema: SchemaExpectation,
    pub sample_lo: f64,
    pub sample_hi: f64,
    pub max_events: usize,
}

impl Default for ValidationRules {
    fn default() -> ValidationRules {
        ValidationRules {
            schema: SchemaExpectation::default(),
            sample_lo: -1e9,
            sample_hi: 1e9,
            max_events: 4096,
        }
    }
}

impl ValidationRules {
    pub fn check_case_00(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_01(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_02(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_03(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_04(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_05(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_06(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_07(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_08(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_09(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_10(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_11(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_12(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_13(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_14(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_15(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_16(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_17(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_18(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_19(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_20(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_21(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_22(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_23(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_24(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_25(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_26(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_27(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_28(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
    pub fn check_case_29(&self, version: u16, sections: &[SectionEnvelope]) -> Result<()> {
        validate_schema(version, sections, &self.schema)
    }
}

pub fn apply_rules(
    rules: &ValidationRules,
    version: u16,
    sections: &[SectionEnvelope],
    channels: &ChannelMap,
    samples: &[SampleBlock],
    events: &[EventRecord],
) -> Result<()> {
    validate_schema(version, sections, &rules.schema)?;
    let _ = channels;
    for block in samples {
        validate_sample_ranges(block, rules.sample_lo, rules.sample_hi)?;
    }
    if events.len() > rules.max_events {
        return Err(crate::error::Error::CapacityLimit);
    }
    let times: Vec<u64> = events.iter().map(|e| e.ts_ns).collect();
    validate_timestamp_monotonic(&times)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_rules() {
        let r = ValidationRules::default();
        assert!(r.sample_hi > r.sample_lo);
    }
}
