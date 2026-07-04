//! Memory substrate for zero-copy BLF parsing.


pub mod blob;
pub mod reader;
pub mod scratch;
pub mod slab;

pub use blob::{Blob, BlobMut};
pub use reader::Reader;
pub use scratch::InlineBuf;
pub use slab::Slab;
