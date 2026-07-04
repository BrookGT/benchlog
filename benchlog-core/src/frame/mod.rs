//! BLF frame header, trailer, and run metadata.


pub mod header;
pub mod trailer;
pub mod run_info;
pub mod sections;
pub mod validate;

pub use header::{BlfHeader, BLF_MAGIC};
pub use trailer::BlfTrailer;
pub use run_info::RunInfo;
pub use sections::{SectionKind, SectionEnvelope};
