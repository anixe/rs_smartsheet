use dto::Error as DtoError;
use reqwest::Error as ReqwestError;
use serde_json::error::Error as SerdeJsonError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Network(String),
    InvalidJson(String),
    InvalidSheetName(String),
    SmartsheetOther { code: u64, message: String },
    Other(String),
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Error {
        Error::Network(error.to_string())
    }
}

impl From<SerdeJsonError> for Error {
    fn from(error: SerdeJsonError) -> Error {
        Error::InvalidJson(error.to_string())
    }
}

impl<'a> From<&'a str> for Error {
    fn from(error: &'a str) -> Error {
        Error::Other(error.to_string())
    }
}

impl From<DtoError> for Error {
    fn from(error: DtoError) -> Error {
        Error::SmartsheetOther {
            code: error.get_code(),
            message: error.into_message(),
        }
    }
}
