// src/error/mod.rs
use std::fmt;
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum NtpError {
    Io(io::Error),
    Serde(bincode::Error),
    InvalidResponse,
    Timeout,
    Unknown(String),
}

impl fmt::Display for NtpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NtpError::Io(err) => write!(f, "IO error: {}", err),
            NtpError::Serde(err) => write!(f, "Serialization/Deserialization error: {}", err),
            NtpError::InvalidResponse => write!(f, "Invalid response received"),
            NtpError::Timeout => write!(f, "Timeout occurred"),
            NtpError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for NtpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NtpError::Io(err) => Some(err),
            NtpError::Serde(err) => Some(err),
            NtpError::InvalidResponse => None,
            NtpError::Timeout => None,
            NtpError::Unknown(_) => None,
        }
    }
}

impl From<io::Error> for NtpError {
    fn from(err: io::Error) -> Self {
        NtpError::Io(err)
    }
}

impl From<bincode::Error> for NtpError {
    fn from(err: bincode::Error) -> Self {
        NtpError::Serde(err)
    }
}

impl From<FromUtf8Error> for NtpError {
    fn from(err: FromUtf8Error) -> Self {
        NtpError::Unknown(err.to_string())
    }
}
