use std::io;
use std::convert::From;
use std::error;
use std::fmt;
use std::result;
use std::string;
use url;
use reqwest;

use rustc_serialize::json;
use libarchive::error::ArchiveError;

pub type Result<T> = result::Result<T, NodError>;

#[derive(Debug)]
pub enum NodError {
    Io(io::Error),
    Http(reqwest::Error),
    Url(url::ParseError),
    Decode(json::DecoderError),
    Other(&'static str),
    Archive(ArchiveError),
}

impl From<io::Error> for NodError {
    fn from(err: io::Error) -> NodError {
        NodError::Io(err)
    }
}

impl From<reqwest::Error> for NodError {
    fn from(err: reqwest::Error) -> NodError {
        NodError::Http(err)
    }
}

impl From<url::ParseError> for NodError {
    fn from(err: url::ParseError) -> NodError {
        NodError::Url(err)
    }
}

impl From<string::FromUtf8Error> for NodError {
    fn from(err: string::FromUtf8Error) -> NodError {
        NodError::Other("Decoding error")
    }
}

impl From<json::DecoderError> for NodError {
    fn from(err: json::DecoderError) -> NodError {
        NodError::Decode(err)
    }
}

impl From<ArchiveError> for NodError {
    fn from(err: ArchiveError) -> NodError {
        NodError::Archive(err)
    }
}

impl fmt::Display for NodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodError::Io(ref err) => write!(f, "IO error: {}", err),
            NodError::Http(ref err) => write!(f, "Network error: {}", err),
            NodError::Decode(ref err) => write!(f, "Decode error: {}", err),
            NodError::Other(err) => write!(f, "Other error: {}", err),
            NodError::Url(ref err) => write!(f, "url error: {}", err),
            NodError::Archive(ref err) => write!(f, "Archive error: {}", err),
        }
    }
}


impl error::Error for NodError {
    fn description(&self) -> &str {
        match *self {
            NodError::Io(ref err) => err.description(),
            NodError::Http(ref err) => err.description(),
            NodError::Decode(ref err) => err.description(),
            NodError::Archive(ref err) => err.description(),
            NodError::Url(ref err) => err.description(),
            NodError::Other(err) => err,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            NodError::Io(ref err) => Some(err as &error::Error),
            NodError::Http(ref err) => Some(err as &error::Error),
            NodError::Decode(ref err) => Some(err as &error::Error),
            NodError::Archive(ref err) => Some(err as &error::Error),
            NodError::Url(ref err) => Some(err as &error::Error),
            NodError::Other(_) => None,
        }
    }
}
