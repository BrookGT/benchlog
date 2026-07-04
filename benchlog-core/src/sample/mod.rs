//! Sample batch decode, gaps, and interpolation.


pub mod int_batch;
pub mod float_batch;
pub mod gaps;
pub mod interpolate;
pub mod decode;
pub mod encode;
pub mod windows;

pub use decode::{SampleBlock, decode_block};
pub use encode::encode_block;
