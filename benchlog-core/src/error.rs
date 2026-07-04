//! Shared error type for benchlog parsers and validators.


use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    UnexpectedEof,
    Unexpected { expected: &'static str, at: usize },
    LengthOverflow { field: &'static str },
    DepthLimit,
    OutOfRange { what: &'static str },
    InvalidEncoding { scheme: &'static str },
    StaleHandle,
    Protocol { detail: &'static str },
    CapacityLimit,
    IntegrityFailed { detail: &'static str },
    SchemaMismatch { detail: &'static str },
    TimestampGap { expected: u64, found: u64 },
    ChannelUnknown { id: u32 },
    SampleKindMismatch { channel: u32 },
}

impl Error {
    pub fn unexpected(expected: &'static str, at: usize) -> Self {
        Error::Unexpected { expected, at }
    }
    pub fn protocol(detail: &'static str) -> Self {
        Error::Protocol { detail }
    }
    pub fn integrity(detail: &'static str) -> Self {
        Error::IntegrityFailed { detail }
    }
    pub fn code(&self) -> &'static str {
        match self {
            Error::UnexpectedEof => "eof",
            Error::Unexpected { .. } => "unexpected",
            Error::LengthOverflow { .. } => "length_overflow",
            Error::DepthLimit => "depth_limit",
            Error::OutOfRange { .. } => "out_of_range",
            Error::InvalidEncoding { .. } => "invalid_encoding",
            Error::StaleHandle => "stale_handle",
            Error::Protocol { .. } => "protocol",
            Error::CapacityLimit => "capacity_limit",
            Error::IntegrityFailed { .. } => "integrity_failed",
            Error::SchemaMismatch { .. } => "schema_mismatch",
            Error::TimestampGap { .. } => "timestamp_gap",
            Error::ChannelUnknown { .. } => "channel_unknown",
            Error::SampleKindMismatch { .. } => "sample_kind_mismatch",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedEof => write!(f, "unexpected end of input"),
            Error::Unexpected { expected, at } => {
                write!(f, "unexpected at {at}: expected {expected}")
            }
            Error::LengthOverflow { field } => write!(f, "length overflow in {field}"),
            Error::DepthLimit => write!(f, "depth limit exceeded"),
            Error::OutOfRange { what } => write!(f, "{what} out of range"),
            Error::InvalidEncoding { scheme } => write!(f, "invalid {scheme}"),
            Error::StaleHandle => write!(f, "stale handle"),
            Error::Protocol { detail } => write!(f, "protocol: {detail}"),
            Error::CapacityLimit => write!(f, "capacity limit"),
            Error::IntegrityFailed { detail } => write!(f, "integrity failed: {detail}"),
            Error::SchemaMismatch { detail } => write!(f, "schema mismatch: {detail}"),
            Error::TimestampGap { expected, found } => {
                write!(f, "timestamp gap expected {expected} found {found}")
            }
            Error::ChannelUnknown { id } => write!(f, "unknown channel {id}"),
            Error::SampleKindMismatch { channel } => {
                write!(f, "sample kind mismatch on channel {channel}")
            }
        }
    }
}

impl From<core::str::Utf8Error> for Error {
    fn from(_: core::str::Utf8Error) -> Self {
        Error::InvalidEncoding { scheme: "utf8" }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn codes() {
        assert_eq!(Error::StaleHandle.code(), "stale_handle");
    }
}
