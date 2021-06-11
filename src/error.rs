//! The error type of the library

use std::{
    result::Result as StdResult,
};

use thiserror::Error as ThisError;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;

/// A result alias over the errors of this crate
pub type Result<T> = StdResult<T, Error>;

/// A general error that is used in this crate.
#[derive(Debug, ThisError)]
pub enum Error {
    /// An error that arises from a failure in Reqwest.
    #[error("failed to connect to the twitter API: {0}")]
    ReqwestError(#[from] ReqwestError),
    /// An error that arises from a failure in parsing Json.
    /// If the inside is Some, then there is aditional information on why it couldn't be parsed
    #[error("failed to parse json correctly")]
    JsonError(Option<JsonError>)
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::JsonError(Some(err))
    }
}
