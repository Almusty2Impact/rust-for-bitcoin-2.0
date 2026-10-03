//! Transaction analysis: SegWit detection, RBF signalling, size/weight summary.

use serde::Serialize;

use crate::decode::DecodedTransaction;

/// Full transaction analysis summary.
#[derive(Debug, Clone, Serialize)]
pub struct TransactionAnalysis {
    /// Transaction ID.
    pub txid: String,
    /// Version.
    pub version: i32,
    /// Number of inputs.
    pub input_count: usize,
    /// Number of outputs.
    pub output_count: usize,
    /// Lock time.
    pub locktime: u32,
    /// Has witness data (SegWit).
    pub has_witness: bool,
    /// SegWit variant: None, Native, or Mixed (some inputs witness, some not).
    pub segwit_variant: String,
    /// Any input signals Replace-By-Fee.
    pub signals_rbf: bool,
    /// Is this a coinbase transaction?
    pub is_coinbase: bool,
    /// Stripped (non-witness) size in bytes.
    pub size: usize,
    /// Total serialized size in bytes.
    pub total_size: usize,
    /// BIP141 weight.
    pub weight: usize,
    /// Virtual size (ceil(weight / 4)).
    pub vsize: usize,
    /// Weight discount from SegWit (total_size - size).
    pub witness_discount: usize,
    /// Savings in vbytes compared to a non-SegWit equivalent.
    pub vsize_savings: usize,
}

/// Perform full analysis on a decoded transaction.
pub fn analyze(decoded: &DecodedTransaction) -> TransactionAnalysis {
    let segwit_variant = if !decoded.has_witness {
        "None".to_string()
    } else if decoded
        .inputs
        .iter()
        .all(|inp| inp.script_sig_hex.is_empty())
    {
        "Native".to_string()
    } else {
        "Mixed".to_string()
    };

    let witness_discount = decoded.total_size.saturating_sub(decoded.size);

    // Estimate vsize without SegWit: each input ~148 vB (P2PKH), each output ~34 vB
    // plus 10 vB overhead. With SegWit, P2WPKH input ~68 vB.
    // A rough estimate of savings: (total_size - vsize)
    let non_segwit_vsize = decoded.total_size;
    let vsize_savings = non_segwit_vsize.saturating_sub(decoded.vsize);

    TransactionAnalysis {
        txid: decoded.txid.clone(),
        version: decoded.version,
        input_count: decoded.inputs.len(),
        output_count: decoded.outputs.len(),
        locktime: decoded.locktime,
        has_witness: decoded.has_witness,
        segwit_variant,
        signals_rbf: decoded.signals_rbf,
        is_coinbase: decoded.is_coinbase,
        size: decoded.size,
        total_size: decoded.total_size,
        weight: decoded.weight,
        vsize: decoded.vsize,
        witness_discount,
        vsize_savings,
    }
}
