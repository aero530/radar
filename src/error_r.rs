use thiserror::Error;

use crate::codes::MessageCode;

/// Everything that can go wrong parsing or plotting a NEXRAD Level 3 file.
///
/// This crate does not panic on malformed, truncated, or otherwise
/// unsupported input; every such case is reported through this type
/// instead, propagated via `?` through the `nom`-based block parsers.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Product type `{0}` is not supported")]
    ProductType(MessageCode),

    #[error("Product version is {0:?} but currently only version <= {1:?} are supported")]
    SupportedVersion(u8, Option<u8>),

    #[error("Missing command line argument input")]
    MissingInput,

    #[error("File is too short to be a NEXRAD Level 3 product: expected at least {expected} header bytes, found {actual}")]
    TooShort { expected: usize, actual: usize },

    #[error("file io error")]
    Io(#[from] std::io::Error),

    #[error("serialization / deserialization error")]
    Serde,

    #[error("Failed to convert JSON")]
    Json(#[from] serde_json::Error),

    #[error("byte error")]
    Byte,

    #[error("string error")]
    Utf8,

    #[error("Product has no symbology block to plot")]
    NoSymbologyData,

    #[error("Symbology block has no data layers to plot")]
    NoSymbologyLayers,

    #[error("Error - `{0}`")]
    Other(String),

    #[error("Error - `{0}`")]
    Nom(String),
}

impl<E> From<plotters::drawing::DrawingAreaErrorKind<E>> for Error
where
    E: std::error::Error + Send + Sync,
{
    fn from(value: plotters::drawing::DrawingAreaErrorKind<E>) -> Self {
        Error::Other(value.to_string())
    }
}

impl From<nom::Err<nom::error::Error<&[u8]>>> for Error {
    fn from(value: nom::Err<nom::error::Error<&[u8]>>) -> Self {
        Error::Nom(format!("{value:?}"))
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        Error::Other(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nom_error_conversion_does_not_recurse_forever() {
        // Regression test: this `From` impl used to be
        // `fn from(value) -> Self { value.into() }`, which is unconditional
        // infinite recursion (a stack overflow) rather than an actual
        // conversion — triggered by every `?` on a `nom` parser anywhere in
        // this crate, i.e. on any malformed input.
        let nom_error =
            nom::Err::Error(nom::error::Error::new(&b"abc"[..], nom::error::ErrorKind::Tag));
        let error: Error = nom_error.into();
        assert!(matches!(error, Error::Nom(_)));
    }

    #[test]
    fn boxed_error_conversion_preserves_the_message() {
        let boxed: Box<dyn std::error::Error> = "something specific went wrong".into();
        let error: Error = boxed.into();
        assert_eq!(error.to_string(), "Error - `something specific went wrong`");
    }
}