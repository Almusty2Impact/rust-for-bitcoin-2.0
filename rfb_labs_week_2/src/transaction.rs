use std::fmt;

use crate::error::TransactionError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputType {
    P2pkh,
    P2wpkh,
    P2tr,
    OpReturn,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TxOutput {
    pub value: u64,
    pub recipient: String,
    pub output_type: OutputType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct OutPoint {
    pub txid: String,
    pub vout: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputKind {
    Regular {
        previous_output: OutPoint,
        value: u64,
        sequence: u32,
    },
    Coinbase {
        block_height: u32,
        reward: u64,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Transaction {
    pub version: i32,
    pub inputs: Vec<InputKind>,
    pub outputs: Vec<TxOutput>,
    pub locktime: u32,
}

pub trait BitcoinValue {
    fn value(&self) -> u64;

    fn value_in_btc(&self) -> f64 {
        self.value() as f64 / 100_000_000.0
    }
}

impl Transaction {
    pub fn new(version: i32, locktime: u32) -> Self {
        Self {
            version,
            inputs: Vec::new(),
            outputs: Vec::new(),
            locktime,
        }
    }

    pub fn add_input(&mut self, input: InputKind) {
        self.inputs.push(input);
    }

    pub fn add_output(&mut self, output: TxOutput) {
        self.outputs.push(output);
    }

    pub fn total_input_value(&self) -> u64 {
        self.inputs
            .iter()
            .map(|input| match input {
                InputKind::Regular { value, .. } => *value,
                InputKind::Coinbase { reward, .. } => *reward,
            })
            .sum()
    }

    pub fn total_output_value(&self) -> u64 {
        self.outputs.iter().map(|o| o.value).sum()
    }

    pub fn fee(&self) -> Result<u64, TransactionError> {
        let inputs = self.total_input_value();
        let outputs = self.total_output_value();
        inputs
            .checked_sub(outputs)
            .ok_or(TransactionError::OutputsExceedInputs {
                total_inputs: inputs,
                total_outputs: outputs,
            })
    }

    pub fn validate(&self) -> Result<(), TransactionError> {
        // No inputs
        if self.inputs.is_empty() {
            return Err(TransactionError::NoInputs);
        }

        // No outputs
        if self.outputs.is_empty() {
            return Err(TransactionError::NoOutputs);
        }

        // Zero-value outputs (except OP_RETURN)
        for output in &self.outputs {
            if output.value == 0 && output.output_type != OutputType::OpReturn {
                return Err(TransactionError::ZeroValueOutput);
            }
        }

        // Outputs exceed inputs
        let total_in = self.total_input_value();
        let total_out = self.total_output_value();
        if total_out > total_in {
            return Err(TransactionError::OutputsExceedInputs {
                total_inputs: total_in,
                total_outputs: total_out,
            });
        }

        // Classify inputs
        let mut has_regular = false;
        let mut coinbase_count = 0;
        for input in &self.inputs {
            match input {
                InputKind::Regular {
                    previous_output, ..
                } => {
                    has_regular = true;
                    if previous_output.txid.is_empty() {
                        return Err(TransactionError::InvalidTxid);
                    }
                }
                InputKind::Coinbase { .. } => {
                    coinbase_count += 1;
                }
            }
        }

        // Mixed coinbase and regular
        if has_regular && coinbase_count > 0 {
            return Err(TransactionError::CoinbaseMixedWithRegularInputs);
        }

        // Multiple coinbase inputs
        if coinbase_count > 1 {
            return Err(TransactionError::MultipleCoinbaseInputs);
        }

        Ok(())
    }
}

impl BitcoinValue for TxOutput {
    fn value(&self) -> u64 {
        self.value
    }
}

impl BitcoinValue for InputKind {
    fn value(&self) -> u64 {
        match self {
            InputKind::Regular { value, .. } => *value,
            InputKind::Coinbase { reward, .. } => *reward,
        }
    }
}

pub fn highest_value_output(transaction: &Transaction) -> Option<&TxOutput> {
    transaction.outputs.iter().max_by_key(|o| o.value)
}

pub fn find_outputs_for_recipient<'a>(
    transaction: &'a Transaction,
    recipient: &str,
) -> Vec<&'a TxOutput> {
    transaction
        .outputs
        .iter()
        .filter(|o| o.recipient == recipient)
        .collect()
}

impl fmt::Display for OutPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.txid, self.vout)
    }
}

impl fmt::Display for TxOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} sats -> {} ({:?})",
            self.value, self.recipient, self.output_type
        )
    }
}

impl fmt::Display for InputKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputKind::Regular {
                previous_output,
                value,
                sequence,
            } => write!(
                f,
                "Regular({}) for {} sats (seq={})",
                previous_output, value, sequence
            ),
            InputKind::Coinbase {
                block_height,
                reward,
            } => write!(
                f,
                "Coinbase(height={}, reward={} sats)",
                block_height, reward
            ),
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Transaction(version={}, locktime={})",
            self.version, self.locktime
        )?;
        writeln!(f, "  {} inputs", self.inputs.len())?;
        for input in &self.inputs {
            writeln!(f, "    {input}")?;
        }
        writeln!(f, "  {} outputs", self.outputs.len())?;
        for output in &self.outputs {
            writeln!(f, "    {output}")?;
        }
        writeln!(f, "  total input:  {} sats", self.total_input_value())?;
        writeln!(f, "  total output: {} sats", self.total_output_value())?;
        match self.fee() {
            Ok(fee) => writeln!(f, "  fee:          {} sats", fee),
            Err(e) => writeln!(f, "  fee:          INVALID ({e})"),
        }
    }
}
