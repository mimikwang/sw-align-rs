//! Custom errors for the crate

/// Shorthand for Result with crate error
pub type Result<T> = std::result::Result<T, Error>;

/// Custom error type for crate
pub type Error = &'static str;

/// Out of bounds error
pub const ERR_OUT_OF_BOUNDS: Error = "out of bounds";

/// Not found error
pub const ERR_NOT_FOUND: Error = "not found";

/// Catch all for all other error types
pub const ERR_CATCH_ALL: Error = "an error occurred";
