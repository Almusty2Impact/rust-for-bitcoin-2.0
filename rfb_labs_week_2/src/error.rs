use std::fmt;

/// Expected failures produced by transaction validation and coin selection.
#[derive(Debug, PartialEq, Eq)]
pub enum TransactionError {
    NoInputs,
    NoOutputs,
    ZeroValueOutput,
    OutputsExceedInputs {
        total_inputs: u64,
        total_outputs: u64,
    },
    CoinbaseMixedWithRegularInputs,
    MultipleCoinbaseInputs,
    InvalidTxid,
    InsufficientFunds {
        available: u64,
        required: u64,
    },
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoInputs => write!(f, "transaction has no inputs"),
            Self::NoOutputs => write!(f, "transaction has no outputs"),
            Self::ZeroValueOutput => write!(
                f,
                "transaction contains a zero-value output (only OP_RETURN may have zero value)"
            ),
            Self::OutputsExceedInputs {
                total_inputs,
                total_outputs,
            } => write!(
                f,
                "outputs ({total_outputs} sats) exceed inputs ({total_inputs} sats)"
            ),
            Self::CoinbaseMixedWithRegularInputs => write!(
                f,
                "coinbase and regular inputs must not appear in the same transaction"
            ),
            Self::MultipleCoinbaseInputs => {
                write!(f, "a transaction may contain at most one coinbase input")
            }
            Self::InvalidTxid => write!(f, "regular input has an empty or invalid txid"),
            Self::InsufficientFunds {
                available,
                required,
            } => write!(
                f,
                "insufficient funds: have {available} sats but need {required} sats"
            ),
        }
    }
}

impl std::error::Error for TransactionError {}
