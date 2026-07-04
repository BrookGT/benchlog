//! Wire codecs: LEB128, delta packing, zlib stub.


pub mod leb128;
pub mod delta_pack;
pub mod zlib_stub;

pub use leb128::{read_u64, write_u64};
