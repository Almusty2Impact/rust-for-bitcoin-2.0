//! Error types for the Transaction Explorer.

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error type returned by all explorer functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplorerError {
    /// The provided hex string is not valid hexadecimal.
    InvalidHex(String),
    /// The raw bytes could not be consensus-decoded as a transaction.
    DecodeFailed(String),
    /// An RPC call to Bitcoin Core failed.
    RpcError(String),
    /// The transaction was not found on the node.
    TransactionNotFound(String),
    /// A previous output needed for fee calculation was missing.
    PrevOutputMissing(String),
    /// The script is not a recognised standard type.
    UnknownScriptType(String),
    /// IO or configuration error.
    ConfigError(String),
}

impl Display for ExplorerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHex(msg) => write!(f, "invalid hex: {msg}"),
            Self::DecodeFailed(msg) => write!(f, "transaction decode failed: {msg}"),
            Self::RpcError(msg) => write!(f, "RPC error: {msg}"),
            Self::TransactionNotFound(msg) => write!(f, "transaction not found: {msg}"),
            Self::PrevOutputMissing(msg) => write!(f, "previous output missing: {msg}"),
            Self::UnknownScriptType(msg) => write!(f, "unknown script type: {msg}"),
            Self::ConfigError(msg) => write!(f, "configuration error: {msg}"),
        }
    }
}

impl Error for ExplorerError {}

pub type ExplorerResult<T> = Result<T, ExplorerError>;
