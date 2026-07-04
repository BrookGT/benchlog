//! Integrity, range, and schema validation.


pub mod integrity;
pub mod range;
pub mod schema;
pub mod rules;
pub mod crossref;
pub mod audit;

pub use rules::{ValidationRules, apply_rules};
