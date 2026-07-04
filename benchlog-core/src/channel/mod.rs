//! Channel descriptors, SI units, and scaling.


pub mod descriptor;
pub mod units;
pub mod scaling;
pub mod presets;
pub mod calibration;

pub use descriptor::{ChannelDesc, ChannelMap};
pub use units::UnitCode;
pub use scaling::{LinearScale, apply_scale};
