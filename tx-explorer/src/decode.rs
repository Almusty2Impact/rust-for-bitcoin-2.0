//! Decode raw transaction bytes into structured data.

use bitcoin::consensus::Decodable;
use bitcoin::transaction::{Transaction, TxIn};
use bitcoin::Amount;

use serde::Serialize;

use crate::error::{ExplorerError, ExplorerResult};

/// A fully decoded transaction with all fields exposed.
#[derive(Debug, Clone, Serialize)]
pub struct DecodedTransaction {
    /// The transaction ID (double-SHA256 of the serialized tx, byte-reversed).
    pub txid: String,
    /// The wtxid (witness-serialised hash, byte-reversed).
    pub wtxid: String,
    /// Transaction version (1 or 2).
    pub version: i32,
    /// Whether this transaction has witness data (SegWit).
    pub has_witness: bool,
    /// Decoded inputs.
    pub inputs: Vec<DecodedInput>,
    /// Decoded outputs.
    pub outputs: Vec<DecodedOutput>,
    /// Lock time.
    pub locktime: u32,
    /// Total serialized size in bytes (non-witness).
    pub size: usize,
    /// Total serialized size including witness data.
    pub total_size: usize,
    /// BIP141 weight.
    pub weight: usize,
    /// Virtual size (ceil(weight / 4)).
    pub vsize: usize,
    /// Whether any input signals RBF (BIP125).
    pub signals_rbf: bool,
    /// Whether this is a coinbase transaction.
    pub is_coinbase: bool,
}

/// A decoded transaction input.
#[derive(Debug, Clone, Serialize)]
pub struct DecodedInput {
    /// Previous transaction ID.
    pub prev_txid: String,
    /// Previous output index.
    pub prev_vout: u32,
    /// ScriptSig hex (empty for SegWit).
    pub script_sig_hex: String,
    /// ScriptSig disassembled opcodes.
    pub script_sig_asm: String,
    /// Witness items as hex strings.
    pub witness: Vec<String>,
    /// Sequence number.
    pub sequence: u32,
    /// Whether this input signals Replace-By-Fee.
    pub signals_rbf: bool,
}

/// A decoded transaction output.
#[derive(Debug, Clone, Serialize)]
pub struct DecodedOutput {
    /// Output index.
    pub index: usize,
    /// Value in satoshis.
    pub value_sats: u64,
    /// Value as a formatted BTC string.
    pub value_btc: String,
    /// scriptPubKey hex.
    pub script_pubkey_hex: String,
    /// scriptPubKey disassembled opcodes.
    pub script_pubkey_asm: String,
}

/// Decode a raw hex string into a Transaction.
pub fn decode_hex(raw_hex: &str) -> ExplorerResult<Transaction> {
    let bytes =
        hex::decode(raw_hex.trim()).map_err(|e| ExplorerError::InvalidHex(e.to_string()))?;
    decode_bytes(&bytes)
}

/// Decode raw bytes into a Transaction.
pub fn decode_bytes(bytes: &[u8]) -> ExplorerResult<Transaction> {
    Transaction::consensus_decode(&mut &bytes[..])
        .map_err(|e| ExplorerError::DecodeFailed(e.to_string()))
}

/// Build a fully decoded transaction report from a Transaction.
pub fn build_decoded_tx(tx: &Transaction) -> DecodedTransaction {
    let txid = tx.compute_txid().to_string();
    let wtxid = tx.compute_wtxid().to_string();

    let is_coinbase = tx.is_coinbase();

    // Serialize to compute sizes
    let total_size = tx.total_size();
    let weight = tx.weight();
    let vsize = tx.vsize();
    // stripped_size = total_size - witness_size
    // weight = stripped_size * 3 + total_size
    // => stripped_size = (weight - total_size) / 3
    let stripped_size = (weight.to_wu() as usize - total_size) / 3;

    let has_witness = tx.input.iter().any(|inp| !inp.witness.is_empty());

    let signals_rbf = tx.input.iter().any(|inp| inp.sequence.is_rbf());

    let inputs: Vec<DecodedInput> = tx
        .input
        .iter()
        .map(|inp: &TxIn| {
            let script_sig_asm = inp.script_sig.to_asm_string();
            let witness: Vec<String> = inp.witness.iter().map(hex::encode).collect();

            DecodedInput {
                prev_txid: inp.previous_output.txid.to_string(),
                prev_vout: inp.previous_output.vout,
                script_sig_hex: inp.script_sig.to_hex_string(),
                script_sig_asm,
                witness,
                sequence: inp.sequence.to_consensus_u32(),
                signals_rbf: inp.sequence.is_rbf(),
            }
        })
        .collect();

    let outputs: Vec<DecodedOutput> = tx
        .output
        .iter()
        .enumerate()
        .map(|(i, out)| {
            let value_btc = Amount::from_sat(out.value.to_sat()).to_btc().to_string();
            DecodedOutput {
                index: i,
                value_sats: out.value.to_sat(),
                value_btc,
                script_pubkey_hex: out.script_pubkey.to_hex_string(),
                script_pubkey_asm: out.script_pubkey.to_asm_string(),
            }
        })
        .collect();

    DecodedTransaction {
        txid,
        wtxid,
        version: tx.version.0,
        has_witness,
        inputs,
        outputs,
        locktime: tx.lock_time.to_consensus_u32(),
        size: stripped_size,
        total_size,
        weight: weight.to_wu() as usize,
        vsize,
        signals_rbf,
        is_coinbase,
    }
}
