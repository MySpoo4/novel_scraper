use std::{fmt, result};

#[derive(Debug)]
pub enum Error {
    CommandError { cmd: String, message: String },
    ElementError { selector: String },
    AttributeError { selector: String, attr: String },
    ClientBuildError,
    ProxyError,
    EpubBuildError,
    ChapterError,
    RequestError { url: String, message: String },
    Error { message: String },
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CommandError { cmd, message } => write!(
                f,
                "Failed to run command: '{}'. Message: '{}'",
                cmd, message
            ),
            Error::ElementError { selector } => write!(f, "Failed to find element: '{}'", selector),
            Error::AttributeError { selector, attr } => write!(
                f,
                "Failed to retrieve attribute: '{}' from selector: '{}'.",
                attr, selector
            ),
            Error::ClientBuildError => write!(f, "Failed to build http client!"),
            Error::ProxyError => write!(f, "Failed to parse proxy url!"),
            Error::EpubBuildError => write!(f, "Failed to build epub!"),
            Error::ChapterError => write!(f, "Failed to write chapter!"),
            Error::RequestError { url, message } => {
                write!(f, "Failed to url: '{}'. Message: '{}'", url, message)
            }
            Error::Error { message } => write!(f, "Error: {}", message),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
