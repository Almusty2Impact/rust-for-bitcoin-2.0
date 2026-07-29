use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error type shared by the RPC transport and all labs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabError {
    /// `bitcoin-cli` could not be started.
    Spawn(String),
    /// Bitcoin Core returned a non-zero exit status.
    Rpc(String),
    /// A successful RPC response could not be interpreted.
    Parse(String),
    /// A required field was absent from a response.
    MissingField(&'static str),
}

impl Display for LabError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spawn(message) => write!(formatter, "could not start bitcoin-cli: {message}"),
            Self::Rpc(message) => write!(formatter, "Bitcoin Core RPC failed: {message}"),
            Self::Parse(message) => write!(formatter, "could not parse RPC response: {message}"),
            Self::MissingField(field) => write!(formatter, "RPC response is missing `{field}`"),
        }
    }
}

impl Error for LabError {}

impl From<serde_json::Error> for LabError {
    fn from(error: serde_json::Error) -> Self {
        Self::Parse(error.to_string())
    }
}

pub type LabResult<T> = Result<T, LabError>;
