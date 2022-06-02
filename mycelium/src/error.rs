use crate::types::metadata;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Http error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Decoding from hex: {0}")]
    FromHexError(#[from] hex::FromHexError),
    #[error("Error decoding json: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Invalid metadata: {0}")]
    InvalidMetadataError(#[from] metadata::InvalidMetadataError),
    #[error("Metadata error: {0}")]
    MetadataError(#[from] metadata::MetadataError),
    #[error("Codec error: {0}")]
    CodecError(#[from] codec::Error),
}
