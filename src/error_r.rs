use thiserror::Error;

use crate::codes::MessageCode;

/// error definitions
#[derive(Error, Debug)]
pub enum Error {
    #[error("Product type `{0}` is not supported")]
    ProductType(MessageCode),

    #[error("Product version is {0:?} but currently only version <= {1:?} are supported")]
    SupportedVersion(u8, Option<u8>),

    #[error("Missing command line argument input")]
    MissingInput,

    #[error("file io error")]
    FileError(#[from] std::io::Error),
    
    #[error("serialization / deserialization error")]
    Serde,
    
    #[error("Failed to convert JSON")]
    Json(#[from] serde_json::Error),

    #[error("byte error")]
    ByteError,
    
    #[error("string error")]
    StringError,

    #[error("Error - `{0}`")]
    Other(String),

    #[error("Error - `{0}`")]
    Nom(String),
}

impl<E> From<plotters::drawing::DrawingAreaErrorKind<E>> for Error where
E: std::error::Error + Send + Sync,
{
    fn from(value: plotters::drawing::DrawingAreaErrorKind<E>) -> Self {
        value.into()
    }
}

impl From<nom::Err<nom::error::Error<&[u8]>>> for Error {
    fn from(value: nom::Err<nom::error::Error<&[u8]>>) -> Self {
        value.into()
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(_value: Box<dyn std::error::Error>) -> Self {
        
        Error::Other("other error".to_owned())
    }
}