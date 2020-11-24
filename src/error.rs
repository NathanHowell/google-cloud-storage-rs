#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Google {
        #[from]
        source: crate::google::error::ErrorResponse,
        #[cfg(feature = "backtrace")]
        backtrace: Backtrace,
    },
    #[cfg(feature = "gouth")]
    #[error(transparent)]
    Gouth {
        #[from]
        source: gouth::Error,
        #[cfg(feature = "backtrace")]
        backtrace: Backtrace,
    },
    #[error(transparent)]
    IOError {
        #[from]
        source: std::io::Error,
        #[cfg(feature = "backtrace")]
        backtrace: Backtrace,
    },
    #[error(transparent)]
    InvalidHeaderValue {
        #[from]
        source: reqwest::header::InvalidHeaderValue,
        #[cfg(feature = "backtrace")]
        backtrace: Backtrace,
    },
    #[error(transparent)]
    Reqwest {
        #[from]
        source: reqwest::Error,
        #[cfg(feature = "backtrace")]
        backtrace: Backtrace,
    },
    #[error(transparent)]
    Serialization {
        #[from]
        source: serde_json::error::Error,
        #[cfg(feature = "backtrace")]
        backtrace: Backtrace,
    },
    #[error(transparent)]
    Url {
        #[from]
        source: url::ParseError,
        #[cfg(feature = "backtrace")]
        backtrace: Backtrace,
    },
    #[error("Invalid request url {url}")]
    InvalidRequestUrl {
        url: Url,
        #[cfg(feature = "backtrace")]
        backtrace: Backtrace,
    },
    #[error(transparent)]
    Other {
        #[from]
        source: Box<dyn std::error::Error>,
        #[cfg(feature = "backtrace")]
        backtrace: Backtrace,
    },
}
