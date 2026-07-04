//! Discrete events: markers, alarms, annotations.


pub mod markers;
pub mod alarms;
pub mod annotations;
pub mod classify;

pub use markers::{EventRecord, parse_events};
