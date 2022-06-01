use crate::metadata;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("http error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("decoding from hex: {0}")]
    FromHexError(#[from] hex::FromHexError),
    #[error("error decoding json: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("invalid metadata: {0}")]
    MetadataError(#[from] metadata::InvalidMetadataError),
    #[error("codec error {0}")]
    CodecError(#[from] codec::Error),
}
