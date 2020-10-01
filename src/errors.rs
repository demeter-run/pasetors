#[derive(Debug, PartialEq)]
/// Errors for token operations.
pub enum Errors {
    /// Error for a token with an invalid format.
    TokenFormatError,
    /// Error for a failed Base64 (URL-safe without padding) decoding.
    Base64DecodingError,
    /// Error for a failed token validation
    TokenValidationError,
    /// Error for an invalid key.
    KeyError,
    /// Error for a failed encryption operation.
    EncryptError,
    /// Error for a failed attempt to generate bytes using a CSPRNG.
    CsprngError,
    /// Error for a conversion that would be lossy.
    LossyConversionError,
}

impl From<base64::DecodeError> for Errors {
    fn from(_: base64::DecodeError) -> Self {
        Errors::Base64DecodingError
    }
}

impl From<rand_core::Error> for Errors {
    fn from(_: rand_core::Error) -> Self {
        Errors::CsprngError
    }
}

impl From<core::num::TryFromIntError> for Errors {
    fn from(_: core::num::TryFromIntError) -> Self {
        Errors::LossyConversionError
    }
}
