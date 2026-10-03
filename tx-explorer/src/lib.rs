//! Bitcoin Transaction Explorer — decode, analyze and inspect any transaction field by field.
//!
//! This library provides the core logic for decoding raw transactions, identifying
//! script types, deriving addresses, calculating fees and weight, detecting SegWit
//! and RBF signalling, and formatting the results as human-readable tables or JSON.

pub mod analysis;
pub mod decode;
pub mod error;
pub mod fees;
pub mod output;
pub mod rpc;
pub mod script;
pub mod mempool;

pub use error::{ExplorerError, ExplorerResult};
