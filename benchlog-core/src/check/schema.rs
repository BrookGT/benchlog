//! BLF schema version and section presence checks.


use crate::error::{Error, Result};
use crate::frame::sections::{SectionEnvelope, SectionKind};

#[derive(Debug, Clone, Copy)]
pub struct SchemaExpectation {
    pub min_version: u16,
    pub require_run_info: bool,
    pub require_channel_map: bool,
}

impl Default for SchemaExpectation {
    fn default() -> SchemaExpectation {
        SchemaExpectation {
            min_version: 1,
            require_run_info: true,
            require_channel_map: true,
        }
    }
}

pub fn validate_schema(version: u16, sections: &[SectionEnvelope], expect: &SchemaExpectation) -> Result<()> {
    if version < expect.min_version {
        return Err(Error::SchemaMismatch { detail: "version" });
    }
    let has = |k: SectionKind| sections.iter().any(|s| s.kind == k);
    if expect.require_run_info && !has(SectionKind::RunInfo) {
        return Err(Error::SchemaMismatch { detail: "run_info" });
    }
    if expect.require_channel_map && !has(SectionKind::ChannelMap) {
        return Err(Error::SchemaMismatch { detail: "channel_map" });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_too_old() {
        let e = SchemaExpectation::default();
        assert!(validate_schema(0, &[], &e).is_err());
    }
}
